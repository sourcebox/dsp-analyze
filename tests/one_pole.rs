//! One pole filter.

use core::f32::consts::TAU;

use dsp_analyze::*;

/// Sample rate in Hz.
const SAMPLE_RATE: f32 = 48000.0;

/// Block size in samples.
const BLOCK_SIZE: usize = 16;

/// One pole filter.
#[derive(Debug)]
pub struct OnePole {
    /// Coefficient a0.
    a0: f32,

    /// Coefficient b1.
    b1: f32,

    /// Last state.
    z: f32,
}

impl OnePole {
    /// Returns a new filter with a normalized frequency.
    pub fn new(frequency: f32) -> Self {
        let b1 = f32::exp(-TAU * frequency);
        let a0 = 1.0 - b1;

        Self { a0, b1, z: 0.0 }
    }

    /// Processes the next sample and returns the result.
    pub fn process(&mut self, input: f32) -> f32 {
        self.z = input * self.a0 + self.z * self.b1;

        self.z
    }

    /// Runs a highpass filter on a sample, e.g. for dc blocking.
    pub fn process_highpass(&mut self, inout: &mut f32) {
        *inout -= self.process(*inout);
    }

    /// Sets a new normalized frequency.
    pub fn set_frequency(&mut self, frequency: f32) {
        self.b1 = f32::exp(-TAU * frequency);
        self.a0 = 1.0 - self.b1;
    }
}

#[test]
fn one_pole_lp() {
    let mut one_pole = OnePole::new(1000.0 / SAMPLE_RATE);

    let mut analyzer = FftAnalyzer::new(FftAnalyzerConfig {
        block_size: BLOCK_SIZE,
        ..Default::default()
    });
    analyzer.run(|_, out_samples| {
        for sample in out_samples.iter_mut() {
            *sample = one_pole.process(*sample);
        }
    });
    analyzer.plot_magnitude("One-pole LP 1kHz", "out/one_pole/one_pole_lp_1k_mag.svg");
    analyzer.plot_phase("One-pole LP 1kHz", "out/one_pole/one_pole_lp_1k_phase.svg");
    analyzer.save_output("out/one_pole/one_pole_lp_1k.wav");
}

#[test]
fn one_pole_hp() {
    let mut one_pole = OnePole::new(1000.0 / SAMPLE_RATE);

    let mut analyzer = FftAnalyzer::new(FftAnalyzerConfig {
        block_size: BLOCK_SIZE,
        ..Default::default()
    });
    analyzer.run(|_, out_samples| {
        for sample in out_samples.iter_mut() {
            one_pole.process_highpass(sample);
        }
    });
    analyzer.plot_magnitude("One-pole HP 1kHz", "out/one_pole/one_pole_hp_1k_mag.svg");
    analyzer.plot_phase("One-pole HP 1kHz", "out/one_pole/one_pole_hp_1k_phase.svg");
    analyzer.save_output("out/one_pole/one_pole_hp_1k.wav");
}
