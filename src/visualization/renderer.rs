use core::num;

use macroquad::prelude::*;

use crate::utils::normalization::map_range;
use crate::utils::{mel_scale::MelScale, normalization::normalize, mel_scale::triangular_kernel};
use crate::visualization::color_mapping::ColorMapper;
use crate::constants::*;

pub struct Renderer {
    mel_scale: MelScale,
    color_mapper: ColorMapper,
    amplitudes: Vec<f32>,
    nearest_mel_bins: Vec<usize>,
    kernel: Vec<f32>,
}

impl Renderer {
    pub fn new() -> Self {
        let mel_scale = MelScale::new(MIN_FREQ, MAX_FREQ, SAMPLE_RATE as f32, FFT_SIZE, NUM_MEL_BANDS);
        let color_mapper = ColorMapper::new();

        let mel_amplitudes = vec![0.0; FFT_SIZE / 2];

        let num_pixels = screen_height() as usize;

        let nearest_mel_bins = Renderer::nearest_mel_bins(num_pixels, NUM_MEL_BANDS);

        let kernel = triangular_kernel(SMOOTHING_KERNEL_SIZE);
        

        Self {
            mel_scale,
            color_mapper,
            amplitudes: mel_amplitudes,
            nearest_mel_bins,
            kernel,
        }
    }

    // Handle window size changes
    pub fn on_window_resize(&mut self, new_width: f32, new_height: f32) {
        self.nearest_mel_bins = Renderer::nearest_mel_bins(new_height as usize, NUM_MEL_BANDS);
    }

    fn nearest_mel_bins(num_pixels: usize, num_mel_bands: usize) -> Vec<usize> {
        (0..num_pixels)
            .map(|y| {
                let normalized_y = y as f32 / num_pixels as f32;
                let mel_band = normalized_y * (num_mel_bands - 1) as f32;
                let clamped_mel_band = mel_band.round().min(num_mel_bands as f32 - 1.0);
                clamped_mel_band as usize
            })
            .collect::<Vec<usize>>()
    }

    pub fn update(&mut self, fft_data: &[f32]) {
        // Map FFT data to Mel scale and normalize

        let mut amplitudes = self.mel_scale.map_fft_to_mel(fft_data);
        self.mel_scale.output_smoothing(&mut amplitudes, &self.kernel);
        normalize(&mut amplitudes);

        self.amplitudes = amplitudes;
        
    }



    pub fn render(&mut self) { // Change to mutable reference

        clear_background(BLACK);

        let num_pixels = screen_height() as usize;

        // Ensure nearest_mel_bins is up-to-date
        if self.nearest_mel_bins.len() != num_pixels {
            self.on_window_resize(screen_width(), screen_height());
        }
        
        // Draw the bars
        for y in 0..num_pixels {
            let mel_bin = self.nearest_mel_bins[y];
            let amplitude = self.amplitudes[NUM_MEL_BANDS - 1 - mel_bin];
            let color = self.color_mapper.map_amplitude_to_color(amplitude);
            draw_rectangle(0.0, y as f32, screen_width(), 1.0, color);
        }
    }


}
