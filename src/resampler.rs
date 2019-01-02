pub struct Resampler {
    //#[cfg(not(target_arch = "wasm32"))]
    //resampler: SpeexResampler
}

impl Resampler {
    pub fn new(nes_sample_rate: u32, output_sample_rate: u32) -> Self {
        Resampler { 
            //resampler: SpeexResampler::new(1, nes_sample_rate, output_sample_rate, 0).unwrap() 
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn process(&self, input: &mut [i16], output: &mut [u8]) {

    }

    #[cfg(target_arch = "wasm32")]
    pub fn process(&self, input: &mut [i16], output: &mut [u8]) {

    }
}
