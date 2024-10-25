use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Stream, StreamConfig};

use std::sync::{Arc, Mutex};

use crate::FFT_SIZE;

use super::fft::FFTProcessor;

pub struct AudioInput {
    stream: Stream,
    fft_processor: FFTProcessor,
    shared_buffer: Arc<Mutex<Vec<f32>>>,
}

impl AudioInput {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Set up audio input stream
        let host = cpal::default_host();
        let device = host.default_input_device().expect("No input device available");
        let config = device.default_input_config()?;

        let sample_format = config.sample_format();
        let config: StreamConfig = config.into();

        let fft_processor = FFTProcessor::new(FFT_SIZE);
        let shared_buffer = Arc::new(Mutex::new(vec![0.0; FFT_SIZE]));

        // Clone the shared buffer for use in the callback
        let shared_buffer_clone = shared_buffer.clone();

        // Create the input stream
        let stream = device.build_input_stream(
            &config,
            move |data: &[f32], _: &_| {
                // Copy audio samples to shared buffer
                let mut buffer = shared_buffer_clone.lock().unwrap();

                // Ensure we don't exceed the buffer size
                let len = data.len().min(buffer.len());
                buffer[..len].copy_from_slice(&data[..len]);

                // Optional: Print the buffer to verify it's non-zero
                // println!("{:?}", &buffer[..10.min(buffer.len())]);
            },
            move |err| {
                eprintln!("Stream error: {}", err);
            },
            None,
        )?;

        stream.play()?;

    
        stream.play()?;

        Ok(Self {
            stream,
            fft_processor,
            shared_buffer,
        })
    }

    pub fn capture_fft(&self) -> Vec<f32> {
        // Lock the shared buffer to get a copy of the current samples
        let samples = {
            let buffer = self.shared_buffer.lock().unwrap();
            buffer.clone()
        };

        // Proceed with FFT processing
        self.fft_processor.process(&samples)
    }

}
