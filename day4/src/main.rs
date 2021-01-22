use std::collections::HashMap;
use std::fs;
use regex::{Regex, Captures};

struct Passport<'a> {
    data: HashMap<&'a str, &'a str>,
}

impl<'a> Passport<'a> {
    fn new() -> Passport<'a> {
        Passport {
            data: HashMap::new(),
        }
    }

    fn add(&mut self, key: &'a str, value: &'a str) {
        self.data.insert(key, value);
    }

    fn check(&self, keys: &Vec<&str>) -> bool {
        for k in keys {
            if !self.data.contains_key(k) ||
             !Passport::validate(k, self.data.get(k).unwrap()){
                return false;
            }
            //if !self.data.contains_key(k) {
            //    return false;
            //}
        }
        //println!("{:?}", &self.data);
        true
    }

    fn validate(key: &'a str, value: &'a str) -> bool {
        match key {
            "byr" => {
                let v = value.parse::<u16>();
                match v {
                    Err(_) => false,
                    Ok(x) => x >= 1920 && x <= 2002
                }
            },
            "iyr" => {
                let v = value.parse::<u16>();
                match v {
                    Err(_) => false,
                    Ok(x) => x >= 2010 && x <= 2020
                }
            },
            "eyr" => {
                let v = value.parse::<u16>();
                match v {
                    Err(_) => false,
                    Ok(x) => x >= 2020 && x <= 2030
                }
            },
            "hgt" => {
                let rx = Regex::new(r"^(\d*)(in|cm)$").unwrap();
                match rx.captures(value) {
                    None => false,
                    Some(x) => {
                        let s: u16 = x.get(1).unwrap().as_str().parse().unwrap();
                        let m = x.get(2).unwrap().as_str();
                        if m == "cm" {
                            s >= 150 && s <= 193
                        } else if m == "in" {
                            s >= 59 && s <= 76
                        } else {
                            false
                        }
                    },
                }
            },
            "hcl" => {
                let rx = Regex::new(r"^#[0-9a-f]{6,6}$").unwrap();
                rx.is_match(value)
            }
            "ecl" => {
                vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value)
            }
            "pid" => {
                let rx = Regex::new(r"^[0-9]{9,9}$").unwrap();
                rx.is_match(value)
            }
            "cid" => true,
            _ => false
            
        }
    }
}
fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let content_it = content.split("\r\n\r\n");

    let mut passports = Vec::new();

    let sep = Regex::new(r"\s+").unwrap();

    let keys_all = vec!["byr", "iyr", "eyr", "hgt",
                                      "hcl", "ecl", "pid"];
    for (i, it) in content_it.enumerate() {
        //println!("{}\n{}", i, it);
        let kv = sep.split(it).map(|x| x.split(':').collect::<Vec<&str>>());

        let mut passport = Passport::new();
        for i in kv {
            //println!("k: {}, v:{}", i[0], i[1]);
            passport.add(i[0], i[1]);
        }
        passports.push(passport);
    }

    let valid = passports.iter().filter(|x|x.check(&keys_all)).count();

    println!("valid: {}", valid);

}
