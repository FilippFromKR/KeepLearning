use std::borrow::BorrowMut;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>);
}

trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}


impl<T: Debug> SayHi for Box<T> {}

impl<T: Debug> SayHi for Rc<T> {}

impl<T> MutMeSomehow for Rc<T> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        **self.get_mut().borrow_mut() = todo!();
    }
}

impl<T> MutMeSomehow for Box<T> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        // SAFETY: modifying T, doesn't move all struct
        let res = self.poll();
        unsafe { Pin::get_unchecked_mut(self) }.as_mut() = todo!();
    }
}

