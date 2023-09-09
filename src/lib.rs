mod traits;
mod interval;
mod avl_tree;


pub use interval::Interval;
pub use traits::IntervalTree;
pub use avl_tree::AVLIntervalTree;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::error::Error;
    use rand::{thread_rng, Rng};
    use super::*;

    fn random_interval<T: Rng>(rng: &mut T) -> Interval<i8> {
        let start = rng.gen::<i8>();
        let stop = rng.gen::<i8>();
        Interval::new(
            i8::min(start, stop),
            i8::max(start, stop)
        )
    }

    fn random_interval_small<T: Rng>(rng: &mut T) -> Interval<i8> {
        let mut start = rng.gen::<i8>();
        let (start, stop) = if start >= i8::MAX - 5 {
            (start - 5, start)
        } else {
            (start, start + 5)
        };
        Interval::new(start, stop)
    }

    fn test_item_in_tree<T: Rng>(
        rng: &mut T,
        tree: &mut AVLIntervalTree<i8>,
        items_in_tree: &mut HashSet<i8>
    ) -> Result<(), Box<dyn Error>> {
        let x: i8 = rng.gen();
        //assert_eq!(items_in_tree.contains(&x), tree.contains_value(x));
        let contained_in_tree = tree.contains_value(x);
        let contained_in_set = items_in_tree.contains(&x);
        if contained_in_tree != contained_in_set {
            tree.print_tree()?;
            println!(
                "Failed test value: {x} (set: {contained_in_set}, tree: {contained_in_tree})"
            );
            assert_eq!(contained_in_set, contained_in_tree);
        }
        Ok(())
    }

    #[test]
    fn random_test_avl_tree_insert() -> Result<(), Box<dyn Error>> {
        let mut tree: AVLIntervalTree<i8> = AVLIntervalTree::empty();
        let mut items_in_tree: HashSet<i8> = HashSet::new();
        let mut rng = thread_rng();
        const ITERATIONS: i32 = 10;
        const SAMPLES_PER_ITERATION: i32 = 10;
        for _ in 0..ITERATIONS {
            let interval = random_interval_small(&mut rng);
            items_in_tree.extend(interval.start()..=interval.stop());
            println!("Insert: {interval}");
            tree.insert(interval);
            tree.print_tree()?;
            assert!(tree.is_avl());
            for _ in 0..SAMPLES_PER_ITERATION {
                test_item_in_tree(&mut rng, &mut tree, &mut items_in_tree)?;
            }
        }
        Ok(())
    }

    #[test]
    fn random_test_avl_tree() -> Result<(), Box<dyn Error>> {
        let mut tree: AVLIntervalTree<i8> = AVLIntervalTree::empty();
        let mut items_in_tree: HashSet<i8> = HashSet::new();
        let mut rng = thread_rng();
        const ITERATORS: i32 = 1000;
        const SAMPLES_PER_ITERATION: i32 = 100;
        for _ in 0..ITERATORS {
            let perform_random_insert = rng.gen_bool(0.5);
            if perform_random_insert || items_in_tree.is_empty() {
                let interval = random_interval(&mut rng);
                // for x in interval.start()..=interval.stop() {
                //     items_in_tree.insert(x);
                // }
                items_in_tree.extend(interval.start()..=interval.stop());
                println!("Insert: {}", interval);
                tree.insert(interval);
            } else {
                let interval = random_interval(&mut rng);
                for x in interval.start()..=interval.stop() {
                    items_in_tree.remove(&x);
                }
                tree.delete(&interval);
                println!("Delete: {}", interval);
            }
            assert!(tree.is_avl());
            for _ in 0..SAMPLES_PER_ITERATION {
                test_item_in_tree(&mut rng, &mut tree, &mut items_in_tree)?;
            }
        }
        tree.print_tree()?;
        Ok(())
    }

    // #[test]
    // fn it_works() -> Result<(), Box<dyn Error>> {
    //     let mut tree = AVLIntervalTree::empty();
    //     assert!(tree.is_avl());
    //     assert_eq!(tree.number_of_nodes(), 0);
    //
    //     tree.insert(Interval::new(5, 17));
    //     tree.print_tree()?;
    //
    //     assert!(tree.is_avl());
    //     assert_eq!(tree.number_of_nodes(), 1);
    //     assert!(tree.contains_value(12));
    //     assert!(!tree.contains_value(3));
    //
    //     tree.insert(Interval::new(18, 25));
    //     tree.print_tree()?;
    //
    //     assert!(tree.is_avl());
    //     assert_eq!(tree.number_of_nodes(), 1);
    //     assert!(tree.contains_value(21));
    //
    //     tree.insert(Interval::new(-3, 1));
    //     tree.print_tree()?;
    //
    //     assert!(tree.is_avl());
    //     assert_eq!(tree.number_of_nodes(), 2);
    //     assert!(tree.contains_value(0));
    //
    //     tree.delete(&Interval::new(9, 12));
    //     tree.print_tree()?;
    //
    //     assert!(tree.is_avl());
    //     assert_eq!(tree.number_of_nodes(), 3);
    //     assert!(!tree.contains_value(12));
    //     assert!(tree.contains_value(5));
    //
    //     Ok(())
    // }
}
