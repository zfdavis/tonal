//! # Tonal
//!
//! A basic music theory and synthesis library.

#![warn(missing_docs)]

mod synth;

use std::time::Duration;
use synth::Samples;

/// Represents a musical pitch.
///
/// The pitch is stored as the number of half steps away from A4. It is valid
/// to use the tuple constructor to create a pitch directly. In fact, this
/// allows doing math on notes for programatic contruction of music.
///
/// The implementation of
/// [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html)
/// produces A4.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub struct Pitch(pub i32);

impl Pitch {
    /// Creates a new pitch with the given note name and octave.
    ///
    /// # Examples
    ///
    /// ```
    /// use tonal::*;
    ///
    /// let a4 = Pitch::new(Name::A, 4);
    /// assert_eq!(a4, Pitch::default());
    /// let c3 = Pitch::new(Name::C, 3);
    /// assert_eq!(c3, Pitch(-21));
    /// ```
    pub fn new(name: Name, octave: u8) -> Self {
        Self((i32::from(octave) - 4) * 12 + name as i32 - 9)
    }

    /// Creates a new pitch from the given frequency in hertz.
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
    /// use tonal::*;
    ///
    /// let a4 = Pitch::new_from_freq(440.0);
    /// assert_eq!(a4, Pitch::new(Name::A, 4));
    /// let c3 = Pitch::new_from_freq(130.81);
    /// assert_eq!(c3, Pitch::new(Name::C, 3));
    /// ```
    ///
    /// This example will panic when run:
    ///
    /// ```should_panic
    /// use tonal::*;
    ///
    /// let invalid = Pitch::new_from_freq(0.0);
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
    /// use tonal::*;
    ///
    /// let a4 = Pitch::default();
    /// assert!((a4.freq() - 440.0).abs() < std::f64::EPSILON);
    /// ```
    pub fn freq(self) -> f64 {
        let b = 2f64.powf(12f64.recip());
        440.0 * b.powi(self.0)
    }
}

/// Represents a pitch or group of pitches with shared volume and length.
///
/// The reason both chords and single notes are represented by the same strcut
/// is because I don't know of a better way of doing it without duplicating
/// code.
#[derive(Clone, Debug, PartialEq)]
pub struct Chord {
    pitches: Vec<Pitch>,
    length: Length,
    volume: f64,
}

impl Chord {
    /// Creates a new chord.
    pub fn new(pitches: Vec<Pitch>, length: Length, volume: f64) -> Self {
        Self {
            pitches,
            length,
            volume,
        }
    }

    /// Creates a new major chord based off of the root.
    ///
    /// This will construct a major chord by including the root, the pitch a
    /// major third above it, and the pitch a perfect fifth above it.
    ///
    /// # Examples
    ///
    /// ```
    /// use tonal::*;
    ///
    /// let c4 = Pitch::new(Name::C, 4);
    /// let c_maj = Chord::new_major(c4, Length::Whole, 0.5);
    /// let correct = [c4, Pitch::new(Name::E, 4), Pitch::new(Name::G, 4)];
    /// assert_eq!(c_maj.pitches(), &correct);
    /// ```
    pub fn new_major(root: Pitch, length: Length, volume: f64) -> Self {
        Self::new(
            vec![root, Pitch(root.0 + 4), Pitch(root.0 + 7)],
            length,
            volume,
        )
    }

    /// Allows access to the pitches in this chord.
    pub fn pitches(&self) -> &[Pitch] {
        &self.pitches
    }

    /// Allows mutable access to the pitches in this chord.
    pub fn pitches_mut(&mut self) -> &mut Vec<Pitch> {
        &mut self.pitches
    }

    /// Returns an iterator of PCM samples representing this chord.
    ///
    /// This is mostly useful for music playback.
    pub fn samples(&self, bpm: f64, rate: u32) -> Samples<'_> {
        let rate = f64::from(rate);

        Samples::new(
            0,
            (self.length.duration(bpm).as_secs_f64() * rate).round() as u32,
            &self.pitches,
            rate,
            self.volume * 8_192.0,
        )
    }
}

/// Represents the length of a musical note.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
#[repr(i32)]
pub enum Length {
    /// A sixteenth of a whole note.
    Sixteenth = 2,
    /// An eigth of a whole note.
    Eigth = 1,
    /// A quarter of a whole note.
    Quarter = 0,
    /// A half of a whole note.
    Half = -1,
    /// A whole note.
    Whole = -2,
}

impl Length {
    /// Calculates the time needed for a length in a ceratin BPM.
    ///
    /// # Examples
    ///
    /// ```
    /// use tonal::*;
    /// use std::time::Duration;
    ///
    /// let bpm = 60.0;
    /// let whole = Length::Whole;
    /// assert_eq!(whole.duration(bpm), Duration::from_secs(4));
    /// ```
    pub fn duration(self, bpm: f64) -> Duration {
        Duration::from_secs_f64(60.0 / (bpm * 2f64.powi(self as i32)))
    }
}

/// Represents the alphabetic name of a pitch.
///
/// The 'S' means 'Sharp'.
#[allow(missing_docs)]
#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Name {
    /// A.
    A = 9,
    /// A sharp / B flat.
    AS = 10,
    /// B.
    B = 11,
    /// C.
    C = 0,
    /// C sharp / D flat.
    CS = 1,
    /// D.
    D = 2,
    /// D sharp / E flat.
    DS = 3,
    /// E.
    E = 4,
    /// F.
    F = 5,
    /// F sharp / G flat.
    FS = 6,
    /// G.
    G = 7,
    /// G sharp / A flat.
    GS = 8,
}
