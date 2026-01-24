//! First-order IIR allpass filter.

use core::f32::consts::PI;

use dsp_analyze::*;

/// Sample rate in Hz.
const SAMPLE_RATE: f32 = 48000.0;

/// Block size in samples.
const BLOCK_SIZE: usize = 16;

/// First-order allpass filter.
#[derive(Debug, Default)]
pub struct Allpass {
    /// Coefficient.
    a0: f32,

    /// Last state.
    z: f32,
}

impl Allpass {
    /// Returns a new filter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the normalized frequency.
    pub fn set_frequency(&mut self, freq: f32) {
        let k = (freq * PI).tan();
        self.a0 = (k - 1.0) / (k + 1.0);
    }

    /// Processes the next sample and returns the result.
    #[inline(always)]
    pub fn process(&mut self, input: f32) -> f32 {
        let v = input - self.a0 * self.z;
        let output = self.a0 * v + self.z;
        self.z = v;

        output
    }
}

#[test]
fn allpass() {
    let mut allpass = Allpass::new();
    allpass.set_frequency(1000.0 / SAMPLE_RATE);

    let mut analyzer = FftAnalyzer::new(FftAnalyzerConfig {
        block_size: BLOCK_SIZE,
        ..Default::default()
    });
    analyzer.run(|_, out_samples| {
        for sample in out_samples.iter_mut() {
            *sample = allpass.process(*sample);
        }
    });
    analyzer.plot_magnitude("Allpass 1kHz", "out/allpass/allpass_1k_mag.svg");
    analyzer.plot_phase("Allpass 1kHz", "out/allpass/allpass_1k_phase.svg");
    analyzer.save_output("out/allpass/allpass_1k.wav");
}
