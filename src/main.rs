// Import crates
extern crate glutin;
extern crate drawing;

// Define namespaces
use drawing::image;

// Import modules
mod game;

fn main() {
    // Define constants
    let window_width = 1600.0;
    let window_height = 900.0;

    // Define image size
    let image_width = 16;
    let image_height = 9;

    // Create screen buffer
    let screen_buffer: image::Image = 
        image::Image::new(image_width, image_height, image::Color::BLACK);

    // Initialize events loop
    let events_loop = glutin::EventsLoop::new();

    // Initialize window builder
    let window_builder = glutin::WindowBuilder::new()
        .with_title("")
        .with_dimensions(glutin::dpi::LogicalSize::new(window_width, window_height));

    // Initialize window
    let windowed_context = glutin::ContextBuilder::new()
        .build_windowed(window_builder, &events_loop)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    // Set window properties
    windowed_context.window().set_resizable(false);

    // Start game
    game::run(windowed_context, events_loop, screen_buffer, (window_width, window_height));
}
