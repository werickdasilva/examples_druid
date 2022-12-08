use druid::{
    widget::{Button, Flex, Label},
    AppLauncher, Data, Env, Widget, WindowDesc,
};

#[derive(Clone, Data)]
struct AppData {
    number: i32,
}

fn main() {
    let window = WindowDesc::new(view);
    let data = AppData { number: 0 };
    AppLauncher::with_window(window).launch(data).unwrap();
}

fn view() -> impl Widget<AppData> {
    let label =
        Label::new(|data: &AppData, _env: &Env| format!("Value of the number is {}!", data.number));
    let button_increment =
        Button::new("Increment").on_click(|_, data: &mut AppData, _| data.number += 1);
    let button_decrement =
        Button::new("Decrement").on_click(|_, data: &mut AppData, _| data.number -= 1);
    Flex::column()
        .with_child(label)
        .with_child(button_increment)
        .with_spacer(5.)
        .with_child(button_decrement)
}
