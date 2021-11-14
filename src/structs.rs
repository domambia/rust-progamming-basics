// Smillar to classes
/**
 * Structs - are used to create custom data types
 */

// Traditional Struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// Tuple Struct

// struct Color(u8, u8, u8);

// Person Struct
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Constructor
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get full name

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn setLastName(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // let mut color = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };

    // let mut color = Color(255, 0, 0);
    // color.0 = 233;
    // println!("Color: {} {} {}", color.0, color.1, color.2);

    let mut person = Person::new("Winnie", "Mutinda");
    println!("Person: {}", person.full_name());
    person.setLastName("Dauglous");
    println!("Person: {}", person.full_name());

    println!("Person: {:?}", person.to_tuple());
}
