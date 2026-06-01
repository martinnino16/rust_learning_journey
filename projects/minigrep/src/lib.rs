use std::{error::Error, fs, path::{Path, PathBuf}};

use clap::Parser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchMatch {
    pub path: PathBuf,
    pub line_number: usize,
    pub line: String,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    pattern: String,
    path: PathBuf,
    #[arg(short = 'i', long)]
    case_insensitive: bool,
    #[arg(short = 'r', long)]
    recursive: bool,
}

impl Config {
    pub fn new(
        pattern: impl Into<String>,
        path: impl Into<PathBuf>,
        case_insensitive: bool,
        recursive: bool
    ) -> Self {
        Self {
            pattern: pattern.into(),
            path: path.into(),
            case_insensitive,
            recursive
        } 
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let matches = if config.path.is_file() {
        search_in_file(&config, &config.path)?
    } else if config.path.is_dir() {
        if config.recursive {
            search_recursive(&config, &config.path)?
        } else {
            return Err("Path is a directory. use - r to search recursive".into());
        }
    } else {
        return Err("Invalid path".into());
    };
    print_matches(&matches); 
    Ok(())
}

pub fn search_in_file(config: &Config, path: &Path) -> Result<Vec<SearchMatch>, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    let results = if config.case_insensitive {
        search_case_insensitive(&config.pattern, &contents)
    } else {
        search(&config.pattern, &contents)
    };

    let matches = results
        .into_iter()
        .map(|(line_number, line)| SearchMatch {
            path: path.to_path_buf(),
            line_number,
            line: line.to_string(),
        })
    .collect();

    Ok(matches)
}

pub fn search_recursive(config: &Config, dir: &Path) -> Result<Vec<SearchMatch>, Box<dyn Error>> {
    let mut all_matches = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            all_matches.extend(search_recursive(config, &path)?);
        } else if path.is_file() {
            all_matches.extend(search_in_file(config, &path)?);
        }
    } 
    Ok(all_matches)
}

fn print_matches(matches: &[SearchMatch]) {
    for m in matches {
        println!("{}:{}: {}", m.path.display(), m.line_number, m.line);
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query) )
        .map(|(i, line)| (i+1, line))
        .collect()

}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let query = query.to_lowercase();

    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(&query))
        .map(|(i, line)| (i+1, line))
        .collect()

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::{Path, PathBuf};
    use tempfile::tempdir;

    fn write_file(dir: &Path, name: &str, contents: &str) -> PathBuf {
        let path = dir.join(name);
        fs::write(&path, contents).expect("failed to write test file");
        path
    }

    fn sort_matches(matches: &mut [SearchMatch]) {
        matches.sort_by(|a, b| {
            a.path
                .cmp(&b.path)
                .then(a.line_number.cmp(&b.line_number))
                .then(a.line.cmp(&b.line))
        });
    }

    #[test]
    fn case_sensitive() {
        let query = "pattern";
        let contents = "\
content of the file
 Rust is fast
This line has pattern
Another line
PATTERN in uppercase
pattern appears again";

        assert_eq!(
            vec![(3, "This line has pattern"), (6, "pattern appears again")],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
content of the file
 Rust is fast
This line has pattern
Another line
PATTERN in uppercase
pattern appears again";

        assert_eq!(
            vec![(2, " Rust is fast")],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn search_in_real_file_case_sensitive() {
        let temp = tempdir().unwrap();

        let file_path = write_file(
            temp.path(),
            "sample.txt",
            "\
hello
This line has pattern
PATTERN in uppercase
pattern appears again",
        );

        let config = Config::new("pattern", &file_path, false, false);

        let matches = search_in_file(&config, &file_path).unwrap();

        assert_eq!(
            vec![
                SearchMatch {
                    path: file_path.clone(),
                    line_number: 2,
                    line: "This line has pattern".to_string(),
                },
                SearchMatch {
                    path: file_path.clone(),
                    line_number: 4,
                    line: "pattern appears again".to_string(),
                },
            ],
            matches
        );
    }

    #[test]
    fn search_in_real_file_case_insensitive() {
        let temp = tempdir().unwrap();

        let file_path = write_file(
            temp.path(),
            "sample.txt",
            "\
hello
Rust is fast
rUsT is expressive
Go is simple",
        );

        let config = Config::new("rust", &file_path, true, false);

        let matches = search_in_file(&config, &file_path).unwrap();

        assert_eq!(
            vec![
                SearchMatch {
                    path: file_path.clone(),
                    line_number: 2,
                    line: "Rust is fast".to_string(),
                },
                SearchMatch {
                    path: file_path.clone(),
                    line_number: 3,
                    line: "rUsT is expressive".to_string(),
                },
            ],
            matches
        );
    }

    #[test]
    fn search_recursive_in_real_directory() {
        let temp = tempdir().unwrap();

        let src_dir = temp.path().join("src");
        let nested_dir = src_dir.join("nested");
        fs::create_dir_all(&nested_dir).unwrap();

        let main_rs = write_file(
            &src_dir,
            "main.rs",
            "\
fn main() {
    println!(\"pattern here\");
}",
        );

        let lib_rs = write_file(
            &nested_dir,
            "lib.rs",
            "\
pub fn demo() {
    let x = \"another pattern\";
}",
        );

        let config = Config::new("pattern", temp.path(), false, true);

        let mut matches = search_recursive(&config, temp.path()).unwrap();
        sort_matches(&mut matches);

        let mut expected = vec![
            SearchMatch {
                path: main_rs.clone(),
                line_number: 2,
                line: "    println!(\"pattern here\");".to_string(),
            },
            SearchMatch {
                path: lib_rs.clone(),
                line_number: 2,
                line: "    let x = \"another pattern\";".to_string(),
            },
        ];
        sort_matches(&mut expected);

        assert_eq!(expected, matches);
    }
}
