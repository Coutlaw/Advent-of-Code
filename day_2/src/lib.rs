use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_sensitive: bool,
}

impl Config {
	pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
		// throw away first arg since its the program name
			args.next();

			// query string from Iterator
			let query = match args.next() {
				Some(arg) => arg,
				None => return Err("Didn't get a query string"),
			};
			// filename string form Iterator
			let filename = match args.next() {
				Some(arg) => arg,
				None => return Err("Didn't get a filename"),
			};

			// case sensitive boolean
			let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
	
			Ok(Config { query, filename, case_sensitive })
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
	let contents = fs::read_to_string(config.filename)?;

	let results = if config.case_sensitive {
		search(&config.query, &contents)
	} else {
		search_case_insensitive(&config.query, &contents)
	};

	for line in results {
		println!("{}", line);
	}

	Ok(())
}

// the lifetime is connected to the return value, since its a slice over contents
// we need the vector to live as long as the contents its a reference too!
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	contents.lines().filter(|line| line.contains(query)).collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	contents.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn case_sensitive() {
		let query = "duct";
		// over to the left to remove tabs from the string
		let contents = "\
Rust: 
safe, fast, productive.
Pick three.
Duct tape.";
// should fail to find duct tape because its case sensitive

		assert_eq!(vec!["safe, fast, productive."], search(query, contents));

	}

	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

		assert_eq!(
			vec!["Rust:", "Trust me."],
			search_case_insensitive(query, contents)
		);
	}
}