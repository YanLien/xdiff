use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
    #[serde(flatten)]
    address: Address,
}

fn main() {
    let user = User {
        name: String::from("Alice"),
        age: 30,
        address: Address {
            street: String::from("123 Main St."),
            city: String::from("Anytown"),
        },
    };
    
    let json = serde_json::to_string(&user).unwrap();
    println!("{}", json);
    // {"name":"Alice","age":30,"street":"123 Main St.","city":"Anytown"}

    let deserialized_user: User = serde_json::from_str(&json).unwrap();
    println!("{:#?}", deserialized_user);
    // User {
    //     name: "Alice",
    //     age: 30,
    //     address: Address {
    //         street: "123 Main St.",
    //         city: "Anytown",
    //     },
    // }
}
