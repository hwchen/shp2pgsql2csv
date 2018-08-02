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
        static ref RE_CREATE_TABLE: Regex = Regex::new("(?i)create table(?-i).+?(?i)values(?-i) ").unwrap();
        static ref RE_INSERT: Regex = Regex::new("(?i)insert into(?-i).+?(?i)values(?-i) ").unwrap();
        static ref RE_END: Regex = Regex::new("(?i)commit;analyze(?-i)").unwrap();
    }

//    Save for future, for captruing create table?
//    let create_table_split = RE_CREATE_TABLE.splitn(input, 2).nth(1)
//        .ok_or(format_err!("no insert statements?"))?;

    // trim end commands

    let values = RE_INSERT.split(input).skip(1);

    let mut buf = String::new();
    // hack to split on "),",
    // because those characters could be in a string
    for row in values {
        // better to check in the loop or reallocate the string at the end?
        // probably makes no difference for this scale.
        let row = RE_END.splitn(row, 2).nth(0)
            .ok_or(format_err!("no row?"))?;

        let row = row
            .trim_left()
            .trim_left_matches("(")
            .trim_right()
            .trim_right_matches(");");
        buf.push_str(row);
        buf.push_str("\n");
    }

    // now read buf into csv and then out again?
    // So that I get consistent double quoting


    Ok(buf)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic_sql() {
        let test_case = "INSERT INTO `page` VALUES (1,0,'April','',1,0,0,0.778582929065,'20140312223924','20140312223929',4657771,20236,0), (2,0,'August','',0,0,0,0.123830928525,'20140312221818','20140312221822',4360163,11466,0);";

        let res = transform_sql_to_csv(test_case).unwrap();
        println!("{}", res);
        panic!();
    }

    #[test]
    fn test_shp2psql_output() {
        let test_case = r#"DROP TABLE IF EXISTS "shapes2017"."places";BEGIN;CREATE TABLE "shapes2017"."places" (gid serial,"statefp" varchar(2),"placefp" varchar(5),"placens" varchar(8),"geoid" varchar(7),"name" varchar(100),"namelsad" varchar(100),"lsad" varchar(2),"classfp" varchar(2),"pcicbsa" varchar(1),"pcinecta" varchar(1),"mtfcc" varchar(5),"funcstat" varchar(1),"aland" float,"awater" float,"intptlat" varchar(11),"intptlon" varchar(12));ALTER TABLE "shapes2017"."places" ADD PRIMARY KEY (gid);INSERT INTO "shapes2017"."places" ("statefp","placefp","placens","geoid","name","namelsad","lsad","classfp","pcicbsa","pcinecta","mtfcc","funcstat","aland","awater","intptlat","intptlon") VALUES ('01','25840','02403599','0125840','Fayette','Fayette city','25','C1','N','N','G4110','A','22143482','212108','+33.6942153','-087.8311690');INSERT INTO "shapes2017"."places" ("statefp","placefp","placens","geoid","name","namelsad","lsad","classfp","pcicbsa","pcinecta","mtfcc","funcstat","aland","awater","intptlat","intptlon") VALUES ('01','32536','02406632','0132536','Gu-Win','Gu-Win town','43','C1','N','N','G4110','A','5028328','0','+33.9443187','-087.8703679');INSERT INTO "shapes2017"."places" ("statefp","placefp","placens","geoid","name","namelsad","lsad","classfp","pcicbsa","pcinecta","mtfcc","funcstat","aland","awater","intptlat","intptlon") VALUES ('01','02908','02403122','0102908','Ashville','Ashville city','25','C1','N','N','G4110','A','49762931','487107','+33.8351967','-086.2700148');INSERT INTO "shapes2017"."places" ("statefp","placefp","placens","geoid","name","namelsad","lsad","classfp","pcicbsa","pcinecta","mtfcc","funcstat","aland","awater","intptlat","intptlon") VALUES ('01','46696','02406094','0146696','Margaret','Margaret town','43','C1','N','N','G4110','A','25438710','39224','+33.6728398','-086.4639420');INSERT INTO "shapes2017"."places" ("statefp","placefp","placens","geoid","name","namelsad","lsad","classfp","pcicbsa","pcinecta","mtfcc","funcstat","aland","awater","intptlat","intptlon") VALUES ('01','56400','02407036','0156400','Odenville','Odenville town','43','C1','N','N','G4110','A','36176037','241048','+33.7008754','-086.4233712');COMMIT;ANALYZE "shapes2017"."places";"#;

        let res = transform_sql_to_csv(test_case).unwrap();
        println!("{}", res);
        panic!();
    }
}
