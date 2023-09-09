use crate::interval::Interval;

pub trait IntervalTree<T: num::PrimInt + std::fmt::Display> {

    /// Create and return a new empty tree.
    fn empty() -> Self;


    /// Check whether the tree is empty.
    fn is_empty(&self) -> bool;
    
    /// Amount of nodes stored in the tree.
    fn number_of_nodes(&self) -> i32;

    /// Insert an interval into the tree.
    fn insert(&mut self, interval: Interval<T>);


    /// Delete an interval from the tree.
    fn delete(&mut self, interval: &Interval<T>);


    /// Check whether an interval is contained in the tree.
    fn contains(&self, interval: &Interval<T>) -> bool;
    

    /// Insert a single value into the tree.
    fn insert_value(&mut self, value: T) {
        self.insert(Interval::new(value, value));
    }


    /// Delete a single value from the tree.
    fn delete_value(&mut self, value: T) {
        self.delete(&Interval::new(value, value))
    }

    /// Check whether a single value is contained in the tree.
    fn contains_value(&self, value: T) -> bool {
        self.contains(&Interval::new(value, value))
    }
}