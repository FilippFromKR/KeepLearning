use std::borrow::{Borrow, Cow};
use std::cell::{Cell, Ref, RefCell};
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;
use std::pin::Pin;

trait Storage<K, V>
{
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

#[derive(Debug)]
pub struct User{
    pub id: u64,
    pub email: Cow<'static, str>,
    activated: bool,
}

impl User {
    pub fn new(id: u64, email: &'static str) -> Self {
        Self {
            id,
            email: Cow::from(email),
            activated: false,
        }
    }
}

pub struct UserRepo
{
    storage: Box<dyn Storage<u64, User>>,
}

impl UserRepo
{
    pub fn set(&mut self, key: u64, val: User)
    {
        self.storage.set(key, val)
    }

    pub fn get(&self, key: u64) -> Option<&User>
    {
        self.storage.get(&key)
    }

    fn remove(&mut self, key: u64) -> Option<User>
    {
        self.storage.remove(&key)
    }
}

type StorageLocal = HashMap<u64, User>;

impl Storage<u64, User> for StorageLocal
{
    fn set(&mut self, key: u64, val: User)
    {
        self.insert(key, val);
    }

    fn get(&self, key: &u64) -> Option<&User>
    {
        self.get(key)
    }

    fn remove(&mut self, key: &u64) -> Option<User> {
        self.remove(key)
    }
}

enum StaticStorage {
    Map(StorageLocal)
}

impl Storage<u64, User> for StaticStorage {
    fn set(&mut self, key: u64, val: User) {
        match self {
            Self::Map(st) => st.set(key, val),
        }
    }

    fn get(&self, key: &u64) -> Option<&User> {
        match self {
            Self::Map(st) => st.get(key),
        }
    }

    fn remove(&mut self, key: &u64) -> Option<User> {
        match self {
            Self::Map(st) => st.remove(key),
        }
    }
}


#[cfg(test)]
pub mod test_repo {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::marker::PhantomData;
    use std::rc::Rc;

    use crate::dispatch::{StaticStorage, Storage, StorageLocal, User, UserRepo};

    #[test]
    fn test() {
        let mut repo = UserRepo { storage: Box::new(StorageLocal::new()) };

        repo.set(1, User {
            id: 1,
            email: Default::default(),
            activated: false,
        });
        repo.set(2, User {
            id: 2,
            email: Default::default(),
            activated: false,
        });
        repo.set(3, User {
            id: 3,
            email: Default::default(),
            activated: false,
        });
        println!("{:?} ", repo.get(1));
        println!("{:?} ", repo.get(1));
        repo.set(1, User {
            id: 3,
            email: Default::default(),
            activated: false,
        });
        println!("{:?} ", repo.get(1));
    }

    #[test]
    fn test1() {
        let mut repo = StaticStorage::Map(StorageLocal::new());
        repo.set(1, User {
            id: 1,
            email: Default::default(),
            activated: false,
        });
        repo.set(2, User {
            id: 2,
            email: Default::default(),
            activated: false,
        });
        repo.set(3, User {
            id: 3,
            email: Default::default(),
            activated: false,
        });
        println!("{:?} ", repo.get(&1));
        println!("{:?} ", repo.get(&1));
        repo.set(1, User {
            id: 3,
            email: Default::default(),
            activated: false,
        });
        println!("{:?} ", repo.get(&1));
    }

    #[test]
    fn test_size() {
        let mut vec: Vec<()> = Vec::with_capacity(0);
        let mut counter: usize = 0;
        loop
        {
            counter += 1;
            vec.push(());
            if counter == 1
            {
                break;
            }
        }
        struct Unit;
        impl Unit {
            fn new() -> Self
            {
                Self
            }
            fn do_stuff() {
                println!("COUNTER {}", std::mem::size_of_val(&()));
            }
        }
        println!("COUNTER {}", std::mem::size_of_val(&()));
        println!("VEC {:?}", std::mem::size_of_val(&Unit::new()));
        println!("PHANTOM {:?}", std::mem::size_of_val::<PhantomData<Rc<()>>>(&std::marker::PhantomData::default()));
    }
}
