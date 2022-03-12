use std::fs::{File};
use std::io::Write;

use json_gen::generate;
use json_gen::json_template::JsonTemplate;

use rand::Rng;

use lipsum::lipsum;

use serde_json::{to_writer, to_writer_pretty};

const OBJECTS_ARRAY: &str = r#"
{
    "|id": "seq()",
    "|before": "str(5)",
    "|after": "str(10)",
    "|date": "dt()",
    "|score": "int()",
    "|exists": "bool()",
    "obj": {
        "|uid": "seq()"
    }
}
"#;

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

pub fn create_strings_array(length: usize, bounds: (usize, usize), output_file: &str) {
    let mut lines = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..length {
        lines.push(
            format!(
                "\"{}\"",
                lipsum(
                    rng.gen_range(bounds.0..bounds.1)
                )
            )
        );
    }

    let result = format!("[{}]", lines.join(","));

    write_string_to_file(output_file, &result)
}

pub fn create_ints_array(length: usize, bounds: (usize, usize), output_file: &str) {
    let mut ints = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..length {
        ints.push(
            rng.gen_range(bounds.0..bounds.1).to_string()
        );
    }

    let result = format!("[{}]", ints.join(","));

    write_string_to_file(output_file, &result)
}

fn write_string_to_file(output_file: &str, output: &str) {
    let mut file = match File::create(output_file) {
        Err(why) => panic!("couldn't create {}: {}", output_file, why),
        Ok(file) => file,
    };

    match file.write_all(output.as_ref()) {
        Err(why) => panic!("couldn't write to {}: {}", output_file, why),
        Ok(_) => println!("successfully wrote to {}", output_file),
    };
}
