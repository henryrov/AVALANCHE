use cursive::Cursive;
use cursive::views::{Button, Dialog, DummyView, EditView,
                     LinearLayout, SelectView, Menubar,
                     ViewRef};
use cursive::event::Key;
use cursive::traits::*;
use dirs::data_dir;

use R01_AVALANCHE::{AppData, Habit};

fn main() {
    let app_data = match std::fs::exists(format!("{}/{}",
                                                 data_dir()
                                                 .unwrap()
                                                 .to_str()
                                                 .unwrap(),
                                                 ".avalanche")).unwrap() {
        true => AppData::read_from_file(format!("{}/{}",
                                                data_dir()
                                                .unwrap().
                                                to_str().unwrap(),
                                                ".avalanche").as_str())
            .expect("Failed to open data file"),
        false => AppData {
            version: AppData::CURRENT_VERSION,
            habits: Vec::new(),
        }
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

    siv.add_layer(Dialog::around(LinearLayout::vertical()
                                 .child(Dialog::text(
                                     "Press esc to select the menu"
                                 ))
                                 .child(habit_select)
                                 .full_screen())
                  .title("R01_AVALANCHE"));

    for habit in &app_data.habits {
        println!("{}", habit.name);
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
                records: Vec::new()
            }),
            None => panic!(),
        }
        
        s.pop_layer();
    }

    s.add_layer(Dialog::around(EditView::new()
            .on_submit(ok)
            .with_name("name"))
        .title("Enter a name for the habit")
        .button("Ok", |s| {
            let name =
                s.call_on_name("name", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
            ok(s, &name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }));
}

fn delete_habit(s: &mut Cursive) {
    fn ok (s: &mut Cursive) {
        let mut select = s.find_name::<SelectView<String>>("habit_select").unwrap();
        let selected_id = select.selected_id().unwrap();
        select.remove_item(selected_id);
        match s.user_data::<AppData>() {
            Some(data) => {
                data.habits.remove(selected_id);
            },
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
            s.add_layer(Dialog::around(LinearLayout::horizontal()
                                       .child(Button::new("Yes", ok))
                                       .child(DummyView::new())
                                       .child(Button::new("No", |s| {
                                           s.pop_layer();
                                       }))
            ).title(format!("Delete {}?", habit_name)));
        },
    }                                  
}

fn draw_records_page(s: &mut Cursive, name: &str) {
    let data = s.user_data::<AppData>().unwrap();
    let habit = data.find_habit_by_name(name).unwrap();

    draw_records_menubar(s);

    let record_select = SelectView::<LinearLayout>::new()
        .with_name("record_select")
        .scrollable();
    
    s.add_layer(Dialog::text(format!("Name: {}\nAwesome: yes", name))
        .title(format!("{}'s info", name)));
}

fn add_record(s: &mut Cursive) {

}

fn delete_record(s: &mut Cursive) {

}
    

fn back(s: &mut Cursive) {
    draw_habits_menubar(s);
    s.pop_layer();
}

fn save_data(s: &mut Cursive) {
    let data = s.user_data::<AppData>().unwrap();
    data.write_to_file(format!("{}/{}",
                               data_dir()
                               .unwrap().
                               to_str().unwrap(),
                               ".avalanche").as_str())
        .expect("Failed to write data")
}
