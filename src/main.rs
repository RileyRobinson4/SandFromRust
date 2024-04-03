extern crate gtk;
extern crate cairo;


use gtk::prelude::*;
use gtk::subclass::drawing_area;
use gtk::{glib, Application, ApplicationWindow, DrawingArea};
use glib::timeout_add;
use std::cell::Cell;

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::sync::Arc;
use std::thread;
use std::time::Duration;


const APP_ID: &str = "Sand";

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
    let mut sand_state = Cell::new([[0u32; NUM_ROWS]; NUM_COLS]);
    
    initialize_game_state(& mut sand_state.get_mut());  


    // Create a drawing area.
    let mut drawing_area = DrawingArea::new();

    drawing_area.set_draw_func(move |_, context, x, y| {
        process_game_state(& mut sand_state.get_mut());
        draw_stuff(context, x, y, & mut(sand_state.get_mut()));
    });


    // Create a window
    let mut window = ApplicationWindow::builder()
        .application(app)
        .default_width((CELL_SIZE * (NUM_ROWS as f64)) as i32)
        .default_height((CELL_SIZE * (NUM_ROWS as f64)) as i32)
        .title("Sand")
        .child(&drawing_area)
        .build();

    let draw_duration = Duration::new(0u64, 10000000u32);
    // glib::timeout_add(draw_duration, move || {
    //     //let drawing_area = drawing_area.clone();
    //     //drawing_area.queue_draw();

    //     return gtk::glib::ControlFlow::Continue; 
    // });

    glib::source::timeout_add_local_full(draw_duration, gtk::glib::Priority::HIGH, move || {
        drawing_area.queue_draw();
        return gtk::glib::ControlFlow::Continue; 
    });

    // glib::source::timeout_add_seconds_local(1, move || {
    //     drawing_area.queue_draw();
    //     return gtk::glib::ControlFlow::Continue; 
    // });

    
    window.present();

    // Present window
//    window.present();
}


fn draw_stuff(context: &cairo::Context, x: i32, y: i32, sand_state: & mut [[u32; NUM_COLS]; NUM_ROWS]) {
    context.set_source_rgb(0.0, 0.0, 0.0);
    if context.paint().is_err()
    {
        print!("Couldn't paint");
    }

    // Set line width
    context.set_line_width(1.0);

    context.set_source_rgb(1.0, 1.0, 1.0);

    // Draw grid lines
    for i in 0..=NUM_ROWS {
        let y = (i as f64) * CELL_SIZE;
        context.move_to(0.0, y);
        context.line_to(1.0 + (NUM_COLS as f64) * CELL_SIZE, y);
        if context.stroke().is_err()
        {
            print!("Couldn't stroke");
        }
    }

    for i in 0..=NUM_COLS {
        let x = (i as f64) * CELL_SIZE;
        context.move_to(x, 0.0);
        context.line_to(x, 1.0 + (NUM_ROWS as f64) * CELL_SIZE);
        if context.stroke().is_err()
        {
            print!("Couldn't stroke");
        }
    }

    //sand_state[0][5] = 1u32;

    // Fill cells
    for row in 0..NUM_ROWS {
        for col in 0..NUM_COLS {
            let x = (col as f64) * CELL_SIZE;
            let y = (row as f64) * CELL_SIZE;
            //let cell_color = if (row + col) % 2 == 0 { 1.0 } else { 0.0 }; // Alternate cell color
            let cell_color = if sand_state[row][col] > 0u32 { 1.0 } else { 0.0 };
            context.set_source_rgb(cell_color, cell_color, cell_color);
            context.rectangle(x, y, CELL_SIZE, CELL_SIZE);
            if context.fill().is_err()
            {
                print!("Couldn't fill context")
            }
         }
     }

}


fn initialize_game_state(sand_state: & mut [[u32; NUM_COLS]; NUM_ROWS])
{
    sand_state[0][5] = 1u32;
}

fn process_game_state(sand_state: & mut [[u32; NUM_COLS]; NUM_ROWS])
{
    //thread::sleep(Duration::from_millis(200));


    for row in 0..NUM_ROWS - 1
    {
        for col in 0..NUM_COLS - 1 
        {
            if sand_state[row][col] > 0
            {
                sand_state[row][col] = 0;
                sand_state[row + 1][col] = 1;
                return;
            }
        }
    }
}