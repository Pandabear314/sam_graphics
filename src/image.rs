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
    pub width: i32,
    pub height: i32,
}

// Implement Image methods
impl Image
{
    pub fn new(pixels: Vec<Vec<Color>>) -> Image
    {
        // Determine size of image
        let im_height: usize = pixels.len();
        let im_width: usize = pixels[0].len();

        // Flatten pixels and flip upside down
        let mut flat_data: Vec<u8> = Vec::with_capacity(im_height * im_width);
        for i_row in (0..im_height).rev()
        {
            for i_pixel in 0..im_width
            {
                flat_data.push(pixels[i_row][i_pixel].red);
                flat_data.push(pixels[i_row][i_pixel].green);
                flat_data.push(pixels[i_row][i_pixel].blue);
                flat_data.push(255 as u8);                      // Add alpha value
            }
        }

        // Return image struct
        return Image{data: flat_data, width: im_width as i32, height: im_height as i32};
    }

    pub fn get_ptr(&self) -> *const std::ffi::c_void
    {
        return self.data.as_ptr() as *const std::ffi::c_void;
    }
}