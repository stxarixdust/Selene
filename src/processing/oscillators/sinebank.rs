use std::f64::consts::TAU;

use rand::random;
use vst::prelude::AudioBuffer;

#[derive(Default)]
pub struct Sinebank {
    amplitudes: [f32; 32]
}

impl Sinebank {
    pub fn new() -> Self {
        Sinebank {
            amplitudes: [0f32; 32]
        }
    }

    pub fn process_buffer (
        &mut self,
        buffer: &mut AudioBuffer<f32>,
        t: u32,
        pitch: f64,
        samplerate: f64
    ) {
        let samples = buffer.samples();
        let (_, mut outputs) = buffer.split();
        let output_count = outputs.len();

        for sample_idx in 0..samples {
            for buf_idx in 0..output_count {
                let buff = outputs.get_mut(buf_idx);
                let x = (t + sample_idx as u32) as f64;
                buff[sample_idx] =
                    ((x * pitch * TAU / samplerate) as f32).sin();
            }
        }
    }
}