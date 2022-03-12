use json_generator::{create_ints_array, create_objects_array, create_strings_array};

fn main() {
    println!("Hello, world!");

    create_objects_array(10, "/Users/opa_oz/Desktop/result-objects.json", true);
    create_strings_array(10, (5, 10), "/Users/opa_oz/Desktop/result-strings.json");
    create_ints_array(10, (5, 10), "/Users/opa_oz/Desktop/result-ints.json");
}
