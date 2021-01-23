mod parse;
use parse::parser::*;
use serde_json;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let page = parse::page::ParlerPage::from_html(&buffer).unwrap();
    //println!("{:#?}", page);
    serde_json::to_writer_pretty(io::stdout(), &page)?;
    Ok(())
}
