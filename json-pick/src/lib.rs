use std::fs::{metadata, File};
use std::io::{BufReader};
use std::path::{Path};
use serde_json::{from_reader, json, to_writer, Value};

fn prepare_output(path: &str) -> &Path {
    let p = Path::new(path);

    return p;
}

fn check_input(path: &str) -> &Path {
    match metadata(path.clone()) {
        Ok(m) => {
            if m.is_dir() {
                panic!("the input path {} to the file shouldn't point to a folder.", path);
            }
        }
        Err(e) =>
            if !Path::new(path).exists() {
                panic!("the error occurred with the output file: {}", e.to_string())
            }
    }

    return Path::new(path);
}

fn json_array_to_file(json: Vec<Value>, output_path: &Path) {
    let output_file = match File::create(output_path) {
        Err(why) => panic!("couldn't create {:?}: {}", output_path.to_str(), why),
        Ok(file) => file,
    };

    match to_writer(output_file, &json) {
        Err(why) => panic!("couldn't write to {:?}: {}", output_path.to_str(), why),
        Ok(_) => println!("successfully wrote to {:?}", output_path.to_str()),
    }
}

fn move_field(obj: &mut Value, d: &Value, field: &str) {
    obj[field] = Value::from(match d.get(field) {
        Some(v) => match v {
            Value::Null => Value::Null,
            Value::Bool(v) => Value::Bool(*v),
            Value::Number(v) => Value::Number(v.clone()),
            Value::String(v) => Value::String(v.to_string()),
            Value::Array(v) => Value::Array(v.to_vec()),
            Value::Object(v) => Value::Object(v.clone()),
        },
        _ => panic!("Wtf is this objects?")
    });
}

pub fn pick_fields(fields_str: &str, output_path: &str, input_path: &str) {
    let fields_split = fields_str.split(',');

    let fields: Vec<&str> = fields_split
        .collect::<Vec<&str>>()
        .iter()
        .map(|&s| s.trim())
        .collect::<Vec<&str>>();

    let input = check_input(input_path);
    let output = prepare_output(output_path);

    let file = match File::open(input) {
        Err(why) => panic!("couldn't read file {}: {}", input_path, why),
        Ok(f) => f
    };
    let reader = BufReader::new(file);
    let data: Vec<Value> = from_reader(reader).unwrap();

    let mut result: Vec<Value> = Vec::new();

    for d in data.iter() {
        let mut obj = json!({});

        for field in fields.iter() {
            move_field(&mut obj, d, field);
        }

        result.push(obj);
    }

    json_array_to_file(result, output);
}

pub fn pick_field(field: &str, output_path: &str, input_path: &str) {
    let input = check_input(input_path);
    let output = prepare_output(output_path);

    let file = match File::open(input) {
        Err(why) => panic!("couldn't read file {}: {}", input_path, why),
        Ok(f) => f
    };
    let reader = BufReader::new(file);
    let data: Vec<Value> = from_reader(reader).unwrap();

    let mut result: Vec<Value> = Vec::new();

    for d in data.iter() {
        let mut obj = json!({});

        move_field(&mut obj, d, field);

        result.push(obj);
    }

    json_array_to_file(result, output);
}
