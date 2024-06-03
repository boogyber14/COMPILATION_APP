use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash, Clone)] // Added Clone trait
struct UniqueString(String);

impl<'a> Into<UniqueString> for &'a str {
    fn into(self) -> UniqueString {
        UniqueString(self.to_string())
    }
}

fn main() {
    let mut unique_strings: HashSet<UniqueString> = HashSet::new();

    let my_str = "Hello, world!";
    let my_string: UniqueString = my_str.into();

    if unique_strings.insert(my_string.clone()) {
        println!("Inserted: {:?}", my_string);
    } else {
        println!("String already exists: {:?}", my_string);
    }

    let my_str2 = "Hello, world!";
    let my_string2: UniqueString = my_str2.into();

    if unique_strings.insert(my_string2.clone()) {
        println!("Inserted: {:?}", my_string2);
    } else {
        println!("String already exists: {:?}", my_string2);
    }
}
