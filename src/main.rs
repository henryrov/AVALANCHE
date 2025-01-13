use dirs::data_dir;
use R01_AVALANCHE::{Date, Habit, Record, Time, UserData};

mod app;

fn main() {
    let app_data = app::AppData {
        user_data: UserData::try_from_file(format!(
            "{}/{}",
            data_dir().unwrap().to_str().unwrap(),
            ".avalanche"
        )),
        selected_habit: None,
        selected_record: None,
        unsaved_changes: false,
    };

    let mut siv = cursive::default();
    siv.set_user_data(app_data);
    app::habits_page::draw(siv);
}
