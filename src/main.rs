use naive_structures::tree;

fn main() {
    let mut t = tree::rbtree::RBTree::new();
    t.insert("a", 123);
    t.insert("g", 234);
    t.insert("f", 345);
    t.insert("k", 456);

    println!("{:?}", t);
}
