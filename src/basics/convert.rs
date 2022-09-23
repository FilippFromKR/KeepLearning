use std::borrow::BorrowMut;
use std::borrow::Cow;
/// crate ref_cast
/// usa cases:
// use ref_cast::RefCast;
//
// #[derive(RefCast)]
// #[repr(transparent)]
// struct U(String);
//
// fn main() {
//     let s = String::new();
//
//     // Safely cast from `&String` to `&U`.
//     let u = U::ref_cast(&s);
// }
/// be caryfull with derefMut
/// sounds like bad practice but still very useful and has sense for new types., so just thinks twice
use std::fmt::Debug;
use std::ops::Deref;

use rand::{Rng, thread_rng};
use regex::Regex;

#[derive(Debug)]
pub struct EmailString<'a>
{
    email: Cow<'a, str>,
}

impl<'a> EmailString<'a>
{
    fn is_valid(str: &str) -> bool
    {
        let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
        email_regex.is_match(str)
    }
}

impl<'a> TryFrom<&'a str> for EmailString<'a>
{
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error>
    {
        match EmailString::is_valid(value.as_ref())
        {
            true => Ok(
                Self
                {
                    email: Cow::from(value)
                }
            ),
            _ => Err(()),
        }
    }
}

impl TryFrom<String> for EmailString<'_>
{
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error>
    {
        match EmailString::is_valid(value.as_ref())
        {
            true => Ok(
                Self
                {
                    email: Cow::from(value)
                }
            ),
            _ => Err(()),
        }
    }
}


impl<'a> Deref for EmailString<'a>
{
    type Target = str;

    fn deref(&self) -> &Self::Target
    {
        &*self.email
    }
}

#[derive(Debug)]
pub struct RandomPointer<T: Debug> {
    elements: Vec<T>,
}

impl<T: Debug> RandomPointer<T> {
    pub fn new(el: T, el2: T, el3: T) -> Self {
        let mut vec = Vec::with_capacity(3);
        vec.push(el);
        vec.push(el2);
        vec.push(el3);
        Self
        {
            elements: vec
        }
    }
    fn random_n(&self) -> usize
    {
        let mut result = rand::thread_rng();
        let random_n = result.gen_range(0..3);
        random_n
    }
}

impl<T: Debug> TryFrom<Vec<T>> for RandomPointer<T> {
    type Error = ();

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err(());
        } else {
            Ok(Self
            {
                elements: value
            })
        }
    }
}

impl<T: Debug> AsRef<T> for RandomPointer<T> {
    fn as_ref(&self) -> &T {
        let rand_n = self.random_n();
        &self.elements[rand_n]
    }
}

impl<T: Debug> AsMut<T> for RandomPointer<T> {
    fn as_mut(&mut self) -> &mut T {
        let rand_n = self.random_n();
        &mut self.elements[rand_n]
    }
}

#[cfg(test)]
mod test_convert {
    use crate::convert::{EmailString, RandomPointer};

    #[test]
    fn valid_email()
    {
        let valid = "some@gmail.com";
        assert!(EmailString::try_from(valid.to_owned()).is_ok());
    }

    #[test]
    fn invalid_email()
    {
        let valid = "gleb_protasov";
        assert!(EmailString::try_from(valid.to_owned()).is_err());
    }

    #[test]
    fn unique_ref()
    {
        let mut pointer = RandomPointer::new("one", "Two", "Three");
        let random_el = pointer.as_mut();
        *random_el = "new_one";
        println!("{:?}", pointer)
    }
}