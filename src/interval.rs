use std::error::Error;
use std::fmt::Formatter;


#[derive(Copy, Clone, Debug)]
pub struct Interval<T: num::PrimInt + std::fmt::Display> {
    start: T,
    stop: T
}

#[derive(thiserror::Error, Debug)]
pub enum IntervalError {
    #[error("Cannot merge non-overlapping/non-adjacent intervals")]
    MergeOnDisjointIntervals
}

impl<T: num::PrimInt + std::fmt::Display> std::fmt::Display for Interval<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.start, self.stop)
    }
}

impl<T: num::PrimInt + std::fmt::Display> Interval<T> {
    pub fn new(start: T, stop: T) -> Self {
        Self{start, stop}
    }

    pub fn start(&self) -> T {
        self.start
    }

    pub fn stop(&self) -> T {
        self.stop
    }

    pub fn contains_value(&self, value: T) -> bool {
        self.start <= value && value <= self.stop
    }

    pub fn contains_interval(&self, other: &Interval<T>) -> bool {
        self.contains_value(other.start) && self.contains_value(other.stop)
    }

    pub fn overlaps_with(&self, other: &Interval<T>) -> bool {
        self.contains_value(other.start)
            || self.contains_value(other.stop)
            || other.contains_value(self.start)
            || other.contains_value(self.stop)
    }

    pub fn left_adjacent_to(&self, other: &Interval<T>) -> bool {
        self.stop + T::one() == other.start
    }

    pub fn right_adjacent_to(&self, other: &Interval<T>) -> bool {
        self.start == other.stop + T::one()
    }

    pub fn adjacent_to(&self, other: &Interval<T>) -> bool {
        self.left_adjacent_to(other) || self.right_adjacent_to(other)
    }

    pub fn can_merge_with(&self, other: &Interval<T>) -> bool {
        self.overlaps_with(other) || self.adjacent_to(other)
    }

    pub fn merge(&self, other: &Interval<T>) -> Result<Interval<T>, Box<dyn Error>> {
        if self.can_merge_with(other) {
            Ok(self.merge_unchecked(other))
        } else {
            Err(IntervalError::MergeOnDisjointIntervals.into())
        }
    }

    pub fn merge_unchecked(&self, other: &Interval<T>) -> Interval<T> {
        Self{
            start: T::min(self.start, other.start),
            stop: T::max(self.stop, other.stop)
        }
    }

    pub  fn merge_inplace(&mut self, other: &Interval<T>) -> Result<(), Box<dyn Error>> {
        if self.can_merge_with(other) {
            self.merge_inplace_unchecked(other);
            Ok(())
        } else {
            Err(IntervalError::MergeOnDisjointIntervals.into())
        }
    }

    pub fn merge_inplace_unchecked(&mut self, other: &Interval<T>) {
        self.start = self.start.min(other.start);
        self.stop = self.stop.max(other.stop);
    }

    pub fn is_left_of(&self, other: &Interval<T>) -> bool {
        self.stop < other.start
    }

    pub fn is_right_of(&self, other: &Interval<T>) -> bool {
        self.start > other.stop
    }
}
