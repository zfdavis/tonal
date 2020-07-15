use super::Pitch;
use std::f64::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Samples<'a> {
    current: u32,
    max: u32,
    pitches: &'a [Pitch],
    rate: f64,
    volume: f64,
}

impl<'a> Samples<'a> {
    pub fn new(
        current: u32,
        max: u32,
        pitches: &'a [Pitch],
        rate: f64,
        volume: f64,
    ) -> Self {
        Self {
            current,
            max,
            pitches,
            rate,
            volume,
        }
    }
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
