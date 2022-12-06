use druid::{
    im::Vector,
    widget::{Checkbox, Flex, Label, List},
    AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc,
};

fn main() -> Result<(), druid::PlatformError> {
    let window = WindowDesc::new(build);
    let data = AppData::new(vec![
        TodoItem::new("Item 01"),
        TodoItem::new("Item 02"),
        TodoItem::new("Item 03"),
        TodoItem::new("Item 04"),
        TodoItem::new("Item 05"),
    ]);
    AppLauncher::with_window(window).launch(data)
}

#[derive(Clone, Data, Lens)]
struct AppData {
    items: Vector<TodoItem>,
}

impl AppData {
    fn new(vec: Vec<TodoItem>) -> Self {
        Self {
            items: Vector::from(vec),
        }
    }
}

#[derive(Clone, Data, Lens)]
struct TodoItem {
    done: bool,
    text: String,
}

impl TodoItem {
    fn new(text: impl Into<String>) -> Self {
        Self {
            done: false,
            text: text.into(),
        }
    }
}

fn view_todos() -> impl Widget<TodoItem> {
    let checkbox = Checkbox::new("").lens(TodoItem::done);
    let label = Label::raw().lens(TodoItem::text);

    Flex::row()
        .with_child(checkbox)
        .with_spacer(10.)
        .with_flex_child(label, 1.)
}

fn build() -> impl Widget<AppData> {
    let list = List::new(view_todos).lens(AppData::items);
    Flex::column().with_child(list).expand_width()
}
