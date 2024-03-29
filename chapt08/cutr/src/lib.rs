use crate::Extract::*;
use clap::{App, Arg};
use regex::Regex;
use std::{
    error::Error, 
    fs::File,
    io::{self, BufRead, BufReader},
    num::NonZeroUsize,
    ops::Range
};
use csv::{ ReaderBuilder, StringRecord, WriterBuilder };

type MyResult<T> = Result<T, Box<dyn Error>>;
//type PositionList = Vec<Range<usize>>;

// Change PositionList to Enum, add RangeTo,RangeFrom type
#[derive(Debug)]
pub enum PositionList {
    RangeToPos(Vec<std::ops::RangeTo<usize>>),
    RangeFromPos(Vec<std::ops::RangeFrom<usize>>),
    RangePos(Vec<Range<usize>>),
}

#[derive(Debug)]
pub enum Extract {
    // PositionList: Vec<Range<usize>>
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    delimiter: u8,
    extract: Extract,
}

fn my_extract_fields(record: &StringRecord,
    field_pos: &[Range<usize>]) -> Vec<String> {
    let mut fields = Vec::<String>::new();
    for r in field_pos {
        for i in r.clone().collect::<Vec<_>>() {
            match record.get(i) {
                None => (),
                Some(f) => fields.push(f.to_string()),
            }        
        }
    }
    fields
}

// book one way
fn book1_extract_fields(
    record: &StringRecord,
    field_pos: &[Range<usize>],
) -> Vec<String> {
    field_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| record.get(i)))
        .map(String::from) // turn &str values into String values.
        .collect()
}

// book two way
fn extract_fields<'a>(
    record: &'a StringRecord,
    field_pos: &[Range<usize>],
) -> Vec<&'a str> {
    field_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| record.get(i)))
        .collect()
}


pub fn get_args() -> MyResult<Config> {
    let matches = App::new("cutr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cut")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("delimiter")
                .value_name("DELIMITER")
                .short("d")
                .long("delim")
                .default_value("\t")
                .help("Use delim as the field delimiter character instead of the tab character"),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTE")
                .short("b")
                .long("bytes")
                .conflicts_with_all(&["fields", "chars"])
                .help("select only these bytes"),
        )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .short("c")
                .long("chars")
                .conflicts_with_all(&["fields", "bytes"])
                .help("select only these characters"),
        )
        .arg(
            Arg::with_name("fields")
                .value_name("FIELDS")
                .short("f")
                .long("fields")
                .conflicts_with_all(&["chars", "bytes"])
                .help("select only these fields; also print any line that contains no delimiter character"),
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    /*
    let delim = matches
        .value_of("delimiter")
        .map(parse_delim_u8)
        .transpose()?;
        //.map_err(|e| format!("illegal delimiter -- {}", e))?;
    */
    let delimiter = matches.value_of("delimiter").unwrap();
    let delim_bytes = delimiter.as_bytes();
    if delim_bytes.len() != 1 {
        return Err(From::from(format!(
            "--delim \"{}\" must be a single byte",
            delimiter
        )));
    }

    let fields = matches.value_of("fields").map(parse_pos).transpose()?;
    let bytes = matches.value_of("bytes").map(parse_pos).transpose()?;
    let chars = matches.value_of("chars").map(parse_pos).transpose()?;

    let extract = if let Some(field_pos) = fields {
        Fields(field_pos)
    } else if let Some(byte_pos) = bytes {
        Bytes(byte_pos)
    } else if let Some(char_pos) = chars {
        Chars(char_pos)
    } else {
        return Err(From::from("Must have --fields, --bytes, or --chars"));
    };
    

    Ok(Config {
        files,
        delimiter: *delim_bytes.first().unwrap(),
        extract,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn parse_delim_u8(val: &str) -> MyResult<u8> {
    match val.parse() {
        Ok(n) => Ok(n),
        _ => Err(From::from(val)),
    }
}

pub fn parse_index(input: &str) -> Result<usize, String> {
    let value_error = || format!("illegal list value: \"{}\"", input);
    input
        .starts_with('+')
        .then(|| Err(value_error()))
        .unwrap_or_else(|| {
            input
                .parse::<NonZeroUsize>()
                .map(|n| usize::from(n) - 1)
                .map_err(|_| value_error())
        })
}

fn parse_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) => Ok(n),
        _ => Err(From::from(val)),
    }
}

