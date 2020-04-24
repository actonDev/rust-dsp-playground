#[macro_use]
extern crate vst;

use vst::buffer::AudioBuffer;
use vst::plugin::{Info, Plugin};

use dsp_playground::biquad;
use dsp_playground::filter;

#[derive(Default)]
struct BasicPlugin {
    // note: using options cause I haven't implemented the default yet
    filter_process: Option<biquad::Process>,
}

impl Plugin for BasicPlugin {
    fn init(&mut self) {
        let params = biquad::Params::from_audio_filter_params(
            filter::Params {
                fc: 500.0,
                q: 10.0,
                gain_db: 6.0,
            },
            filter::Type::LowPass,
            44100,
        );
        self.filter_process = Some(biquad::Process::new(params));
    }
    fn get_info(&self) -> Info {
        Info {
            name: "actondev DSP playground Basic Plugin vst 0.2.0".to_string(),
            vendor: "actondev".to_string(),
            unique_id: 1358, // Used by hosts to differentiate between plugins.

            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        // Option::as_mut(&self) : important :)
        let process: &mut biquad::Process = self.filter_process.as_mut().unwrap();
        // For each input and output
        for (input, output) in buffer.zip() {
            // For each input sample and output sample in buffer
            for (in_sample, out_sample) in input.into_iter().zip(output.into_iter()) {
                // *out_sample = *in_sample * 0.5;
                *out_sample = process.process(in_sample);
            }
        }
    }
}

plugin_main!(BasicPlugin); // Important!
