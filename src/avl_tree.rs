use std::error::Error;
use std::io::Write;
use crate::interval::Interval;
use crate::traits::IntervalTree;

struct AVLNode<T: num::PrimInt + std::fmt::Display> {
    height: i32,
    interval: Interval<T>,
    left: Option<Box<AVLNode<T>>>,
    right: Option<Box<AVLNode<T>>>
}

pub enum AVLCase {
    LeftLeft, LeftRight, RightRight, RightLeft, Balanced
}

impl<T: num::PrimInt + std::fmt::Display> AVLNode<T> {
    fn with_value(interval: Interval<T>) -> Self {
        Self{height: 1, left: None, right: None, interval}
    }

    fn merge_down(&mut self) {
        let mut interval = self.interval;
        // left
        if let Some(left_child) = &mut self.left {
            let _ = left_child.merge_down_helper(
                &mut interval,
                &|node| node.left.as_mut(),
                &|node| node.right.as_mut(),
            );
        }
        // right
        if let Some(right_child) = &mut self.right {
            let _ = right_child.merge_down_helper(
                &mut interval,
                &|node| node.right.as_mut(),
                &|node| node.left.as_mut(),
            );
        }
        self.interval = interval;
        self.maybe_drop_children();
        self.recompute_height();
        self.balance_after_deletion();
    }

    fn merge_down_helper<F1, F3>(&mut self,
                                 interval: &mut Interval<T>,
                                 main_side_getter: &F1,
                                 other_side_getter: &F3) -> bool
    where
        F1: Fn(&mut Self) -> Option<&mut Box<Self>>,
        F3: Fn(&mut Self) -> Option<&mut Box<Self>>
    {
        let mut done_merging = false;
        if let Some(child) = (main_side_getter)(self) {
            done_merging = child.merge_down_helper(interval,
                                                   main_side_getter,
                                                   other_side_getter);
        }
        if !done_merging {
            if interval.can_merge_with(&self.interval) {
                self.height = -1;      // Mark for deletion
                interval.merge_inplace_unchecked(&self.interval);
                if let Some(other_child) = (other_side_getter)(self) {
                    done_merging = other_child.merge_down_helper(interval,
                                                                 main_side_getter,
                                                                 other_side_getter);
                }
            } else {
                done_merging = true;
            }
        }
        if done_merging {
            self.maybe_drop_children();
            self.recompute_height();
            self.balance_after_deletion();
        }
        done_merging
    }

    fn balance_after_insertion(&mut self, inserted_interval: Interval<T>) {
        let balance = self.left_child_height() - self.right_child_height();

        let case = if balance > 1 {            // left imbalance
            let bound = self.left.as_ref().expect("AVL Broken").interval.start();
            if inserted_interval.stop() < bound {
                AVLCase::LeftLeft
            } else {
                AVLCase::LeftRight
            }
        } else if balance < -1 {    // right imbalance
            let bound = self.right.as_ref().expect("AVL Broken").interval.start();
            if inserted_interval.stop() < bound {
                AVLCase::RightLeft
            } else {
                AVLCase::RightRight
            }
        } else {
            AVLCase::Balanced
        };

        self.start_rotating(case);
    }

    fn balance_after_deletion(&mut self) {
        let balance = self.balance_score();

        let case = if balance > 1 {
            if self.left_child_balance() >= 0 {
                AVLCase::LeftLeft
            } else {
                AVLCase::LeftRight
            }
        } else if balance < - 1 {
            if self.right_child_balance() <= 0 {
                AVLCase::RightRight
            } else {
                AVLCase::RightLeft
            }
        } else {
            AVLCase::Balanced
        };

        self.start_rotating(case);
    }

    fn start_rotating(&mut self, case: AVLCase) {
        match case {
            AVLCase::LeftLeft => {
                self.rotate_right();
            }
            AVLCase::LeftRight => {
                self.rotate_left();
                self.rotate_right();
            }
            AVLCase::RightRight => {
                self.rotate_left();
            }
            AVLCase::RightLeft => {
                self.rotate_right();
                self.rotate_left();
            },
            AVLCase::Balanced => {}
        }
    }

    fn rotate_left(&mut self) {
        let mut y = *self.right.take().expect("AVL Tree broken");
        self.right = y.left.take();
        let mut temp = Self::with_value(
            Interval::new(T::zero(), T::zero())
        );
        std::mem::swap(&mut temp, self);
        y.right.replace(temp.into());
        std::mem::swap(self, &mut y);
        y.recompute_height();
        self.recompute_height();
    }

