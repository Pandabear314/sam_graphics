// Import modules
mod window;
mod image;

fn main()
{
    // Create image
    let black_pixel = image::Color{red: 0, green: 0, blue: 0};
    let white_pixel = image::Color{red: 255, green: 255, blue: 255};

    let mut pixels: image::Image = image::Image::new(6, 6, black_pixel);
    pixels.set_pixel(1, 1, white_pixel);
    pixels.set_pixel(4, 1, white_pixel);
    pixels.set_pixel(1, 3, white_pixel);
    pixels.set_pixel(2, 4, white_pixel);
    pixels.set_pixel(3, 4, white_pixel);
    pixels.set_pixel(4, 3, white_pixel);

    // Create new window
    window::create_window(1600.0, 900.0, pixels.width as i32, pixels.height as i32, pixels.get_ptr());
}
