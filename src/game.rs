// Import crates
extern crate gl;
extern crate glutin;
extern crate rand;

extern crate drawing;

// Define namespaces
use drawing::image;

pub fn run(
    window_context: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::Window>, 
    mut event_loop: glutin::EventsLoop, 
    mut screen_buffer: image::Image,
    window_size: (f64, f64)) {

    // Define frame rate
    let frame_length = std::time::Duration::from_micros(((1.0 / 60.0) * 1e6) as u64);

    // Define game state
    let mut game_state = 0; // Default, TODO: Make enum

    // Load GL
    let _open_gl = gl::load_with(|ptr| window_context.get_proc_address(ptr) as *const _);

    // Start game loop
    let mut run_game = true;
    while run_game {
        // Get current time
        let now = std::time::Instant::now();

        // Handle window events
        event_loop.poll_events(|event|
            match event {
                glutin::Event::WindowEvent {event: glutin::WindowEvent::CloseRequested, ..} => run_game = false,
                _ => (),
            }
        );

        // Handle game states
        match game_state {
            0 => default_state(&mut screen_buffer),
            _ => (),
        }

        // Draw to buffer
        buffer_draw(&screen_buffer, &window_size);

        // Wait until end of frame
        while now.elapsed() < frame_length {
            std::thread::sleep(std::time::Duration::from_micros(100));
        }

        // Update screen
        let _ = window_context.swap_buffers();
    }
}

fn default_state(screen_buffer: &mut image::Image) {
    // Clear buffer
    *screen_buffer = image::Image::new(screen_buffer.width, screen_buffer.height, image::Color::BLACK);

    // Get two random numbers
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let rand_x0 = rng.gen::<usize>() % screen_buffer.width;
    let rand_y0 = rng.gen::<usize>() % screen_buffer.height;
    let rand_x1 = rng.gen::<usize>() % screen_buffer.width;
    let rand_y1 = rng.gen::<usize>() % screen_buffer.height;

    // Draw a line
    screen_buffer.draw_line(rand_x0, rand_y0, rand_x1, rand_y1, image::Color::WHITE);
}

fn buffer_draw(screen_buffer: &image::Image, window_size: &(f64, f64)) {
    // Interface with open gl
    unsafe {
        // Create texture
        let mut tex_id: gl::types::GLuint = 0;
        gl::GenTextures(1, &mut tex_id);
        gl::BindTexture(gl::TEXTURE_2D, tex_id);

        // Move pixels to texture
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, 
            screen_buffer.width as i32, screen_buffer.height as i32, 
            0, gl::RGBA, gl::UNSIGNED_BYTE, screen_buffer.get_ptr());

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        // Create read framebuffer
        let mut read_fbo_id: gl::types::GLuint = 0;
        gl::GenFramebuffers(1, &mut read_fbo_id);
        gl::BindFramebuffer(gl::READ_FRAMEBUFFER, read_fbo_id);

        // Bind texture to read framebuffer
        gl::FramebufferTexture2D(gl::READ_FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, tex_id, 0);

        // Transfer from read framebuffer to draw framebuffer
        gl::BlitFramebuffer(0, 0, screen_buffer.width as i32, screen_buffer.height as i32, 
            0, 0, window_size.0 as i32, window_size.1 as i32, 
            gl::COLOR_BUFFER_BIT, gl::NEAREST);

        // Clear out read framebuffer
        gl::BindFramebuffer(gl::READ_FRAMEBUFFER, 0);
        gl::DeleteFramebuffers(1, &mut read_fbo_id);
    };
}