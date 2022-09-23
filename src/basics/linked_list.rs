use std::borrow::{Borrow, BorrowMut};
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::{Arc, Mutex, Weak};

#[derive(Debug)]
enum Node<T: Debug> {
    Some(Mutex<Arc<Node<T>>>, Mutex<Weak<Node<T>>>, T),
    None,
}


impl<T: Debug> Node<T> {
    fn is_none(&self) -> bool {
        match self {
            Node::None => true,
            Node::Some(..) => false
        }
    }
    fn get_next(&self) -> Option<Arc<Node<T>>> {
        if let Node::Some(next, ..) = self {
            let next = next.lock().unwrap();
            return match &**next {
                Node::Some(..) => { Some(Arc::clone(&*next)) }
                _ => None,
            };
        }
        None
    }

    fn get_prev(&self) -> Option<Weak<Node<T>>> {
        if let Node::Some(_, prev, ..) = self {
            let prev = prev.lock().unwrap();
            return match &*prev.upgrade().unwrap() {
                Node::Some(..) => { Some(Weak::clone(&*prev)) }
                _ => None,
            };
        }
        None
    }
    fn get_data(&self) -> Option<&T> {
        if let Node::Some(.., data) = self {
            Some(data)
        } else {
            None
        }
    }
}

impl<T: Debug> Default for Node<T> {
    fn default() -> Self {
        Node::None
    }
}


#[derive(Debug)]
pub struct LinkedList<T: Debug> {
    first: Mutex<Arc<Node<T>>>,
}

impl<T: Debug> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            first: Mutex::new(Arc::new(Node::None))
        }
    }
    pub fn push(&self, data: T) {
        let mut first = self.first.lock().unwrap();
        if first.is_none()
        {
            *first = Arc::new(Node::Some(
                Mutex::new(Arc::new(Node::None)),
                Mutex::new(Weak::new()),
                data,
            ));
        } else if !first.is_none() && first.get_next().is_none()
        {
            let new = Arc::new(Node::Some(
                Mutex::new(Arc::new(Node::None)),
                Mutex::new(Arc::downgrade(&*first)),
                data,
            ));
            if let Node::Some(next, ..) = &*Arc::clone(&*first)
            {
                *next.lock().unwrap() = new.clone();
            }
        } else {
            let mut next = Arc::clone(&first.get_next().unwrap());
            let mut prev = next.get_prev().unwrap();

            while !next.is_none()
            {
                if let Node::Some(mb_next, ..) = &*Arc::clone(&next)
                {
                    prev = Arc::downgrade(&next);
                    next = Arc::clone(&mb_next.lock().unwrap());
                }
            }
            let new = Arc::new(Node::Some(
                Mutex::new(Arc::new(Node::None)),
                Mutex::new(prev.clone()),
                data,
            ));
            if let Node::Some(next, ..) = &*Arc::clone(&prev.upgrade().unwrap())
            {
                *next.lock().unwrap() = new
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        self.first.lock().unwrap().is_none()
    }
    pub fn remove(&self, data: T)
        where T: PartialEq
    {
        if !self.is_empty()
        {
            let mut next = self.first.lock().unwrap();
            let mut prev = Arc::clone(&next);
            loop {
                if next.is_none() {
                    return;
                }
                // 1 2 3 4  3
                if next.get_data().unwrap() == &data {

                    if let Node::Some(next, prev, ..) = &*Arc::clone(&*next) {
                        let mut next = next.lock().unwrap();
                        let prev = prev.lock().unwrap();

                        if let Node::Some(n, ..) = &*Arc::clone(&prev.upgrade().unwrap()) {
                            let mut n = n.lock().unwrap();
                            *n = Arc::clone(&*next);

                        }
                        if let Node::Some(_, p, ..) = &*Arc::clone(&*next) {
                         *p.lock().unwrap() = Weak::clone(&*prev)
                        }
                    }
                    return;
                }
                prev = Arc::clone(&next);
                match next.get_next() {
                    Some(ne) => *next = ne,
                    None => return
                }
            }
        }
    }
    pub fn show_all(&self) {
        let mut next = self.first.lock().unwrap();
        while !next.is_none() {
            if let Node::Some(.., data) = &**next {
                println!("{:?}", data);
            }
            if let Node::Some(el, ..) = &*Arc::clone(&next) {
                *next = Arc::clone(&el.lock().unwrap());
            }
        }
    }
}

#[cfg(test)]
mod list_test {
    use std::sync::{Arc, Mutex};

    use crate::linked_list::{Node};
    use crate::LinkedList;

    #[test]
    fn test() {
        let list = LinkedList::new();
        &list.push("1");
        &list.push("2");
        &list.push("3");
        &list.push("4");
        &list.push("5");
        &list.push("6");
        &list.push("7");
        &list.push("8");
        list.remove("3");

        list.show_all();
    }

}




