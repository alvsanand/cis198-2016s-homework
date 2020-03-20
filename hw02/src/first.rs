#[derive(Debug)]
struct Node {
    element: i32,
    left: Link,
    right: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
pub struct BST {
    root: Node,
}

impl BST {
    pub fn new() -> BST {
        BST {
            root: Node {
                element: 0,
                left: Link::Empty,
                right: Link::Empty,
            },
        }
    }

    fn _create_node(element: i32) -> Node {
        Node {
            element: element,
            left: Link::Empty,
            right: Link::Empty,
        }
    }

    fn _insert(node: &mut Node, _element: i32) -> bool {
        match node {
            Node {
                element,..
            } if *element == _element => false,
            Node {
                element,
                left,
                right,
            } => {
                if *element > _element {
                    match right {
                        Link::Empty => {
                            node.right = Link::More(Box::new(BST::_create_node(_element)));
                            true
                        },
                        Link::More(link_node) => {
                            BST::_insert(link_node, _element)
                        }
                    }                    
                }
                else {
                    match left {
                        Link::Empty => {
                            node.left = Link::More(Box::new(BST::_create_node(_element)));
                            true
                        },
                        Link::More(link_node) => {
                            BST::_insert(link_node, _element)
                        }
                    } 
                }
            },
        }
    }

    fn _find(node: &mut Node, _element: i32) -> bool {
        match node {
            Node {
                element,..
            } if *element == _element => true,
            Node {
                element,
                left,
                right,
            } => {
                if *element > _element {
                    match right {
                        Link::Empty => {
                            false
                        },
                        Link::More(link_node) => {
                            BST::_find(link_node, _element)
                        }
                    }                    
                }
                else {
                    match left {
                        Link::Empty => {
                            false
                        },
                        Link::More(link_node) => {
                            BST::_find(link_node, _element)
                        }
                    } 
                }
            },
        }
    }

    pub fn insert(&mut self, element: i32) -> bool {
        BST::_insert(&mut self.root, element)
    }

    pub fn find(&mut self, element: i32) -> bool {
        BST::_find(&mut self.root, element)
    }
}
