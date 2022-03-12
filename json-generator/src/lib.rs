use std::fs::{File};

use json_gen::generate;
use json_gen::json_template::JsonTemplate;

use serde_json::{to_writer, to_writer_pretty};

const OBJECTS_ARRAY: &str = "
{
    \"|id\": \"seq()\",
    \"|before\": \"str(5)\",
    \"|after\": \"str(10)\",
    \"|date\": \"dt()\",
    \"|score\": \"int()\",
    \"|exists\": \"bool()\"
}
";

pub fn create_objects_array(length: usize, output_file: &str, pretty: bool) {
    let mut json_template = JsonTemplate::from_str(OBJECTS_ARRAY, "|").unwrap();

    let generated_values = generate(&mut json_template, length, false, &mut vec![]);

    let file = match File::create(output_file) {
        Err(why) => panic!("couldn't create {}: {}", output_file, why),
        Ok(file) => file,
    };


    if pretty {
        match to_writer_pretty(file, &generated_values) {
            Err(why) => panic!("couldn't write to {}: {}", output_file, why),
            Ok(_) => println!("successfully wrote to {}", output_file),
        }
    } else {
        match to_writer(file, &generated_values) {
            Err(why) => panic!("couldn't write to {}: {}", output_file, why),
            Ok(_) => println!("successfully wrote to {}", output_file),
        }
    }
}
