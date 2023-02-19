use gtk::prelude::*;
use gtk::{glib, Button, ApplicationWindow};
use adw::{Application};

const APP_ID: &str = "org.lyndeno.gnome-smbios";

fn main() -> glib::ExitCode {
    println!("Hello, world!");

    // Create app
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to activate signal
    app.connect_activate(build_ui);

    // Run app
    app.run()
}

fn build_ui(app: &Application) {
    // Create a button
    let button = Button::builder()
        .label("Press")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of button
    button.connect_clicked(|button| {
        // set the label to "Hello" after click
        button.set_label("Hello");
    });

    // Create window and set title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("GTK SMBIOS")
        .child(&button)
        .build();

    //Present
    window.present();
}
