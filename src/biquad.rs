pub struct Params {
    pub a0: f64,
    pub a1: f64,
    pub a2: f64,
    pub b1: f64,
    pub b2: f64,
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

impl Process {
    // processing one sample
    pub fn process(&mut self, sample: f64) -> f64 {
        let samples = &mut self.samples;
        let params = &self.params;
        samples.sin = sample;
        let direct = samples.sin * params.a0;

        let forw_1 = samples.sin_1 * params.a1;
        let forw_2 = samples.sin_2 * params.a2;

        let bakw_1 = -samples.sout_1 * params.b1;
        let bakw_2 = -samples.sout_2 * params.b2;

        let out = direct + forw_1 + forw_2 + bakw_1 + bakw_2;

        samples.sin_2 = samples.sin_1;
        samples.sin_1 = samples.sin;

        samples.sout_2 = samples.sout_1;
        samples.sout_1 = out;
        out
    }
}
