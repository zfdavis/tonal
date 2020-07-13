//! # Tonic
//!
//! A basic music theory library.

// #![warn(missing_docs)]

use std::f64::consts::PI;

/// Represents a musical pitch.
///
/// The pitch is stored as the number of half steps away from A4. It is valid
/// to use the tuple constructor to create a pitch directly. In fact, this
/// allows doing math on notes for programatic contruction of music.
///
/// The implementation of `Default` produces A4.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub struct Pitch(pub i32);

impl Pitch {
    /// Creates a new pitch with the given note name and octave.
    ///
    /// # Examples
    ///
    /// ```
    /// let a4 = tonic::Pitch::new(tonic::Name::A, 4);
    /// assert_eq!(a4, tonic::Pitch::default());
    /// let c3 = tonic::Pitch::new(tonic::Name::C, 3);
    /// assert_eq!(c3, tonic::Pitch(-21));
    /// ```
    pub fn new(name: Name, octave: u8) -> Self {
        Self((i32::from(octave) - 4) * 12 + name as i32 - 9)
    }

    /// Creates a new pitch from the given frequency in hertz.
    ///
    ///
    /// Because this function uses floating point math, there is a *small*
    /// chance the result might be off.
    /// 
    /// # Panics
    ///
    /// Panics if `freq` is not greater than 0.
    /// 
    /// # Examples
    ///
    /// ```
    /// let a4 = tonic::Pitch::new_from_freq(440.0);
    /// assert_eq!(a4, tonic::Pitch::new(tonic::Name::A, 4));
    /// let c3 = tonic::Pitch::new_from_freq(130.81);
    /// assert_eq!(c3, tonic::Pitch::new(tonic::Name::C, 3));
    /// ```
    /// 
    /// This example will panic when run:
    /// 
    /// ```should_panic
    /// let invalid = tonic::Pitch::new_from_freq(0.0);
    /// ```
    pub fn new_from_freq(freq: f64) -> Self {
        assert!(freq > 0.0, "Frequency must be greater than 0");
        Self((12.0 * (freq / 440.0).log2()).round() as i32)
    }

    /// Calculates the frequency in hertz.
    ///
    /// # Examples
    /// 
    /// ```
    /// let a4_freq = tonic::Pitch::default().freq();
    /// assert!((a4_freq - 440.0).abs() < std::f64::EPSILON);
    /// ```
    pub fn freq(self) -> f64 {
        let b = 2f64.powf(12f64.recip());
        440.0 * b.powi(self.0)
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
    C = 0,
    CS,
    D,
    DS,
    E,
    F,
    FS,
    G,
    GS,
    A,
    AS,
    B,
}
