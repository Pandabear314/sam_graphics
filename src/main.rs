// Import modules
mod window;
mod image;

fn main()
{
    // Create image
    let black_pixel = image::Color{red: 0, green: 0, blue: 0};
    let white_pixel = image::Color{red: 255, green: 255, blue: 255};

    let mut pixels: image::Image = image::Image::new(16, 9, black_pixel);
    pixels.draw_rect(1, 1, 5, 7, white_pixel);

    // Create new window
    window::create_window(1600.0, 900.0, pixels.width as i32, pixels.height as i32, pixels.get_ptr());
}