    fn rotate_right(&mut self) {
        let mut y = *self.left.take().expect("AVL Tree broken");
        self.left = y.right.take();
        let mut temp = Self::with_value(
            Interval::new(T::zero(), T::zero())
        );
        std::mem::swap(&mut temp, self);
        y.right.replace(temp.into());
        std::mem::swap(self, &mut y);
        y.recompute_height();
        self.recompute_height();
    }

    fn balance_score(&self) -> i32 {
        self.left_child_height() - self.right_child_height()
    }

    fn left_child_balance(&self) -> i32 {
        self.left.as_ref().map_or(0, |node| node.balance_score())
    }

    fn right_child_balance(&self) -> i32 {
        self.right.as_ref().map_or(0, |node| node.balance_score())
    }

    fn left_child_height(&self) -> i32 {
        self.left.as_ref().map_or(0, |node| node.height)
    }

    fn right_child_height(&self) -> i32 {
        self.right.as_ref().map_or(0, |node| node.height)
    }

    fn recompute_height(&mut self) {
        self.height = self.left_child_height()
            .max(self.right_child_height()) + 1;
    }

    fn maybe_drop_children(&mut self) {
        if self.left_child_height() < 0 {
            self.left.take();
        }
        if self.right_child_height() < 0 {
            self.right.take();
        }
    }

    fn get_and_delete_successor(&mut self) -> Interval<T> {
        let node = self.right.as_mut().expect(
            "get_and_delete_successor() called without right node"
        );
        Self::get_and_delete_successor_helper(node)
    }

    fn get_and_delete_successor_helper(node: &mut Self) -> Interval<T> {
        if let Some(child) = node.left.as_mut() {
            let interval = Self::get_and_delete_successor_helper(child);
            node.maybe_drop_children();
            node.recompute_height();
            node.balance_after_deletion();
            interval
        } else {
            node.height = -1;
            node.interval
        }
    }

    fn print_tree<W: std::io::Write>(&self,
                                     writer: &mut W,
                                     indent: usize) -> Result<(), Box<dyn std::error::Error>> {
        writeln!(writer, "Node(")?;
        writeln!(writer, "{:indent$}height={}", "", self.height, indent=2*indent + 2)?;
        writeln!(writer, "{:indent$}interval={}", "", self.interval, indent=2*indent + 2)?;
        write!(writer, "{:indent$}left=", "", indent=2*indent + 2)?;
        match &self.left {
            None => writeln!(writer, "null")?,
            Some(node) => node.print_tree(writer, indent + 1)?
        }
        write!(writer, "{:indent$}right=", "", indent=2*indent + 2)?;
        match &self.right {
            None => writeln!(writer, "null")?,
            Some(node) => node.print_tree(writer, indent + 1)?
        }
        writeln!(writer, "{:indent$})", "", indent=2*indent)?;
        Ok(())
    }

    fn is_avl(&self) -> bool {
        let balance = self.balance_score();
        (-1..=1).contains(&balance)
    }

    fn tree_is_avl(&self) -> bool {
        self.is_avl()
            && self.left.as_ref().map_or(true, |node| node.tree_is_avl())
            && self.right.as_ref().map_or(true, |node| node.tree_is_avl())
    }

}

impl<T: num::PrimInt + std::fmt::Display> AVLNode<T> {
    fn insert(&mut self, new_interval: Interval<T>) {
        if self.interval.can_merge_with(&new_interval) {
            self.interval.merge_inplace_unchecked(&new_interval);
            // merge down recomputes height and re-balances
            self.merge_down();
        } else {
            let child = if new_interval.is_left_of(&self.interval) {
                &mut self.left
            } else {
                &mut self.right
            };
            match child {
                None => { child.replace(Self::with_value(new_interval).into()); }
                Some(node) => { node.insert(new_interval); }
            }
            self.recompute_height();
            self.balance_after_insertion(new_interval);
        }
    }

