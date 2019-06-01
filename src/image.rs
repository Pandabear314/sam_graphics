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

    // Get a c_void pointer to the image data
    pub fn get_ptr(&self) -> *const std::ffi::c_void
    {
        return self.data.as_ptr() as *const std::ffi::c_void;
    }
}
