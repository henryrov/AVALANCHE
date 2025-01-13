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

fn save_data(s: &mut Cursive) {
    let app_data = s.user_data::<AppData>().unwrap();
    let user_data = &mut app_data.user_data;
    user_data
        .write_to_file(format!(
            "{}/{}",
            data_dir().unwrap().to_str().unwrap(),
            ".avalanche"
        ))
        .expect("Failed to write data")
}
