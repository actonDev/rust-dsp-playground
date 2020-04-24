//! Biquad filters
//! 
//! Credits: https://www.earlevel.com/main/2012/11/26/biquad-c-source-code/

use crate::filter;
use std::f64::consts::PI;
use std::i16;

#[derive(std::cmp::PartialEq, std::fmt::Debug)]
pub struct Params {
    pub a0: f64,
    pub a1: f64,
    pub a2: f64,
    pub b1: f64,
    pub b2: f64,
}

impl Params {
    pub fn from_audio_filter_params(
        filter_params: filter::Params,
        filter_type: filter::Type,
        fs: i32,
    ) -> Params {
        match filter_type {
            filter::Type::LowPass => low_pass(filter_params, fs),
            filter::Type::HighPass => high_pass(filter_params, fs),
            filter::Type::BandPass => band_pass(filter_params, fs),
            filter::Type::Notch => notch(filter_params, fs),
            filter::Type::Peak => peak(filter_params, fs),
            filter::Type::LowShelf => low_shelf(filter_params, fs),
            filter::Type::HighShelf => high_shelf(filter_params, fs),
        }
    }
}

fn low_pass(filter_params: filter::Params, fs: i32) -> Params {
    let fc = filter_params.fc / fs as f64;
    let k = (PI * fc).tan();
    let q = filter_params.q;
    let norm = 1.0 / (1.0 + k / q + k * k);

    let a0 = k * k * norm;
    Params {
        a0: a0,
        a1: 2.0 * a0,
        a2: a0,
        b1: 2.0 * (k * k - 1.0) * norm,
        b2: (1.0 - k / q + k * k) * norm,
    }
}

fn high_pass(filter_params: filter::Params, fs: i32) -> Params {
    let fc = filter_params.fc / fs as f64;
    let k = (PI * fc).tan();
    let q = filter_params.q;
    let norm = 1.0 / (1.0 + k / q + k * k);

    let a0 = norm;
    Params {
        a0: a0,
        a1: -2.0 * a0,
        a2: a0,
        b1: 2.0 * (k * k - 1.0) * norm,
        b2: (1.0 - k / q + k * k) * norm,
    }
}

fn band_pass(filter_params: filter::Params, fs: i32) -> Params {
    let fc = filter_params.fc / fs as f64;
    let k = (PI * fc).tan();
    let q = filter_params.q;
    let norm = 1.0 / (1.0 + k / q + k * k);

    let a0 = k / q * norm;
    Params {
        a0: a0,
        a1: 0.0,
        a2: -a0,
        b1: 2.0 * (k * k - 1.0) * norm,
        b2: (1.0 - k / q + k * k) * norm,
    }
}

fn notch(filter_params: filter::Params, fs: i32) -> Params {
    let fc = filter_params.fc / fs as f64;
    let k = (PI * fc).tan();
    let q = filter_params.q;
    let norm = 1.0 / (1.0 + k / q + k * k);

    let a0 = (1.0 + k * k) * norm;
    let a1 = 2.0 * (k * k - 1.0) * norm;
    Params {
        a0: a0,
        a1: a1,
        a2: a0,
        b1: a1,
        b2: (1.0 - k / q + k * k) * norm,
    }
}

fn peak(filter_params: filter::Params, fs: i32) -> Params {
    let fc = filter_params.fc / fs as f64;
    let k = (PI * fc).tan();
    let q = filter_params.q;
    // let v = filter_params.gain_db;
    let v = 10.0f64.powf(filter_params.gain_db.abs() / 20.0);

    if filter_params.gain_db >= 0.0 {
        // boost
        let norm = 1.0 / (1.0 + 1.0 / q * k + k * k);
        let a0 = (1.0 + v / q * k + k * k) * norm;
        let a1 = 2.0 * (k * k - 1.0) * norm;
        let a2 = (1.0 - v / q * k + k * k) * norm;
        let b1 = a1;
        let b2 = (1.0 - 1.0 / q * k + k * k) * norm;
        Params { a0, a1, a2, b1, b2 }
    } else {
        // cut
        let norm = 1.0 / (1.0 + v/q * k + k * k);
        let a0 = (1.0 + 1.0/q * k + k * k) * norm;
        let a1 = 2.0 * (k * k - 1.0) * norm;
        let a2 = (1.0 - 1.0/q * k + k * k) * norm;
        let b1 = a1;
        let b2 = (1.0 - v/q * k + k * k) * norm;

        Params { a0, a1, a2, b1, b2 }
    }
}

