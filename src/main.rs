use cursive::Cursive;
use cursive::views::{Button, Dialog, DummyView, EditView,
                     LinearLayout, SelectView, Menubar};
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
        .add_leaf("Quit", on_quit);

    siv.set_autohide_menu(false);
    
    let habit_select = SelectView::<String>::new()
        .on_submit(on_submit)
        .with_name("habit_select")
        .scrollable();
    println!("{}", app_data.habits.len());

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
    let mut select = s.find_name::<SelectView<String>>("habit_select").unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No name to remove")),
        Some(focus) => {
            select.remove_item(focus);
            match s.user_data::<AppData>() {
                Some(data) => {
                    data.habits.remove(focus);
                },
                None => panic!(),
        }
        }
    }
}

fn on_submit(s: &mut Cursive, name: &str) {
    s.pop_layer();
    s.add_layer(Dialog::text(format!("Name: {}\nAwesome: yes", name))
        .title(format!("{}'s info", name))
        .button("Quit", Cursive::quit));
}

fn on_quit(s: &mut Cursive) {
    match s.user_data::<AppData>() {
        Some(data) => data.write_to_file(format!("{}/{}",
                                                 data_dir()
                                                 .unwrap().
                                                 to_str().unwrap(),
                                                 ".avalanche").as_str())
            .expect("Failed to write data"),
        None => panic!("Failed to write data"),
    }
    s.quit();
}
