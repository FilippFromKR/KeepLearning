use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug)]
enum Node<T: Debug>
{
    Exist(Option<RefCell<Rc<Node<T>>>>, T),
    Null,
}

impl<T: Debug> Node<T>
{
    fn new(next: Option<RefCell<Rc<Node<T>>>>, data: T) -> Self {
        Node::Exist(next, data)
    }
    fn get_data(self) -> Option<T>
    {
        match self {
            Node::Null => None,
            Node::Exist(.., t) => Some(t)
        }
    }
}


#[derive(Debug)]
pub struct Stack<T: Debug>
{
    root: RefCell<Rc<Node<T>>>,
}

impl<T: Debug> Stack<T>
{
    fn new() -> Self
    {
        Self
        {
            root: RefCell::new(Rc::new(Node::Null))
        }
    }
    pub fn push(&self, data: T)
    {
        if self.is_empty() {
            *self.root.borrow_mut() = Rc::new(Node::new(None, data));
            return;
        }
        let first = self.root.borrow();
        let new_nod = Node::new(Some(RefCell::new(Rc::clone(&*first))), data);
        std::mem::drop(first);
        *self.root.borrow_mut() = Rc::new(new_nod)
    }
    pub fn pop(&self) -> Option<T>
    {
        if self.is_empty() {
            return None;
        }
        let first = Rc::clone(&*self.root.borrow());
        match &*first {
            Node::Null => None,
            Node::Exist(None, data) => {
                std::mem::drop(first);
                let first = self.root.replace(Rc::new(Node::Null));
                let result = Rc::try_unwrap(first).unwrap();
                Some(result.get_data().unwrap())
            }
            Node::Exist(Some(el), ..) =>
                {
                    *self.root.borrow_mut() = Rc::clone(&*el.borrow());
                    let result = Rc::try_unwrap(first).unwrap();
                    Some(result.get_data().unwrap())
                }
        }
    }

    pub fn is_empty(&self) -> bool
    {
        let nd = Rc::clone(&*self.root.borrow());
        match &*nd {
            Node::Null => true,
            _ => false,
        }
    }
}

pub struct GlobalStack<T: Debug>(Rc<Stack<T>>);

impl<T: Debug> GlobalStack<T>
{
    pub fn new() -> Self {
        GlobalStack(Rc::new(Stack::new()))
    }
}

impl<T: Debug> Deref for GlobalStack<T>
{
    type Target = Stack<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Debug> Clone for GlobalStack<T>
{
    fn clone(&self) -> Self {
        GlobalStack(Rc::clone(&self.0))
    }
}

#[cfg(test)]
mod test_stack {
    use crate::stack::GlobalStack;

    #[test]
    fn test() {
        let stack = GlobalStack::new();
        let stack2 = stack.clone();
        let stack3 = stack.clone();
        stack.push("str");
        stack.push("str2");
        stack.push("str3");
        println!("{:?}", stack.pop());
        println!("{:?}", stack.pop());
        println!("{:?}", stack.pop());
        println!("{:?}", stack.pop());
        println!("{:?}", stack.pop());
        stack.push("str");
        stack2.push("str2");
        stack3.push("str3");
        println!("{:?}", stack3.pop());
        println!("{:?}", stack2.pop());
        println!("{:?}", stack2.pop());
    }
}