// Hands-On Data Structures and Algorithms in Rust
// Video 9: Own Memory on the Heap with Box, String, and Vecs

#[derive(Debug)]
pub struct LinkedList<T> {
    data: T,
    next: Option<Box<LinkedList<T>>>,
}

impl<T: std::ops::AddAssign> LinkedList<T> {
    pub fn add_up(&mut self, n: T) {
        self.data += n;
    }
}

pub fn run() {
    let aninmal_list = LinkedList {
        data: "Cheese Boi",
        next: Some(Box::new(LinkedList {
            data: "Danger Zebra",
            next: None,
        })),
    };
    let mut number_list = LinkedList {
        data: 5,
        next: Some(Box::new(LinkedList {
            data: 7,
            next: None,
        })),
    };
    if let Some(ref mut v) = number_list.next {
        v.add_up(10);
    }

    let mut v = Vec::with_capacity(100); // Capacity extends automatically
    v.push("henlo".to_string());
    v.push("buh bai".to_string());

    println!(
        "{:?}\n{:?}\n{:?} - {:?}",
        aninmal_list,
        number_list,
        v[1],
        v.capacity()
    );
}
