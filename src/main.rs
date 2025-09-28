use gtk4::gdk::{Display, prelude::*};
use gtk4::{Application, ApplicationWindow, Label, CssProvider, Button, Box as GtkBox, Orientation}; 
use gtk4::prelude::WidgetExt;
use gtk4::prelude::BoxExt;
use gtk4::prelude::ButtonExt;
use gtk4::prelude::GtkWindowExt; 

const CSS: &str = r#"
window.background {
  background-color: rgba(0, 0, 0, 0.5);
}

.custom-label {
  color: green;
}
"#;

fn main() {
    let app = Application::builder()
        .application_id("org.hyprland.transparentwidgetrust")
        .build();

    app.connect_activate(build_ui);
    
    app.run();
}

fn build_ui(app: &Application) {
    // --- 1. Setup Window ---
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hello Transparent World")
        .default_width(500)
        .default_height(500)
        .build();

    // --- 2. Enable Settings ---
    window.set_decorated(false);

    let provider = CssProvider::new();
    provider.load_from_data(CSS);

    if let Some(display) = Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    // --- 3. Add Content ---
    let label = Label::builder()
        .label("Hello World from Rust!")
        .build();

    label.add_css_class("custom-label");

    let button = Button::builder()
        .label("Click me!")
        .build();

    button.connect_clicked(|_| {
        println!("Button clicked!");
    });

    let container = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .build();

    container.append(&label);
    container.append(&button);

    window.set_child(Some(&container));

    // --- 4. Show the Window ---
    window.present();
}
