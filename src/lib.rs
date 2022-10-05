#[macro_use]
extern crate vst;

use vst::prelude::*;
use vst::plugin::{Info, Plugin, Category};

#[derive(Default)]
struct Selene;

impl Plugin for Selene {
    fn new(_host: HostCallback) -> Self {
        Selene
    }
    fn get_info(&self) -> Info {
        Info {
            name: "Selene".to_string(),
            inputs: 0,
            outputs: 2,
            unique_id: 1337,
            category: Category::Synth,
            ..Default::default()
        }
    }
}

plugin_main!(Selene);
