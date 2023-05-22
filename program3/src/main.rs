// A binary search tree
// Basically just a node to make it simpler
struct SearchTree<T: PartialOrd + Copy> {
    data: T,
    left: Option<Box<SearchTree<T>>>,
    right: Option<Box<SearchTree<T>>>,
}

impl<T: PartialOrd + Copy> SearchTree<T> {
    // Creates a new tree with the root data given
    fn new(data: T) -> SearchTree<T> {
        SearchTree {
            data: data,
            left: None,
            right: None,
        }
    }

    // Inserts new data into a tree
    // Returns true if successful, false if failed
    fn insert(self: &mut Self, data: T) -> bool {
        if data < self.data {
            match &mut self.left {
                Some(ptr) => ptr.insert(data),
                None => {
                    self.left = Some(Box::new(SearchTree::new(data)));
                    true
                }
            }
        } else if data > self.data {
            match &mut self.right {
                Some(ptr) => ptr.insert(data),
                None => {
                    self.right = Some(Box::new(SearchTree::new(data)));
                    true
                }
            }
        } else {
            false
        }
    }

    // Searches for some data in a tree
    // Returns true if found, false if not found
    fn search(self: &Self, data: T) -> bool {
        if data < self.data {
            match &self.left {
                Some(ptr) => ptr.search(data),
                None => false,
            }
        } else if data > self.data {
            match &self.right {
                Some(ptr) => ptr.search(data),
                None => false,
            }
        } else {
            true
        }
    }

    // Traverses a tree in order and adds the elements to the provided vector
    fn traverse(&self, vector: &mut Vec<T>) {
        match &self.left {
            Some(ptr) => ptr.traverse(vector),
            None => ()
        }
        vector.push(self.data);
        match &self.right {
            Some(ptr) => ptr.traverse(vector),
            None => ()
        }
    }
}

fn main() {
    // Testing methods new, search, and traverse
    print!("Initializing tree with 0 at root...");
    let mut tree = SearchTree::new(0);
    println!(" SUCCESS");
    println!("  tree contains 0: {}", tree.search(0));
    println!("  tree contains 2: {}", tree.search(2));
    let mut vector: Vec<i32> = vec![];
    tree.traverse(&mut vector);
    println!("  tree = {:?}", vector);

    // Testing methods insert, search, and traverse
    print!("Inserting -2, -1, 1, 2, and 3 into the tree...");
    let mut result = tree.insert(-2) &&
        tree.insert(-1) &&
        tree.insert(2) &&
        tree.insert(1) &&
        tree.insert(3);
    println!(" {}", if result {"SUCCESS"} else {"FAILURE"});
    println!("  tree contains 0: {}", tree.search(0));
    println!("  tree contains 2: {}", tree.search(2));
    println!("  tree contains 4: {}", tree.search(4));
    vector.clear();
    tree.traverse(&mut vector);
    println!("  tree = {:?}", vector);

    // Testing methods insert and traverse
    print!("Trying to insert 1 again...");
    result = tree.insert(1);
    println!(" {}", if result {"SUCCESS"} else {"FAILURE"});
    vector.clear();
    tree.traverse(&mut vector);
    println!("  tree = {:?}", vector);
}
