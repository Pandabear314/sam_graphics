// Import modules
mod window;
mod image;

fn main()
{
    // // Create image
    // let black_pixel = image::Color{red: 0, green: 0, blue: 0};
    // let white_pixel = image::Color{red: 255, green: 255, blue: 255};

    // let mut pixels: image::Image = image::Image::new(16, 9, black_pixel);
    // pixels.draw_rect(1, 1, 5, 7, white_pixel);

    // Define image size
    let image_width: usize = 160;
    let image_height: usize = 90;

    // Define draw function
    let draw_fun = || rand_pixel(image_width, image_height);

    // Create new window
    window::create_window(1600.0, 900.0, image_width as i32, image_height as i32, draw_fun);
}

// Define drawing function
fn rand_pixel(image_width: usize, image_height: usize) -> *const std::ffi::c_void
{
    // Define colors
    let black_pixel = image::Color{red: 0, green: 0, blue: 0};
    let white_pixel = image::Color{red: 255, green: 255, blue: 255};

    // Create blank image
    let mut pixels: image::Image = image::Image::new(image_width, image_height, black_pixel);

    // Get two random numbers
    let rand_x: usize = (random() * image_width as f64).floor() as usize;
    let rand_y: usize = (random() * image_height as f64).floor() as usize;

    // Change pixels
    pixels.set_pixel(rand_x, rand_y, white_pixel);

    // return pointer
    return pixels.get_ptr();
}

// Define "random" function
fn random() -> f64
{
    // Get current time
    let time: f64;
    match std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH)
    {
        Ok(n) => time = n.as_micros() as f64,
        Err(_) => panic!("Invalid system time"),
    }

    // Create "random" value
    let rand_val: f64 = time / std::f64::consts::PI;

    return rand_val - rand_val.floor();
}