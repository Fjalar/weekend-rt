use std::f32;

#[derive(Clone, Copy, Debug)]
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

#[allow(dead_code)]
impl Interval {
    pub(crate) const fn new(min: f32, max: f32) -> Interval {
        Interval { min, max }
    }

    pub(crate) fn tight(a: Interval, b: Interval) -> Self {
        Interval {
            min: if a.min <= b.min { a.min } else { b.min },
            max: if a.max >= b.max { a.max } else { b.max },
        }
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

    pub(crate) fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
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

    pub(crate) fn expand(self, delta: f32) {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding);
    }

    pub(crate) fn middle(&self) -> f32 {
        self.min + self.size() / 2.0
    }
}