fn low_shelf(filter_params: filter::Params, fs: i32) -> Params {
    let fc = filter_params.fc / fs as f64;
    let k = (PI * fc).tan();
    let v = 10.0f64.powf(filter_params.gain_db.abs() / 20.0);

    if filter_params.gain_db >= 0.0 {
        // boost
        let norm = 1.0 / (1.0 + 2f64.sqrt() * k + k * k);
        let a0 = (1.0 + (2f64*v).sqrt() * k + v * k * k) * norm;
        let a1 = 2.0 * (v * k * k - 1.0) * norm;
        let a2 = (1.0 - (2f64*v).sqrt() * k + v * k * k) * norm;
        let b1 = 2.0 * (k * k - 1.0) * norm;
        let b2 = (1.0 - 2f64.sqrt() * k + k * k) * norm;

        Params { a0, a1, a2, b1, b2 }
    } else {
        // cut
        let norm = 1.0 / (1.0 + (2f64*v).sqrt() * k + v * k * k);
        let a0 = (1.0 + 2f64.sqrt() * k + k * k) * norm;
        let a1 = 2.0 * (k * k - 1.0) * norm;
        let a2 = (1.0 - 2f64.sqrt() * k + k * k) * norm;
        let b1 = 2.0 * (v * k * k - 1.0) * norm;
        let b2 = (1.0 - (2f64*v).sqrt() * k + v * k * k) * norm;

        Params { a0, a1, a2, b1, b2 }
    }
}

fn high_shelf(filter_params: filter::Params, fs: i32) -> Params {
    let fc = filter_params.fc / fs as f64;
    let k = (PI * fc).tan();
    let v = 10.0f64.powf(filter_params.gain_db.abs() / 20.0);

    if filter_params.gain_db >= 0.0 {
        // boost
        let norm = 1.0 / (1.0 + 2f64.sqrt() * k + k * k);
        let a0 = (v + (2f64*v).sqrt() * k + k * k) * norm;
        let a1 = 2.0 * (k * k - v) * norm;
        let a2 = (v - (2f64*v).sqrt() * k + k * k) * norm;
        let b1 = 2.0 * (k * k - 1.0) * norm;
        let b2 = (1.0 - 2f64.sqrt() * k + k * k) * norm;

        Params { a0, a1, a2, b1, b2 }
    } else {
        // cut
        let norm = 1.0 / (v + (2f64*v).sqrt() * k + k * k);
        let a0 = (1.0 + 2f64.sqrt() * k + k * k) * norm;
        let a1 = 2.0 * (k * k - 1.0) * norm;
        let a2 = (1.0 - 2f64.sqrt() * k + k * k) * norm;
        let b1 = 2.0 * (k * k - v) * norm;
        let b2 = (v - (2f64*v).sqrt() * k + k * k) * norm;

        Params { a0, a1, a2, b1, b2 }
    }
}

pub const LOWPASS_FC_1000_Q_0_7071_GAIN_6: Params = Params {
    a0: 0.00460399444634034,
    a1: 0.00920798889268068,
    a2: 0.00460399444634034,
    b1: -1.7990948352036205,
    b2: 0.8175108129889816,
};

#[derive(Default)]
struct Samples {
    sin: f64,
    // past samples
    sin_1: f64,
    sin_2: f64,
    sout_1: f64,
    sout_2: f64,
}

pub struct Process {
    pub params: Params,
    samples: Samples,
}

impl Process {
    pub fn new(params: Params) -> Self {
        Self {
            params: params,
            samples: Samples::default(),
        }
    }
}

impl Default for Params {
    fn default() -> Self {
        Params {
            a0: 1.0,
            a1: 0.0,
            a2: 0.0,
            b1: 0.0,
            b2: 0.0,
        }
    }
}

/**
 * Representing samples in float from -1.0 to 1.0 range
 *
 * Internal representation is in f64.
 * For example, for an int8 range that goes from -128 to 127
 * - -127 will give -1.0
 * - +127 will give +1.0
 */
pub trait FloatOfMax1<T> {
    fn to_f64(&self) -> f64;
    fn from_f64(&self, x: f64) -> T;
}

impl FloatOfMax1<f64> for f64 {
    fn to_f64(&self) -> f64 {
        *self
    }

    fn from_f64(&self, x: f64) -> f64 {
        x
    }
}

impl FloatOfMax1<f32> for f32 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }

    fn from_f64(&self, x: f64) -> f32 {
        x as f32
    }
}

impl FloatOfMax1<i16> for i16 {
    fn to_f64(&self) -> f64 {
        (*self as f64) / i16::MAX as f64
    }

    fn from_f64(&self, x: f64) -> i16 {
        (x * (i16::MAX as f64)) as i16
    }
}

impl Process {
    /**
     * Processing one sample
     *
     * Input sample can be i16 or f64
     * TODO: use type 2? read that it's better for floating point calculations
     * see https://www.earlevel.com/main/2003/02/28/biquads/
     * <pre>direct form I is usually the best choice for fixed point, and transposed direct form II for floating point.</pre>
     */
    pub fn process<T>(&mut self, sample: &dyn FloatOfMax1<T>) -> T {
        let samples = &mut self.samples;
        let params = &self.params;
        samples.sin = sample.to_f64();

        // biquad calculation
        let direct = samples.sin * params.a0;

        let forw_1 = samples.sin_1 * params.a1;
        let forw_2 = samples.sin_2 * params.a2;

        let bakw_1 = -samples.sout_1 * params.b1;
        let bakw_2 = -samples.sout_2 * params.b2;

        let out = direct + forw_1 + forw_2 + bakw_1 + bakw_2;

        // filling the past samples
        samples.sin_2 = samples.sin_1;
        samples.sin_1 = samples.sin;

        samples.sout_2 = samples.sout_1;
        samples.sout_1 = out;

        // the sample is not really used here.. but we need to just for it's type
        sample.from_f64(out)
    }
}
