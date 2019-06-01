extern crate gl;
extern crate glutin;

pub fn create_window(window_width: f64, window_height: f64, image_width: i32, image_height: i32, image_ptr: *const std::ffi::c_void) {
    // Initialize events loop
    let mut events_loop = glutin::EventsLoop::new();

    // Initialize window builder
    let window_builder = glutin::WindowBuilder::new()
        .with_title("")
        .with_dimensions(glutin::dpi::LogicalSize::new(window_width, window_height));

    // Initialize window
    let windowed_context = glutin::ContextBuilder::new()
        .build_windowed(window_builder, &events_loop)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    // Set window properties
    windowed_context.window().set_resizable(false);

    // Load GL
    let _open_gl = gl::load_with(|ptr| windowed_context.get_proc_address(ptr) as *const _);

    // Event loop
    let mut is_open = true;
    while is_open
    {
        // Check for new events
        events_loop.poll_events(|event|
            match event
            {
                glutin::Event::WindowEvent {event: glutin::WindowEvent::CloseRequested, ..} => is_open = false,
                _ => (),
            }
        );

        // // Determine pixels
        // let texture_width: i32 = 2;
        // let texture_height: i32 = 2;
        // let texture_pixels: *const std::ffi::c_void = 
        //     ([255, 0, 0, 255,   0, 255, 0, 255,
        //       0, 0, 255, 255,   0, 255, 255, 255] as [u8; 16])
        //         .as_ptr() as *const std::ffi::c_void;

        unsafe 
        {
            // Create texture
            let mut tex_id: gl::types::GLuint = 0;
            gl::GenTextures(1, &mut tex_id);
            gl::BindTexture(gl::TEXTURE_2D, tex_id);

            // Move pixels to texture
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, 
                image_width, image_height, 0, gl::RGBA, gl::UNSIGNED_BYTE, image_ptr);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            // Create read framebuffer
            let mut read_fbo_id: gl::types::GLuint = 0;
            gl::GenFramebuffers(1, &mut read_fbo_id);
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, read_fbo_id);

            // Bind texture to read framebuffer
            gl::FramebufferTexture2D(gl::READ_FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, tex_id, 0);

            // Transfer from read framebuffer to draw framebuffer
            gl::BlitFramebuffer(0, 0, image_width, image_height, 
                0, 0, window_width as i32, window_height as i32, 
                gl::COLOR_BUFFER_BIT, gl::NEAREST);

            // Clear out read framebuffer
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, 0);
            gl::DeleteFramebuffers(1, &mut read_fbo_id);
        };

        // Update screen
        let _ = windowed_context.swap_buffers();
        std::thread::sleep(std::time::Duration::from_millis(15));
    }
}
