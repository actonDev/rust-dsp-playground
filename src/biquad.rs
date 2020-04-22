pub struct Biquad {
    pub a0: f64,
    pub a1: f64,
    pub a2: f64,
    pub b1: f64,
    pub b2: f64,
}

pub const LOWPASS_FC_1000_Q_0_7071_GAIN_6: Biquad = Biquad {
    a0: 0.00460399444634034,
    a1: 0.00920798889268068,
    a2: 0.00460399444634034,
    b1: -1.7990948352036205,
    b2: 0.8175108129889816,
};

#[derive(Default)]
pub struct BiquadSamples {
    pub sin: f64,
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

pub fn process(biquad: &Biquad, samples: &mut BiquadSamples) -> f64 {
    let direct = samples.sin * biquad.a0;

    let forw_1 = samples.sin_1 * biquad.a1;
    let forw_2 = samples.sin_2 * biquad.a2;

    let bakw_1 = - samples.sout_1 * biquad.b1;
    let bakw_2 = - samples.sout_2 * biquad.b2;

    let out = direct + forw_1 + forw_2 + bakw_1 + bakw_2;

    // temp
    // let out = direct;
    
    samples.sin_2 = samples.sin_1;
    samples.sin_1 = samples.sin;

    samples.sout_2 = samples.sout_1;
    samples.sout_1 = out;
    out
}