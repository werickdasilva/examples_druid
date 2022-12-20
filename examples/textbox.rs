use druid::{
    widget::{Container, TextBox},
    AppLauncher, Data, Lens, PlatformError, Widget, WidgetExt, WindowDesc,
};

#[derive(Clone, Data, Lens)]
pub struct AppData {
    text_box: String,
}

pub fn view() -> impl Widget<AppData> {
    // TexBox accent doesn't work. ex: á ã
    let text_box = TextBox::new().lens(AppData::text_box);
    Container::new(text_box)
}

fn main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(view).title("TextBox");
    let data = AppData {
        text_box: String::new(),
    };

    AppLauncher::with_window(window).launch(data)
}
