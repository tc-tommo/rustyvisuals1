mod audio;
mod visualization;
mod utils;
mod constants;

use audio::input::AudioInput;
use visualization::renderer::Renderer;
use constants::*;

#[macroquad::main("Audio Visualization")]
async fn main() {
    // Initialize audio input and renderer
    let audio_input = AudioInput::new().expect("Failed to initialize audio input");
    let mut renderer = Renderer::new();

    loop {
        // Capture audio data and compute FFT
        let fft_data = audio_input.capture_fft();

        // Update visualization
        renderer.update(&fft_data);

        // Render visualization
        renderer.render();

        // Wait for the next frame
        macroquad::window::next_frame().await;
    }
}
