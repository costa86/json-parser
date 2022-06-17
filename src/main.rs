use clap::Parser;
use serde_json::{from_str, Value};
use std::fs;

#[derive(Parser, Debug)]
#[clap(author, version, about = "Json file parser")]
struct Args {
    #[clap(short, long, help = "Json file to parse", value_name = "FILE")]
    filename: String,
    #[clap(
        short,
        long,
        help = "Property to search for. Use '.' for nested properties and/or array indexes",
        value_name = "TEXT"
    )]
    keys: String,
}

fn get_result<'a>(file_content: &'a Value, keys: &'a str) -> &'a Value {
    if keys == "." {
        return file_content;
    }
    let keys: Vec<&str> = keys.split(".").collect();

    let mut res = file_content;

    for i in keys {
        res = match res.get(i) {
            Some(x) => x,
            None => {
                let index: usize = i.trim().parse().expect("Could not use index a number");
                res.get(index).expect("Did not find index")
            }
        }
    }
    res
}

fn main() {
    let args = Args::parse();
    let file_content = fs::read_to_string(&args.filename).expect("File not found");
    let file_content: Value = from_str(&file_content).expect("Could not parse file content");
    let res = get_result(&file_content, &args.keys);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use crate::get_result;
    use serde_json::from_str;

    #[test]
    fn test_get_result() {
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
        let content = from_str(&data).unwrap();
        let result_1 = get_result(&content, "age");
        let result_2 = get_result(&content, "phones.0");

        assert_eq!(result_1, 43);
        assert_eq!(result_2, "+44 1234567");
    }
}
