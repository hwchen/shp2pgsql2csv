extern crate failure;
extern crate shp2pgsql2csv;

use failure::Error;
use std::io::{self, Read};
use shp2pgsql2csv::transform_sql_to_csv;

fn main() -> Result<(), Error> {

    let mut buf = String::new();

    io::stdin().read_to_string(&mut buf)?;

    let csv = transform_sql_to_csv(&buf)?;
    println!("{}", csv);

    Ok(())
}
