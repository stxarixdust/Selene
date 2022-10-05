#[macro_use]
extern crate vst;

use vst::plugin::{Info, Plugin, Category};

#[derive(Default)]
struct Selene;

impl Plugin for Selene {
    fn get_info(&self) -> Info {
        Info {
            name: "Selene".to_string(),
            inputs: 0,
            unique_id: 1337,
            category: Category::Synth
            ..Default::default()
        }
    }
}

plugin_main!(Selene);
