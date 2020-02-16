extern crate nom;

mod parser;

use std::str::from_utf8;

fn main() {
    let input = b"{\"key1\":123,\"key2\":\"val123\"}";
    match parser::json_parser(input) {
        Ok(x) => {
            let kv: Vec<(&str, &str)> =
                x.1.iter()
                    .map(|i| (from_utf8(i.0).unwrap(), from_utf8(i.1).unwrap()))
                    .collect();

            println!("{:?}", kv);
        }
        Err(_) => println!("Something went wrong"),
    };
}
