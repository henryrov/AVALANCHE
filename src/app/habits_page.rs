use cursive::event::Key;
use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView};
use cursive::{Cursive, CursiveRunnable};

use crate::app;
use crate::{Habit, UserData};

pub fn draw(mut s: CursiveRunnable) {
    draw_menubar(&mut s);
    s.set_autohide_menu(false);

    let data = s.user_data::<UserData>().unwrap().clone();

    let habit_select = SelectView::<String>::new()
        .on_submit(app::records_page::draw)
        .with_name("habit_select")
        .scrollable();

    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(Dialog::text("Press esc to select the menu"))
                .child(habit_select)
                .full_screen(),
        )
        .title("R01_AVALANCHE"),
    );

    for habit in &data.habits {
        s.call_on_name("habit_select", |view: &mut SelectView<String>| {
            view.add_item_str(habit.name.as_str());
        });
    }

    s.add_global_callback(Key::Esc, |s| s.select_menubar());

    s.run();
}

pub fn draw_menubar(s: &mut Cursive) {
    s.menubar().clear();
    s.menubar()
        .add_leaf("Add habit", add_habit)
        .add_delimiter()
        .add_leaf("Delete selection", delete_habit)
        .add_delimiter()
        .add_leaf("Save", app::save_data)
        .add_delimiter()
        .add_leaf("Quit", Cursive::quit);
}

fn add_habit(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_name("habit_select", |view: &mut SelectView<String>| {
            view.add_item_str(name)
        });

        match s.user_data::<UserData>() {
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
        match s.user_data::<UserData>() {
            Some(data) => {
                data.habits.remove(selected_id);
            }
            None => panic!(),
        }

        s.pop_layer();
    }

    let select = s.find_name::<SelectView<String>>("habit_select").unwrap();
    let selected_id = select.selected_id();
    let data = s.user_data::<UserData>().unwrap();
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
