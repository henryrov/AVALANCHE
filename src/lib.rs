#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io::Write;
use std::ops::{Add, AddAssign};

// Note: Derivations of PartialEq and Debug used in tests

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl Date {
    fn month_length(month: u8, year: u16) -> u8 {
        match month {
            1 => 31,
            2 if year % 4 == 0 => 29,
            2 if year % 4 != 0 => 28,
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
        }
    }

    pub fn is_valid(&self) -> bool {
        if self.day <= Self::month_length(self.month, self.year) && self.day != 0 {
            return true;
        } else {
            return false;
        }
    }

    pub fn is_after(&self, comp: &Date) -> bool {
        if (self.year == comp.year && self.month == comp.month && self.day > comp.day)
            || (self.year == comp.year && self.month > comp.month)
            || (self.year > comp.year)
        {
            return true;
        }
        false
    }

    pub fn is_day_after(&self, comp: &Date) -> bool {
        if self.day == comp.day + 1 && self.month == comp.month && self.year == comp.year {
            return true;
        } else if comp.day == Self::month_length(comp.month, comp.year)
            && self.month == (comp.month + 1)
            && self.day == 1
            && self.year == comp.year
        {
            return true;
        } else if comp.day == 31
            && comp.month == 12
            && self.day == 1
            && self.month == 1
            && self.year == comp.year + 1
        {
            return true;
        }
        false
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Time {
    pub hours: u16,
    pub minutes: u16,
}

impl Time {
    pub fn difference(start: &Time, end: &Time) -> Result<Time, Box<dyn Error>> {
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

    pub fn is_valid_time_of_day(&self) -> bool {
        if self.hours < 24 && self.minutes < 60 {
            return true;
        } else {
            return false;
        }
    }
}

impl Add for Time {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            hours: self.hours + other.hours + (self.minutes + other.minutes) / 60,
            minutes: (self.minutes + other.minutes) % 60,
        }
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Record {
    pub note: String,
    pub date: Date,
    pub start_time: Time,
    pub end_time: Time,
}

impl Record {
    pub fn length(&self) -> Result<Time, Box<dyn Error>> {
        Time::difference(&self.start_time, &self.end_time)
    }
}

pub struct HabitStats {
    pub streak_length: u16,
    pub total_time: Time,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Habit {
    pub name: String,
    pub records: Vec<Record>,
}

impl Habit {
    pub fn get_stats(&self) -> HabitStats {
        let mut total_time = Time {
            hours: 0,
            minutes: 0,
        };

        // Find most recent date with a record

        if self.records.is_empty() {
            return HabitStats {
                streak_length: 0,
                total_time: total_time,
            };
        }

        // Start by assuming the most recent date is the latest record
        // The case of an empty vector was already handled above

        let mut most_recent_date = self.records.last().unwrap().date.clone();

        /* This loop will be used to find the most recent event and the
         * total time.
         */

        for record in &self.records {
            total_time += record.length().unwrap();
            if !most_recent_date.is_after(&record.date) {
                most_recent_date = record.date.clone();
            }
        }

        // Now check for the streak

        let mut comp_date = most_recent_date;
        let mut streak_len: u16 = 0;
        let mut day_before_found = true;
        while day_before_found {
            streak_len += 1;
            for record in &self.records {
                if comp_date.is_day_after(&record.date) {
                    day_before_found = true;
                    comp_date = record.date.clone();
                    break;
                }
                day_before_found = false;
            }
        }

        HabitStats {
            streak_length: streak_len,
            total_time: total_time,
        }
    }
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct UserData {
    pub version: u16,
    pub habits: Vec<Habit>,
}

impl UserData {
    pub fn find_habit_by_name(&self, name: &str) -> Option<&Habit> {
        for habit in &self.habits {
            if habit.name == name {
                return Some(habit);
            }
        }
        None
    }

    pub fn write_to_file(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut file = fs::File::create(filename)?;
        let file_contents = ron::to_string(&self)?;
        file.write(file_contents.as_bytes())?;
        Ok(())
    }

    pub fn read_from_file(filename: &str) -> Result<UserData, Box<dyn Error>> {
        let file = fs::File::open(filename)?;
        let data: UserData = ron::de::from_reader(file)?;
        return Ok(data);
    }

    pub const CURRENT_VERSION: u16 = 1;
}

#[cfg(test)]
mod lib_tests;
