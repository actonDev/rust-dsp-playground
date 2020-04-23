pub struct Params {
    pub fc: f64, // frequency cut off
    pub q: f64, // resonance
    pub gain_db: f64, // peak gain (at fc)
}

pub enum Type {
    LowPass,
}