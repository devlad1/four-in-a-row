
#[cfg(test)]
mod tests {
    use std::usize::MAX;
    use crate::game::ai::alphabeta::alphabeta_algorithm::{GameNode, get_best_move};

    #[derive(Debug, Clone)]
    struct Tree<S: Copy> {
        sons: Vec<Tree<S>>,
        data: S,
    }

    impl<S: Copy> Tree<S> {
        fn new(data: S) -> Tree<S> {
            Tree {
                sons: vec![],
                data: data,
            }
        }

        fn insert(&mut self, index: usize, son: S) {
            if index > self.sons.len() {
                panic!(
                    "Tried inserting at index {} when length is {}",
                    index,
                    self.sons.len()
                )
            }

            self.sons.insert(index, Tree::new(son))
        }

        fn get(&mut self, index: usize) -> &mut Tree<S> {
            self.sons.get_mut(index).unwrap()
        }
    }

    impl GameNode<Tree<usize>, usize> for Tree<usize> {
        fn evaluate(&self) -> f64 {
            self.data as f64
        }

        fn get_children_nodes(&self) -> std::collections::HashMap<usize, Box<Tree<usize>>> {
            self.sons
                .iter()
                .enumerate()
                .map(|(index, son)| (index, Box::new(son.clone())))
                .collect()
        }
    }

    fn build_tree_for_tests() -> Tree<usize> {
        let mut root = Tree::new(3);

        root.insert(0, 11);
        root.insert(1, 3);

        root.get(0).insert(0, 20);
        root.get(0).insert(1, 5);

        root.get(0).get(0).insert(0, 20);
        root.get(0).get(0).insert(1, 15);

        root.get(0).get(0).get(0).insert(0, 20);
        root.get(0).get(0).get(0).insert(1, MAX);

        root.get(0).get(0).get(1).insert(0, 15);

        root.get(0).get(1).insert(0, 1);

        root.get(0).get(1).get(0).insert(0, 1);

        root.get(1).insert(0, 15);
        root.get(1).insert(1, 3);

        root.get(1).get(0).insert(0, 15);
        root.get(1).get(0).insert(1, 0);

        root.get(1).get(0).get(0).insert(0, 17);
        root.get(1).get(0).get(0).insert(1, 15);

        root.get(1).get(0).get(1).insert(0, 0);

        root.get(1).get(1).insert(0, 3);

        root.get(1).get(1).get(0).insert(0, 3);
        root.get(1).get(1).get(0).insert(1, 5);

        root
    }

    #[test]
    fn test_min_depth() {
        let tree = build_tree_for_tests();

        let actual_best_move = get_best_move(&tree, 1);

        assert_eq!(actual_best_move, Some(0))
    }

    #[test]
    fn test_medium_depth() {
        let tree = build_tree_for_tests();

        let actual_best_move = get_best_move(&tree, 2);

        assert_eq!(actual_best_move, Some(0))
    }

    #[test]
    fn test_inf_depth() {
        let tree = build_tree_for_tests();

        let actual_best_move = get_best_move(&tree, MAX);

        assert_eq!(actual_best_move, Some(1))
    }
}
