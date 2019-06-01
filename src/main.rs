// Import modules
mod window;
mod image;

fn main()
{
    // Create image
    let black_pixel = image::Color{red: 0, green: 0, blue: 0};
    let white_pixel = image::Color{red: 255, green: 255, blue: 255};

    let pixels: image::Image = 
        image::Image::new(vec![
            vec![black_pixel, black_pixel, black_pixel, black_pixel, black_pixel, black_pixel],
            vec![black_pixel, white_pixel, black_pixel, black_pixel, white_pixel, black_pixel],
            vec![black_pixel, black_pixel, black_pixel, black_pixel, black_pixel, black_pixel],
            vec![black_pixel, white_pixel, black_pixel, black_pixel, white_pixel, black_pixel],
            vec![black_pixel, black_pixel, white_pixel, white_pixel, black_pixel, black_pixel],
            vec![black_pixel, black_pixel, black_pixel, black_pixel, black_pixel, black_pixel]
        ]);

    // Create new window
    window::create_window(1600.0, 900.0, pixels.width, pixels.height, pixels.get_ptr());
}