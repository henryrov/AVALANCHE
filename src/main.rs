use std::error::Error;

use cursive::event::Key;
use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView, TextView};
use cursive::Cursive;

use dirs::data_dir;

use R01_AVALANCHE::{AppData, Date, Habit, Record, Time};

fn main() {
    let app_data = match std::fs::exists(format!(
        "{}/{}",
        data_dir().unwrap().to_str().unwrap(),
        ".avalanche"
    ))
    .unwrap()
    {
        true => AppData::read_from_file(
            format!("{}/{}", data_dir().unwrap().to_str().unwrap(), ".avalanche").as_str(),
        )
        .expect("Failed to open data file"),
        false => AppData {
            version: AppData::CURRENT_VERSION,
            habits: Vec::new(),
        },
    };

    let mut siv = cursive::default();

    siv.menubar()
        .add_leaf("Add habit", add_habit)
        .add_delimiter()
        .add_leaf("Delete selection", delete_habit)
        .add_delimiter()
        .add_leaf("Save", save_data)
        .add_delimiter()
        .add_leaf("Quit", Cursive::quit);
    siv.set_autohide_menu(false);

    let habit_select = SelectView::<String>::new()
        .on_submit(draw_records_page)
        .with_name("habit_select")
        .scrollable();

    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(Dialog::text("Press esc to select the menu"))
                .child(habit_select)
                .full_screen(),
        )
        .title("R01_AVALANCHE"),
    );

    for habit in &app_data.habits {
        siv.call_on_name("habit_select", |view: &mut SelectView<String>| {
            view.add_item_str(habit.name.as_str());
        });
    }

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());
    siv.set_user_data(app_data);

    siv.run();
}

fn draw_habits_menubar(s: &mut Cursive) {
    s.menubar().clear();
    s.menubar()
        .add_leaf("Add habit", add_habit)
        .add_delimiter()
        .add_leaf("Delete selection", delete_habit)
        .add_delimiter()
        .add_leaf("Save", save_data)
        .add_delimiter()
        .add_leaf("Quit", Cursive::quit);
}

fn draw_records_menubar(s: &mut Cursive) {
    s.menubar().clear();
    s.menubar()
        .add_leaf("Add record", add_record)
        .add_delimiter()
        .add_leaf("Delete selection", delete_record)
        .add_delimiter()
        .add_leaf("Save", save_data)
        .add_delimiter()
        .add_leaf("Back", back)
        .add_delimiter()
        .add_leaf("Quit", Cursive::quit);
}

fn add_habit(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_name("habit_select", |view: &mut SelectView<String>| {
            view.add_item_str(name)
        });

        match s.user_data::<AppData>() {
            Some(data) => data.habits.push(Habit {
                name: String::from(name),
                records: Vec::new(),
            }),
            None => panic!(),
        }

        s.pop_layer();
    }

    s.add_layer(
        Dialog::around(EditView::new().on_submit(ok).with_name("name"))
            .title("Enter a name for the habit")
            .button("Ok", |s| {
                let name = s
                    .call_on_name("name", |view: &mut EditView| view.get_content())
                    .unwrap();
                ok(s, &name);
            })
            .button("Cancel", |s| {
                s.pop_layer();
            }),
    );
}

fn delete_habit(s: &mut Cursive) {
    fn ok(s: &mut Cursive) {
        let mut select = s.find_name::<SelectView<String>>("habit_select").unwrap();
        let selected_id = select.selected_id().unwrap();
        select.remove_item(selected_id);
        match s.user_data::<AppData>() {
            Some(data) => {
                data.habits.remove(selected_id);
            }
            None => panic!(),
        }

        s.pop_layer();
    }

    let select = s.find_name::<SelectView<String>>("habit_select").unwrap();
    let selected_id = select.selected_id();
    let data = s.user_data::<AppData>().unwrap();
    match selected_id {
        None => s.add_layer(Dialog::info("Nothing selected")),
        Some(focus) => {
            let habit_name = data.habits[focus].name.clone();
            s.add_layer(
                Dialog::around(
                    LinearLayout::horizontal()
                        .child(Button::new("Yes", ok))
                        .child(DummyView::new())
                        .child(Button::new("No", |s| {
                            s.pop_layer();
                        })),
                )
                .title(format!("Delete {}?", habit_name)),
            );
        }
    }
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

fn draw_records_page(s: &mut Cursive, name: &str) {
    let record_select = SelectView::<String>::new()
        .on_submit(show_record_info)
        .with_name("record_select")
        .scrollable()
        .full_screen();

    let stats_dialog = Dialog::new().with_name("stats_dialog");

    let data = s.user_data::<AppData>().unwrap();
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

fn time_from_strings(hours_string: String, minutes_string: String) -> Result<Time, Box<dyn Error>> {
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

fn add_record(s: &mut Cursive) {
    fn ok(s: &mut Cursive, record: Record) {
        s.call_on_name("record_select", |view: &mut SelectView<String>| {
            view.add_item_str(record_item_builder(&record));
        });

        let habit_select = s.find_name::<SelectView<String>>("habit_select").unwrap();
        let habit_id = habit_select.selected_id().unwrap();
        let data = s.user_data::<AppData>().unwrap();
        data.habits[habit_id].records.push(record);
        let habit = data.habits[habit_id].clone();

        s.pop_layer();
        write_habit_stats(s, &habit);
    }

    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new("Date:"))
                .child(
                    LinearLayout::horizontal()
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
                        ),
                )
                .child(TextView::new("Start Time:"))
                .child(
                    LinearLayout::horizontal()
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
                        ),
                )
                .child(TextView::new("End Time:"))
                .child(
                    LinearLayout::horizontal()
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
                        ),
                )
                .child(TextView::new("Note:"))
                .child(EditView::new().with_name("note")),
        )
        .title("New record")
        .button("Ok", |s| {
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

            match date_from_strings(date_year, date_month, date_day) {
                Err(_) => {
                    s.add_layer(Dialog::info("Failed to parse date"));
                    return;
                }
                Ok(date_result) => {
                    let date = date_result;

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

                    match time_from_strings(start_time_hours, start_time_minutes) {
                        Err(_) => {
                            s.add_layer(Dialog::info("Failed to parse start time"));
                            return;
                        }
                        Ok(start_time_result) => {
                            let start_time = start_time_result;

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

                            match time_from_strings(end_time_hours, end_time_minutes) {
                                Err(_) => {
                                    s.add_layer(Dialog::info("Failed to parse end time"));
                                    return;
                                }
                                Ok(end_time_result) => {
                                    let end_time = end_time_result;

                                    let note = s
                                        .call_on_name("note", |view: &mut EditView| {
                                            view.get_content().to_string()
                                        })
                                        .unwrap();

                                    let record = Record {
                                        note: note,
                                        date: date,
                                        start_time: start_time,
                                        end_time: end_time,
                                    };

                                    ok(s, record);
                                }
                            }
                        }
                    }
                }
            }
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
        let data = s.user_data::<AppData>().unwrap();
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
    draw_habits_menubar(s);
    s.pop_layer();
}

fn save_data(s: &mut Cursive) {
    let data = s.user_data::<AppData>().unwrap();
    data.write_to_file(
        format!("{}/{}", data_dir().unwrap().to_str().unwrap(), ".avalanche").as_str(),
    )
    .expect("Failed to write data")
}
