use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct UniqueString(String);

impl<'a> From<&'a UniqueString> for String {
    fn from(unique_string: &'a UniqueString) -> Self {
        unique_string.0.clone()
    }
}

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

    // Convert UniqueString back to String
    let my_string_as_string: String = (&my_string).into();
    println!("Converted back to String: {}", my_string_as_string);
}
