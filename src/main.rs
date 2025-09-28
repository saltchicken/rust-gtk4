use gtk4::gdk::{Display, prelude::*};
use gtk4::{Application, ApplicationWindow, Label, CssProvider}; 
use gtk4::prelude::GtkWindowExt; 

const CSS: &str = r#"
window.background {
  background-color: rgba(0, 0, 0, 0.5);
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

    window.set_child(Some(&label));

    // --- 4. Show the Window ---
    window.present();
}
