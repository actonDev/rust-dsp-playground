#[macro_use]
extern crate more_asserts;

#[allow(unused_variables)]
mod common;
mod helper;

use dsp_playground::biquad;
use dsp_playground::filter;

const PATH_WHITE_NOISE: &str = "tests/assets/white_noise_mono.wav";
const PATH_SNAPSHOT_LOWPASS: &str = "tests/assets/snapshot_lowpass_fc_1000_Q_0.7071_gain_6.wav";

#[test]
fn self_rmse_is_0() {
    let white_noise: Vec<i16> = helper::audio_file_samples(PATH_WHITE_NOISE);
    let rmse = helper::rmse(&white_noise, &white_noise);
    assert_eq!(rmse, 0.0);
}

#[test]
fn not_self_rmse() {
    let white_noise: Vec<i16> = helper::audio_file_samples(PATH_WHITE_NOISE);
    let white_noise_filtered: Vec<i16> = helper::audio_file_samples(PATH_SNAPSHOT_LOWPASS);
    let rmse = helper::rmse(&white_noise, &white_noise_filtered);
    assert_gt!(rmse, 4_000.0);
}

#[test]
/**
 * Self correlation index should be 1
 *
 * This serves as a basis for our following tests
 */
fn self_cci_is_1() {
    let white_noise: Vec<i16> = helper::audio_file_samples(PATH_WHITE_NOISE);
    let cci = helper::cross_correlation_index(&white_noise, &white_noise);
    // println!("cci {}", cci);
    assert_eq!(cci, 1.0);
}

#[test]
fn not_self_cci_less_than_1() {
    let white_noise: Vec<i16> = helper::audio_file_samples(PATH_WHITE_NOISE);
    let white_noise_filtered: Vec<i16> = helper::audio_file_samples(PATH_SNAPSHOT_LOWPASS);
    let cci = helper::cross_correlation_index(&white_noise, &white_noise_filtered);
    // println!("cci {}", cci);
    assert_lt!(cci, 0.1);
}

#[test]
fn low_pass_snaphost() {
    let white_noise: Vec<i16> = helper::audio_file_samples(PATH_WHITE_NOISE);
    let white_noise_filtered_snapshot: Vec<i16> = helper::audio_file_samples(PATH_SNAPSHOT_LOWPASS);

    let biquad_params = biquad::LOWPASS_FC_1000_Q_0_7071_GAIN_6;
    let mut biquad_process = biquad::Process::new(biquad_params);

    let mut filtered: Vec<i16> = Vec::new();

    for s in &white_noise {
        let sout = biquad_process.process(s);
        filtered.push(sout);
    }

    let cci = helper::cross_correlation_index(&white_noise_filtered_snapshot, &filtered);
    assert_gt!(cci, 0.9999);

    let rmse = helper::rmse(&white_noise_filtered_snapshot, &filtered);
    assert_lt!(rmse, 1.0);
}
#[test]
fn write_low_pass_filtered_file() {
    common::cleanup_temp_files();
    let white_noise: Vec<i16> = helper::audio_file_samples(PATH_WHITE_NOISE);
    let white_noise_filtered_snapshot: Vec<i16> = helper::audio_file_samples(PATH_SNAPSHOT_LOWPASS);

    let biquad_params = biquad::LOWPASS_FC_1000_Q_0_7071_GAIN_6;
    let mut biquad_process = biquad::Process::new(biquad_params);

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let filtered_out_path = "tests/assets/temp_lowpass_fc_1000_Q_0.7071_gain_6.wav";
    let mut writer = hound::WavWriter::create(filtered_out_path, spec).unwrap();

    for s in &white_noise {
        let sout = biquad_process.process(s);
        writer.write_sample(sout).unwrap();
    }

    writer.finalize().unwrap();

    let filtered: Vec<i16> = helper::audio_file_samples(filtered_out_path);

    let cci = helper::cross_correlation_index(&white_noise_filtered_snapshot, &filtered);
    // println!("cci {}", cci);
    assert_gt!(cci, 0.9999);
}

#[test]
fn low_pass_params_1_000_hz() {
    let params: biquad::Params = biquad::Params::from_audio_filter_params(
        filter::Params {
            fc: 1_000.0,
            q: 0.7071,
            gain_db: 6.0,
        },
        filter::Type::LowPass,
        44100,
    );

    assert_eq!(params, biquad::LOWPASS_FC_1000_Q_0_7071_GAIN_6);
}

#[test]
fn low_pass_params_500_hz() {
    let params: biquad::Params = biquad::Params::from_audio_filter_params(
        filter::Params {
            fc: 500.0,
            q: 2.0,
            gain_db: 6.0,
        },
        filter::Type::LowPass,
        48000,
    );

    let expected = biquad::Params {
        a0: 0.0010533158426539336,
        a1: 0.002106631685307867,
        a2: 0.0010533158426539336,
        b1: -1.9636112661281218,
        b2: 0.9678245294987373,
    };
    assert_eq!(params, expected);
}