fn parse_pos(range: &str) -> MyResult<PositionList> {
    let range_re = Regex::new(r"^(\d+)-(\d+)$").unwrap();
    let rangeto_re = Regex::new(r"^-(\d+)$").unwrap();
    let rangefrom_re = Regex::new(r"^(\d+)-$").unwrap();
    range
        .split(',')
        .into_iter()
        .map(|val| {
            // fix to parse PositionList(rangeto,rangefrom, range)
            parse_index(val).map(|n| { n..n+1 })
            .or_else(|e| {
                rangeto_re.captures(val).ok_or(e).and_then(|captures| {
                    let n = parse_index(&captures[1])?;
                    if n <= 0 {
                        return Err(format!("number:{} <= 0",n));
                    }
                    Ok(..n)
                })
            }) // parse -3 or 5-
            .or_else(|e| {
                rangefrom_re.captures(val).ok_or(e).and_then(|captures| {
                    let n = parse_index(&captures[1])?;
                    if n < 0 {
                        return Err(format!("number:{} < 0",n));
                    }
                    Ok(n..)
                })

            })
            .or_else(|e| {
                range_re.captures(val).ok_or(e).and_then(|captures| {
                    let n1 = parse_index(&captures[1])?;
                    let n2 = parse_index(&captures[2])?;
                    if n1 >= n2 {
                        return Err(format!(
                            "First number in range ({}) \
                            must be lower than second number ({})",
                            n1 + 1,
                            n2 + 1
                        ));
                    }
                    Ok(n1..n2 + 1)
                })
            })
        })
        // TO DO:  convert to MyResult<PositionList>
        .collect::<Result<_,_>>()
        .map(|r| PositionList::RangePos(r))
        .map_err(From::from)

    /*
    let VRanges: PositionList = val.split(',')
            .collect::<Vec<_>>()
            .iter()
            .map(|s| {


            })
            .collect().transpose()?.unwrap_or_default(); 
    */
    /*
    let vecRanges = val.split(',').collect::<Vec<_>>().iter().map(|r| {
        if let se = r.split('-').collect::<Vec<_>>() {
            let start = parse_int(&se[0]).unwrap();//(&se[0]).parse()::<i32>()?;
            let end = parse_int(&se[1]).unwrap(); //(&se[1]).parse()::<i32>()?;
            Range {
                start, 
                end
            }

        } else {
            let start = parse_int(&r).unwrap();//(&r).parse()::<i32>()?;
            let mut end = parse_int(&r).unwrap();//(&r).parse()::<i32>()?;
            end += 1;
            Range {
                start,
                end
            }
        } 
    }).collect();
    */
    
     
}

#[cfg(test)]
mod unit_tests {
    use super::{extract_bytes, extract_chars, extract_fields, parse_pos};
    use csv::StringRecord;


    #[test]
    fn test_extract_fields() {
        let rec = StringRecord::from(vec!["Captain", "Sham", "12345"]);
        assert_eq!(extract_fields(&rec, &[0..1]), &["Captain"]);
        assert_eq!(extract_fields(&rec, &[1..2]), &["Sham"]);
        assert_eq!(
            extract_fields(&rec, &[0..1, 2..3]),
            &["Captain", "12345"]
        );
        assert_eq!(extract_fields(&rec, &[0..1, 3..4]), &["Captain"]);
        assert_eq!(extract_fields(&rec, &[1..2, 0..1]), &["Sham", "Captain"]);
    }


    #[test]
    fn test_parse_pos() {
        // The empty string is an error
        assert!(parse_pos("").is_err());

        // Zero is an error
        let res = parse_pos("0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"", );

        let res = parse_pos("0-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"",);

        // A leading "+" is an error
        let res = parse_pos("+1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "illegal list value: \"+1\"",
        );

        let res = parse_pos("+1-2");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "illegal list value: \"+1-2\"",
        );

        let res = parse_pos("1-+2");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "illegal list value: \"1-+2\"",
        );

        // Any non-number is an error
        let res = parse_pos("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"",);

        let res = parse_pos("1,a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(),
            "illegal list value: \"a\"",);

        let res = parse_pos("1-a");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "illegal list value: \"1-a\"",
        );

        let res = parse_pos("a-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "illegal list value: \"a-1\"",
        );

        // Wonky ranges
        let res = parse_pos("-");
        assert!(res.is_err());

        let res = parse_pos(",");
        assert!(res.is_err());

        let res = parse_pos("1,");
        assert!(res.is_err());

        let res = parse_pos("1-");
        assert!(res.is_err());

        let res = parse_pos("1-1-1");
        assert!(res.is_err());

        let res = parse_pos("1-1-a");
        assert!(res.is_err());

        // First number must be less than second
        let res = parse_pos("1-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (1) must be lower than second number (1)"
        );

        let res = parse_pos("2-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (2) must be lower than second number (1)"
        );

        // All the following are acceptable
        let res = parse_pos("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("01");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("1,3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("001,0003");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1,2..3]);

        let res = parse_pos("1-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("0001-03");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("1,7,3-5");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 6..7, 2..5]);

        let res = parse_pos("15,19-20");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![14..15, 18..20]);

        
    }
}


pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => match &config.extract {
                Fields(field_pos) => {
                    let mut reader = ReaderBuilder::new()
                        .delimiter(config.delimiter)
                        .has_headers(false)
                        .from_reader(file);

                    let mut wtr = WriterBuilder::new()
                        .delimiter(config.delimiter)
                        .from_writer(io::stdout());
                    for record in reader.records() {
                        let record = record?;
                        match &field_pos {
                            PositionList::RangeToPos(field_pos_to) => {

                            },
                            PositionList::RangeFromPos(field_pos_from) => {
                            },
                            PositionList::RangePos(field_pos_pos) => {
                                wtr.write_record(extract_fields(
                                        &record, field_pos_pos,
                                ))?;

                            },
                        }
                        /*
                        wtr.write_record(extract_fields(
                                &record, field_pos,
                        ))?;
                        */
                    }

                }
                Bytes(byte_pos) => {
                    match &byte_pos {
                        PositionList::RangePos(range_byte_pos) => {
                            for line in file.lines() {
                                println!("{}", extract_bytes(&line?, &range_byte_pos));
                            }

                        }
                        _ => (),
                    }
                    /*
                    for line in file.lines() {
                        println!("{}", extract_bytes(&line?, range_byte));
                    }
                    */
                }
                Chars(char_pos) => {
                    match &char_pos {
                        PositionList::RangePos(range_char_pos) => {
                            for line in file.lines() {
                                println!("{}", extract_chars(&line?, &range_char_pos));
                            }

                        }
                        _ => (),
                    }
                    /*
                    for line in file.lines() {
                        println!("{}", extract_chars(&line?, range_char));
                    }
                    */
                }
            },
        }
    }
    Ok(())
}

fn my_extract_chars(line: &str, char_pos: &[Range<usize>]) -> String {
    let mut exchars = String::new();
    let charline: Vec<_> = line.chars().collect(); 
    for pos in char_pos {
        for index in pos.start..pos.end {
            if let Some(nthch) = charline.get(index) {
                exchars.push(*nthch);
            }
        }
    }
    exchars
}

// book
fn extract_chars(line: &str, char_pos: &[Range<usize>]) -> String {
    let chars: Vec<_> = line.chars().collect();
    let mut selected: Vec<char> = vec![];

    for range in char_pos.iter().cloned() {
        for i in range {
            // one way
            if let Some(val) = chars.get(i) {
                selected.push(*val)
            }
            // other way
            // selected.extend(range.filter_map(|i| chars.get(i)));
        }
    }
    selected.iter().collect()

    // this next version
    /*
     * let chars: Vec<_> = line.chars().collect();
     * char_pos
     *      .iter()
     *      .cloned()
     *      .map(|range| range.filter_map(|i| chars.get(i)))
     *      .flatten()
     *      .collect()
     */
    // used Iterator::filter_map
    /*
     * let chars: Vec<_> = line.chars().collect();
     * char_pos
     *      .iter()
     *      .cloned()
     *      .flat_map(|range| range.filter_map(|i| chars.get(i)))
     *      .collect()
     */
}


#[test]
fn test_extract_chars() {
    assert_eq!(extract_chars("", &[0..1]), "".to_string());
    assert_eq!(extract_chars("ábc", &[0..1]), "á".to_string());
    assert_eq!(extract_chars("ábc", &[0..1, 2..3]), "ác".to_string());
    assert_eq!(extract_chars("ábc", &[0..3]), "ábc".to_string());
    assert_eq!(extract_chars("ábc", &[2..3, 1..2]), "cb".to_string());
    assert_eq!(
        extract_chars("ábc", &[0..1, 1..2, 4..5]), 
        "áb".to_string()
    );

}

fn my_extract_bytes(line: &str, byte_pos: &[Range<usize>]) -> String {
    let byteline = line.as_bytes();
    let mut exbytes = Vec::<_>::new();
    for pos in byte_pos {
        for index in pos.start..pos.end 
        {
            if let Some(subbyte) = byteline.get(index) {
                exbytes.push(subbyte.clone());
            }
        }
    }        
    String::from_utf8_lossy(&exbytes).into_owned()

}

// book
fn extract_bytes(line: &str, byte_pos: &[Range<usize>]) -> String {
    let bytes = line.as_bytes();
    let selected: Vec<_> = byte_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| bytes.get(i)).copied())
        .collect();
    String::from_utf8_lossy(&selected).into_owned()
}

#[test]
fn test_extract_bytes() {
    //assert_eq!(extract_bytes("ábc", &[0..1]), "".to_string());
    assert_eq!(extract_bytes("ábc", &[0..2]), "á".to_string());
    assert_eq!(extract_bytes("ábc", &[0..3]), "áb".to_string());
    assert_eq!(extract_bytes("ábc", &[0..4]), "ábc".to_string());
    assert_eq!(extract_bytes("ábc", &[3..4, 2..3]), "cb".to_string());
    assert_eq!(extract_bytes("ábc", &[0..2, 5..6]), "á".to_string());
    
}


