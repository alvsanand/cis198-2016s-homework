use std::collections::VecDeque;

#[derive(Debug)]
struct Node<T> {
    element: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T> Node<T> {
    pub fn new(element: T) -> Node<T> {
        Node {
            element: element,
            left: Link::None,
            right: Link::None,
        }
    }
}

type Link<T> = Option<Box<Node<T>>>;

trait InsertSearch<T: Default + Ord> {
    fn insert(&mut self, e: T) -> bool;
    fn search(&self, e: T) -> bool;
}

impl<T: Default + Ord> InsertSearch<T> for Link<T> {
    fn insert(&mut self, e: T) -> bool {
        self.as_deref_mut().map(|node| {
            if node.element == e {
                false
            } else {
                if node.element > e {
                    match node.left {
                        Link::Some(_) => node.left.insert(e),
                        _ => {
                            node.left = Link::Some(Box::new(Node::new(e)));
                            true
                        }
                    }
                } else {
                    match node.right {
                        Link::Some(_) => node.right.insert(e),
                        _ => {
                            node.right = Link::Some(Box::new(Node::new(e)));
                            true
                        }
                    }
                }
            }
        }) == Option::Some(true)
    }
    fn search(&self, e: T) -> bool {
        self.as_ref().map(|node| {
            if node.element == e {
                true
            } else {
                if node.element > e {
                    node.left.search(e)
                } else {
                    node.right.search(e)
                }
            }
        }) == Option::Some(true)
    }
}

#[derive(Debug)]
pub struct BST<T: Default + Ord> {
    root: Link<T>,
}

impl<T: Default + Ord> BST<T> {
    pub fn new() -> BST<T> {
        BST { root: None }
    }

    pub fn insert(&mut self, e: T) -> bool {
        match self.root {
            Link::Some(_) => self.root.insert(e),
            _ => {
                self.root = Link::Some(Box::new(Node::new(e)));
                true
            }
        }
    }

    pub fn find(&self, e: T) -> bool {
        match self.root {
            Link::Some(_) => self.root.search(e),
            _ => false,
        }
    }
}

pub struct IntoIter<T: Copy> {
    next: VecDeque<Link<T>>,
}

impl<T: Copy> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next.pop_front();

        match next {
            Some(link) => match link {
                Some(node) => {
                    if node.left.is_some() {
                        self.next.push_back(node.left);
                    }
                    if node.right.is_some() {
                        self.next.push_back(node.right);
                    }
                    let value = node.element;
                    Some(value)
                }
                _ => None,
            },
            _ => None,
        }
    }
}

impl<T: Default + Copy + Ord> IntoIterator for BST<T> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            next: VecDeque::from(vec![self.root]),
        }
    }
}

pub struct Iter<'a, T: 'a> {
    next: VecDeque<Option<&'a Node<T>>>,
}

impl<'a, T: Default + Copy + Ord> IntoIterator for &'a BST<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            next: VecDeque::from(vec![self.root.as_ref().map(|link| &**link)]),
        }
    }
}

impl<'a, T: Default + Copy + Ord> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.pop_front().map(|link| {
            link.map(|node| {
                if node.left.is_some() {
                    self.next.push_back(node.left.as_ref().map(|node| &**node));
                }
                if node.right.is_some() {
                    self.next.push_back(node.right.as_ref().map(|node| &**node));
                }

                &node.element
            })
        }).flatten()
    }
}

pub struct IterMut<'a, T: 'a> {
    next: VecDeque<Option<&'a mut Node<T>>>,
}

impl<'a, T: Default + Copy + Ord> IntoIterator for &'a mut BST<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            next: VecDeque::from(vec![self.root.as_mut().map(|link| &mut **link)]),
        }
    }
}

impl<'a, T: Default + Copy + Ord> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.pop_front().map(|link| {
            link.map(|node| {
                if node.left.is_some() {
                    self.next.push_back(node.left.as_mut().map(|node| &mut **node));
                }
                if node.right.is_some() {
                    self.next.push_back(node.right.as_mut().map(|node| &mut **node));
                }

                &mut node.element
            })
        }).flatten()
    }
}
