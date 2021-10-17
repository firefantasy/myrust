use clap::{Arg, App};
use regex::{Regex, Match};
use std::io;
use tokio::fs::File;
use tokio::io::{BufReader, AsyncBufReadExt};
use glob::glob;
use colored::*;

fn pprint(n: u32, content: &str, arr:Vec<Match>) {
    let mut last = 0;
    let mut format_str = "".to_owned();
    for i in 0..arr.len() {
        let start= arr[i].start();
        let end = arr[i].end();
        let word = &content[last..start][..];
        format_str.push_str(&format!("{}", word));
        let word = &content[start..end];
        format_str.push_str(&format!("{}", word.red()));
        last = end;
    }
    format_str.push_str(&format!("{}", &content[last..]));
    print!("{0: >6}: {1: <3}", n, format_str);
}

#[tokio::main]
async fn main()  -> io::Result<()> {
    let matches = App::new("my grep base on rust")
    .arg(Arg::with_name("pattern"))
    .arg(Arg::with_name("filename"))
    .get_matches();

    let pattern = matches.value_of("pattern").unwrap();
    let filename = matches.value_of("filename").unwrap();
    let re = Regex::new(pattern).unwrap();

    for entry in glob(&filename).expect("is a path") {
        match entry {
            Ok(path) => {
                let f = File::open(&path).await?;
                let mut reader = BufReader::new(f);
                let mut buf = String::new();
                let mut count = 0;
                println!("{}", path.display());
                loop {
                    reader.read_line(&mut buf).await?;
                    if buf.is_empty() {
                        break;
                    }
                    count = count + 1;
                    let line = buf.as_str();
                    
                    let arr: Vec<_> = re.find_iter(line).into_iter().collect();
                    if !arr.is_empty() {
                        pprint(count, line, arr);
                    }
                    buf.clear();
                }
            },
            Err(e) => println!("{:?}", e)
        }
    }

    Ok(())
}
