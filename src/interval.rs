use std::f32;

pub(crate) struct Interval {
    pub(crate) min: f32,
    pub(crate) max: f32,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f32::INFINITY,
            max: f32::NEG_INFINITY,
        }
    }
}
impl Interval {
    pub(crate) fn new(min: f32, max: f32) -> Interval {
        Interval { min, max }
    }

    pub(crate) fn size(&self) -> f32 {
        self.max - self.min
    }

    pub(crate) fn contains(&self, x: f32) -> bool {
        self.min <= x && x < self.max
    }

    pub(crate) fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub(crate) fn empty() -> Self {
        Interval::default()
    }

    pub(crate) fn all() -> Self {
        Interval {
            min: f32::NEG_INFINITY,
            max: f32::INFINITY,
        }
    }
}