#[test]
fn high_pass() {
    let params: biquad::Params = biquad::Params::from_audio_filter_params(
        filter::Params {
            fc: 10_000.0,
            q: 0.7071,
            gain_db: 6.0,
        },
        filter::Type::HighPass,
        44100,
    );

    let expected = biquad::Params {
        a0: 0.33699935872014053,
        a1: -0.6739987174402811,
        a2: 0.33699935872014053,
        b1: -0.17124071441396285,
        b2: 0.1767567204665992,
    };
    assert_eq!(params, expected);
}

#[test]
fn high_pass2() {
    let params: biquad::Params = biquad::Params::from_audio_filter_params(
        filter::Params {
            fc: 500.0,
            q: 2.0,
            gain_db: 6.0,
        },
        filter::Type::HighPass,
        48000,
    );

    let expected = biquad::Params {
        a0: 0.9828589489067148,
        a1: -1.9657178978134295,
        a2: 0.9828589489067148,
        b1: -1.9636112661281218,
        b2: 0.9678245294987373,
    };
    assert_eq!(params, expected);
}

#[test]
fn band_pass() {
    let params: biquad::Params = biquad::Params::from_audio_filter_params(
        filter::Params {
            fc: 10_000.0,
            q: 0.7071,
            gain_db: 6.0,
        },
        filter::Type::BandPass,
        44100,
    );

    let expected = biquad::Params {
        a0: 0.4116216397667004,
        a1: 0.0,
        a2: -0.4116216397667004,
        b1: -0.17124071441396285,
        b2: 0.1767567204665992,
    };
    assert_eq!(params, expected);
}

#[test]
fn notch() {
    let params: biquad::Params = biquad::Params::from_audio_filter_params(
        filter::Params {
            fc: 10_000.0,
            q: 0.7071,
            gain_db: 6.0,
        },
        filter::Type::Notch,
        44100,
    );

    let expected = biquad::Params {
        a0: 0.5883783602332997,
        a1: -0.17124071441396285,
        a2: 0.5883783602332997,
        b1: -0.17124071441396285,
        b2: 0.1767567204665992,
    };
    assert_eq!(params, expected);
}

#[test]
fn peak_gain() {
    let params: biquad::Params = biquad::Params::from_audio_filter_params(
        filter::Params {
            fc: 10_000.0,
            q: 0.7071,
            gain_db: 6.0,
        },
        filter::Type::Peak,
        44100,
    );

    let expected = biquad::Params {
        a0: 1.4096715060854925,
        a1: -0.17124071441396285,
        a2: -0.23291478561889328,
        b1: -0.17124071441396285,
        b2: 0.1767567204665992,
    };
    assert_eq!(params, expected);
}

#[test]
fn peak_cut() {
    let params: biquad::Params = biquad::Params::from_audio_filter_params(
        filter::Params {
            fc: 10_000.0,
            q: 0.7071,
            gain_db: -6.0,
        },
        filter::Type::Peak,
        44100,
    );

    let expected = biquad::Params {
        a0: 0.7093851267355851,
        a1: -0.12147561589684117,
        a2: 0.12538858854956486,
        b1: -0.12147561589684117,
        b2: -0.1652262847148502,
    };
    assert_eq!(params, expected);
}

#[test]
fn low_shelf_gain() {
    let params: biquad::Params = biquad::Params::from_audio_filter_params(
        filter::Params {
            fc: 10_000.0,
            q: 0.7071,
            gain_db: 6.0,
        },
        filter::Type::LowShelf,
        44100,
    );

    let expected = biquad::Params {
        a0: 1.4199974570112823,
        a1: 0.32913667880445635,
        a2: 0.25714197787478704,
        b1: -0.17124139038429181,
        b2: 0.1767613656973212,
    };
    assert_eq!(params, expected);
}

#[test]
fn low_shelf_cut() {
    let params: biquad::Params = biquad::Params::from_audio_filter_params(
        filter::Params {
            fc: 10_000.0,
            q: 0.7071,
            gain_db: -6.0,
        },
        filter::Type::LowShelf,
        44100,
    );

    let expected = biquad::Params {
        a0: 0.7042266132678395,
        a1: -0.1205927444016058,
        a2: 0.12448005792162257,
        b1: 0.231786808616687,
        b2: 0.181086224207755,
    };
    assert_eq!(params, expected);
}

#[test]
fn high_shelf_gain() {
    let params: biquad::Params = biquad::Params::from_audio_filter_params(
        filter::Params {
            fc: 10_000.0,
            q: 0.7071,
            gain_db: 6.0,
        },
        filter::Type::HighShelf,
        44100,
    );

    let expected = biquad::Params {
        a0: 1.5052125083174621,
        a1: -0.8420495621854,
        a2: 0.342357029180967,
        b1: -0.17124139038429181,
        b2: 0.1767613656973212,
    };
    assert_eq!(params, expected);
}

#[test]
fn high_shelf_cut() {
    let params: biquad::Params = biquad::Params::from_audio_filter_params(
        filter::Params {
            fc: 10_000.0,
            q: 0.7071,
            gain_db: -6.0,
        },
        filter::Type::HighShelf,
        44100,
    );

    let expected = biquad::Params {
        a0: 0.6643580188672545,
        a1: -0.11376559086378224,
        a2: 0.11743283072694259,
        b1: -0.5594223789215313,
        b2: 0.22744763765194606,
    };
    assert_eq!(params, expected);
}