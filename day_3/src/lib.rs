use std::error::Error;
use std::fs;
use std::env;

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
	let contents = fs::read_to_string(config.filename)?;
    
	let results = if config.version_2 {
        search2(&contents)
    } else {
        search(&contents)
    };

	println!("{}", results);

	Ok(())
}

// Day 3 part 1
// the lifetime is connected to the return value, since its a slice over contents
fn search<'a>(contents: &'a str) -> i32 {
    let mut trees_hit = 0;
    let mut total_right = 3;
    let right = 3;
    //let down = 1;

    for line in contents.lines().skip(1) { 

        let new_line: String = {
            if total_right > line.len() - 1 {
                if line.contains("--->") {
                    let repeate_num_times = line.len() / total_right;
                    let mut iter = line.split_whitespace();
                    if let Some(val) = iter.next() {
                        String::from(val.repeat(repeate_num_times))
                    } else {
                        String::from(line)
                    }
                } else {
                    total_right = total_right - line.len();
                    String::from(line)
                }
            } else {
                String::from(line)
            }
        };

        // traverse line
        if let Some(ch) = new_line.chars().nth(total_right) {
            if ch == '#' {
                trees_hit += 1;
            }
        } else {
            panic!("ahhhhh")
        }
        total_right += right;
    }
    trees_hit
}

// Day 2 part 2
// the lifetime is connected to the return value, since its a slice over contents
// we need the vector to live as long as the contents its a reference too!
fn search2<'a>(contents: &'a str) -> i32 {
    
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part_1() {
		let contents = "\
..##.........##.........##.........##.........##.........##.......  --->
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
";

		assert_eq!(7, search(contents));

    }
    
    #[test]
	fn part_2() {
		let contents = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";

		assert_eq!(1, search2(contents));

	}
}