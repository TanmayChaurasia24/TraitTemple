use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    password: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Temp {
    username: String,
    password: String
}

pub fn serde_eg() {
    let u = User {
        username: String::from("tanmay"),
        password: String::from("tanmay"),
    };

    // Serialize to JSON string
    let serialize_string = serde_json::to_string(&u);
    match serialize_string {
        Ok(json_stringg) => {
            println!("Struct is converted to string: {}", json_stringg);

            // Deserialize from JSON string back to User struct
            let deserialized_result: Result<Temp, _> = serde_json::from_str(&json_stringg);
            match deserialized_result {
                Ok(temp) => println!("Deserialized: {:?}", temp),
                Err(e) => println!("Deserialization error: {}", e),
            }
        }
        Err(_) => println!("Error while serializing struct!"),
    }
}
