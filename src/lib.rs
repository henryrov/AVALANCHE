#![allow(non_snake_case)]

use std::error::Error;
use std::fs;
use std::io::Write;
use serde::{Deserialize, Serialize};

// Note: Derivations of PartialEq and Debug used in tests

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Date {
    pub fn is_valid(&self) -> bool {
        let month_length = match self.month {
            1 => 31,
            2 if self.year % 4 == 0 => 29,
            2 if self.year % 4 != 0 => 28,
            3 => 31,
            4 => 30,
            5 => 31,
            6 => 30,
            7 => 31,
            8 => 31,
            9 => 30,
            10 => 31,
            11 => 30,
            12 => 31,
            _ => 0, // Not a real month
        };

        if self.day <= month_length && self.day != 0 {
            return true;
        } else {
            return false;
        }
    }
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct Time {
    hours: u16,
    minutes: u16,
}

impl Time {
    pub fn difference(start: &Time, end: &Time)
                      -> Result<Time, Box<dyn Error>> {
        let start_minutes: u16 = start.hours * 60 + start.minutes;
        let end_minutes: u16 = end.hours * 60 + end.minutes;
        if end_minutes < start_minutes {
            return Err("End time before start time".into());
        }
        let difference = end_minutes - start_minutes;
        let difference_hours = difference / 60;
        let difference_minutes = difference % 60;
        Ok(Time {
            hours: difference_hours,
            minutes: difference_minutes,
        })
    }

    pub fn is_valid(&self) -> bool {
        if self.hours < 24 && self.minutes < 60 {
            return true;
        } else {
            return false;
        }
    }
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct Record {
    note: String,
    date: Date,
    start_time: Time,
    end_time: Time,
}
    
#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct Habit {
    name: String,
    records: Vec<Record>,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct AppData {
    version: u16,
    habits: Vec<Habit>,
}

impl AppData {
    pub fn write_to_file(&self, filename: &str)
                         -> Result<(), Box<dyn Error>> {
        let mut file = fs::File::create(filename)?;
        let file_contents = ron::to_string(&self)?;
        file.write(file_contents.as_bytes())?;
        Ok(())
    }

    pub fn read_from_file(filename: &str)
                          -> Result<AppData, Box<dyn Error>> {
        let file = fs::File::open(filename)?;
        let data: AppData = ron::de::from_reader(file)?;
        return Ok(data);
    }

    pub const CURRENT_VERSION: u16 = 1;
}

#[cfg(test)]
mod lib_tests;
