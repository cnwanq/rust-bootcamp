use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input).unwrap();
    let mut res = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Player = result.unwrap();
        res.push(record);
    }

    let json = serde_json::to_string_pretty(&res)?;
    fs::write(output, json)?;
    Ok(())
}
