use std::io;
use std::collections::HashMap;

type Passport = HashMap<String,String>;

fn parse_passport(s: &str) -> Passport {
    let mut result = Passport::new();

    for line in s.split("\n") {
        for entry in line.split(" ") {
            let key_value: Vec<&str> = entry.split(":").collect();

            result.insert(key_value[0].to_string(), key_value[1].to_string());
        }
    }

    result
}

fn validate_fields(p: &Passport) -> bool {
    let fields: Vec<String> = [
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
    ].iter().map(|x| x.to_string()).collect();
    
    for field in fields {
        if p.get(&field).is_none() { return false; }
    }

    true
}

fn validate_passport(p: &Passport) -> bool {
    if !validate_fields(p) { return false; }

    // birth year
    let byr = p.get(&"byr".to_string()).unwrap();
    if let Ok(birth_year) = byr.parse::<usize>() {
        if birth_year < 1920 || birth_year > 2002 { return false; }
    }
    else { return false; }

    // issue year
    let iyr = p.get(&"iyr".to_string()).unwrap();
    if let Ok(issue_year) = iyr.parse::<usize>() {
        if issue_year < 2010 || issue_year > 2020 { return false; }
    }
    else { return false; }

    // expiration year
    let eyr = p.get(&"eyr".to_string()).unwrap();
    if let Ok(expiration_year) = eyr.parse::<usize>() {
        if expiration_year < 2020 || expiration_year > 2030 { return false; }
    }
    else { return false; }

    // height
    let hgt = p.get(&"hgt".to_string()).unwrap();
    if hgt.contains("cm") {
        if let Ok(height_cm) = hgt.replace("cm","").parse::<usize>() {
            if height_cm < 150 || height_cm > 193 { return false; }
        }
        else { return false; }
    }
    else if hgt.contains("in") {
        if let Ok(height_in) = hgt.replace("in","").parse::<usize>() {
            if height_in < 59 || height_in > 76 { return false; }
        }
        else { return false; }
    }
    else { return false; }

    // hair color
    let hcl = p.get(&"hcl".to_string()).unwrap();
    if !hcl.contains("#") || hcl.len() != 7 { return false; }
    else if usize::from_str_radix(&hcl.replace("#",""), 16).is_err() { return false; }

    // eye color
    let ecl = p.get(&"ecl".to_string()).unwrap();
    let valid_colors: Vec<String> = [
        "amb",
        "blu",
        "brn",
        "gry",
        "grn",
        "hzl",
        "oth",
    ].iter().map(|x| x.to_string()).collect();
    if !valid_colors.contains(ecl) { return false; }

    // passport ID
    let pid = p.get(&"pid".to_string()).unwrap();
    
    pid.len() == 9 && pid.parse::<usize>().is_ok()
}

fn read_passports() -> Result<Vec<Passport>, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut passports = Vec::<Passport>::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        if size == 1 {
            passports.push(parse_passport(buffer.trim()));
            buffer.clear();
            continue;
        }
    }

    if buffer.len() > 0 {
        passports.push(parse_passport(buffer.trim()));
    }

    if passports.len() == 0 { Err(()) }
    else { Ok(passports) }
}

fn part1() {
    if let Ok(passports) = read_passports() {
        println!("{}", passports.iter().filter(|x| validate_fields(x)).count());
    }
    else { panic!("couldn't read passports!"); }
}

fn part2() {
    if let Ok(passports) = read_passports() {
        println!("{}", passports.iter().filter(|x| validate_passport(x)).count());
    }
    else { panic!("couldn't read passports!"); }
}

fn main() {
    // part1();
    part2();
}
