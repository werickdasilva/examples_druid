use druid::{widget::Flex, AppLauncher, Color, PlatformError, Widget, WindowDesc};

fn main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(ui);
    let data = 0;
    AppLauncher::with_window(window)
        .configure_env(|env, _| env.set(druid::theme::WINDOW_BACKGROUND_COLOR, Color::WHITE))
        .launch(data)
}

pub fn ui() -> impl Widget<i32> {
    Flex::row()
}
