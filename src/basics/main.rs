use std::env;
use convert::EmailString;
use rast_basics::convert;
use rast_basics::cow::ConfigRead;

fn main()
{
    env::set_var("APP_CONF","Some");
    let mut reader = ConfigRead::new();
    reader.print_config();
}