use std::env;
use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
	
	let contents = fs::read_to_string(config.file_path)?;

	let results = if config.ignore_case {
		search_case_insensitive(&config.query, &contents)
	} else {
		search_case_sensitive(&config.query, &contents)
	};

	for result in results {
		println!("{result}");
	}

	Ok(())
}

pub struct Config{
	pub query: String,
	pub file_path: String,
	pub ignore_case: bool
}

impl Config{
 	pub fn build(
		mut args: impl Iterator<Item = String>
	) -> Result<Config, &'static str>  {
		// ignore the program name in the first argument
		args.next();

		let query = match args.next() {
			Some(arg) => arg,
			None  	  => return Err("Didn't get a QUERY string")
		};

		let file_path = match args.next() {
			Some(arg) => arg,
			None  	  => return Err("Didn't get a FILE PATH")
		};

		let mut ignore_case = env::var("IGNORE_CASE").is_ok();
		
		ignore_case = match args.next() {
			Some(arg) => match arg.as_str() {
				"ci" => true,
				"CS" => false,
				_    => ignore_case
			},
			None  	  => ignore_case,
		};

		Ok(Config { query, file_path, ignore_case })
 	}
}

pub fn search_case_sensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
	contents
		.lines()
		.filter(|line| line.contains(query))
		.collect()
}

pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
	contents
		.lines()
		.filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
		.collect()
}

#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "\
Rust.
powerful, strong, productive.
Duct Tape.
Have a nice day.";

	assert_eq!(vec!["powerful, strong, productive."], search_case_sensitive(query, contents));
	}

	#[test]
	fn case_insensitive() {
		let query = "Duct";
		let contents = "\
Rust.
powerful, strong, productive.
DuCt tape!!!
Have a nice day.";

	assert_eq!(vec!["powerful, strong, productive.","DuCt tape!!!"], search_case_insensitive(query, contents));
	}
}
