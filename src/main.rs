extern crate gtk;
extern crate cairo;

use gtk::gdk::RGBA;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, DrawingArea};
use cairo::{ ImageSurface, Format, Context };

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

const CELL_SIZE: f64 = 20.0;
const NUM_ROWS: usize = 25;
const NUM_COLS: usize = 25;

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a drawing area.
    let drawing_area = DrawingArea::new();
    drawing_area.set_draw_func(|_, context, x, y| {
        draw_stuff(context, x, y);
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(500)
        .default_height(550)
        .title("My GTK App")
        .child(&drawing_area)
        .build();

            
    // Present window
    window.present();
}


fn draw_stuff(context: &cairo::Context, x: i32, y: i32) {
    context.set_source_rgb(0.0, 0.0, 0.0);
    let paint_result = context.paint();
    if paint_result.is_err()
    {
        print!("Couldn't paint");
    }

    print!("x: {}, y: {}", x, y);

    // Set line width
    context.set_line_width(1.0);

    // Draw grid lines
    for i in 0..=NUM_ROWS-1 {
        let y = (i as f64) * CELL_SIZE;
        context.move_to(0.0, y);
        context.line_to((NUM_COLS as f64) * CELL_SIZE, y);
        context.stroke();
    }

    for i in 0..=NUM_COLS-1 {
        let x = (i as f64) * CELL_SIZE;
        context.move_to(x, 0.0);
        context.line_to(x, (NUM_ROWS as f64) * CELL_SIZE);
        context.stroke();
    }

    // Fill cells
    for row in 0..NUM_ROWS {
        for col in 0..NUM_COLS {
            let x = (col as f64) * CELL_SIZE;
            let y = (row as f64) * CELL_SIZE;
            let cell_color = if (row + col) % 2 == 0 { 1.0 } else { 0.0 }; // Alternate cell color
            context.set_source_rgb(cell_color, cell_color, cell_color);
            context.rectangle(x, y, CELL_SIZE, CELL_SIZE);
            context.fill();
        }
    }



}
