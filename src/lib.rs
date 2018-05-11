extern crate regex;

use std::result;
use std::string::String;
use std::path::PathBuf;
use regex::Regex;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

type Result<T> = result::Result<T, String>;

pub struct Find {
    envs: Vec<String>,
    dirs: Vec<PathBuf>,
}

impl Find {
    pub fn execute(&self) -> Result<PathBuf> {
        unimplemented!();
    }

    pub fn search_env(&mut self, env: &str) -> &mut Find {
        unimplemented!();
    }

    // Search in
    pub fn search_dir(&mut self, dir: PathBuf) -> &mut Find {
        unimplemented!();
    }
}

// Get the version from a *.so.* file
pub fn parse_version(path: &PathBuf) -> Option<Vec<u64>> {
    unimplemented!();
}