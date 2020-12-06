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
    
	let results: i64 = if config.version_2 {

        //Right 1, down 1.
        let one = search2(&contents, 1, 1);
        println!("one: {}", one);
        //Right 3, down 1.
        let two = search2(&contents, 1, 3);
        println!("two: {}", two);
        //Right 5, down 1.
        let three = search2(&contents, 1, 5);
        println!("three: {}", three);

        //Right 7, down 1.
        let four = search2(&contents, 1, 7);
        println!("four: {}", four);

        //Right 1, down 2.
        let five = search2(&contents, 2, 1);
        println!("five: {}", five);

        one * two * three * four * five
    } else {
        search(&contents)
    };

	println!("{}", results);

	Ok(())
}

// Day 3 part 1
fn search(contents: &str) -> i64 {
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
fn search2(contents: &str, down: usize, right: usize) -> i64 {
    let mut trees_hit = 0;
    let mut total_right = right;
    let use_skip = down > 1;
    let mut skip = false;
    //let down = 1;

    for line in contents.lines().skip(1) { 
        if use_skip && skip {
            skip = false;
            continue;
        }
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
        if use_skip {
            skip = true;
        }
    }
    trees_hit
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
}