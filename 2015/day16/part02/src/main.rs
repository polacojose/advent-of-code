use std::{collections::HashMap, fs, str::FromStr};
use struct_iterable::Iterable;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Iterable)]
struct MFCSAM {
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

#[derive(Debug)]
struct ParseMFCSAMError;
impl FromStr for MFCSAM {
    type Err = ParseMFCSAMError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^.+?:").unwrap();
        let binding = re.replace_all(s, "");
        let parts = binding.trim().split(",").collect::<Vec<&str>>();

        let mut fields: HashMap<String, u32> = HashMap::new();
        for part in parts {
            let binding = part.replace(" ", "");
            let (key, value) = binding.split_once(":").ok_or(ParseMFCSAMError)?;

            fields.insert(
                key.to_owned(),
                u32::from_str(value).map_err(|_| ParseMFCSAMError)?,
            );
        }

        Ok(MFCSAM {
            children: fields.get("children").copied(),
            cats: fields.get("cats").copied(),
            samoyeds: fields.get("samoyeds").copied(),
            pomeranians: fields.get("pomeranians").copied(),
            akitas: fields.get("akitas").copied(),
            vizslas: fields.get("vizslas").copied(),
            goldfish: fields.get("goldfish").copied(),
            trees: fields.get("trees").copied(),
            cars: fields.get("cars").copied(),
            perfumes: fields.get("perfumes").copied(),
        })
    }
}

lazy_static! {
    static ref READ_OUT: MFCSAM = {
        let file_string = fs::read_to_string("input-mfcsam.txt").unwrap();
        let master: MFCSAM = file_string.parse().unwrap();
        master
    };
    static ref AUNTS: Vec<MFCSAM> = {
        let file_string = fs::read_to_string("input-aunts.txt").unwrap();

        let mut aunts = Vec::new();
        for line in file_string.lines() {
            aunts.push(line.parse().unwrap());
        }

        aunts
    };
}

fn get_master_field(name: String) -> Option<u32> {
    for (field_name, field_value) in READ_OUT.iter() {
        if let Some(field_value) = field_value.downcast_ref::<Option<u32>>() {
            if field_name == name {
                return *field_value;
            }
        }
    }

    assert!(false);
    None
}

fn main() {
    for (i, aunt) in AUNTS.iter().enumerate() {
        let mut aunt_matches = true;
        for (field_name, field_value) in aunt.iter() {
            if let Some(&field_value) = field_value.downcast_ref::<Option<u32>>() {
                if let Some(field_value) = field_value {
                    let master_field_value = get_master_field(field_name.to_owned()).unwrap();

                    if field_name == "cats" || field_name == "trees" {
                        if field_value <= master_field_value {
                            aunt_matches = false;
                            break;
                        }
                    } else if field_name == "pomeranians" || field_name == "goldfish" {
                        if field_value >= master_field_value {
                            aunt_matches = false;
                            break;
                        }
                    } else if field_value != master_field_value {
                        aunt_matches = false;
                        break;
                    }
                }
            }
            if !aunt_matches {
                break;
            }
        }

        if aunt_matches {
            println!("Sue {}: {:?}", i + 1, aunt);
            break;
        }
    }
}
