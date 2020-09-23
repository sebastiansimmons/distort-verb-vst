#[macro_use]
extern crate vst;

use vst::plugin::{Info, Plugin, Category, PluginParameters};
use vst::buffer::AudioBuffer;
use vst::util::AtomicFloat;
//use vst::event::Event;
//use vst::api::Events;
//use rand::random;
use std::sync::Arc;

//const THRESHOLD: f32 = 0.5;
#[derive(Default)]
struct DistortVerb {
    params: Arc<DistortVerbParameters>,
}

struct DistortVerbParameters {
    threshold: AtomicFloat,
}

impl Default for DistortVerbParameters {
    fn default() -> DistortVerbParameters {
        DistortVerbParameters {
            threshold: AtomicFloat::new(0.5),
        }
    }
}



impl Plugin for DistortVerb {
    fn get_info(&self) -> Info {
        Info {
            name: "Distort Verb".to_string(),

            vendor: "Sebastian Simmons".to_string(),

            unique_id: 54948926,

            // Set to 0 since this is a synth
            inputs: 2,

            outputs: 2,

            category: Category::Effect,

            parameters: 1,

            // Fill the rest with default values
            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {

        for (input_buffer, output_buffer) in buffer.zip() {
            for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {

                if *input_sample >= 0.0 {
                    *output_sample = input_sample.min(self.params.threshold.get()) / self.params.threshold.get();
                }
                else {
                    *output_sample = input_sample.max(-self.params.threshold.get()) / self.params.threshold.get();
                }

            }
        }

    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }
}

impl PluginParameters for DistortVerbParameters {
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.threshold.get(),
            _ => 0.0,
        }
    }

    fn set_parameter(&self, index: i32, value: f32) {
        match index {
            // We don't want to divide by zero, so we'll clamp the value
            0 => self.threshold.set(value.max(0.01)),
            _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Threshold".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            // Convert to a percentage
            0 => format!("{}", self.threshold.get() * 100.0),
            _ => "".to_string(),
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "%".to_string(),
            _ => "".to_string(),
        }
    }
}



plugin_main!(DistortVerb);
