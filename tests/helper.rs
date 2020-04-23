//! Audio file helpers
//!
//!

use std::f64;

pub fn audio_file_samples(path: &str) -> Vec<i16> {
    let mut reader = hound::WavReader::open(path).unwrap();
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

    samples_vec
}

/**
 * Cross Correlation Index
 *
 * Shoul return 1 when the 2 vectors contain the same items.
 *
 * See https://github.com/actonDev/wavelet-denoiser/blob/master/src/metric-cci.py
 */
pub fn cross_correlation_index(s1: &Vec<i16>, s2: &Vec<i16>) -> f64 {
    if s1.len() != s2.len() {
        return 0.0;
    }

    let s1_mean = mean(s1);
    let s2_mean = mean(s2);

    let mut sum_diff_sq_1 = 0.0;
    let mut sum_diff_sq_2 = 0.0;

    let mut sum_nominator: f64 = 0.0;
    for it in s1.iter().zip(s2.iter()) {
        let (x1, x2) = it;
        // *bi = 2 * *ai;

        let diff_s1 = (*x1 as f64) - s1_mean;
        let diff_s2 = (*x2 as f64) - s2_mean;

        sum_nominator += diff_s1 * diff_s2;

        sum_diff_sq_1 += diff_s1.powi(2);
        sum_diff_sq_2 += diff_s2.powi(2);
        // sumDiffsBSquared += diffB * *2;

        // counter += 1;
    }

    let cii: f64 = sum_nominator / (sum_diff_sq_1 * sum_diff_sq_2).sqrt() as f64;

    return cii;
}

/**
 * Root Mean Square Error
 */
pub fn rmse(s1: &Vec<i16>, s2: &Vec<i16>) -> f64 {
    if s1.len() != s2.len() {
        return f64::MAX;
    }

    let mut sum_sq: f64 = 0.0;
    // let mut sum_mean_sq : f64 = 0.0;
    for it in s1.iter().zip(s2.iter()) {
        let (x1, x2) = it;
        // note: without converting to i32 I was getting multiply overflow error

        // sum_mea/n_sq += ((x1 - x2) as i32).pow(2) as f64 / s1.len() as f64;
        sum_sq += ((x1 - x2) as i32).pow(2) as f64;
        // println!("sum mean sq {}", sum_mean_sq);
    }

    // println!("sum_mean_sq res is {}", sum_mean_sq.sqrt());

    // sum.sqrt()

    let sum_sq_res = (sum_sq / s1.len() as f64).sqrt();
    // println!("sum res {}", sum_sq_res);

    sum_sq_res
    // (sum / s1.len() as f64 ).sqrt()
    // 4824.83864474248
}

pub fn mean(xs: &Vec<i16>) -> f64 {
    let mut sum: f64 = 0.0;
    for x in xs {
        sum += *x as f64;
    }

    sum as f64 / xs.len() as f64
}
