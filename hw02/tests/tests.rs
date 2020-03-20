extern crate hw02;

#[test]
fn test_insert() {
    let mut bst = hw02::first::BST::new();

    assert_eq!(bst.insert(1), true);
    assert_eq!(bst.insert(2), true);
    assert_eq!(bst.insert(3), true);

    assert_eq!(bst.insert(1), false);
    assert_eq!(bst.insert(2), false);
    assert_eq!(bst.insert(3), false);

    assert_eq!(bst.insert(1), false);
    assert_eq!(bst.insert(2), false);
    assert_eq!(bst.insert(3), false);
}

#[test]
fn test_find() {
    let mut bst = hw02::first::BST::new();

    assert_eq!(bst.find(1), false);
    assert_eq!(bst.find(-1), false);

    bst.insert(5);
    bst.insert(-5);
    bst.insert(2);
    bst.insert(-2);
    bst.insert(6);
    bst.insert(-6);

    assert_eq!(bst.find(1), false);
    assert_eq!(bst.find(-1), false);
    assert_eq!(bst.find(2), true);
    assert_eq!(bst.find(-2), true);
    assert_eq!(bst.find(5), true);
    assert_eq!(bst.find(-5), true);
    assert_eq!(bst.find(6), true);
    assert_eq!(bst.find(-6), true);
}
