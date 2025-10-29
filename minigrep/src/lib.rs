mod config;
mod result_ex;

use crate::config::Config;
use crate::result_ex::ResultEx;
use anyhow::Result;
use std::{env, fs};

pub fn run_using_env() -> Result<()> {
    run(env::args())
}

pub fn run<T>(args: T) -> Result<()>
where
    T: IntoIterator<Item = String>,
{
    Config::parse(args)
        .m_product(read_file)
        .map(|(cfg, contents)| {
            search(&cfg.pattern, &contents)
                .for_each(|result| println!("{result}"))
        })
}

fn read_file(cfg: &Config) -> Result<String> {
    Ok(fs::read_to_string(&cfg.target)?)
}

fn search<'a>(pattern: &'a str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    contents.lines().filter(move |l| l.contains(pattern))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents).collect::<Vec<&str>>());
    }
}
