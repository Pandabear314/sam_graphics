// Allow unused functions in this module
#![allow(dead_code)]

// Define Color struct
#[derive(Copy, Clone)]
pub struct Color 
{
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

// Define Image struct
pub struct Image
{
    data: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

// Implement Image methods
impl Image
{
    // Define constants
    const DEFAULT_ALPHA: u8 = 255 as u8;
    const BYTES_PER_PIXEL: usize = 4;

    // Define constructor
    pub fn new(im_width: usize, im_height: usize, background_color: Color) -> Image
    {
        // Create flat image with only background color
        let n_pixels = im_height * im_width;
        let mut flat_data: Vec<u8> = Vec::with_capacity(n_pixels * Image::BYTES_PER_PIXEL);
        for _ in 0..n_pixels
        {
            flat_data.push(background_color.red);
            flat_data.push(background_color.green);
            flat_data.push(background_color.blue);
            flat_data.push(Image::DEFAULT_ALPHA);
        }

        // Return image struct
        return Image{data: flat_data, width: im_width, height: im_height};
    }

    // Modify a pixel in the image
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color)
    {
        // Determine offset into data
        let inv_y: usize = self.height - y - 1;                 // Invert y coordinate as image is stored upside down

        let offset: usize = 
            (inv_y * self.width * Image::BYTES_PER_PIXEL)  +    // Ofset for full rows
            (x * Image::BYTES_PER_PIXEL);                       // Ofset for partial row

        // Modify data
        self.data[offset + 0] = color.red;
        self.data[offset + 1] = color.green;
        self.data[offset + 2] = color.blue;
    }

    // Draw a rectangle on the image
    pub fn draw_outline_rect(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: Color)
    {
        // Make (x0, y0) top left corner, and (x1, y1) bottom right corner
        let tl_x = std::cmp::min(x0, x1);
        let tl_y = std::cmp::min(y0, y1);

        let br_x = std::cmp::max(x0, x1);
        let br_y = std::cmp::max(y0, y1);

        // Draw top and bottom of rectangle
        for x in tl_x..=br_x
        {
            self.set_pixel(x, tl_y, color);
            self.set_pixel(x, br_y, color);
        }

        // Draw sides of rectangle
        for y in tl_y+1..br_y
        {
            self.set_pixel(tl_x, y, color);
            self.set_pixel(br_x, y, color);
        }
    }

    pub fn draw_filled_rect(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: Color)
    {
        // Make (x0, y0) top left corner, and (x1, y1) bottom right corner
        let tl_x = std::cmp::min(x0, x1);
        let tl_y = std::cmp::min(y0, y1);

        let br_x = std::cmp::max(x0, x1);
        let br_y = std::cmp::max(y0, y1);

        // Fill in rectangular region
        for x in tl_x..=br_x
        {
            for y in tl_y..=br_y
            {
                self.set_pixel(x, y, color);
            }
        }
    }

    // Draw a line on the image
    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: Color)
    {
        // Determine number of points to space between the given points
        let x_dist: f64 = x1 as f64 - x0 as f64;
        let y_dist: f64 = y1 as f64 - y0 as f64;

        let n_points: f64 = (x_dist.powi(2) + y_dist.powi(2)).sqrt().ceil();

        // Determine the step size
        let x_step: f64 = x_dist / n_points;
        let y_step: f64 = y_dist / n_points;

        // Set the pixel at each point along the line
        for i_point in 0..=n_points as usize
        {
            let new_x: usize = (x0 as f64 + (x_step * i_point as f64)) as usize;
            let new_y: usize = (y0 as f64 + (y_step * i_point as f64)) as usize;

            self.set_pixel(new_x, new_y, color);
        }
    }

    // Get a c_void pointer to the image data
    pub fn get_ptr(&self) -> *const std::ffi::c_void
    {
        return self.data.as_ptr() as *const std::ffi::c_void;
    }
}
