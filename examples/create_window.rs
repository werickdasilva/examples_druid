use druid::{
    widget::{Container, Flex},
    AppLauncher, Widget, WindowDesc,
};

fn main() -> Result<(), druid::PlatformError> {
    let window = WindowDesc::new(main_build)
        .title("Create Window")
        .with_min_size((300., 200.))
        .set_position((400., 200.))
        .window_size((400., 300.));

    let data = 0_u32;
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(data)
}

fn main_build() -> impl Widget<u32> {
    Container::new(Flex::row())
}
