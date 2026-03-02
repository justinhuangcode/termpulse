//! ETA and throughput estimation.
//!
//! Uses exponential moving average (EMA) for smooth, responsive estimates
//! that adapt to changing speeds.

use std::time::{Duration, Instant};

/// ETA and throughput estimator using exponential moving average.
#[derive(Debug)]
pub struct Estimator {
    start_time: Instant,
    last_update: Instant,
    last_percent: f64,
    /// Smoothed rate of progress (percent per second).
    smoothed_rate: Option<f64>,
    /// EMA smoothing factor (0..1). Higher = more responsive, lower = smoother.
    alpha: f64,
}

impl Estimator {
    /// Create a new estimator.
    ///
    /// `alpha` controls smoothing (default 0.3):
    /// - Higher values (0.5-0.9) respond faster to speed changes
    /// - Lower values (0.1-0.3) give smoother, more stable estimates
    pub fn new(alpha: f64) -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_update: now,
            last_percent: 0.0,
            smoothed_rate: None,
            alpha: alpha.clamp(0.01, 0.99),
        }
    }

    /// Update the estimator with a new progress percentage (0.0 - 100.0).
    pub fn update(&mut self, percent: f64) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs_f64();

        if elapsed > 0.001 {
            let delta_percent = percent - self.last_percent;
            if delta_percent > 0.0 {
                let current_rate = delta_percent / elapsed;
                self.smoothed_rate = Some(match self.smoothed_rate {
                    Some(prev) => self.alpha * current_rate + (1.0 - self.alpha) * prev,
                    None => current_rate,
                });
            }
        }

        self.last_percent = percent;
        self.last_update = now;
    }

    /// Estimated time remaining to reach 100%.
    ///
    /// Returns `None` if not enough data to estimate.
    pub fn eta(&self) -> Option<Duration> {
        let rate = self.smoothed_rate?;
        if rate <= 0.0 {
            return None;
        }
        let remaining = 100.0 - self.last_percent;
        if remaining <= 0.0 {
            return Some(Duration::ZERO);
        }
        let secs = remaining / rate;
        // Cap at 24 hours to avoid absurd estimates
        if secs > 86400.0 {
            return None;
        }
        Some(Duration::from_secs_f64(secs))
    }

    /// Total elapsed time since the estimator was created.
    pub fn elapsed(&self) -> Duration {
        self.last_update.duration_since(self.start_time)
    }

    /// Current smoothed throughput in percent per second.
    pub fn rate(&self) -> Option<f64> {
        self.smoothed_rate
    }

    /// Format ETA as a human-readable string (e.g., "2m 30s", "< 1s").
    pub fn eta_display(&self) -> String {
        match self.eta() {
            None => "--:--".to_string(),
            Some(d) if d.is_zero() => "< 1s".to_string(),
            Some(d) => {
                let total_secs = d.as_secs();
                if total_secs < 60 {
                    format!("{total_secs}s")
                } else if total_secs < 3600 {
                    let m = total_secs / 60;
                    let s = total_secs % 60;
                    format!("{m}m {s:02}s")
                } else {
                    let h = total_secs / 3600;
                    let m = (total_secs % 3600) / 60;
                    format!("{h}h {m:02}m")
                }
            }
        }
    }
}

impl Default for Estimator {
    fn default() -> Self {
        Self::new(0.3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_eta_is_none() {
        let e = Estimator::default();
        assert!(e.eta().is_none());
    }

    #[test]
    fn eta_display_no_data() {
        let e = Estimator::default();
        assert_eq!(e.eta_display(), "--:--");
    }

    #[test]
    fn rate_starts_none() {
        let e = Estimator::default();
        assert!(e.rate().is_none());
    }

    #[test]
    fn eta_display_format_seconds() {
        let mut e = Estimator::new(1.0); // alpha=1 for deterministic tests
        // Simulate: jumped from 0% to 90% in negligible time
        // We need a real time gap, so we manually set state
        e.smoothed_rate = Some(10.0); // 10% per second
        e.last_percent = 90.0;
        // Remaining: 10% at 10%/s = 1s
        let eta = e.eta().unwrap();
        assert!(eta.as_secs() <= 1);
    }

    #[test]
    fn alpha_clamped() {
        let e = Estimator::new(5.0); // should be clamped to 0.99
        assert!((e.alpha - 0.99).abs() < f64::EPSILON);
    }
}
