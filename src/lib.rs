// somewhat permissive
// cares only about the value tuples
// so as long as there's a "insert into x values", followed by tuples it wil parse
// doesn't matter if there's a semicolon at the end
//
// It's a little heavy duty...
// first pass is to break into lines
// next pass is to parse with csv, to handle escaping and quotations gracefully

extern crate csv;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use failure::Error;
use regex::Regex;


pub fn transform_sql_to_csv(input: &str) -> Result<String, Error> {
    // first match on the insert
    lazy_static! {
        static ref RE_INSERT: Regex = Regex::new("(?i)insert into(?-i).+(?i)values(?-i) ").unwrap();
    }

    let values = RE_INSERT.split(input).nth(1)
        .ok_or(format_err!("could not find an insert statement"))?;

    let values = values
        .trim_right()
        .trim_right_matches(';')
        .trim_right()
        .trim_right_matches(')');

    let mut buf = String::new();
    // hack to split on "),",
    // because those characters could be in a string
    for row in values.split("),") {
        let row = row.trim_left().trim_left_matches("(");
        buf.push_str(row);
        buf.push_str("\n");
    }
    println!("{:?}", buf);

    // now read buf into csv and then out again?
    // So that I get consistent double quoting


    Ok(input.to_owned())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic_sql() {
        let test_case = "INSERT INTO `page` VALUES (1,0,'April','',1,0,0,0.778582929065,'20140312223924','20140312223929',4657771,20236,0), (2,0,'August','',0,0,0,0.123830928525,'20140312221818','20140312221822',4360163,11466,0);";

        transform_sql_to_csv(test_case).unwrap();
        panic!();
    }
}
