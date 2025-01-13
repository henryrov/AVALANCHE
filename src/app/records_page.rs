use std::error::Error;

use cursive::traits::*;
use cursive::views::{Dialog, EditView, LinearLayout, SelectView, TextView};
use cursive::Cursive;

use crate::app;
use crate::{Date, Habit, Record, Time, UserData};

pub fn draw(s: &mut Cursive, name: &str) {
    let record_select = SelectView::<String>::new()
        .on_submit(show_record_info)
        .with_name("record_select")
        .scrollable()
        .full_screen();

    let stats_dialog = Dialog::new().with_name("stats_dialog");

    let data = s.user_data::<UserData>().unwrap();
    let habit = data.find_habit_by_name(name).unwrap().clone();

    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(stats_dialog)
                .child(record_select),
        )
        .title("Record view"),
    );

    write_habit_stats(s, &habit);

    for record in &habit.records {
        s.call_on_name("record_select", |view: &mut SelectView<String>| {
            view.add_item_str(record_item_builder(record));
        });
    }

    draw_records_menubar(s);
}

fn draw_records_menubar(s: &mut Cursive) {
    s.menubar().clear();
    s.menubar()
        .add_leaf("Add record", add_record)
        .add_delimiter()
        .add_leaf("Delete selection", delete_record)
        .add_delimiter()
        .add_leaf("Save", app::save_data)
        .add_delimiter()
        .add_leaf("Back", back)
        .add_delimiter()
        .add_leaf("Quit", Cursive::quit);
}

fn write_habit_stats(s: &mut Cursive, habit: &Habit) {
    s.call_on_name("stats_dialog", |view: &mut Dialog| {
        let stats = habit.get_stats();
        view.set_content(TextView::new(format!(
            "Most recent streak: {} days \
                     | Total time spent: {} hours and {} minutes",
            stats.streak_length, stats.total_time.hours, stats.total_time.minutes
        )))
    });
}

fn show_record_info(s: &mut Cursive, info: &str) {
    s.add_layer(Dialog::info(info));
}

fn record_item_builder(record: &Record) -> String {
    format!(
        "{}-{}-{}: {:02}:{:02} - {:02}:{:02} - {}",
        record.date.year,
        record.date.month,
        record.date.day,
        record.start_time.hours,
        record.start_time.minutes,
        record.end_time.hours,
        record.end_time.minutes,
        record.note
    )
}

