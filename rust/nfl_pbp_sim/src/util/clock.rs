use std::time::Duration;

pub fn mins_secs(duration: Duration) -> (f32, f32) {
    let tot_secs = duration.as_secs_f32();
    let mins = tot_secs.div_euclid(60.0);
    let secs = tot_secs - 60.0 * mins;
    (mins, secs)
}
