// Import crates
extern crate rand;

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
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let rand_x = rng.gen::<usize>() % image_width;
    let rand_y = rng.gen::<usize>() % image_height;

    // Change pixels
    pixels.set_pixel(rand_x, rand_y, white_pixel);
    pixels.set_pixel(3, image_height-1, white_pixel);

    // return pointer
    return pixels.get_ptr();
}