use lazy_static::lazy_static;
use reqwest;
use regex::Regex;
use csv::Writer;
use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize)]
struct DigitalCurrencyAddress<'a> {
    address_type: &'a str,
    address: &'a str,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let body = get_sdn_list_text()?;

    parse_digital_currency_address(body.as_str());

    Ok(())
}

fn get_sdn_list_text() -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://www.treasury.gov/ofac/downloads/sdnlist.txt")?.text()?;

    Ok(body)
}

fn parse_digital_currency_address(text: &str) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(Digital[\s]{0,}Currency[\s]{0,}Address)[\s]{0,}-[\s]{0,}([\w]{3,4})[\s]{0,}([\w-]+)").unwrap();
    }
    let lines: Vec<DigitalCurrencyAddress> = RE.captures_iter(text).map(|cap| {

        let address_type_result = cap.get(2);
        let address_type = match address_type_result {
            Some(t) => t.as_str(),
            None => "",
        };
        let address_result = cap.get(3);
        let address = match address_result {
            Some(a) => a.as_str(),
            None => "",
        };

        DigitalCurrencyAddress {
            address_type,
            address,
        }
        
    }).collect();   

    let mut csvWriter = Writer::from_writer(vec![]);
    for line in lines {
        csvWriter.serialize(line);
    }

    let data = String::from_utf8(csvWriter.into_inner().unwrap()).unwrap();

    let mut file = File::create("address.csv").unwrap();
    file.write_all(data.as_bytes());
}

fn print_type_of<T>(_: T) {
    println!("{}", std::any::type_name::<T>())
}

