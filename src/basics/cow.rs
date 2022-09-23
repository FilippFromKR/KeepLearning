use std::borrow::{BorrowMut, Cow};
use std::env;
use std::slice::from_raw_parts;

/// Use cases
/// 1)
// #[macro_use] extern crate serde_derive;
// extern crate serde_json;
//
// use std::borrow::Cow;
//
// #[derive(Debug, Deserialize)]
// struct Foo<'input> {
//     #[serde(borrow)]
//     bar: Cow<'input, str>,
// }
//
// fn main() {
//     let input = r#"{ "bar": "baz" }"#;
//     let foo: Foo = serde_json::from_str(input).unwrap();
//
//     match foo.bar {
//         Cow::Borrowed(x) => println!("borrowed {}", x),
//         Cow::Owned(x) => println!("owned {}", x),
//     }
// }
/// 2)
//
// fn describe(error: &Error) -> Cow<'static, str> {
//     match *error {
//         Error::OutOfMemory => "out of memory".into(),
//         Error::StackOverflow => "stack overflow".into(),
//         Error::MachineOnFire => "machine on fire".into(),
//         Error::Unfathomable => "machine bewildered".into(),
//         Error::FileNotFound(ref path) => {
//             format!("file not found: {}", path.display()).into()
//         }
use clap::{App, Arg, ArgMatches};

pub struct ConfigRead<'a>
{
    conf: Cow<'a, str>,
}

impl<'a> ConfigRead<'a>
{
    pub fn new() -> Self
    {
        Self
        {
            conf: Cow::from("/etc/app/app.conf"),
        }
    }
    pub fn print_config(&mut self) -> Result<(), String>
    {
       self.read()?;
        println!("{:?}", self.conf);
        Ok(())
    }

    fn read(&mut self) -> Result<(), String>
    {
        match env::var("APP_CONF")
        {
            Ok(path) => self.conf = Cow::from(path),
            _ => ()
        };
        let matches = App::new("rast_basics")
            .arg(Arg::with_name("conf")
                .long("conf")
                .required(false)
                .takes_value(true))
            .get_matches();

        match matches.get_one::<String>("conf")
        {
            Some(str) if str.is_empty() => return Err("Empty conf arg.".to_owned()),
            Some(str) => *self.conf.borrow_mut() = Cow::from(str.to_owned()),
            |
            None => (),
        }

        Ok(())
    }
}
#[cfg(test)]
mod test_cow {
    #[test]
    pub fn test(){
       let result = std::process::Command::new("target/debug/rast_basics").arg("-- --conf path").output();
       println!("{:?}",result.unwrap())
    }
}