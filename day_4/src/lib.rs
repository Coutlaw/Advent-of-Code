use std::error::Error;
use std::fs;
use std::env;
use std::collections::HashSet;

pub struct Config {
    pub filename: String,
    pub version_2: bool,
}

impl Config {
	pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
		// throw away first arg since its the program name
            args.next();
            
			// filename string form Iterator
			let filename = match args.next() {
				Some(arg) => arg,
				None => return Err("Didn't get a filename"),
            };
            let version_2 = env::var("V2").is_ok();
	
			Ok(Config { filename, version_2 })
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
	let required_fields: HashSet<&str> = 
		vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].into_iter().collect();

	let contents = fs::read_to_string(config.filename)?;
    
	let results: i32 = if config.version_2 {
		//TODO : search 2
		360
    } else {
        search(&contents, required_fields)
    };

	println!("{}", results);

	Ok(())
}

// Day 4 part 1
fn search(contents: &str, required: HashSet<&str>) -> i32 {

	let mut fields: HashSet<&str> = HashSet::new();
	let mut total = 0;

    for line in contents.lines() {
		let l = line.trim();
		if l.is_empty() {
			if required.difference(&fields).count() == 0 {
				total += 1;
			}
			fields.clear();
			continue
		}

		for field in l.split(" ").into_iter() {
			let value = field.split(":").into_iter().next().expect("bad format");
			if value  != "cid" && value != "" {
				fields.insert(value);
			}
		}
	}
    total
}

// Day 2 part 2
fn search2(contents: &str, required: HashSet<&str>) -> i32 {
    let mut fields: HashSet<&str> = HashSet::new();
	let mut total = 0;

    for line in contents.lines() {
		let l = line.trim();
		if l.is_empty() {
			if required.difference(&fields).count() == 0 {
				total += 1;
			}
			fields.clear();
			continue
		}

		for field in l.split(" ").into_iter() {
			let mut property = field.split(":").into_iter();
			let name = property.next().expect("bad format");
			let value = property.next().expect("bad format");

			match name {
				"byr" => {
					if value as i32 < 2002 && value as i32 > 1920 {
						
					}
				}
				_ => continue
			}
		}
	}
    total
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part_1() {
		let contents = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in

";
		let required_fields: HashSet<&str> = 
		vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].into_iter().collect();

		assert_eq!(2, search(contents, required_fields));
	}
	
	#[test]
	fn part_1_v2() {
		let contents = "\
byr:1937
eyr:2030 pid:154364481
hgt:158cm iyr:2015 ecl:brn hcl:#c0946f cid:155

cid:279
eyr:2029
pid:675014709 ecl:amb
byr:1985 hgt:179in hcl:z iyr:2025

iyr:2011 hgt:181cm hcl:#341e13 pid:282499883 byr:1953
eyr:2023
ecl:brn

eyr:2040 iyr:1984 pid:2371396209 byr:1951 cid:283 hgt:164cm
hcl:#623a2f

iyr:2014 byr:1966 hgt:153cm pid:900693718 eyr:2020 ecl:gry hcl:#866857

";

		let required_fields: HashSet<&str> = 
		vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].into_iter().collect();

		assert_eq!(4, search(contents, required_fields));
	}
}