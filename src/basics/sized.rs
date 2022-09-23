// https://github.com/pretzelhammer/rust-blog/blob/master/posts/sizedness-in-rust.md

use std::borrow::Borrow;

use clap::command;

use crate::dispatch::{User, UserRepo};

trait CommandHandler<C: Command + ?Sized>
{
    type Context: ?Sized;
    type Result;
    fn handle(&self, cmd: &C, ctx: &Self::Context) -> Self::Result;
}

pub trait Command
{}

impl Command for CreateUser {}

type CreateUser = dyn Fn(&str) -> Result<(), String>;

pub trait UserRepository {
    fn create_user(&self, id: u64, email: &str);
}


impl CommandHandler<CreateUser> for User {
    type Context = dyn UserRepository;
    type Result = Result<(), String>;

    fn handle(&self, cmd: &CreateUser, ctx: &Self::Context) -> Self::Result {
        match cmd(self.email.borrow()) {
            Ok(_) => ctx.create_user(self.id, self.email.borrow()),
            Err(err) => return Err(err)
        }
        Ok(())
    }
}

#[cfg(test)]
pub mod test_sized {
    use std::cell::RefCell;

    use crate::dispatch::User;

    use super::*;
    static  EMAIL: &str = "some@gmail.com";
    type MockRepo = RefCell<Vec<(u64, String)>>;

    impl UserRepository for MockRepo {
        fn create_user(&self, id: u64, email: &str) {
            self.borrow_mut().push((id, email.to_owned()))
        }
    }

    #[test]
    fn test()
    {
        let user = User::new(1, EMAIL);
       assert!(user.handle(&|str| Ok(()) ,&MockRepo::new(Vec::new())).is_ok());
    }
}

