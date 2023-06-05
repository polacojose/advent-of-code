use std::fs;

use json::number::Number;

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();

    for line in file_string.lines() {
        let parsed = json::parse(line).unwrap();

        let total_value = match parsed {
            json::JsonValue::Object(object) => get_object_value(&object),
            json::JsonValue::Array(array) => get_array_value(&array.iter().collect()),
            _ => 0,
        };

        println!("{}", total_value);
    }
}

fn get_array_value(array: &Vec<&json::JsonValue>) -> i32 {
    let mut total_value: i32 = 0;
    for item in array.iter() {
        let member_value = match item {
            json::JsonValue::Null => 0,
            json::JsonValue::Short(_) => 0,
            json::JsonValue::String(_) => 0,
            json::JsonValue::Number(number) => {
                let number: f64 = (*number as Number).into();
                number as i32
            }
            json::JsonValue::Boolean(_) => 0,
            json::JsonValue::Object(object) => get_object_value(object),
            json::JsonValue::Array(array) => get_array_value(&array.iter().collect()),
        };
        total_value += member_value;
    }
    total_value as i32
}

fn get_object_value(object: &json::object::Object) -> i32 {
    let mut total_value = 0;
    for (_, item) in object.iter() {
        let member_value = match item {
            json::JsonValue::Null => 0,
            json::JsonValue::Short(short) => {
                if short == "red" {
                    return 0;
                } else {
                    0
                }
            }
            json::JsonValue::String(string) => {
                if string == "red" {
                    return 0;
                } else {
                    0
                }
            }
            json::JsonValue::Number(number) => {
                let number: f64 = (*number as Number).into();
                number as i32
            }
            json::JsonValue::Boolean(_) => 0,
            json::JsonValue::Object(object) => get_object_value(object),
            json::JsonValue::Array(array) => get_array_value(&array.iter().collect()),
        };
        total_value += member_value;
    }
    total_value
}
