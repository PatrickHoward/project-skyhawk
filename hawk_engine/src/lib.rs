pub mod math;
pub mod renderer;
pub mod input;
pub mod clock;

use input::{Input, InputMapping};

use math::{Vec3f32, Vec2f32, matrix::Mat4f32, rotation::Axis};

use clock::Clock;

use renderer::{
    color::Color,
    open_gl,
    vertex::{AsGLVert, GLVert, Vertex},
};

use std::{ffi::CString, time};

use sdl2::event::Event;

pub fn start() {
    // let points: [Vec3f32; 4] = [
    //     Vec3f32::new(0.5, 0.5, 0.0),
    //     Vec3f32::new(0.5, -0.5, 0.0),
    //     Vec3f32::new(-0.5, -0.5, 0.0),
    //     Vec3f32::new(-0.5, 0.5, 0.0),
    // ];

    let points: [Vec3f32; 36] = [
        Vec3f32::new(-0.5f32, -0.5f32, -0.5f32),
        Vec3f32::new(0.5f32, -0.5f32, -0.5f32),
        Vec3f32::new(0.5f32,  0.5f32, -0.5f32),
        Vec3f32::new(0.5f32,  0.5f32, -0.5f32),
        Vec3f32::new(-0.5f32,  0.5f32, -0.5f32),
        Vec3f32::new(-0.5f32, -0.5f32, -0.5f32),

        Vec3f32::new( -0.5f32, -0.5f32,  0.5f32),
        Vec3f32::new(0.5f32, -0.5f32,  0.5f32),
        Vec3f32::new(0.5f32,  0.5f32,  0.5f32),
        Vec3f32::new(0.5f32,  0.5f32,  0.5f32),
        Vec3f32::new(-0.5f32,  0.5f32,  0.5f32),
        Vec3f32::new(-0.5f32, -0.5f32,  0.5f32),

        Vec3f32::new( -0.5f32,  0.5f32,  0.5f32),
        Vec3f32::new(-0.5f32,  0.5f32, -0.5f32),
        Vec3f32::new(-0.5f32, -0.5f32, -0.5f32),
        Vec3f32::new(-0.5f32, -0.5f32, -0.5f32),
        Vec3f32::new(-0.5f32, -0.5f32,  0.5f32),
        Vec3f32::new(-0.5f32,  0.5f32,  0.5f32),

        Vec3f32::new(0.5f32,  0.5f32,  0.5f32),
        Vec3f32::new(0.5f32,  0.5f32, -0.5f32),
        Vec3f32::new(0.5f32, -0.5f32, -0.5f32),
        Vec3f32::new(0.5f32, -0.5f32, -0.5f32),
        Vec3f32::new(0.5f32, -0.5f32,  0.5f32),
        Vec3f32::new(0.5f32,  0.5f32,  0.5f32),

        Vec3f32::new(-0.5f32, -0.5f32, -0.5f32),
        Vec3f32::new(0.5f32, -0.5f32, -0.5f32),
        Vec3f32::new(0.5f32, -0.5f32,  0.5f32),
        Vec3f32::new(0.5f32, -0.5f32,  0.5f32),
        Vec3f32::new(-0.5f32, -0.5f32,  0.5f32),
        Vec3f32::new(-0.5f32, -0.5f32, -0.5f32),

        Vec3f32::new(-0.5f32,  0.5f32, -0.5f32),
        Vec3f32::new(0.5f32,  0.5f32, -0.5f32),
        Vec3f32::new(0.5f32,  0.5f32,  0.5f32),
        Vec3f32::new(0.5f32,  0.5f32,  0.5f32),
        Vec3f32::new(-0.5f32,  0.5f32,  0.5f32),
        Vec3f32::new(-0.5f32,  0.5f32, -0.5f32),
    ];

    let colors: [Color; 36] = [ Color::white(); 36 ];

    // let tex_cords: [Vec2f32; 4] = [
    //     Vec2f32::new(1.0f32, 1.0f32),
    //     Vec2f32::new(1.0f32, 0.0f32),
    //     Vec2f32::new(0.0f32, 0.0f32),
    //     Vec2f32::new(0.0f32, 1.0f32),
    // ];

    let tex_cords: [Vec2f32; 36] = [
        Vec2f32::new(0.0f32, 0.0f32),
        Vec2f32::new(1.0f32, 0.0f32),
        Vec2f32::new(1.0f32, 1.0f32),
        Vec2f32::new(1.0f32, 1.0f32),
        Vec2f32::new(0.0f32, 1.0f32),
        Vec2f32::new(0.0f32, 0.0f32),

        Vec2f32::new(0.0f32, 0.0f32),
        Vec2f32::new(1.0f32, 0.0f32),
        Vec2f32::new(1.0f32, 1.0f32),
        Vec2f32::new(1.0f32, 1.0f32),
        Vec2f32::new(0.0f32, 1.0f32),
        Vec2f32::new(0.0f32, 0.0f32),

        Vec2f32::new(1.0f32, 0.0f32),
        Vec2f32::new(1.0f32, 1.0f32),
        Vec2f32::new(0.0f32, 1.0f32),
        Vec2f32::new(0.0f32, 1.0f32),
        Vec2f32::new(0.0f32, 0.0f32),
        Vec2f32::new(1.0f32, 0.0f32),

        Vec2f32::new(1.0f32, 0.0f32),
        Vec2f32::new(1.0f32, 1.0f32),
        Vec2f32::new(0.0f32, 1.0f32),
        Vec2f32::new(0.0f32, 1.0f32),
        Vec2f32::new(0.0f32, 0.0f32),
        Vec2f32::new(1.0f32, 0.0f32),

        Vec2f32::new(0.0f32, 1.0f32),
        Vec2f32::new(1.0f32, 1.0f32),
        Vec2f32::new(1.0f32, 0.0f32),
        Vec2f32::new(1.0f32, 0.0f32),
        Vec2f32::new(0.0f32, 0.0f32),
        Vec2f32::new(0.0f32, 1.0f32),

        Vec2f32::new(0.0f32, 1.0f32),
        Vec2f32::new(1.0f32, 1.0f32),
        Vec2f32::new(1.0f32, 0.0f32),
        Vec2f32::new(1.0f32, 0.0f32),
        Vec2f32::new(0.0f32, 0.0f32),
        Vec2f32::new(0.0f32, 1.0f32),
    ];

    let indices: [u32; 6] = [
        0, 1, 3,
        1, 2, 3,
    ];

    let cube_pos: [Vec3f32; 10] = [
        Vec3f32::zero(),
        Vec3f32::new(2.0f32,  5.0f32, -15.0f32),
        Vec3f32::new(-1.5f32, -2.2f32, -2.5f32),
        Vec3f32::new(-3.8f32, -2.0f32, -12.3f32),
        Vec3f32::new(2.4f32, -0.4f32, -3.5f32),
        Vec3f32::new(-1.7f32,  3.0f32, -7.5f32),
        Vec3f32::new(1.3f32, -2.0f32, -2.5f32),
        Vec3f32::new(1.5f32,  2.0f32, -2.5f32),
        Vec3f32::new(1.5f32,  0.2f32, -1.5f32),
        Vec3f32::new(-1.3f32,  1.0f32, -1.5f32)
    ];

    let mut verts: Vec<GLVert> = vec![];
    for i in 0..points.len() {
        verts.push(Vertex::new(points[i], colors[i], tex_cords[i]).as_glvert());
    }

    // Initialize SDL and OpenGL
    let sdl = sdl2::init().expect("Failed to initialize SDL!");
    let video_subsystem = sdl.video().unwrap();
    let mut timer = sdl.timer().unwrap();



    let gl_attributes = video_subsystem.gl_attr();
    gl_attributes.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attributes.set_context_version(4, 5);

    let window = video_subsystem
        .window("Hello GL", 800, 600)
        .allow_highdpi()
        .opengl()
        .build()
        .unwrap();

    let _gl_context = open_gl::make_context(&window);

    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // -----

    // Create Textures
    let box_texture = open_gl::GlTexture::from_path(std::path::Path::new("container.jpg"));
    let ferris_texture = open_gl::GlTexture::from_path(std::path::Path::new("ferris.png"));
    // -----

    // Create shaders
    let vert_shader =
        renderer::shader::VertexShader::new(
            &CString::new(include_str!("triangle.vert")).unwrap())
            .expect("Failed to create vertex shader");

    let frag_shader = renderer::shader::FragmentShader::new(
        &CString::new(include_str!("triangle.frag")).unwrap(),
    )
    .expect("Failed to create fragment shader");

    let shader_program =
        renderer::shader::GLShaderProgram::new(frag_shader, vert_shader)
        .expect("Failed to create Shader program");
    // ----

    let clear_color = Color::black().as_tuple();

    // Create buffers
    let ebo = open_gl::ElementBuffer::new();
    let vbo = open_gl::ArrayBuffer::new();
    let vao = open_gl::VertexArray::new();
    // ----

    const camera_radius: f32 = 10.0f32;

    let mut view = Mat4f32::identity();
    view.translate(Vec3f32::new(0.0f32, 0.0f32, -3.0f32));

    let projection = Mat4f32::perspective(
        45.0f32,
        800.0f32 / 600.0f32,
        0.1f32,
        100.0f32
    );

    let gl_model = open_gl::GlMat4f32Handle::new(
        &shader_program,
        "model",
    );

    let gl_view = open_gl::GlMat4f32Handle::new(
        &shader_program,
        "view",
    );

    let gl_projection = open_gl::GlMat4f32Handle::new(
        &shader_program,
        "projection"
    );

    let mut input = Input::new();

    vao.bind();

    vbo.bind();
    vbo.buffer_data(&verts);
    vbo.unbind();

    // ebo.bind();
    // ebo.buffer_data(&indices);

    vbo.bind();

    GLVert::vertex_attr_pointer();

    vbo.unbind();
    vao.unbind();

    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(clear_color.0, clear_color.1, clear_color.2, clear_color.3);
    }

    let mut clock = Clock::new(timer.performance_counter());

    let mut ev_pump = sdl.event_pump().unwrap();

    'main: loop {

        clock.throttle(60);
        clock.tick(timer.performance_counter(), timer.performance_frequency());

        for ev in ev_pump.poll_iter() {
            match ev {
                Event::Quit { .. } =>
                    break 'main,
                Event::KeyDown { scancode: Some(key), .. } =>
                    input.set(InputMapping::Keyboard(key as i32), true),
                Event::KeyUp { scancode: Some(key), .. } =>
                    input.set(InputMapping::Keyboard(key as i32), false),
                Event::MouseButtonDown { mouse_btn, .. } =>
                    input.set(InputMapping::Mouse(mouse_btn), true),
                Event::MouseButtonUp { mouse_btn, .. } =>
                    input.set(InputMapping::Mouse(mouse_btn), false),
                _ => {}
            }
        }

        input.tick();

        const EXIT_PROGRAM: InputMapping = InputMapping::Keyboard(sdl2::keyboard::Scancode::Escape as i32);
        if input.mapping_down(EXIT_PROGRAM) {
            break 'main;
        }

        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) };
        shader_program.use_program();

        shader_program.set_i32("texture_a", 0);
        shader_program.set_i32("texture_b", 1);

        unsafe { gl::ActiveTexture(gl::TEXTURE0); }
        box_texture.bind();

        unsafe { gl::ActiveTexture(gl::TEXTURE1); }
        ferris_texture.bind();

        vao.bind();
        // ebo.bind();

        let camera_x = (timer.ticks() as f32).sin() * camera_radius;
        let camera_z = (timer.ticks() as f32).cos() * camera_radius;

        let view = Mat4f32::look_at(
            Vec3f32::new(camera_x, 0.0f32, camera_z),
            Vec3f32::zero(),
            Vec3f32::new(0.0f32, 1.0f32, 0.0f32)
        );

        gl_view.transform(&view);
        gl_projection.transform(&projection);

        let mut i = 0;
        for trans in cube_pos.iter() {
            let mut model = Mat4f32::identity();
            model.translate(*trans);
            model.rotate(20.0f32 * i as f32, Axis::ARBITRARY(Vec3f32::new(1.0f32, 0.3f32, 0.5f32)));

            gl_model.transform(&model);

            unsafe {
                // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const gl::types::GLvoid);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

            i += 1;
        }

        window.gl_swap_window();
    }
}
