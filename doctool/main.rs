use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Error as IOError;

mod plainformat;
mod htmlformat;

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let mut units = Vec::new();
    if args.len() == 0 {
        println!("no arguments were given.");
        println!("usage: doctool [files...]");
        return;
    }
    let mut htmlformatter = false;
    for filepath in args {
        if filepath == "--html" {
            htmlformatter = true;
        }
        let content = match read_file(filepath.clone()) {
            Ok(v) => v,
            Err(err) => {
                println!("error while reading file '{}': {}", filepath, err);
                return;
            }
        };
        let mut cur_units = match parse_units(content) {
            Ok(v) => v,
            Err(err) => {
                println!("error in file '{}': {}", filepath, err);
                return;
            }
        };
        units.append(&mut cur_units);
    }
    // println!("{:#?}", units);
    match htmlformatter {
        true => htmlformat::print_units(units),
        false => plainformat::print_units(units)
    }
}

fn read_file(path: String) -> Result<String, IOError> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    let _ = file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn parse_units(string: String) -> Result<Vec<DocUnit>, String> {
    let mut result = Vec::new();
    let mut buffer = String::new();
    let mut level = 0;
    let mut quoted = false;
    for ch in string.chars() {
        match ch {
            '\"' => {
                quoted = !quoted;
            },
            '(' => {
                if !quoted {
                    if level == 0 {
                        match DocUnit::from_string(buffer)? {
                            Some(v) => result.push(v),
                            None => {}
                        }
                        buffer = String::new();
                    }
                    level += 1;
                }
            },
            ')' => {
                if !quoted {
                    level -= 1;
                }
            },
            ch => {
                if level == 0 && !quoted {
                    buffer.push(ch);
                }
            }
        }
    }
    Ok(result)
}

// a type that contains an element and a description of that element
type ValDesc = (String, String);

// a documented function
#[derive(Debug)]
pub struct DocUnit {
    pub fn_name: String,
    pub arguments: Vec<ValDesc>,
    pub throws: Option<Vec<ValDesc>>,
    pub returns: Vec<ValDesc>,
    pub description: Option<String>
}

impl DocUnit {
    pub fn from_string(string: String) -> Result<Option<DocUnit>, String> {
        let fn_name = match DocUnit::parse_doc_property(&string, "function") {
            Ok(v) => v,
            Err(_) => return Ok(None)
        };
        let returns = DocUnit::parse_valdesc_table(DocUnit::parse_doc_property(&string, "returns")?)?;
        let description = match DocUnit::parse_doc_property(&string, "description") {
            Ok(v) => Some(v),
            Err(_) => None
        };
        let arguments = DocUnit::parse_valdesc_table(DocUnit::parse_doc_property(&string, "arguments")?)?;
        let throws = match DocUnit::parse_doc_property(&string, "throws") {
            Ok(v) => Some(DocUnit::parse_valdesc_table(v)?),
            Err(_) => None
        };

        Ok(Some(DocUnit {
            fn_name: fn_name,
            returns: returns,
            description: description,
            arguments: arguments,
            throws: throws
        }))
    }

    fn parse_doc_property(string: &String, prop_name: &'static str) -> Result<String, String> {
        let index = match string.find(&prop_name) {
            Some(i) => i,
            None => return Err(format!("could not find property '{}'.", prop_name))
        };
        let mut buffer = String::new();
        let char_iter = string.chars().skip(index);
        let mut active = false;
        for ch in char_iter {
            if ch == '@' {
                break;
            }
            if ch == ' ' && active == false {
                active = true;
            }
            else if !active {}
            else {
                buffer.push(ch);
            }
        }
        return Ok(beautify(buffer))
    }

    fn parse_valdesc_table(string: String) -> Result<Vec<ValDesc>, String> {
        let mut result = Vec::new();
        for split in string.split(';') {
            result.push(DocUnit::parse_valdesc(split.to_owned())?);
        }
        Ok(result)
    }

    fn parse_valdesc(string: String) -> Result<ValDesc, String> {
        let split = string.clone().split(":").map(|x| x.to_owned()).collect::<Vec<String>>();
        if split.len() != 2 && split.len() != 1 {
            return Err(format!("expected value and description seperated by ':',  found {} parts:\n{}", split.len(), string));
        }
        let val = split.get(0).unwrap().to_owned();
        let desc = match split.len() {
            1 => String::new(),
            2 => split.get(1).unwrap().to_owned(),
            _ => unreachable!()
        };
        Ok((beautify(val), beautify(desc)))
    }

}

fn beautify(string: String) -> String {
    string.trim().to_owned()
}