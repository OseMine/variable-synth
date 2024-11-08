use std::f32::consts::PI;

pub struct AmsynthOscillator {
    rads: f32,
    twopi_rate: f32,
    random: f32,
    rate: i32,
    random_count: i32,
    waveform: Waveform,
    frequency: f32,
    pulse_width: f32,
    polarity: f32,
    sync_frequency: f32,
    sync_enabled: bool,
    sync_rads: f64,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Waveform {
    Sine,
    Pulse,
    Saw,
    Noise,
    Random,
}

impl AmsynthOscillator {
    pub fn new(sample_rate: f32) -> Self {
        AmsynthOscillator {
            rads: 0.0,
            twopi_rate: 2.0 * PI / sample_rate,
            random: 0.0,
            rate: sample_rate as i32,
            random_count: 0,
            waveform: Waveform::Sine,
            frequency: 440.0,
            pulse_width: 0.0,
            polarity: 1.0,
            sync_frequency: 0.0,
            sync_enabled: false,
            sync_rads: 0.0,
        }
    }

    pub fn set_waveform(&mut self, waveform: Waveform) {
        self.waveform = waveform;
    }

    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq;
    }

    pub fn set_pulse_width(&mut self, pw: f32) {
        self.pulse_width = pw;
    }

    pub fn set_sync_frequency(&mut self, freq: f32) {
        self.sync_frequency = freq;
    }

    pub fn set_sync_enabled(&mut self, enabled: bool) {
        self.sync_enabled = enabled;
    }

    pub fn set_polarity(&mut self, polarity: f32) {
        self.polarity = polarity;
    }

    pub fn generate(&mut self) -> f32 {
        match self.waveform {
            Waveform::Sine => self.do_sine(),
            Waveform::Pulse => self.do_square(),
            Waveform::Saw => self.do_saw(),
            Waveform::Noise => self.do_noise(),
            Waveform::Random => self.do_random(),
        }
    }

    fn do_sine(&mut self) -> f32 {
        let mut output = self.rads.sin();
        self.rads += self.twopi_rate * self.frequency;
        if self.rads >= 2.0 * PI {
            self.rads -= 2.0 * PI;
        }
        output * self.polarity
    }

    fn do_square(&mut self) -> f32 {
        let mut output = if self.rads < PI * self.pulse_width { 1.0 } else { -1.0 };
        self.rads += self.twopi_rate * self.frequency;
        if self.rads >= 2.0 * PI {
            self.rads -= 2.0 * PI;
        }
        output * self.polarity
    }

    fn do_saw(&mut self) -> f32 {
        let mut output = (self.rads / PI) - 1.0;
        self.rads += self.twopi_rate * self.frequency;
        if self.rads >= 2.0 * PI {
            self.rads -= 2.0 * PI;
        }
        output * self.polarity
    }

    fn do_noise(&mut self) -> f32 {
        rand::random::<f32>() * 2.0 - 1.0
    }

    fn do_random(&mut self) -> f32 {
        if self.random_count == 0 {
            self.random = rand::random::<f32>() * 2.0 - 1.0;
        }
        self.random_count = (self.random_count + 1) % (self.rate / 30);
        self.random
    }

    pub fn reset(&mut self) {
        self.rads = 0.0;
        self.random = 0.0;
        self.random_count = 0;
        self.sync_rads = 0.0;
    }
}
