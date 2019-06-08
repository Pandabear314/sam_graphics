// Allow unused functions in this module
#![allow(dead_code)]

// Define Color struct
#[derive(Copy, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub const BLACK: Color = Color{red: 0, green: 0, blue: 0};
    pub const WHITE: Color = Color{red: 255, green: 255, blue: 255};
}

// Define Image struct
pub struct Image {
    data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

// Implement Image methods
impl Image {
    // Define constants
    const DEFAULT_ALPHA: u8 = 255 as u8;
    const BYTES_PER_PIXEL: u32 = 4;

    // Define constructor
    pub fn new(im_width: u32, im_height: u32, background_color: Color) -> Image {
        // Create flat image with only background color
        let n_pixels = im_height * im_width;
        let mut flat_data: Vec<u8> = Vec::with_capacity((n_pixels * Image::BYTES_PER_PIXEL) as usize);
        for _ in 0..n_pixels {
            flat_data.push(background_color.red);
            flat_data.push(background_color.green);
            flat_data.push(background_color.blue);
            flat_data.push(Image::DEFAULT_ALPHA);
        }

        // Return image struct
        return Image{data: flat_data, width: im_width, height: im_height};
    }

    // Modify a pixel in the image
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        // Determine offset into data
        let inv_y: u32 = self.height - y - 1;                   // Invert y coordinate as image is stored upside down

        let offset: usize = 
            ((inv_y * self.width * Image::BYTES_PER_PIXEL)  +   // Ofset for full rows
            (x * Image::BYTES_PER_PIXEL)) as usize;             // Ofset for partial row

        // Modify data
        self.data[offset + 0] = color.red;
        self.data[offset + 1] = color.green;
        self.data[offset + 2] = color.blue;
    }

    // Draw an outline of a rectangle on the image
    pub fn draw_outline_rect(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, color: Color) {
        // Make (x0, y0) top left corner, and (x1, y1) bottom right corner
        let tl_x = std::cmp::min(x0, x1);
        let tl_y = std::cmp::min(y0, y1);

        let br_x = std::cmp::max(x0, x1);
        let br_y = std::cmp::max(y0, y1);

        // Draw top and bottom of rectangle
        for x in tl_x..=br_x {
            self.set_pixel(x, tl_y, color);
            self.set_pixel(x, br_y, color);
        }

        // Draw sides of rectangle
        for y in tl_y+1..br_y {
            self.set_pixel(tl_x, y, color);
            self.set_pixel(br_x, y, color);
        }
    }

    // Draw a filled in rectangle on the image
    pub fn draw_filled_rect(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, color: Color) {
        // Make (x0, y0) top left corner, and (x1, y1) bottom right corner
        let tl_x = std::cmp::min(x0, x1);
        let tl_y = std::cmp::min(y0, y1);

        let br_x = std::cmp::max(x0, x1);
        let br_y = std::cmp::max(y0, y1);

        // Fill in rectangular region
        for x in tl_x..=br_x {
            for y in tl_y..=br_y {
                self.set_pixel(x, y, color);
            }
        }
    }

    // Draw a line on the image
    pub fn draw_line(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, color: Color) {
        // Determine number of points to space between the given points
        let x_dist: f64 = x1 as f64 - x0 as f64;
        let y_dist: f64 = y1 as f64 - y0 as f64;

        let n_points: f64 = (x_dist.powi(2) + y_dist.powi(2)).sqrt().ceil();

        // Determine the step size
        let x_step: f64 = x_dist / n_points;
        let y_step: f64 = y_dist / n_points;

        // Set the pixel at each point along the line
        for i_point in 0..=n_points as u32 {
            let new_x: u32 = (x0 as f64 + (x_step * i_point as f64)) as u32;
            let new_y: u32 = (y0 as f64 + (y_step * i_point as f64)) as u32;

            self.set_pixel(new_x, new_y, color);
        }
    }

    // Get a c_void pointer to the image data
    pub fn get_ptr(&self) -> *const std::ffi::c_void {
        return self.data.as_ptr() as *const std::ffi::c_void;
    }
}
