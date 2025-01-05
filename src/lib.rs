#![allow(non_snake_case)]

use std::error::Error;
use std::fs;
use std::io::Write;
use serde::{Deserialize, Serialize};

// Note: Derivations of PartialEq and Debug used in tests

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
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

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Time {
    pub hours: u16,
    pub minutes: u16,
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

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Record {
    pub note: String,
    pub date: Date,
    pub start_time: Time,
    pub end_time: Time,
}
    
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Habit {
    pub name: String,
    pub records: Vec<Record>,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct AppData {
    pub version: u16,
    pub habits: Vec<Habit>,
}

impl AppData {
    pub fn find_habit_by_name(&self, name: &str) -> Option<&Habit> {
        for habit in &self.habits {
            if habit.name == name {
                return Some(habit);
            }
        }
        None
    }
    
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
