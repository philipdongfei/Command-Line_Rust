use crate::EntryType::*;
use clap::{App, Arg};
use regex::Regex;
use std::error::Error;
use walkdir::DirEntry;
use walkdir::WalkDir;
use std::str::FromStr;


type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq )]
enum EntryType {
    Dir,
    File,
    Link,
}

impl FromStr for EntryType {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "d" => Ok(Dir),
            "f" => Ok(File),
            "l" => Ok(Link),
            _ => Err(format!("Invalid type").into()),
        }
    }
}


#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
    max_depth: Option<usize>,
    min_depth: Option<usize>,
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) => Ok(n),
        _ => Err(From::from(val)),
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("findr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust find")
        .arg(
            Arg::with_name("paths")
                .value_name("PATH")
                .help("Search paths")
                .default_value(".")
                .multiple(true),

        )
        .arg(
            Arg::with_name("names")
                .value_name("NAME")
                .short("n")
                .long("name")
                .help("Name")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("types")
                .value_name("TYPE")
                .short("t")
                .long("type")
                .help("Entry type")
                .possible_values(&["f", "d", "l"])
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("max_depth")
                .value_name("MAX_DEPTH")
                .long("maxdepth")
                .help("Max Depth")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("min_depth")
                .value_name("MIN_DEPTH")
                .long("mindepth")
                .help("Min Depth")
                .conflicts_with("max_depth")
                .takes_value(true),

        )
        .get_matches();
    /*
    let paths = matches.values_of_lossy("paths").unwrap();
    let names = matches.values_of_lossy("names").unwrap_or_default()
        .into_iter().map(|n| Regex::new(&n).unwrap_or_default()).collect();
    let types = matches.values_of_lossy("types").unwrap_or_default()
        .into_iter().map(|t| EntryType::from_str(&t).unwrap_or_default()).collect();
    */
    let names = matches
        .values_of_lossy("names")
        .map(|vals| {
            vals.into_iter()
                .map(|name| {
                    Regex::new(&name)
                        .map_err(|_| format!("Invalid --name \"{}\"", name))
                })
            .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .unwrap_or_default();

    // clap should disallow anything but "d," "f," or "l"
    let entry_types = matches
        .values_of_lossy("types")
        .map(|vals| {
            vals.iter()
                .map(|val| match val.as_str() {
                    "d" => Dir,
                    "f" => File,
                    "l" => Link,
                    _ => unreachable!("Invalid type"),
                })
                .collect()
        })
        .unwrap_or_default(); 
    let paths = matches.values_of_lossy("paths").unwrap();

    let max_depth = matches.value_of("max_depth")
            .map(parse_positive_int)
            .transpose()
            .map_err(|e| format!("illegal max depth count -- {}", e))?;

    let min_depth = matches.value_of("min_depth")
            .map(parse_positive_int)
            .transpose()
            .map_err(|e| format!("illegal min depth count -- {}", e))?;


    Ok(Config {
        paths ,
        names ,
        entry_types ,
        max_depth,
        min_depth,
    })

}

pub fn my_run(config: Config) -> MyResult<()> {
    for path in config.paths {
        let mut walker = WalkDir::new(path);
        if let Some(num_maxdepth) = config.max_depth {
            walker = walker.max_depth(num_maxdepth);
        } else if let Some(num_mindepth) = config.min_depth {
            walker = walker.min_depth(num_mindepth);
        }
        for entry in walker {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => {
                    let mut re_match = false;
                    let file_type = entry.file_type();
                    if !config.names.is_empty() {
                        for re in &config.names {
                            if !re.is_match(&entry.file_name().to_string_lossy()){
                                continue;
                            } else {
                                re_match = true;
                                break;
                            }
                        }
                    } else {
                        re_match = true;
                    }
                    if !re_match {
                        continue;
                    }
                    if config.entry_types.is_empty() {
                        println!("{}", entry.path().display());
                    } else {
                        for et in &config.entry_types {
                            match et {
                                Dir => {
                                    if file_type.is_dir() {
                                        println!("{}", entry.path().display());

                                    }

                                }
                                File => {
                                    if file_type.is_file() {
                                        println!("{}", entry.path().display());

                                    }

                                }
                                Link => {
                                    if file_type.is_symlink() {
                                        println!("{}", entry.path().display());
                                    } 

                                }
                            }
                        }

                    }
                }
            }
        }
    }
    Ok(())
}

pub fn run(config: Config) -> MyResult<()> {
    /*
    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => {
                    if (config.entry_types.is_empty()
                        || config.entry_types.iter().any(|entry_type| {
                            match entry_type {
                                Link => entry.file_type().is_symlink(),
                                Dir => entry.file_type().is_dir(),
                                File=> entry.file_type().is_file(),
                            }
                        }))
                        && (config.names.is_empty() 
                            || config.names.iter().any(|re| {
                                re.is_match(
                                    &entry.file_name().to_string_lossy(),
                                )
                            }))
                    {
                        println!("{}", entry.path().display());
                    }
                }
            }
        }
    }
    */
    // refactor this code
    let type_filter = |entry: &DirEntry| {
        config.entry_types.is_empty()
            || config
                .entry_types
                .iter()
                .any(|entry_type| match entry_type {
                    Link => entry.path_is_symlink(),
                    Dir => entry.file_type().is_dir(),
                    File => entry.file_type().is_file(),
                })
    };

    let name_filter = |entry: &DirEntry| {
        config.names.is_empty()
            || config
                .names
                .iter()
                .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in &config.paths {
        let mut walker = WalkDir::new(path);
        if let Some(num_maxdepth) = config.max_depth {
            walker = walker.max_depth(num_maxdepth);
        } else if let Some(num_mindepth) = config.min_depth {
            walker = walker.min_depth(num_mindepth);
        }

        let entries = walker//WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();
        println!("{}", entries.join("\n"));
            
    }

    Ok(())
}
