use gtk::glib::{MainContext, PRIORITY_DEFAULT, clone};
use gtk::{prelude::*, LevelBar};
use gtk::{glib, Button, ApplicationWindow};
use adw::{Application};

use procfs::{CpuInfo, ProcError};

const APP_ID: &str = "org.lyndeno.gnome-smbios";

fn main() -> glib::ExitCode {
    //println!("Hello, world!");

    // Create app
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to activate signal
    app.connect_activate(build_ui);

    // Run app
    app.run()
}

fn build_ui(app: &Application) {
    let count = CpuInfo::new().unwrap().num_cores();
    // Create a button
    let button = Button::builder()
        .label("Press")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();


    let mybox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    let mut v = Vec::new();
    for i in 0..count {
    let bar = gtk::LevelBar::builder()
        .value(0.3)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .min_value(0.0)
        .max_value(4000.0)
        .build();
        v.push(bar);
    }
    for b in &v {
        mybox.append(b);
    }
    mybox.append(&button);


    let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);
    // Connect to "clicked" signal of button
    button.connect_clicked(move |button| {
        // set the label to "Hello" after click
        button.set_label("Hello");
        let thing = adw::AboutWindow::builder()
            .application_name("Lyndon's Program")
            .application_icon("org.example.App")
            .copyright("Lyndon Sanche")
            .version("0.0.1")
            .website("https://lyndeno.ca")
            .developer_name("Lyndon Sanche")
            .build();
        thing.show();
        let sender = sender.clone();
        std::thread::spawn(move || {
            loop {
            for i in 0..count {
                let speed = CpuInfo::new().unwrap().get_field(i, "cpu MHz").unwrap().parse::<f64>().unwrap();
                sender.send(speed).expect("Could not send");
                //bar.set_fraction(i as f64 / 100_f64);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            }
        });
    });


    receiver.attach(
        None,
                    move |signal| {
                        //bar.set_value(signal);
                        for i in 0..count {
                            v[i].set_value(signal);
                        }

                        Continue(true)
                    },);

    // Create window and set title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("GTK SMBIOS")
        .child(&mybox)
        .build();

    //Present
    window.present();
}
