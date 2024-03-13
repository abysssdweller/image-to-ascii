extern crate gtk;
extern crate gio;
extern crate image;
extern crate time;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, FileChooserAction, FileChooserDialog, TextView, ScrolledWindow, WindowPosition};
use gio::prelude::*;
use image::{DynamicImage, GenericImageView, imageops::FilterType};
use time::Instant;

const ASCII_CHARS: [char; 11] = ['@', '#', '0', 'O', 'L', ';', ':', '.', ',', '\'', ' '];

fn convert_to_ascii(image: DynamicImage, resolution: u32) -> String {
    let mut ascii_art = String::new();
    let (width, height) = image.dimensions();
    let small_image = image.resize_exact(width / resolution, height / resolution, FilterType::Nearest);

    for y in 0..small_image.height() {
        for x in 0..small_image.width() {
            let pixel = small_image.get_pixel(x, y);
            let brightness = (pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3;
            let index = (brightness as usize * (ASCII_CHARS.len() - 1)) / 255;
            ascii_art.push(ASCII_CHARS[index]);
        }
        ascii_art.push('\n');
    }

    ascii_art
}

fn main() {
    let application = Application::new(
        Some("com.example.ascii_app"),
                                       Default::default(),
    )
    .expect("Failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("ASCII Art Converter");
        window.set_default_size(1280, 720);
        window.set_position(WindowPosition::Center);
        window.set_resizable(false);

        let text_view = TextView::new();
        let button = Button::with_label("Select Image");
        let text_buffer = text_view.get_buffer().expect("Failed to get text buffer");

        let window_clone = window.clone();

        button.connect_clicked(move |_| {
            let file_dialog = FileChooserDialog::new(
                Some("Select an Image"),
                                                     Some(&window_clone),
                                                     FileChooserAction::Open,
            );
            file_dialog.add_button("Cancel", gtk::ResponseType::Cancel.into());
            file_dialog.add_button("Open", gtk::ResponseType::Ok.into());

            if file_dialog.run() == gtk::ResponseType::Ok.into() {
                if let Some(file_path) = file_dialog.get_filename() {
                    let image = match image::open(&file_path) {
                        Ok(img) => img,
                               Err(err) => {
                                   let err_msg = format!("Error opening image: {}", err);
                                   let dialog = gtk::MessageDialog::new(
                                       Some(&window_clone),
                                                                        gtk::DialogFlags::MODAL,
                                                                        gtk::MessageType::Error,
                                                                        gtk::ButtonsType::Ok,
                                                                        &err_msg,
                                   );
                                   dialog.run();
                                   unsafe { dialog.destroy(); }
                                   return;
                               }
                    };

                    let resolution = 1; // You can adjust this value as needed
                    let start_time = Instant::now();
                    let ascii_art = convert_to_ascii(image, resolution);
                    let end_time = Instant::now();

                    text_buffer.set_text(&ascii_art);

                    println!("Conversion completed in {} milliseconds", (end_time - start_time).whole_milliseconds());
                }
            }

            unsafe { file_dialog.destroy(); }
        });

        let scrolled_window = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scrolled_window.add(&text_view);

        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        v_box.pack_start(&button, false, false, 0);
        v_box.pack_start(&scrolled_window, true, true, 0);

        window.add(&v_box);
        window.show_all();
    });

    application.run(&[]);
}
