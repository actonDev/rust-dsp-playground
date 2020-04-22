use dsp_playground::biquad;
use hound;
use std::i16;

fn main() {
    let mut reader = hound::WavReader::open("assets/white_noise_mono.wav").unwrap();
    let samples = reader.samples::<i16>();
    let mut samples_vec: Vec<i16> = Vec::new();

    let mut count = 0;
    {
        let sampels_vec2 = &mut samples_vec;
        samples.for_each(|s| {
            count += 1;
            sampels_vec2.push(s.unwrap());
        });
    }

    println!("count vec {}", samples_vec.len());

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("assets/write.wav", spec).unwrap();
    for s in &samples_vec {
        writer.write_sample((*s as f32 * 0.1) as i16).unwrap();
    }
    writer.finalize().unwrap();

    println!("count {}", count);

    filter_file(&samples_vec);
}

fn filter_file(samples: &Vec<i16>) {
    let biquad_params = biquad::LOWPASS_FC_1000_Q_0_7071_GAIN_6;
    let mut biquad_samples: biquad::BiquadSamples = Default::default();

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("assets/filtered.wav", spec).unwrap();

    for s in samples {
        let s_float = (*s as f64) / i16::MAX as f64;
        biquad_samples.sin = s_float;
        let sout = biquad::process(&biquad_params, &mut biquad_samples);
        let sout_int = (sout * (i16::MAX as f64)) as i16;
        // println!("s {} sfloat {}", s, s_float);
        // println!("  sout {} sout_int {}", sout, sout_int);
        writer.write_sample(sout_int).unwrap();
    }
}
