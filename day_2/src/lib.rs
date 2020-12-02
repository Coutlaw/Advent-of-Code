use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
	pub filename: String,
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
	
			Ok(Config { filename })
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
	let contents = fs::read_to_string(config.filename)?;

	let results = search(&contents);

	println!("{}", results);

	Ok(())
}

// the lifetime is connected to the return value, since its a slice over contents
// we need the vector to live as long as the contents its a reference too!
fn search<'a>(contents: &'a str) -> i32 {
    let mut count = 0;

    for line in contents.lines() {
        // THIS IS GOING TO GET SHADY, its a quick and dirty solution plz don't judge
        // TODO: make this not be so bad
        let mut iter = line.split_whitespace();
        if let Some(i) = iter.next() {
            let mut numbs = i.split("-");
            let min = numbs.next().unwrap().parse::<usize>().unwrap();
            let max = numbs.next().unwrap().parse::<usize>().unwrap();
            if let Some(i) = iter.next() {
                let ch = i.chars().nth(0).unwrap();
                if let Some(i) = iter.next() {
                    let password = i;
                    let password_values = password.matches(ch).count();
        
                    if password_values >= min && password_values <= max {
                        count += 1;
                    }
                } else {
                    panic!("ahhhh");
                }
            } else {
                panic!("ahhhh");
            }
        } else {
            panic!("ahhhh");
        }

        
    }

    count
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn case_sensitive() {
		// over to the left to remove tabs from the string
		let contents = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";
// should fail to find duct tape because its case sensitive

		assert_eq!(2, search(contents));

	}
}