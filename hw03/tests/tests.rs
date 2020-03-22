extern crate hw03;

#[test]
fn test_insert() {
    let mut bst = hw03::second::BST::new();

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
    let mut bst = hw03::second::BST::new();

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

#[test]
fn test_iterator() {
    let mut bst = hw03::second::BST::new();

    assert_eq!(bst.find(1), false);
    assert_eq!(bst.find(-1), false);

    let elements = vec![5,-5,2,-2,6,-6];
    let expected = vec![5,-5,6,-6,2,-2];

    for e in &elements {
        bst.insert(*e);
    }

    let mut result = vec![];
    result.extend(bst);
    
    assert_eq!(expected, result);
}

#[test]
fn test_iter() {
    let mut bst = hw03::second::BST::new();

    assert_eq!(bst.find(1), false);
    assert_eq!(bst.find(-1), false);

    let elements = vec![5,-5,2,-2,6,-6];
    let expected = vec![5,-5,6,-6,2,-2];

    for e in &elements {
        bst.insert(*e);
    }

    let mut result = vec![];

    for e in &bst {
        result.push(*e);
    }
    
    assert_eq!(expected, result);
}

#[test]
fn test_mut_iter() {
    let mut bst = hw03::second::BST::new();

    assert_eq!(bst.find(1), false);
    assert_eq!(bst.find(-1), false);

    let elements = vec![5,-5,2,-2,6,-6];
    let expected = vec![5,-5,6,-6,2,-2];

    for e in &elements {
        bst.insert(*e);
    }

    let mut result = vec![];

    for e in (&mut bst).into_iter() {
        result.push(*e);
    }
    
    assert_eq!(expected, result);
}
