use std::thread::current;

use tokio::time::{Duration, Instant};

pub struct ResettableBucket {
    bucket_size: usize,
    bucket_interval: Duration,
    current_bucket_size: usize,
    last_reset: Instant,
}

impl ResettableBucket {
    pub fn new(bucket_size: usize, bucket_interval: Duration) -> Self {
        Self {
            bucket_size,
            bucket_interval,
            last_reset: Instant::now(),
            current_bucket_size: bucket_size,
        }
    }

    pub fn try_take(&mut self, current_instant: &Instant) -> Option<Duration> {
        if current_instant.duration_since(self.last_reset) >= self.bucket_interval {
            self.current_bucket_size = self.bucket_size;
            self.last_reset = Instant::now();
        }

        if self.current_bucket_size > 0 {
            self.current_bucket_size -= 1;

            None
        } else {
            Some((self.last_reset + self.bucket_interval) - *current_instant)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mailer::{DAY_IN_SECONDS, HOUR_IN_SECONDS, MINUTE_IN_SECONDS};
    use tokio::time::{Duration, Instant};

    fn get_one_per_period_test_result(period: &Duration) -> (bool, bool, bool) {
        let mut bucket = ResettableBucket::new(1, *period);
        let mut current_instant = Instant::now();
        let first_take_out = bucket.try_take(&current_instant).is_none();
        let next_successive_take_out = bucket.try_take(&current_instant).is_none();
        current_instant = current_instant.checked_add(*period).unwrap();
        let next_period_take_out = bucket.try_take(&current_instant).is_none();

        (first_take_out, next_successive_take_out, next_period_take_out)
    }

    #[test]
    fn test_return_false_on_exhausted_second() {
        let one_second = Duration::from_secs(1);
        let (first_take_out, next_successive_take_out, next_period_take_out) =
            get_one_per_period_test_result(&one_second);

        assert_eq!(first_take_out, true);
        assert_eq!(next_successive_take_out, false);
        assert_eq!(next_period_take_out, true);
    }

    #[test]
    fn test_return_false_on_exhausted_minute() {
        let one_minute = Duration::from_secs(MINUTE_IN_SECONDS);
        let (first_take_out, next_successive_take_out, next_period_take_out) =
            get_one_per_period_test_result(&one_minute);

        assert_eq!(first_take_out, true);
        assert_eq!(next_successive_take_out, false);
        assert_eq!(next_period_take_out, true);
    }

    #[test]
    fn test_return_false_on_exhausted_hour() {
        let one_hour = Duration::from_secs(HOUR_IN_SECONDS);
        let (first_take_out, next_successive_take_out, next_period_take_out) =
            get_one_per_period_test_result(&one_hour);

        assert_eq!(first_take_out, true);
        assert_eq!(next_successive_take_out, false);
        assert_eq!(next_period_take_out, true);
    }

    #[test]
    fn test_return_false_on_exhausted_day() {
        let one_day = Duration::from_secs(DAY_IN_SECONDS);
        let (first_take_out, next_successive_take_out, next_period_take_out) =
            get_one_per_period_test_result(&one_day);

        assert_eq!(first_take_out, true);
        assert_eq!(next_successive_take_out, false);
        assert_eq!(next_period_take_out, true);
    }
}
