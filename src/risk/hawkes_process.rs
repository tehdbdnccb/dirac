use std::arch::x86_64::*;
use std::time::{SystemTime, UNIX_EPOCH};

/// Event types tracked for market manipulation detection
#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum MarketEvent {
    OrderAdd = 0,
    OrderCancel = 1,
    OrderExecute = 2,
    PriceUpdate = 3,
}

const NUM_EVENT_TYPES: usize = 4;

/// Ultra-low latency Hawkes Process for manipulation detection (e.g. spoofing, wash trading)
/// Utilizes Markovian property of Exponential Kernels: h(t) = alpha * e^(-beta * t)
/// Vectorized state updates using SIMD intrinsics.
#[repr(C, align(32))]
pub struct HawkesRiskEngine {
    /// Baseline intensities (\mu) for each event type
    mu: [f64; NUM_EVENT_TYPES],
    /// Jump sizes (\alpha) added upon event occurrence
    alpha: [f64; NUM_EVENT_TYPES],
    /// Decay rates (\beta) for the exponential kernel
    beta: [f64; NUM_EVENT_TYPES],
    /// Current excitation state (Y(t))
    excitation_state: [f64; NUM_EVENT_TYPES],
    /// Last processed timestamp in seconds
    last_update_time: f64,
    /// Alert threshold for intensity
    threshold: f64,
}

impl HawkesRiskEngine {
    pub fn new(mu: [f64; 4], alpha: [f64; 4], beta: [f64; 4], threshold: f64) -> Self {
        Self {
            mu,
            alpha,
            beta,
            excitation_state: [0.0; 4],
            last_update_time: Self::current_time_sec(),
            threshold,
        }
    }

    #[inline(always)]
    fn current_time_sec() -> f64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
    }

    /// Processes a new market event, decays the state, and applies the excitation jump.
    /// Returns true if the conditional intensity exceeds the manipulation threshold.
    #[inline(always)]
    pub fn process_event(&mut self, event_type: MarketEvent) -> bool {
        let current_time = Self::current_time_sec();
        let dt = current_time - self.last_update_time;
        self.last_update_time = current_time;

        unsafe {
            self.update_state_simd(dt, event_type as usize);
        }

        self.check_manipulation_threshold()
    }

    /// AVX2-accelerated state update for concurrent kernel decay calculation
    #[target_feature(enable = "avx2,fma")]
    unsafe fn update_state_simd(&mut self, dt: f64, event_idx: usize) {
        // Load state, beta, and alpha into 256-bit registers (4x f64)
        let state = _mm256_load_pd(self.excitation_state.as_ptr());
        let beta = _mm256_load_pd(self.beta.as_ptr());
        let dt_vec = _mm256_set1_pd(dt);

        // Calculate e^(-beta * dt)
        // Note: In a production environment, use a fast SIMD exp approximation (e.g., Sleef). 
        // Here we map scalar exp for demonstration, but maintain the vector load/store structure.
        let decay_factors = [
            f64::exp(-self.beta[0] * dt),
            f64::exp(-self.beta[1] * dt),
            f64::exp(-self.beta[2] * dt),
            f64::exp(-self.beta[3] * dt),
        ];
        let decay_vec = _mm256_load_pd(decay_factors.as_ptr());

        // Y(t) = Y(t^-) * e^(-beta * dt)
        let mut new_state = _mm256_mul_pd(state, decay_vec);

        // Apply alpha jump to the specific event dimension
        let mut state_arr = [0.0; 4];
        _mm256_store_pd(state_arr.as_mut_ptr(), new_state);
        state_arr[event_idx] += self.alpha[event_idx];
        
        self.excitation_state = state_arr;
    }

    #[inline(always)]
    fn check_manipulation_threshold(&self) -> bool {
        // Conditional Intensity: \lambda(t) = \mu + Y(t)
        // If aggregate intensity of order cancels (spoofing indicator) exceeds threshold, flag it.
        let cancel_intensity = self.mu[MarketEvent::OrderCancel as usize] 
            + self.excitation_state[MarketEvent::OrderCancel as usize];
            
        cancel_intensity > self.threshold
    }
    
    pub fn get_intensity(&self, event_type: MarketEvent) -> f64 {
        let idx = event_type as usize;
        self.mu[idx] + self.excitation_state[idx]
    }
}