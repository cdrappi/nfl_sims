use std::collections::HashMap;

pub fn probability_to_american_odds(p: f32) -> String {
    if (p <= 0.0) | (p >= 1.0) {
        return "N/A".to_string();
    }
    match p >= 0.5 {
        true => format!("-{:.0}", 100.0 * p / (1.0 - p)),
        false => format!("+{:.0}", 100.0 * (1.0 - p) / p),
    }
}

pub fn balanced_line<T: Into<i16> + Copy>(values: &Vec<T>, only_halves: bool) -> (f32, f32) {
    // log::info!("median = {}", values[values.len() / 2]);
    let mut cumsum: f32 = 0.0;
    let counts = count_occurrences(values);

    let mut best_value = 0.0;
    let mut best_diff = 1.0;
    let mut best_over_prob = 0.0;

    for (v, prob) in counts {
        if !only_halves {
            let int_prob_under = cumsum / (1.0 - prob);
            let int_prob_over = (1.0 - prob - cumsum) / (1.0 - prob);
            let int_diff = (int_prob_over - int_prob_under).abs();
            if int_diff < best_diff {
                best_value = v as f32;
                best_diff = int_diff;
                best_over_prob = int_prob_over;
            }
        }

        let half_prob_under = cumsum + prob;
        let half_prob_over = 1.0 - half_prob_under;
        let half_diff = (half_prob_over - half_prob_under).abs();
        if half_diff < best_diff {
            best_value = v as f32 + 0.5;
            best_diff = half_diff;
            best_over_prob = half_prob_over;
        }

        cumsum += prob as f32;
    }

    (best_value, best_over_prob)
}

pub fn make_yards_line(value: f32, prob: f32) -> String {
    if (prob <= 0.0) | (prob >= 1.0) | (value <= 0.5) {
        return String::from("");
    }
    format!("O {:.1} {}", value, probability_to_american_odds(prob))
}

fn count_occurrences<T: Into<i16> + Copy>(values: &Vec<T>) -> Vec<(i16, f32)> {
    let n = values.len() as f32;
    let mut counts = HashMap::new();

    for v in values {
        *counts.entry((*v).into()).or_insert(0.0) += 1.0 / n;
    }

    let mut sorted_counts: Vec<(i16, f32)> = counts.into_iter().collect();
    sorted_counts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    sorted_counts
}
