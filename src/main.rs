use dirs::data_dir;
use AVALANCHE::{Date, Habit, Record, Time, UserData};

mod app;

fn main() {
    let filename = format!("{}/{}", data_dir().unwrap().to_str().unwrap(), ".avalanche");

    let app_data = app::AppData {
        data_file_name: filename.clone(),
        user_data: UserData::try_from_file(&filename),
        selected_habit: None,
        unsaved_changes: false,
    };

    let mut siv = cursive::default();
    siv.set_user_data(app_data);
    app::habits_page::draw(siv);
}
