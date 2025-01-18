use cursive::views::{Dialog, LinearLayout, TextView};
use cursive::Cursive;

use R01_AVALANCHE::UserData;

use dirs::data_dir;

pub mod habits_page;
pub mod records_page;

pub struct AppData {
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
                    .child(TextView::new(
                        "Are you sure you want to quit?",
                    )),
            )
            .button("Save and quit", |s| {
                save_data(s);
                s.quit();
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

fn save_data(s: &mut Cursive) {
    let app_data = s.user_data::<AppData>().unwrap();
    app_data.unsaved_changes = false;
    let user_data = &mut app_data.user_data;
    user_data
        .write_to_file(format!(
            "{}/{}",
            data_dir().unwrap().to_str().unwrap(),
            ".avalanche"
        ))
        .expect("Failed to write data")
}
