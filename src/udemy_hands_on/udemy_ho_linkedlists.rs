// Hands-On Data Structures and Algorithms in Rust
// Video 19: Linked lists and binary trees

use std::cell::RefCell;
use std::rc::{Rc, Weak};
#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }
}

#[derive(Debug)]
pub struct DbNode<T> {
    data: T,
    next: Option<Rc<RefCell<DbNode<T>>>>,
    prev: Option<Weak<RefCell<DbNode<T>>>>,
}

#[derive(Debug)]
pub struct DbList<T> {
    first: Option<Rc<RefCell<DbNode<T>>>>,
    last: Option<Weak<RefCell<DbNode<T>>>>,
}

impl<T> DbList<T> {
    pub fn new() -> Self {
        DbList {
            last: None,
            first: None,
        }
    }

    pub fn push_front(&mut self, data: T) {
        match self.first.take() {
            Some(r) => {
                // Create the new node
                let new_front = Rc::new(RefCell::new(DbNode {
                    data,
                    next: Some(r.clone()),
                    prev: None,
                }));
                // Tell the first object this is now in front of it
                let mut m = r.borrow_mut();
                m.prev = Some(Rc::downgrade(&new_front));
                // Put the new node to the front
                self.first = Some(new_front);
            }
            None => {
                let new_data = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }

    pub fn push_back(&mut self, data: T) {
        match self.last.take() {
            Some(r) => {
                // Create the new back node
                let new_back = Rc::new(RefCell::new(DbNode {
                    data,
                    prev: Some(r.clone()),
                    next: None,
                }));
                // Tell the last object this is now behind it
                let st = Weak::upgrade(&r).unwrap();
                let mut m = st.borrow_mut();
                // Put the new node to the back
                self.last = Some(Rc::downgrade(&new_back));
                m.next = Some(new_back)
            }
            None => {
                let new_data = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }
}

pub fn run() {
    let mut list = LinkedList::new();
    list.push_front("Kylpynalle");
    list.push_back("Disco turkey");
    list.push_front("Treefloof");
    let mut dblist1 = DbList::new();
    dblist1.push_front(1);
    dblist1.push_back(23);
    dblist1.push_front(10);
    dblist1.push_front(42);
    println!("{:?}", list);
    if let Some(ref first_node) = dblist1.first {
        println!("First node data: {:?}", first_node.borrow().data);
    } else {
        println!("The list is empty");
    }
}

#[cfg(test)]
mod tests {}
