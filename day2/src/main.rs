use regex::Regex;
use std::fs;

enum PasswordValidationMethod {
    Method1,
    Method2,
}

struct PasswordRule<'a> {
    min_count: u8,
    max_count: u8,
    char: &'a str,
    pw: &'a str,
}

impl PasswordRule<'_> {
    fn check(&self, method: PasswordValidationMethod) -> bool {
        return match method {
            PasswordValidationMethod::Method1 => {
                let found = self
                    .pw
                    .chars()
                    .filter(|c| c.to_string() == self.char)
                    .count() as u8;

                if found >= self.min_count && found <= self.max_count {
                    return true;
                }
                false
            }
            PasswordValidationMethod::Method2 => {
                if (self
                    .pw
                    .chars()
                    .nth(self.min_count as usize - 1)
                    .unwrap()
                    .to_string()
                    == self.char)
                    ^ (self
                        .pw
                        .chars()
                        .nth(self.max_count as usize - 1)
                        .unwrap()
                        .to_string()
                        == self.char)
                {
                    return true;
                }
                false
            }
        };
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let content_it = content.split("\n");
    let re = Regex::new(r"(\d+)-(\d+)\s*?(\w+):\s*(\w+)").unwrap();

    let mut valid_pw: u32 = 0;
    let mut valid_method2: u32 = 0;
    for (_i, line) in content_it.enumerate() {
        //println!("{}: {}", i, line);
        let m = re.captures(line).unwrap();
        let min_count = m.get(1).unwrap().as_str().parse::<u8>().unwrap();
        let max_count = m.get(2).unwrap().as_str().parse::<u8>().unwrap();
        let char = m.get(3).unwrap().as_str();
        let pw = m.get(4).unwrap().as_str();

        let pw_validator = PasswordRule {
            min_count,
            max_count,
            char,
            pw,
        };

        if pw_validator.check(PasswordValidationMethod::Method1) {
            valid_pw += 1;
        }
        if pw_validator.check(PasswordValidationMethod::Method2) {
            valid_method2 += 1;
        }
    }
    println!("valid_pw: {}", valid_pw);
    println!("valid_pw2: {}", valid_method2);
}
