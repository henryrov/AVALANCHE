use cursive::views::{Dialog, LinearLayout, TextView};
use cursive::Cursive;
use std::error::Error;

use R01_AVALANCHE::UserData;

pub mod habits_page;
pub mod records_page;

pub struct AppData {
    pub data_file_name: String,
    pub user_data: UserData,
    pub selected_habit: Option<usize>,
    pub selected_record: Option<usize>,
    pub unsaved_changes: bool,
}

fn quit(s: &mut Cursive) {
    let app_data = s.user_data::<AppData>().unwrap();
    if app_data.unsaved_changes {
        s.add_layer(
            Dialog::around(
                LinearLayout::vertical()
                    .child(TextView::new("There are unsaved changes."))
                    .child(TextView::new("Are you sure you want to quit?")),
            )
            .button("Save and quit", |s| {
                if save_data(s).is_ok() {
                    s.quit();
                }
            })
            .button("Quit", Cursive::quit)
            .button("Cancel", |s| {
                s.pop_layer();
            }),
        );
    } else {
        s.quit();
    }
}

fn save_data(s: &mut Cursive) -> Result<(), Box<dyn Error>> {
    let app_data = s.user_data::<AppData>().unwrap();
    let user_data = &mut app_data.user_data;
    match user_data.write_to_file(&app_data.data_file_name) {
        Ok(_) => {
            app_data.unsaved_changes = false;
            return Ok(());
        }
        Err(error) => {
            s.add_layer(Dialog::info("Failed to write to data file."));
            return Err(error);
        }
    }
}
