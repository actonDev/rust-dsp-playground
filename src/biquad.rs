use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub struct Biquad {
    a0: f64,
    a1: f64,
    a2: f64,
    b1: f64,
    b2: f64,
}

#[derive(Default)]
pub struct BiquadSamples {
    sout: f64,
    sin: f64,
    // past samples
    sin_1: f64,
    sin_2: f64,
    sout_1: f64,
    sout_2: f64,
}

impl Default for Biquad {
    fn default() -> Self {
        Biquad {
            a0: 1.0,
            a1: 0.0,
            a2: 0.0,
            b1: 0.0,
            b2: 0.0,
        }
    }
}

pub fn process(biquad: &Biquad, samples: &mut BiquadSamples){
    let direct = samples.sin * biquad.a0;
    let forw_1 = samples.sin_1 * biquad.a1;
    let forw_2 = samples.sin_2 * biquad.a2;
    let bakw_1 = - samples.sout_1 * biquad.b1;
    let bakw_2 = - samples.sout_2 * biquad.b2;

    let out = direct + forw_1 + forw_2 + bakw_1 + bakw_2;

    
    samples.sin_2 = samples.sin_1;
    samples.sin_1 = samples.sin;

    samples.sout_2 = samples.sout_1;
    samples.sout_1 = samples.sout;
    samples.sout = out;
}