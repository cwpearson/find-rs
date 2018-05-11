extern crate glob;
extern crate regex;

use glob::MatchOptions;
use std::result;
use std::string::String;
use std::path::{Path, PathBuf};
use std::env;
use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_soname_version() {
        assert_eq!(parse_version(&PathBuf::from("lib.so")), None);
        assert_eq!(parse_version(&PathBuf::from("lib.so9")), None);
        assert_eq!(parse_version(&PathBuf::from("lib.so.9.")), None);
    }

    #[test]
    fn soname_version() {
        assert_eq!(parse_version(&PathBuf::from("lib.so.9")).unwrap(), vec![9]);
        assert_eq!(
            parse_version(&PathBuf::from("lib.so.90.1")).unwrap(),
            vec![90, 1]
        );
    }
}

type Result<T> = result::Result<T, String>;

/// Returns a path to one of the supplied files if such a file can be found in the supplied directory.
fn contains(directory: &Path, files: &[String]) -> Option<PathBuf> {
    // Join the directory to the files to obtain our glob patterns.
    let patterns = files
        .iter()
        .filter_map(|f| directory.join(f).to_str().map(ToOwned::to_owned));

    // Prevent wildcards from matching path separators.
    let mut options = MatchOptions::new();
    options.require_literal_separator = true;

    // Collect any files that match the glob patterns.
    let mut matches = patterns
        .flat_map(|p| {
            if let Ok(paths) = glob::glob_with(&p, &options) {
                paths
                    .filter_map(|r| if let Ok(r) = r { Some(r) } else { None })
                    .collect()
            } else {
                vec![]
            }
        })
        .collect::<Vec<_>>();

    // Sort the matches by their version, preferring shorter and higher versions.
    matches.sort_by_key(|m| parse_version(m));
    matches.pop()
}

pub struct Find {
    targets: Vec<String>,
    env: Option<String>,
    patterns: Vec<String>,
}

impl Find {
    pub fn new(file: &str) -> Find {
        Find {
            targets: vec![file.to_owned()],
            env: None,
            patterns: vec![],
        }
    }

    pub fn or(&mut self, file: &str) -> &mut Find {
        self.targets.push(file.to_owned());
        self
    }

    // Search in a path provided by environment variable
    pub fn search_env(&mut self, env: &str) -> &mut Find {
        self.env = Some(env.to_owned());
        self
    }

    // Search a glob pattern
    pub fn search_glob(&mut self, pattern: &str) -> &mut Find {
        self.patterns.push(pattern.to_owned());
        self
    }

    // Do the search
    pub fn execute(&self) -> Result<PathBuf> {
        /// Searches the supplied directory and, on Windows, any relevant sibling directories.
        macro_rules! search_directory {
        ($directory: ident) => {
            if let Some(file) = contains(&$directory, &self.targets) {
                return Ok(file);
            }

            // On Windows, `libclang.dll` is usually found in the LLVM `bin` directory while
            // `libclang.lib` is usually found in the LLVM `lib` directory. To keep things
            // consistent with other platforms, only LLVM `lib` directories are included in the
            // backup search directory globs so we need to search the LLVM `bin` directory here.
            if cfg!(target_os = "windows") && $directory.ends_with("lib") {
                let sibling = $directory.parent().unwrap().join("bin");
                if let Some(file) = contains(&sibling, &self.targets) {
                    return Ok(file);
                }
            }
        };
    }

        // Search the directory provided by the relevant environment variable if it is set.
        if let &Some( ref env) = &self.env {
            if let Ok(directory) = env::var(env).map(|d| Path::new(&d).to_path_buf()) {
                search_directory!(directory);
            }
        }

        // Search the backup directories.
        for pattern in &self.patterns {
            eprintln!("Searching for {}", pattern);
            let mut options = MatchOptions::new();
            options.case_sensitive = false;
            options.require_literal_separator = true;
            if let Ok(paths) = glob::glob_with(pattern.as_str(), &options) {
                for path in paths.filter_map(|r| if let Ok(r) = r { Some(r) } else { None })
                .filter(|p| p.is_dir()) {
                    eprintln!("Looking in {:?}", path);
                    search_directory!(path);
                }
            }
        }

        let message = format!(
            "couldn't find any of [{}]",
            self.targets
                .iter()
                .map(|f| format!("'{}'", f))
                .collect::<Vec<_>>()
                .join(", ")
        );
        Err(message)
    }
}

// Get the version from a *.so.* file
pub fn parse_version(path: &PathBuf) -> Option<Vec<u64>> {
    let re = Regex::new(r".*\.so\.(\d+(\.\d+)*$)").unwrap();
    let path = path.to_str().unwrap();
    if let Some(caps) = re.captures(path) {
        let version = caps.get(1).unwrap().as_str();
        Some(
            version
                .split('.')
                .map(|s| s.parse::<u64>().unwrap_or(0))
                .collect(),
        )
    } else {
        None
    }
}