fn add_record(s: &mut Cursive) {
    fn add_to_list(s: &mut Cursive, record: Record) {
        s.call_on_name("record_select", |view: &mut SelectView<String>| {
            view.add_item_str(record_item_builder(&record));
        });

        let habit_select = s.find_name::<SelectView<String>>("habit_select").unwrap();
        let habit_id = habit_select.selected_id().unwrap();
        let data = s.user_data::<UserData>().unwrap();
        data.habits[habit_id].records.push(record);
        let habit = data.habits[habit_id].clone();

        s.pop_layer();
        write_habit_stats(s, &habit);
    }

    fn time_from_strings(
        hours_string: String,
        minutes_string: String,
    ) -> Result<Time, Box<dyn Error>> {
        let time = Time {
            hours: hours_string.parse()?,
            minutes: minutes_string.parse()?,
        };

        if time.is_valid_time_of_day() {
            return Ok(time);
        } else {
            return Err("Invalid time".into());
        }
    }

    fn date_from_strings(
        year_string: String,
        month_string: String,
        day_string: String,
    ) -> Result<Date, Box<dyn Error>> {
        let date = Date {
            year: year_string.parse()?,
            month: month_string.parse()?,
            day: day_string.parse()?,
        };
        
        if date.is_valid() {
            return Ok(date);
        } else {
            return Err("Invalid date".into());
        }
    }

    fn parse_date(s: &mut Cursive) -> Result<Date, Box<dyn Error>> {
        let date_year = s
            .call_on_name("date_year", |view: &mut EditView| {
                view.get_content().to_string()
            })
            .unwrap();
        let date_month = s
            .call_on_name("date_month", |view: &mut EditView| {
                view.get_content().to_string()
            })
            .unwrap();
        let date_day = s
            .call_on_name("date_day", |view: &mut EditView| {
                view.get_content().to_string()
            })
            .unwrap();

        return date_from_strings(date_year, date_month, date_day);
    }

    fn parse_start_time(s: &mut Cursive) -> Result<Time, Box<dyn Error>> {
        let start_time_hours = s
            .call_on_name("start_time_hours", |view: &mut EditView| {
                view.get_content().to_string()
            })
            .unwrap();
        let start_time_minutes = s
            .call_on_name("start_time_minutes", |view: &mut EditView| {
                view.get_content().to_string()
            })
            .unwrap();

        return time_from_strings(start_time_hours, start_time_minutes);
    }

    fn parse_end_time(s: &mut Cursive) -> Result<Time, Box<dyn Error>> {
        let end_time_hours = s
            .call_on_name("end_time_hours", |view: &mut EditView| {
                view.get_content().to_string()
            })
            .unwrap();
        let end_time_minutes = s
            .call_on_name("end_time_minutes", |view: &mut EditView| {
                view.get_content().to_string()
            })
            .unwrap();

        return time_from_strings(end_time_hours, end_time_minutes);
    }

    fn is_start_before_end(start: &Time, end: &Time) -> bool {
        let start_minutes: u16 = start.hours * 60 + start.minutes;
        let end_minutes: u16 = end.hours * 60 + end.minutes;
        if end_minutes < start_minutes {
            return false;
        } else {
            return true;
        }
    }

    let date_row = LinearLayout::horizontal()
        .child(
            EditView::new()
                .max_content_width(4)
                .with_name("date_year")
                .fixed_width(5),
        )
        .child(TextView::new("-"))
        .child(
            EditView::new()
                .max_content_width(2)
                .with_name("date_month")
                .fixed_width(3),
        )
        .child(TextView::new("-"))
        .child(
            EditView::new()
                .max_content_width(2)
                .with_name("date_day")
                .fixed_width(3),
        );

    let start_time_row = LinearLayout::horizontal()
        .child(
            EditView::new()
                .max_content_width(2)
                .with_name("start_time_hours")
                .fixed_width(3),
        )
        .child(TextView::new(":"))
        .child(
            EditView::new()
                .max_content_width(2)
                .with_name("start_time_minutes")
                .fixed_width(3),
        );

    let end_time_row = LinearLayout::horizontal()
        .child(
            EditView::new()
                .max_content_width(2)
                .with_name("end_time_hours")
                .fixed_width(3),
        )
        .child(TextView::new(":"))
        .child(
            EditView::new()
                .max_content_width(2)
                .with_name("end_time_minutes")
                .fixed_width(3),
        );

    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new("Date:"))
                .child(date_row)
                .child(TextView::new("Start Time:"))
                .child(start_time_row)
                .child(TextView::new("End Time:"))
                .child(end_time_row)
                .child(TextView::new("Note:"))
                .child(EditView::new().with_name("note")),
        )
        .title("New record")
        .button("Ok", |s| {
            let date: Date;
            match parse_date(s) {
                Ok(result) => {
                    date = result;
                }
                Err(_) => {
                    s.add_layer(Dialog::info("Failed to parse date"));
                    return;
                }
            }

            let start_time: Time;
            match parse_start_time(s) {
                Ok(result) => {
                    start_time = result;
                }
                Err(_) => {
                    s.add_layer(Dialog::info("Failed to parse start time"));
                    return;
                }
            }

            let end_time: Time;
            match parse_end_time(s) {
                Ok(result) => {
                    end_time = result;
                }
                Err(_) => {
                    s.add_layer(Dialog::info("Failed to parse end time"));
                    return;
                }
            }

            if !is_start_before_end(&start_time, &end_time) {
                s.add_layer(Dialog::info("Start time should be before end time"));
                return;
            }

            let note = s
                .call_on_name("note", |view: &mut EditView| view.get_content().to_string())
                .unwrap();

            let record = Record {
                note: note,
                date: date,
                start_time: start_time,
                end_time: end_time,
            };

            add_to_list(s, record);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }),
    );
}

fn delete_record(s: &mut Cursive) {
    fn ok(s: &mut Cursive) {
        let habit_select = s.find_name::<SelectView<String>>("habit_select").unwrap();
        let habit_id = habit_select.selected_id().unwrap();
        let mut record_select = s.find_name::<SelectView<String>>("record_select").unwrap();
        let selected_id = record_select.selected_id().unwrap();
        record_select.remove_item(selected_id);
        let data = s.user_data::<UserData>().unwrap();
        data.habits[habit_id].records.remove(selected_id);
        let habit = data.habits[habit_id].clone();
        write_habit_stats(s, &habit);
        s.pop_layer();
    }

    let record_select = s.find_name::<SelectView<String>>("record_select").unwrap();
    let selected_id = record_select.selected_id();
    match selected_id {
        None => s.add_layer(Dialog::info("Nothing selected")),
        Some(_) => {
            s.add_layer(
                Dialog::new()
                    .button("Yes", ok)
                    .button("No", |s| {
                        s.pop_layer();
                    })
                    .title("Delete record?"),
            );
        }
    }
}

fn back(s: &mut Cursive) {
    app::habits_page::draw_menubar(s);
    s.pop_layer();
}
