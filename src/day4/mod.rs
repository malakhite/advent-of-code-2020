use std::{error};

#[derive(Debug)]
struct Passport {
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<u32>,
}

impl Passport {
    fn from_string(s: String) -> Passport {
        let parts: Vec<&str> = s.split_whitespace().collect::<Vec<&str>>();
        let byr: Option<u32> = match parts.iter().find(|&&s| s.starts_with("byr:")) {
            Some(byr) => match byr.strip_prefix("byr:") {
                Some(byr) => match byr.parse::<u32>() {
                    Ok(byr) => Some(byr),
                    Err(why) => panic!("Unable to parse {} to u32: {}", byr, why)
                },
                None => None,
            },
            None => None,
        };
        let iyr: Option<u32> = match parts.iter().find(|&&s| s.starts_with("iyr:")) {
            Some(iyr) => match iyr.strip_prefix("iyr:") {
                Some(iyr) => match iyr.parse::<u32>() {
                    Ok(iyr) => Some(iyr),
                    Err(why) => panic!("Unable to parse {} to u32: {}", iyr, why)
                },
                None => None,
            },
            None => None,
        };
        let eyr: Option<u32> = match parts.iter().find(|&&s| s.starts_with("eyr:")) {
            Some(eyr) => match eyr.strip_prefix("eyr:") {
                Some(eyr) => match eyr.parse::<u32>() {
                    Ok(eyr) => Some(eyr),
                    Err(why) => panic!("Unable to parse {} to u32: {}", eyr, why)
                },
                None => None,
            },
            None => None,
        };
        let cid: Option<u32> = match parts.iter().find(|&&s| s.starts_with("cid:")) {
            Some(cid) => match cid.strip_prefix("cid:") {
                Some(cid) => match cid.parse::<u32>() {
                    Ok(cid) => Some(cid),
                    Err(why) => panic!("Unable to parse {} to u32: {}", cid, why)
                },
                None => None,
            },
            None => None,
        };
        let hgt: Option<String> = match parts.iter().find(|&&s| s.starts_with("hgt:")) {
            Some(hgt) => match hgt.strip_prefix("hgt:") {
                Some(hgt) => match hgt.parse::<String>() {
                    Ok(hgt) => Some(hgt),
                    Err(why) => panic!("Unable to parse {} to String: {}", hgt, why)
                },
                None => None,
            },
            None => None,
        };
        let hcl: Option<String> = match parts.iter().find(|&&s| s.starts_with("hcl:")) {
            Some(hcl) => match hcl.strip_prefix("hcl:") {
                Some(hcl) => match hcl.parse::<String>() {
                    Ok(hcl) => Some(hcl),
                    Err(why) => panic!("Unable to parse {} to String: {}", hcl, why)
                },
                None => None,
            },
            None => None,
        };
        let ecl: Option<String> = match parts.iter().find(|&&s| s.starts_with("ecl:")) {
            Some(ecl) => match ecl.strip_prefix("ecl:") {
                Some(ecl) => match ecl.parse::<String>() {
                    Ok(ecl) => Some(ecl),
                    Err(why) => panic!("Unable to parse {} to String: {}", ecl, why)
                },
                None => None,
            },
            None => None,
        };
        let pid: Option<String> = match parts.iter().find(|&&s| s.starts_with("pid:")) {
            Some(pid) => match pid.strip_prefix("pid:") {
                Some(pid) => match pid.parse::<String>() {
                    Ok(pid) => Some(pid),
                    Err(why) => panic!("Unable to parse {} to String: {}", pid, why)
                },
                None => None,
            },
            None => None,
        };

        Passport {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
            cid,
        }
    }

    fn validate(&self) -> bool {
        if self.byr == None || self.iyr == None || self.eyr == None || self.hgt == None || self.hcl == None || self.ecl == None || self.pid == None {
            false
        } else {
            true
        }
    }

