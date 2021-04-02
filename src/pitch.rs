use crate::consts::OCTAVE;
use num_integer::Integer;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pitch {
    C(u8),
    CD(u8),
    D(u8),
    DE(u8),
    E(u8),
    F(u8),
    FG(u8),
    G(u8),
    GA(u8),
    A(u8),
    AB(u8),
    B(u8),
}

impl Default for Pitch {
    fn default() -> Self {
        Pitch::C(0)
    }
}

impl Pitch {
    pub fn from_interval(interval: u8) -> Pitch {
        let (octave, pitch) = interval.div_rem(&OCTAVE);
        use Pitch::*;
        match pitch {
            0 => C(octave),
            1 => CD(octave),
            2 => D(octave),
            3 => DE(octave),
            4 => E(octave),
            5 => F(octave),
            6 => FG(octave),
            7 => G(octave),
            8 => GA(octave),
            9 => A(octave),
            10 => AB(octave),
            11 => B(octave),
            _ => unreachable!(),
        }
    }

    pub fn as_interval(&self) -> u8 {
        use Pitch::*;
        match *self {
            C(n) => 0 + n * 12,
            CD(n) => 1 + n * 12,
            D(n) => 2 + n * 12,
            DE(n) => 3 + n * 12,
            E(n) => 4 + n * 12,
            F(n) => 5 + n * 12,
            FG(n) => 6 + n * 12,
            G(n) => 7 + n * 12,
            GA(n) => 8 + n * 12,
            A(n) => 9 + n * 12,
            AB(n) => 10 + n * 12,
            B(n) => 11 + n * 12,
        }
    }

    pub fn add_interval<T>(self, interval: T) -> Option<Pitch>
    where
        T: Into<u8>,
    {
        let interval: u8 = interval.into();
        self.as_interval()
            .checked_add(interval)
            .map(|i| Pitch::from_interval(i))
    }

    pub fn sub_interval<T>(self, interval: T) -> Option<Pitch>
    where
        T: Into<u8>,
    {
        let interval: u8 = interval.into();
        self.as_interval()
            .checked_sub(interval)
            .map(|i| Pitch::from_interval(i))
    }

    pub fn iter(&self) -> Iter {
        Iter { inner: *self }
    }

    pub fn to_string_with_accidental(&self, a: Accidental) -> String {
        use Accidental::*;
        use Pitch::*;
        String::from(match *self {
            C(_) => "C",
            CD(_) => match a {
                Flat => "Db",
                Sharp => "C#",
            },
            D(_) => "D",
            DE(_) => match a {
                Flat => "Eb",
                Sharp => "D#",
            },
            E(_) => "E",
            F(_) => "F",
            FG(_) => match a {
                Flat => "Gb",
                Sharp => "F#",
            },
            G(_) => "G",
            GA(_) => match a {
                Flat => "Ab",
                Sharp => "G#",
            },
            A(_) => "A",
            AB(_) => match a {
                Flat => "Bb",
                Sharp => "A#",
            },
            B(_) => "B",
        })
    }

    pub fn interval_between(&self, other: &Pitch) -> Option<u8> {
        if self <= other {
            Some(other.as_interval() - self.as_interval())
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Accidental {
    #[allow(dead_code)]
    Flat,
    #[allow(dead_code)]
    Sharp,
}

impl IntoIterator for Pitch {
    type Item = Pitch;
    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Iter {
    inner: Pitch,
}

impl Iterator for Iter {
    type Item = Pitch;

    fn next(&mut self) -> Option<Self::Item> {
        use std::mem::replace;
        use Pitch::*;

        match self.inner {
            C(n) => Some(replace(&mut self.inner, CD(n))),
            CD(n) => Some(replace(&mut self.inner, D(n))),
            D(n) => Some(replace(&mut self.inner, DE(n))),
            DE(n) => Some(replace(&mut self.inner, E(n))),
            E(n) => Some(replace(&mut self.inner, F(n))),
            F(n) => Some(replace(&mut self.inner, FG(n))),
            FG(n) => Some(replace(&mut self.inner, G(n))),
            G(n) => Some(replace(&mut self.inner, GA(n))),
            GA(n) => Some(replace(&mut self.inner, A(n))),
            A(n) => Some(replace(&mut self.inner, AB(n))),
            AB(n) => Some(replace(&mut self.inner, B(n))),
            B(n) => n.checked_add(1).map(|n| replace(&mut self.inner, C(n))),
        }
    }
}

impl DoubleEndedIterator for Iter {
    fn next_back(&mut self) -> Option<Self::Item> {
        use std::mem::replace;
        use Pitch::*;

        match self.inner {
            C(n) => n.checked_sub(1).map(|n| replace(&mut self.inner, B(n))),
            CD(n) => Some(replace(&mut self.inner, C(n))),
            D(n) => Some(replace(&mut self.inner, CD(n))),
            DE(n) => Some(replace(&mut self.inner, D(n))),
            E(n) => Some(replace(&mut self.inner, DE(n))),
            F(n) => Some(replace(&mut self.inner, E(n))),
            FG(n) => Some(replace(&mut self.inner, F(n))),
            G(n) => Some(replace(&mut self.inner, FG(n))),
            GA(n) => Some(replace(&mut self.inner, G(n))),
            A(n) => Some(replace(&mut self.inner, GA(n))),
            AB(n) => Some(replace(&mut self.inner, A(n))),
            B(n) => Some(replace(&mut self.inner, AB(n))),
        }
    }
}

impl From<u8> for Pitch {
    fn from(value: u8) -> Pitch {
        Pitch::from_interval(value)
    }
}
