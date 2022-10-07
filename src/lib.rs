mod voice;
mod processing;

#[macro_use]
extern crate vst;

use std::collections::VecDeque;
use processing::*;
use vst::prelude::*;
use vst::plugin::{Info, Plugin, Category};

const MAX_VOICE_COUNT: u8 = 16;

#[derive(Default)]
struct SeleneProcessor {
    osc_a: processing::oscillators::sinebank::Sinebank,
}

#[derive(Default)]
struct Selene {
    sample_rate: f64,
    voices: VecDeque<voice::Voice>,
    processor: SeleneProcessor
}

#[allow(dead_code)]
impl Selene {
    fn process_midi_event(&mut self, data: [u8; 3]) {
        match data {
            [128, _, _] => self.note_off(data),
            [144, 0, _] => self.note_off(data),
            [144, _, _] => self.note_on(data),
            _ => (),
        }
    }
    
    fn note_on(&mut self, data: [u8; 3]) {
        self.voices.push_back(voice::Voice::new_with_velocity(
            data[1],
            data[2])
        );
        if self.voices.len() > MAX_VOICE_COUNT as usize {
            self.voices.pop_front();
        }
    }

    fn note_off(&mut self, data: [u8; 3]) {
        let mut i = 0u8;
        for v in &self.voices {
            if v.note == data[1] {
                self.voices.remove(i.into());
                break;
            }
            i += 1;
        }
    }

    fn note_to_pitch(note: u8) -> f64 {
        const A4_MIDI: i8 = 69;
        const A4_HZ: f64 = 440.0;

        ((f64::from(note as i8 - A4_MIDI)) / 12.).exp2() * A4_HZ
    }
}

impl Plugin for Selene {
    fn new(_host: HostCallback) -> Self {
        Selene {
            sample_rate: 44100.0,
            voices: VecDeque::with_capacity(MAX_VOICE_COUNT.into()),
            processor: SeleneProcessor {
                osc_a: Sinebank::new() 
            }
        }
    }

    fn get_info(&self) -> Info {
        Info {
            name: "Selene".to_string(),
            vendor: "Starburst Audio".to_string(),
            inputs: 0,
            outputs: 2,
            unique_id: 1337,
            category: Category::Synth,
            ..Info::default()
        }
    }

    #[allow(unused_variables)]
    #[allow(clippy::single_match)]
    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(ev) => self.process_midi_event(ev.data),
                _ => (),
            }
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = f64::from(rate);
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        // Iterate over all voices
        let samples = buffer.samples();
        let (_, mut outputs) = buffer.split();
        let output_count = outputs.len();

        for sample_idx in 0..samples {
            for buf_idx in 0..output_count {
                let buff = outputs.get_mut(buf_idx);
                buff[sample_idx] = 0f32;
            }
        }

        // For each note played
        for (_, voice) in self.voices.iter_mut().enumerate() {
            self.processor.osc_a.process_buffer(
                buffer,
                voice.t,
                Selene::note_to_pitch(voice.note),
                self.sample_rate
            );
            // Tick up lifetime of voices by amount of samples played
            voice.tick(buffer.samples().try_into().unwrap());
        }
    }
}

plugin_main!(Selene);
