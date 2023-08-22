use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
	let contents = fs::read_to_string(config.file_path)?;
	// println!("With text: \n {contents}");
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
	Ok(())
}

pub fn knuth_morris_pratt(st: String, patten: String) -> Vec<usize> {
    if st.is_empty() || patten.is_empty() {
        return vec![];
    }

    let string = st.into_bytes();
    let pattern = patten.into_bytes();

    let mut next = vec![0];
    for i in 1..pattern.len() {
        let mut j = next[i - 1];
        while j > 0 && pattern[i] != pattern[j] {
            j = next[j - 1];
        }
        next.push(if pattern[j] == pattern[i] {j + 1} else {j});
    }

    let mut res = vec![];
    let mut j = 0;
    for (i, &x) in string.iter().enumerate() {
        while j > 0 && x != pattern[j] {
            j = next[j - 1];
        }
        if x == pattern[j] {
            j += 1;
        }
        if j == pattern.len() {
            res.push(i + 1 - j);
            // res.push(i - j + 1); 这样会报错 因为当边界条件出现时 比如：i=1 j=2
            j = next[j - 1];
        }
    }
    res

}

pub struct Config {
	pub query: String,
	pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
	pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
		args.next(); // 第一个参数是程序名，由于无需使用，因此这里直接空调用一次
		let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };
		let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path")
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
		Ok(Config {query, file_path, ignore_case})
	}
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut ans = vec![];
    for line in contents.lines() {
        let res = knuth_morris_pratt(String::from(line), String::from(query));
        if res != vec![] {
            ans.push(line);
        }
    }
    ans
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut ans = vec![];
    for line in contents.lines() {
        let res = knuth_morris_pratt(line.to_lowercase(), query.clone());
        if res != vec![] {
            ans.push(line);
        }
    }
    ans
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn case_sensitive() {
		let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, pro."], search(query, contents));
	}

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search_case_insensitive(query, contents));
    }

    #[test]
    fn case_knuth_morris_pratt() {
        let st = "abababc".to_string();
        let patten = "abc".to_string();
        let res = knuth_morris_pratt(st, patten);
        assert_eq!(res, vec![0, 2, 4]);
    }

    #[test]
    fn case_knuth_morris_pratt2() {
        let index = knuth_morris_pratt("abababa".to_string(), "ab".to_string());
        assert_eq!(index, vec![0, 2, 4]);
    }
	
}