    fn validate_data(&self) -> bool {
        let mut valid: bool;
        match self.byr {
            Some(byr) => if byr < 1920 || byr > 2002 {
                // println!("byr fails: {}", byr);
                valid = false
            } else {
                valid = true
            },
            None => valid = false,
        };

        if valid {
            match self.iyr {
                Some(iyr) => if iyr < 2010 || iyr > 2020 {
                    // println!("iyr fails: {}", iyr);
                    valid = false
                } else {
                    valid = true
                },
                None => valid = false,
            };
        }

        if valid {
            match self.eyr {
                Some(eyr) => if eyr < 2020 || eyr > 2030 {
                    // println!("eyr fails: {}", eyr);
                    valid = false
                } else {
                    valid = true
                },
                None => valid = false,
            };
        }

        if valid {
            match &self.hgt {
                Some(hgt) => {
                    let (num, unit) = hgt.split_at(hgt.len() - 2);
                    let (min, max) = if unit == "cm" {
                        (150, 193)
                    } else {
                        (59, 76)
                    };
                    let parsed = match num.parse::<u32>() {
                        Err(_) => 0,
                        Ok(i) => i,
                    };
                    if parsed < min || parsed > max {
                        // println!("hgt fails: {}", hgt);
                        valid = false
                    } else {
                        valid = true
                    }
                },
                None => valid = false,
            };
        }

        if valid {
            match &self.hcl {
                Some(hcl) => {
                    if hcl.starts_with('#') && hcl.len() == 7 {
                        let valid_chars: Vec<char> = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f'];
                        let mut chars: Vec<char> = hcl.chars().collect();
                        chars.remove(0);
                        for c in chars {
                            if valid && valid_chars.contains(&c) {
                            } else {
                                // println!("hcl fails: {}", hcl);
                                valid = false
                            }
                        }
                    } else {
                        // println!("hcl fails: {}", hcl);
                        valid = false
                    }
                },
                None => valid = false,
            }
        }

        if valid {
            match &self.ecl {
                Some(ecl) => {
                    let valid_ecl: Vec<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                    if valid_ecl.contains(&ecl.as_str()) {
                        valid = true
                    } else {
                        // println!("ecl fails: {}", ecl);
                        valid = false
                    }
                },
                None => valid = false,
            }
        }

        if valid {
            match &self.pid {
                Some(pid) => {
                    if pid.len() == 9 {
                        match pid.parse::<i32>() {
                            Ok(_) => valid = true,
                            Err(_) => {
                                // println!("pid fails: {}", pid);
                                valid = false
                            },
                        }
                    } else {
                        // println!("pid fails: {}", pid);
                        valid = false
                    }
                },
                None => valid = false
            }
        }

        valid
    }
}

pub fn part1() -> Result<(), Box<dyn error::Error>> {
    let file = String::from("./puzzle-input/day4.txt");
    let input = super::util::read_file(&file)?;
    let records: Vec<&str> = input.split("\n\n").collect();
    let record_count = records.len();
    let mut passports: Vec<Passport> = Vec::new();
    for record in records {
        passports.push(Passport::from_string(record.parse::<String>()?));
    }
    let mut valid: u32 = 0;
    for passport in passports {
        if passport.validate() {
            valid += 1;
        }
    }

    println!("There are {} valid passports out of {} records", valid, record_count);
    Ok(())
}

pub fn part2() -> Result<(), Box<dyn error::Error>> {
    let file = String::from("./puzzle-input/day4.txt");
    let input = super::util::read_file(&file)?;
    let records: Vec<&str> = input.split("\n\n").collect();
    let record_count = records.len();
    let mut passports: Vec<Passport> = Vec::new();
    for record in records {
        passports.push(Passport::from_string(record.parse::<String>()?));
    }
    let mut valid: u32 = 0;
    for passport in passports {
        if passport.validate_data() {
            valid += 1;
        } else {
            // println!("Invalid passport: {:?}", passport)
        }
    }

    println!("There are {} valid passports out of {} records", valid, record_count);
    Ok(())
}