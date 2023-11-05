// use clap::Parser;

// fn parse_int(s: &str) -> Result<i32, String> {
//     s.parse::<i32>().map_err(|e| e.to_string())
// }

// #[derive(Parser, Debug, Clone)]
// struct MyApp {
//     #[clap(short, long, value_parser = parse_int)]
//     number: i32,
// }

// fn main() {
//     let app = MyApp::parse();
//     println!("Number: {}", app.number);
// }

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    #[serde(flatten)]
    address: Address,
}

fn main() {
    // let data = r#"{
    //     "name": "John Doe",
    //     "age": 30,
    //     "street": "123 Main St",
    //     "city": "Example City"
    // }"#;
    
    let person = Person {
        name: "John Doe".to_string(),
        age: 30,
        address: Address {
            street: "123 Main St".to_string(),
            city: "Example City".to_string(),
        },
    };

    let serialized = serde_json::to_string(&person).unwrap();
    println!("{}", serialized);
}

// {
//     "name": "John Doe",
//     "age": 30,
//     "street": "123 Main St",
//     "city": "Example City"
// }