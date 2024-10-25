use rustfft::{FftPlanner, num_complex::Complex32};
use std::sync::Arc;

pub struct FFTProcessor {
    fft: Arc<dyn rustfft::Fft<f32>>,
}

impl FFTProcessor {
    pub fn new(size: usize) -> Self {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(size);

        Self { fft }
    }

    pub fn process(&self, samples: &[f32]) -> Vec<f32> {
        // Print the first few samples to check if they are non-zero
        // Proceed with FFT processing
        let mut buffer: Vec<Complex32> = samples.iter().map(|&s| Complex32::new(s, 0.0)).collect();
        self.fft.process(&mut buffer);
    
        buffer.iter().map(|c| c.norm()).collect()
    }
}
