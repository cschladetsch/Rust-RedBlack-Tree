// Define colors of the nodes
#[derive(Debug, PartialEq, Eq)]
#[derive(Clone)]
enum Color {
    Red,
    Black,
}

// Define the Red-Black Tree Node
#[derive(Clone)]
struct Node<T> {
    value: T,
    color: Color,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

// Define the Red-Black Tree itself
pub struct RedBlackTree<T: Clone> {
    root: Option<Box<Node<T>>>,
}

impl<T: Clone + Ord> RedBlackTree<T> {
    pub fn new() -> Self {
        RedBlackTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            color: Color::Red,
            left: None,
            right: None,
        });

        if let Some(ref mut root) = self.root {
            Self::insert_recursive(root, new_node);
            root.color = Color::Black;
        } else {
            self.root = Some(new_node);
        }
    }

    fn insert_recursive(parent: &mut Box<Node<T>>, new_node: Box<Node<T>>) {
        if new_node.value < parent.value {
            if let Some(ref mut left) = parent.left {
                Self::insert_recursive(left, new_node);
            } else {
                parent.left = Some(new_node);
            }
        } else {
            if let Some(ref mut right) = parent.right {
                Self::insert_recursive(right, new_node);
            } else {
                parent.right = Some(new_node);
            }
        }

        // TODO: Implement rotation and recoloring to maintain Red-Black Tree properties
    }

    pub fn search(&self, value: T) -> bool {
        Self::search_recursive(self.root.as_ref(), value)
    }

    fn search_recursive(node: Option<&Box<Node<T>>>, value: T) -> bool {
        match node {
            None => false,
            Some(boxed_node) => {
                if value == boxed_node.value {
                    true
                } else if value < boxed_node.value {
                    Self::search_recursive(boxed_node.left.as_ref(), value)
                } else {
                    Self::search_recursive(boxed_node.right.as_ref(), value)
                }
            }
        }
    }

    pub fn remove(&mut self, value: T) {
            if let Some(ref mut root) = self.root {
                if let Some(new_root) = Self::remove_recursive(root, value) {
                    self.root = Some(new_root);
                    if let Some(ref mut root) = self.root {
                        root.color = Color::Black;
                    }
                }
            }
        }

        fn remove_recursive(node: &mut Box<Node<T>>, value: T) -> Option<Box<Node<T>>> {
            if value < node.value {
                if let Some(ref mut left) = node.left {
                    if let Some(new_left) = Self::remove_recursive(left, value) {
                        node.left = Some(new_left);
                    }
                }
            } else if value > node.value {
                if let Some(ref mut right) = node.right {
                    if let Some(new_right) = Self::remove_recursive(right, value) {
                        node.right = Some(new_right);
                    }
                }
            } else {
                // Found the node to delete
                if node.left.is_none() {
                    return node.right.take();
                } else if node.right.is_none() {
                    return node.left.take();
                }

                // Node has two children, find the in-order successor
                let successor = Self::find_min(&mut node.right);
                node.value = successor.value.clone();
                node.right = Self::remove_recursive(&mut node.right, successor.value);
            }

            fix_deletion(node);
            // TODO: Return the node after potential rebalancing
            None
        }

        fn find_min(node: &mut Option<Box<Node<T>>>) -> &mut Box<Node<T>> {
            match node {
                Some(ref mut boxed_node) => {
                    if boxed_node.left.is_none() {
                        boxed_node
                    } else {
                        Self::find_min(&mut boxed_node.left)
                    }
                }
                None => panic!("Attempted to find min of an empty subtree"),
            }
        }

        fn fix_deletion(node: &mut Option<Box<Node<T>>>) {
            if let Some(ref mut boxed_node) = node {
                if boxed_node.color == Color::Red {
                    // Case 1: Node is red, just make it black
                    boxed_node.color = Color::Black;
                } else if Self::is_red(&boxed_node.left) || Self::is_red(&boxed_node.right) {
                    // Case 2: Node is black, has a red child
                    if Self::is_red(&boxed_node.left) {
                        boxed_node.left.as_mut().unwrap().color = Color::Black;
                    } else {
                        boxed_node.right.as_mut().unwrap().color = Color::Black;
                    }
                    boxed_node.color = Color::Red;
                    if Self::is_left_child(node) {
                        Self::rotate_left(node);
                    } else {
                        Self::rotate_right(node);
                    }
                    Self::fix_deletion(node);
                } else {
                    // Case 3: Node is black, both children are black
                    if let Some(ref mut sibling) = Self::sibling(node) {
                        if Self::is_black(&sibling.left) && Self::is_black(&sibling.right) {
                            sibling.color = Color::Red;
                            if Self::is_black(node) {
                                Self::fix_deletion(Self::parent(node));
                            } else {
                                node.as_mut().unwrap().color = Color::Black;
                            }
                        } else {
                            // Case 4: Node is black, sibling has at least one red child
                            if Self::is_left_child(node) && Self::is_black(&sibling.right) {
                                Self::rotate_right(&mut sibling.right);
                            } else if Self::is_right_child(node) && Self::is_black(&sibling.left) {
                                Self::rotate_left(&mut sibling.left);
                            }
                            Self::fix_deletion(node);
                        }
                    }
                }
            }
        }

        fn is_black(node: &Option<Box<Node<T>>>) -> bool {
            match node {
                None => true,
                Some(ref boxed_node) => boxed_node.color == Color::Black,
            }
        }

        fn is_red(node: &Option<Box<Node<T>>>) -> bool {
            !Self::is_black(node)
        }

        fn is_left_child(node: &Option<Box<Node<T>>>) -> bool {
            match Self::parent(node) {
                Some(ref parent) => parent.left == *node,
                None => false,
            }
        }

        fn is_right_child(node: &Option<Box<Node<T>>>) -> bool {
            match Self::parent(node) {
                Some(ref parent) => parent.right == *node,
                None => false,
            }
        }

    fn parent(&self, node: &Option<Box<Node<T>>>) -> Option<&mut Box<Node<T>>> {
        if let Some(ref root) = self.root {
            self.parent_recursive(root, node)
        } else {
            None
        }
    }

    fn parent_recursive<'a>(
        &'a mut self,
        current: &'a mut Box<Node<T>>,
        target: &Option<Box<Node<T>>>,
    ) -> Option<&'a mut Box<Node<T>>> {
        if current.left == *target || current.right == *target {
            return Some(current);
        }

        if target.as_ref().unwrap().value < current.value {
            if let Some(ref mut left) = current.left {
                self.parent_recursive(left, target)
            } else {
                None
            }
        } else {
            if let Some(ref mut right) = current.right {
                self.parent_recursive(right, target)
            } else {
                None
            }
        }
    }

    fn sibling(&self, node: &Option<Box<Node<T>>>) -> Option<&mut Box<Node<T>>> {
        if let Some(ref parent) = self.parent(node) {
            if *node == parent.left {
                parent.right.as_mut()
            } else {
                parent.left.as_mut()
            }
        } else {
            None
        }
    }

    fn rotate_left(node: &mut Option<Box<Node<T>>>) {
        if let Some(mut right) = node.take().and_then(|n| n.right.take()) {
            let mut new_root = right.left.take();
            std::mem::swap(&mut right.left, node);
            std::mem::swap(node, &mut right);
            right.left = new_root;
            *node = right;
        }
    }

    fn rotate_right(node: &mut Option<Box<Node<T>>>) {
        if let Some(mut left) = node.take().and_then(|n| n.left.take()) {
            let mut new_root = left.right.take();
            std::mem::swap(&mut left.right, node);
            std::mem::swap(node, &mut left);
            left.right = new_root;
            *node = left;
        }
    }
}

fn not_main() {
    let mut tree: RedBlackTree<i32> = RedBlackTree::new();

    tree.insert(10);
    tree.insert(5);
    tree.insert(15);

    println!("Is 5 in the tree? {}", tree.search(5)); // Should print: Is 5 in the tree? true
    println!("Is 20 in the tree? {}", tree.search(20)); // Should print: Is 20 in the tree? false

    // TODO: Test removal and other methods
}

