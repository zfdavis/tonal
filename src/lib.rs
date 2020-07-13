use std::f64::consts::PI;

#[derive(Clone, Copy)]
pub struct Pitch(pub i32);

impl Pitch {
    pub fn new(name: Name, octave: i32) -> Self {
        Self((octave - 4) * 12 + name as i32)
    }

    pub fn new_from_freq(freq: f64) -> Self {
        Self((12.0 * (freq / 440.0).log2()).round() as i32)
    }

    pub fn freq(self) -> f64 {
        55.0 * 2f64.powf(3.0 + f64::from(self.0) / 12.0)
    }
}

pub struct Chord {
    pitches: Vec<Pitch>,
    duration: Duration,
    volume: f64,
}

impl Chord {
    pub fn new(root: Pitch, duration: Duration, volume: f64) -> Self {
        Self {
            pitches: vec![root],
            duration,
            volume,
        }
    }

    pub fn new_major(root: Pitch, duration: Duration, volume: f64) -> Self {
        Self {
            pitches: vec![root, Pitch(root.0 + 4), Pitch(root.0 + 7)],
            duration,
            volume,
        }
    }

    pub fn samples(&self, bpm: f64, rate: u32) -> Samples<'_> {
        let rate = f64::from(rate);

        Samples {
            current: 0,
            max: (self.duration.secs(bpm) * rate).round() as u32,
            pitches: &self.pitches,
            rate,
            volume: self.volume * 8_192.0,
        }
    }
}

pub struct Samples<'a> {
    current: u32,
    max: u32,
    pitches: &'a [Pitch],
    rate: f64,
    volume: f64,
}

impl ExactSizeIterator for Samples<'_> {}

impl Iterator for Samples<'_> {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.max {
            return None;
        }

        let time = f64::from(self.current) / self.rate;

        let sample = self
            .pitches
            .iter()
            .map(|pitch| {
                let f = pitch.freq();

                (1..=4i32)
                    .map(|h| {
                        let f = f * f64::from(h);
                        let v = self.volume / 2f64.powi(h - 1);
                        ((time * 2.0 * PI * f).sin() * v) as i16
                    })
                    .sum::<i16>()
            })
            .sum();

        self.current += 1;

        Some(sample)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.max - self.current) as usize;
        (size, Some(size))
    }
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Duration {
    Whole = -2,
    Half,
    Quarter,
    Eigth,
    Sixteenth,
}

impl Duration {
    pub fn secs(self, bpm: f64) -> f64 {
        60.0 / (bpm * 2f64.powi(self as i32))
    }
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Name {
    A = 0,
    AS,
    B,
    C,
    CS,
    D,
    DS,
    E,
    F,
    FS,
    G,
    GS,
}
