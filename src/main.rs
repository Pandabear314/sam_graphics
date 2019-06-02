// Import crates
extern crate rand;

// Import modules
mod window;
mod image;

fn main()
{
    // Define image size
    let image_width: usize = 160;
    let image_height: usize = 90;

    // Define colors
    let black_pixel = image::Color{ red: 0, green: 0, blue: 0 };

    // Create blank image
    let mut pixels: image::Image = image::Image::new(image_width, image_height, black_pixel);

    // Define draw function
    let mut draw_fun = || rand_pixel(&mut pixels);

    // Create new window
    window::create_window(1600.0, 900.0, image_width as i32, image_height as i32, &mut draw_fun);
}

// Define drawing function
fn rand_pixel(pixels: &mut image::Image) -> *const std::ffi::c_void
{
    // Define colors
    let black_pixel = image::Color{ red: 0, green: 0, blue: 0 };
    let white_pixel = image::Color{ red: 255, green: 255, blue: 255 };

    // Create blank image
    *pixels = image::Image::new(pixels.width, pixels.height, black_pixel);

    // Get two random numbers
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let rand_x0 = rng.gen::<usize>() % pixels.width;
    let rand_y0 = rng.gen::<usize>() % pixels.height;
    let rand_x1 = rng.gen::<usize>() % pixels.width;
    let rand_y1 = rng.gen::<usize>() % pixels.height;

    // Draw a line
    pixels.draw_line(rand_x0, rand_y0, rand_x1, rand_y1, white_pixel);

    // return pointer
    return pixels.get_ptr();
}
