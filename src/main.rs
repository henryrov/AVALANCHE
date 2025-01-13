use dirs::data_dir;
use R01_AVALANCHE::{Date, Habit, Record, Time, UserData};

mod app;

fn main() {
    let app_data = match std::fs::exists(format!(
        "{}/{}",
        data_dir().unwrap().to_str().unwrap(),
        ".avalanche"
    ))
    .unwrap()
    {
        true => UserData::read_from_file(
            format!("{}/{}", data_dir().unwrap().to_str().unwrap(), ".avalanche").as_str(),
        )
        .expect("Failed to open data file"),
        false => UserData {
            version: UserData::CURRENT_VERSION,
            habits: Vec::new(),
        },
    };

    let mut siv = cursive::default();
    siv.set_user_data(app_data);
    app::habits_page::draw(siv);
}