    fn delete(&mut self, interval: &Interval<T>) {
        println!("Deleting {interval} from {}", self.interval);
        println!("other contained self: {}", interval.contains_interval(&self.interval));
        println!("intervals overlap: {}", interval.overlaps_with(&self.interval));
        println!("self contains other: {}", self.interval.contains_interval(interval));
        if interval.contains_interval(&self.interval) {
            // This node must be deleted.
            // It is actually easier to first delete other child nodes,
            // and then do deletion as in normal AVL.
            if let Some(node) = self.right.as_mut() {
                node.delete(interval);
            }
            if let Some(node) = self.left.as_mut() {
                node.delete(interval);
            }
            self.maybe_drop_children();
            match (self.left.as_mut(), self.right.as_mut()) {
                (None, None) => {
                    self.height = 0;    // mark for deletion by parent
                },
                (Some(_), None) => {
                    let node = self.left.take().expect("AVL broken");
                    *self = *node;
                },
                (None, Some(_)) => {
                    let node = self.right.take().expect("AVL broken");
                    *self = *node;
                },
                (Some(_), Some(_)) => {
                    self.interval = self.get_and_delete_successor();
                }
            }
        } else if interval.overlaps_with(&self.interval) {
            if self.interval.contains_interval(interval) {
                // This node must be split into two nodes
                let left_interval = Interval::new(
                    self.interval.start(), interval.start() - T::one()
                );
                let right_interval = Interval::new(
                    interval.stop() + T::one(), self.interval.stop()
                );
                self.interval = left_interval;
                let new_node = Self{
                    left: None,
                    right: self.right.take(),
                    interval: right_interval,
                    height: self.right_child_height() + 1
                };
                self.right = Some(new_node.into());
                self.right.as_mut().expect("AVL Broken").balance_after_deletion();
            } else {
                // The interval in this node will become smaller.
                self.interval = if self.interval.contains_value(interval.start()) {
                    if let Some(node) = self.right.as_mut() {
                        node.delete(interval);
                    }
                    Interval::new(self.interval.start(), interval.start() - T::one())
                } else {
                    // assert!(self.interval.contains_value(interval.stop());
                    if let Some(node) = self.left.as_mut() {
                        node.delete(interval);
                    }
                    Interval::new(interval.stop() + T::one(), self.interval.stop())
                };
            }
        } else if interval.is_left_of(&self.interval) {
            if let Some(node) = self.left.as_mut() {
                node.delete(interval);
            }
        } else if let Some(node) = self.right.as_mut() {
            node.delete(interval);
        }
        if self.height > 0 {
            self.maybe_drop_children();
            self.recompute_height();
            self.balance_after_deletion();
        }
    }

    fn contains(&self, interval: &Interval<T>) -> bool {
        if self.interval.contains_interval(interval) {
            true
        } else if interval.is_left_of(&self.interval) {
            self.left
                .as_ref()
                .map_or(false, |node| node.contains(interval))
        } else {
            self.right
                .as_ref()
                .map_or(false, |node| node.contains(interval))
        }
    }

    fn tree_size(&self) -> i32 {
        let left_size = self.left
            .as_ref()
            .map_or(0, |n| n.tree_size());
        let right_size = self.right
            .as_ref()
            .map_or(0, |n| n.tree_size());
        left_size + right_size + 1
    }
}

pub struct AVLIntervalTree<T: num::PrimInt + std::fmt::Display> {
    root: Option<AVLNode<T>>,
}

impl<T: num::PrimInt + std::fmt::Display> AVLIntervalTree<T> {
    pub fn print_tree(&self) -> Result<(), Box<dyn Error>> {
        let mut writer = std::io::Cursor::new(Vec::<u8>::new());
        match &self.root {
            None => writeln!(writer, "Tree()")?,
            Some(node) => {
                writeln!(writer, "Tree(")?;
                write!(writer, "  ")?;
                node.print_tree(&mut writer, 1)?;
                writeln!(writer, ")")?;
            }
        }
        writer.set_position(0);
        let buffer = writer.into_inner();
        let text = std::str::from_utf8(&buffer)?;
        println!("{}", text);
        Ok(())
    }

    pub fn is_avl(&self) -> bool {
        match &self.root {
            None => true,
            Some(node) => node.tree_is_avl()
        }
    }
}

impl<T: num::PrimInt + std::fmt::Display> IntervalTree<T> for AVLIntervalTree<T> {
    fn empty() -> Self {
        Self{root: None}
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn number_of_nodes(&self) -> i32 {
        match &self.root {
            None => 0,
            Some(node) => node.tree_size()
        }
    }

    fn insert(&mut self, interval: Interval<T>) {
        match &mut self.root {
            None => { self.root.replace(AVLNode::with_value(interval)); },
            Some(node) => { node.insert(interval); }
        }
    }

    fn delete(&mut self, interval: &Interval<T>) {
        match &mut self.root {
            None => {}
            Some(node) => {
                node.delete(interval);
                if node.height < 0 {    // Root has been deleted 
                    self.root.take();
                }
            }
        }
    }

    fn contains(&self, interval: &Interval<T>) -> bool {
        match self.root {
            None => false,
            Some(ref node) => node.contains(interval)
        }
    }
}
