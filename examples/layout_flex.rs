use druid::{
    widget::{Container, Flex, Label},
    AppLauncher, Color, Widget, WidgetExt, WindowDesc,
};

fn main() {
    let window = WindowDesc::new(view);
    let data = 0_u32;
    AppLauncher::with_window(window).launch(data).unwrap()
}

fn view() -> impl Widget<u32> {
    let container_01 = Container::new(Label::new("con 01"))
        .background(Color::OLIVE)
        .expand_height()
        .fix_width(60.);
    let container_02 = Container::new(
        Flex::column()
            .with_child(Label::new("Item 01"))
            .with_child(Label::new("Item 02"))
            .with_child(Label::new("Item 03")),
    )
    .background(Color::BLUE)
    .expand();
    Flex::row()
        .with_child(container_01)
        .with_spacer(2.)
        .with_flex_child(container_02, 1.0)
}
