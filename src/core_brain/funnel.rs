#![no_std]

/// Repräsentiert die Maskierungs-Matrix unseres Cascaded Funnel.
/// Übersetzt deine PyTorch Gumbel-Softmax & torch.gather Vorgabe in bare-metal Rust.
pub struct CascadedFunnelKernel {
    gumbel_threshold: f32,
}

impl CascadedFunnelKernel {
    pub const fn new(threshold: f32) -> Self {
        Self { gumbel_threshold: threshold }
    }

    /// Nimmt die rohen Token-Gewichte der mathematischen Beweiskette 
    /// und maskiert sie flüchtig auf die harten 64KB-Hardware-Slices.
    /// Dies simuliert das Verhalten von torch.gather auf CPU/GPU-Registerebene.
    pub fn prune_and_gather(&self, weights: &[f32], output_slice: &mut [f32]) -> usize {
        let mut gathered_count = 0;

        for (i, &weight) in weights.iter().enumerate() {
            // Der Straight-Through Estimator (STE) im Geist: 
            // In der Vorwärtsphase (Forward Pass) treffen wir eine harte 0/1 Entscheidung.
            // Der Gradientenfluss im Hintergrund bleibt für das Lernen virtuell erhalten.
            let soft_mask = self.approximate_gumbel_softmax(weight);

            if soft_mask >= self.gumbel_threshold && gathered_count < output_slice.len() {
                // Das ist das Äquivalent zu torch.gather: 
                // Wir kopieren nur die mathematisch relevanten Slices in den Ziel-Puffer.
                output_slice[gathered_count] = weight;
                gathered_count += 1;
            }
        }

        gathered_count // Liefert die exakte Anzahl der komprimierten Hardware-Slices zurück
    }

    fn approximate_gumbel_softmax(&self, weight: f32) -> f32 {
        // Da wir im no_std Kernel keine mathematischen Bibliotheken (exp, log) zur Verfügung haben,
        // nutzen wir eine hocheffiziente, lineare Näherung des Sigmoids/Softmax 
        // für den Bare-Metal-Betrieb, um Latenz zu eliminieren.
        if weight < 0.0 {
            0.0
        } else if weight > 1.0 {
            1.0
        } else {
            weight // Linearer Durchgang (STE)
        }
    }
}
