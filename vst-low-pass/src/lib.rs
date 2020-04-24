#[macro_use]
extern crate vst;

use vst::plugin::{Info, Plugin};
use vst::buffer::AudioBuffer;

#[derive(Default)]
struct BasicPlugin;

impl Plugin for BasicPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "actondev DSP playground Basic Plugin vst 0.2.0".to_string(),
            vendor: "actondev".to_string(),
            unique_id: 1358, // Used by hosts to differentiate between plugins.

            ..Default::default()
        }
    }

    // Processor that clips samples above 0.4 or below -0.4:
    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        // For each input and output
        for (input, output) in buffer.zip() {
            // For each input sample and output sample in buffer
            for (in_sample, out_sample) in input.into_iter().zip(output.into_iter()) {
                *out_sample = *in_sample * 0.5;
                // *out_sample = if *in_sample > 0.4 {
                //     0.4
                // } else if *in_sample < -0.4 {
                //     -0.4
                // } else {
                //     *in_sample
                // };
            }
        }
    }
}

plugin_main!(BasicPlugin); // Important!
