extern crate log;
extern crate rand;
extern crate statrs;

use rand::distributions::{Distribution, WeightedIndex};
use rand::Rng;
use rand_distr::Poisson;
use statrs::distribution::{
    Beta, ContinuousCDF, Discrete, NegativeBinomial, Normal, Poisson as PoissonDist,
};

pub fn random_discrete<T: Clone + std::fmt::Debug>(weighted_choices: Vec<(T, f32)>) -> T {
    let weighted_index: WeightedIndex<f32> =
        WeightedIndex::new(weighted_choices.iter().map(|(_, w)| *w))
            .expect(format!("{:?}", weighted_choices).as_str());

    let choices: Vec<&T> = weighted_choices.iter().map(|(s, _)| s).collect();
    let mut rng = rand::thread_rng();
    let choice = weighted_index.sample(&mut rng);
    choices[choice].clone()
}

pub fn random_bool(prob_true: f32) -> bool {
    if prob_true < 0.0 || prob_true > 1.0 {
        panic!("Probability must be between 0.0 and 1.0");
    }
    let mut rng = rand::thread_rng();
    rng.gen::<f32>() < prob_true
}

pub fn sigmoid_prob(z: f32) -> f32 {
    1.0 / (1.0 + (-z).exp())
}
pub fn random_sigmoid(z: f32) -> bool {
    let prob_true = sigmoid_prob(z);
    random_bool(prob_true)
}

pub fn poisson_pmf(lambda: f32, x: u8) -> f32 {
    let poisson = PoissonDist::new(lambda as f64).unwrap();
    poisson.pmf(x as u64) as f32
}

pub fn normal_cdf(mean: f32, stdev: f32, x: f32) -> f32 {
    let normal = Normal::new(mean as f64, stdev as f64).unwrap();
    normal.cdf(x as f64) as f32
}

pub fn random_poisson(lambda: f32) -> f32 {
    let poisson = Poisson::new(lambda).unwrap();
    let mut rng = rand::thread_rng();
    poisson.sample(&mut rng)
}

pub fn double_truncated_poisson(lambda: f32, min_inclusive: u8, max_exclusive: u8) -> u8 {
    // if min_inclusive > max_exclusive {
    //     panic!(
    //         "min_inclusive must be less than max_exclusive {} {}",
    //         min_inclusive, max_exclusive
    //     );
    // }
    if min_inclusive == max_exclusive {
        return min_inclusive;
    }
    let pdf_func = |x: u8| poisson_pmf(lambda, x);
    let choices = (min_inclusive..max_exclusive)
        .map(|x| (x, pdf_func(x)))
        .collect();
    random_discrete(choices)
}
pub fn truncated_poisson(lambda: f32, max_exclusive: u8) -> u8 {
    double_truncated_poisson(lambda, 0, max_exclusive)
}

fn get_negbinom(mean: f32, var: f32) -> NegativeBinomial {
    /*
    mu = r*(1-p) / p
    var = r*(1-p) / p^2

    mu / var = p
    var = mu / p

    p = mu / var
    r = mu * p / (1 - p)
    */
    // Define the parameters for the Negative Binomial distribution

    // r must be positive...
    if mean >= var {
        panic!("negbinom mean must be less than var");
    }
    let var = var.max(mean + 1.0);

    let p = mean / var;
    let r = mean * p / (1.0 - p);

    // log::info!(
    //     "negbinom: mean={:?}, stdev={:?}, r={:?}, p={:?}",
    //     mean,
    //     var.sqrt(),
    //     r,
    //     p
    // );

    // Create the Negative Binomial distribution
    NegativeBinomial::new(r as f64, p as f64).unwrap()
}

pub fn random_negbinom(mean: f32, var: f32) -> u8 {
    let neg_binom = get_negbinom(mean, var);
    let mut rng = rand::thread_rng();
    let sample = neg_binom.sample(&mut rng);

    // maybe a little unsafe but we will check for this downstream;
    // only used for yards, which will be under 100 always
    sample.min(i8::MAX as u64) as u8
}

pub fn negbinom_pmf(mean: f32, var: f32, x: u8) -> f32 {
    let neg_binom = get_negbinom(mean, var);
    neg_binom.pmf(x as u64) as f32
}

pub fn truncated_negbinom(mean: f32, var: f32, max_exclusive: u8) -> u8 {
    double_truncated_negbinom(mean, var, 0, max_exclusive)
}

pub fn double_truncated_negbinom(mean: f32, var: f32, min_inclusive: u8, max_exclusive: u8) -> u8 {
    if min_inclusive == max_exclusive {
        return min_inclusive;
    }
    let pdf_func = |x: u8| negbinom_pmf(mean, var, x);
    let choices = (min_inclusive..max_exclusive)
        .map(|x| (x, pdf_func(x)))
        .collect();
    random_discrete(choices)
}

pub fn sample_beta(shape_a: f32, shape_b: f32) -> f32 {
    let distr = Beta::new(shape_a as f64, shape_b as f64).unwrap();
    distr.sample(&mut rand::thread_rng()) as f32
}

pub fn get_linreg_slope_intercept(data: Vec<(f64, f64)>) -> (f64, f64) {
    let n = data.len() as f64;
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xy = 0.0;
    let mut sum_x2 = 0.0;

    for (x, y) in data {
        sum_x += x;
        sum_y += y;
        sum_xy += x * y;
        sum_x2 += x * x;
    }

    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
    let intercept = (sum_y - slope * sum_x) / n;

    (slope, intercept)
}
