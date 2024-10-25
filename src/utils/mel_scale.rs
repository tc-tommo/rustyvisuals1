pub struct MelScale {
  num_mel_bands: usize,
  filters: Vec<Vec<f32>>, // Precomputed filter bank weights
}

impl MelScale {
  pub fn new(
      min_freq: f32,
      max_freq: f32,
      sample_rate: f32,
      fft_size: usize,
      num_mel_bands: usize,
  ) -> Self {
      let min_mel = freq_to_mel(min_freq);
      let max_mel = freq_to_mel(max_freq);

      let mel_points: Vec<f32> = (0..=num_mel_bands + 1)
          .map(|i| {
              let fraction = i as f32 / (num_mel_bands + 1) as f32;
              min_mel + fraction * (max_mel - min_mel)
          })
          .collect();

      let freq_points: Vec<f32> = mel_points.into_iter().map(mel_to_freq).collect();

      let bin_points: Vec<usize> = freq_points
          .iter()
          .map(|&freq| freq_to_bin(freq, sample_rate, fft_size))
          .collect();

      let mut filters = Vec::with_capacity(num_mel_bands);

      for i in 1..=num_mel_bands {
          let start_bin = bin_points[i - 1];
          let center_bin = bin_points[i];
          let end_bin = bin_points[i + 1];

          let mut filter = vec![0.0; fft_size / 2 + 1];
          for bin in start_bin..end_bin {
              let weight = if bin < center_bin {
                  (bin - start_bin) as f32 / (center_bin - start_bin) as f32
              } else {
                  (end_bin - bin) as f32 / (end_bin - center_bin) as f32
              };
              filter[bin] = weight;
          }
          filters.push(filter);
      }

      Self {
          num_mel_bands,
          filters,
      }
  }

  pub fn map_fft_to_mel(&self, fft_data: &[f32]) -> Vec<f32> {
      self.filters
          .iter()
          .map(|filter| {
              fft_data
                  .iter()
                  .zip(filter.iter())
                  .map(|(&fft_value, &weight)| fft_value * weight)
                  .sum()
          })
          .collect()
  }

  pub fn output_smoothing(&self, values: &mut [f32], kernel: &[f32]) {
    let k_size = kernel.len();
    let k_half = k_size / 2;

    let smoothed_values: Vec<f32> = values
      .iter()
      .enumerate()
      .map(|(i, _)| {
          kernel
              .iter()
              .enumerate()
              .map(|(k, &w)| {
                  let idx = i as isize + k as isize - k_half as isize;
                  let idx = idx.clamp(0, (values.len() - 1) as isize) as usize;
                  values[idx] * w
              })
              .sum::<f32>()
      })
      .collect();

    values.copy_from_slice(&smoothed_values);

  }

}

pub fn triangular_kernel(window_size: usize) -> Vec<f32> {
  let mut kernel = Vec::with_capacity(window_size);
  let center = (window_size - 1) as f32 / 2.0;

  for i in 0..window_size {
      let distance = (i as f32 - center).abs();
      let value = 1.0 - (distance / center);
      kernel.push(value);
  }

  // Normalize the kernel to ensure the sum equals 1
  let sum: f32 = kernel.iter().sum();
  kernel.iter_mut().for_each(|v| *v /= sum);

  kernel
}




fn freq_to_bin(freq: f32, sample_rate: f32, fft_size: usize) -> usize {
  let bin = ((freq / sample_rate) * fft_size as f32).floor() as usize;
  let max_bin = fft_size / 2;
  bin.clamp(0, max_bin)
}



// Mel scale conversion functions
pub fn freq_to_mel(freq: f32) -> f32 {
  1127.0 * (1.0 + freq / 700.0).ln()
}

pub fn mel_to_freq(mel: f32) -> f32 {
  700.0 * ((mel / 1127.0).exp() - 1.0)
}
