pub fn normalize(amplitudes: &mut [f32]) {
  let max_value = amplitudes.iter().cloned().fold(0./0., f32::max);
  // print amplitude range


  for amplitude in amplitudes.iter_mut() {
      *amplitude = ((*amplitude + 1.0).ln() / max_value).powf(3.0);
  }

  let sum: f32 = amplitudes.iter().sum();

  for amplitude in amplitudes.iter_mut() {
      *amplitude /= sum;
  }

  let post_norm_max = amplitudes.iter().cloned().fold(0./0., f32::max);
  let post_norm_min = amplitudes.iter().cloned().fold(0./0., f32::min);

  // Scale to [0, 1] range
  for amplitude in amplitudes.iter_mut() {
    *amplitude = (*amplitude - post_norm_min) / (post_norm_max - post_norm_min);
  }
}

pub fn map_range(value: f32, src_min: f32, src_max: f32, dst_min: f32, dst_max: f32) -> f32 {

  ((value - src_min) / (src_max - src_min)) * (dst_max - dst_min) + dst_min
}

