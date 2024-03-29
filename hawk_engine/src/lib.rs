pub mod camera;
pub mod demos;
pub mod input;
pub mod math;
pub mod mem;
pub mod renderer;
pub mod time;

use camera::Camera;
use input::{Input, InputMapping};
use math::{matrix::Mat4f32, rotation::Axis, Vec2, Vec3};
use renderer::opengl::shader::{GLShaderProgram, GlShaderUniform};
use renderer::{
    color::Color,
    opengl,
    opengl::{buffer, texture::GlTexture},
    vertex::{AsGLVert, GLVert, Vertex},
    window::sdl::*,
};
use sdl2::{event::Event, video::Window, EventPump};
use std::ffi::CString;
use time::clock::Clock;

pub fn start(window_config: WindowContextConfig) {
    let points: [Vec3; 36] = demos::multibox::get_verticies();
    let colors: [Color; 36] = [Color::white(); 36];
    let tex_cords: [Vec2; 36] = demos::multibox::get_texture_coordinates();
    let cube_pos = demos::multibox::get_cube_positions();

    let mut verts: Vec<GLVert> = vec![];
    for i in 0..points.len() {
        verts.push(Vertex::new(points[i], colors[i], tex_cords[i]).as_glvert());
    }

    // Initialize SDL and OpenGL
    let window = WindowContext::new(window_config);

    // Create Textures
    let box_texture = GlTexture::from_path(std::path::Path::new("container.jpg"));
    let ferris_texture = GlTexture::from_path(std::path::Path::new("ferris.png"));

    // Create shaders
    let vert_shader =
        opengl::shader::VertexShader::new(&CString::new(include_str!("triangle.vert")).unwrap())
            .expect("Failed to create vertex shader");

    let frag_shader =
        opengl::shader::FragmentShader::new(&CString::new(include_str!("triangle.frag")).unwrap())
            .expect("Failed to create fragment shader");

    let shader_program = opengl::shader::GLShaderProgram::new(frag_shader, vert_shader)
        .expect("Failed to create Shader program");
    // ----

    let clear_color = Color::black().as_tuple();

    // Create buffers
    // let ebo = buffer::ElementBuffer::new();
    let vbo = buffer::ArrayBuffer::new();
    let vao = buffer::VertexArray::new();
    // ----

    let mut view = Mat4f32::identity();
    view.translate(Vec3::new(0.0f32, 0.0f32, -3.0f32));

    let gl_model = GlShaderUniform::new(&shader_program, "model");
    let gl_view = GlShaderUniform::new(&shader_program, "view");
    let gl_projection = GlShaderUniform::new(&shader_program, "projection");

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
        gl::Viewport(0, 0, 0, 0);
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(clear_color.0, clear_color.1, clear_color.2, clear_color.3);
    }

    let mut clock = Clock::new(window.timer.performance_counter());
    let mut camera = Camera::new();
    let mut ev_pump = window.sdl.event_pump().unwrap();

    window.sdl.mouse().capture(true);
    window.sdl.mouse().show_cursor(false);

    'main: loop {
        clock.throttle(60);
        clock.tick(
            window.timer.performance_counter(),
            window.timer.performance_frequency(),
        );

        let exit = process_sdl_events(&mut input, &mut ev_pump);

        const EXIT_PROGRAM: InputMapping =
            InputMapping::Keyboard(sdl2::keyboard::Scancode::Escape as i32);
        if input.mapping_down(EXIT_PROGRAM) || exit {
            break 'main;
        }

        // TODO: Create window abstraction so values here are not hard coded
        window.sdl.mouse().warp_mouse_in_window(
            &window.window,
            (window.size.x / 2.0f32) as i32,
            (window.size.y / 2.0f32) as i32,
        );
        let mouse_offset = input.get_mousepos_offset();
        let scroll_delta = input.get_mouse_scroll_delta();

        camera.addto_yaw(mouse_offset.x);
        camera.addto_pitch(mouse_offset.y);
        camera.addto_fov(scroll_delta);
        camera.tick(clock.dt() as f32, &input);

        render(
            &shader_program,
            &box_texture,
            &ferris_texture,
            &camera,
            &vao,
            &gl_view,
            &gl_projection,
            &gl_model,
            &window,
        );

        input.tick();
    }
}

// TODO: I shouldn't return a bool here
pub fn process_sdl_events(input: &mut Input, ev_pump: &mut EventPump) -> bool {
    for ev in ev_pump.poll_iter() {
        match ev {
            Event::Quit { .. } => return true,
            Event::KeyDown {
                scancode: Some(key),
                ..
            } => input.set(InputMapping::Keyboard(key as i32), true),
            Event::KeyUp {
                scancode: Some(key),
                ..
            } => input.set(InputMapping::Keyboard(key as i32), false),
            Event::MouseButtonDown { mouse_btn, .. } => {
                input.set(InputMapping::Mouse(mouse_btn), true)
            }
            Event::MouseButtonUp { mouse_btn, .. } => {
                input.set(InputMapping::Mouse(mouse_btn), false)
            }
            Event::MouseMotion { x, y, .. } => {
                input.set(InputMapping::MousePos(x, y), false);
            }
            Event::MouseWheel { x: _x, y, .. } => {
                input.set(InputMapping::MouseScroll(y), false);
            }
            _ => {}
        }
    }

    false
}

// TODO: The rendering logic takes a lot of parameters, consider creating an abstraction for the
// renderer
pub fn render(
    shader_program: &GLShaderProgram,
    box_texture: &GlTexture,
    ferris_texture: &GlTexture,
    camera: &Camera,
    vao: &buffer::VertexArray,
    gl_view: &GlShaderUniform,
    gl_projection: &GlShaderUniform,
    gl_model: &GlShaderUniform,
    window_context: &WindowContext,
) {
    unsafe {
        gl::Viewport(0, 0, 1680, 1050);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT)
    };

    shader_program.use_program();

    let texture_a = GlShaderUniform::new(shader_program, "texture_a");

    texture_a.set_i32(0);

    unsafe {
        gl::ActiveTexture(gl::TEXTURE0);
    }

    box_texture.bind();

    vao.bind();
    // ebo.bind();

    gl_view.set_mat(&camera.get_viewmatrix());
    gl_projection.set_mat(&camera.get_perspectivematrix(window_context));

    let mut model = Mat4f32::identity();
    gl_model.set_mat(&model);

    unsafe {
        gl::DrawArrays(gl::TRIANGLES, 0, 36);
    }

    window_context.window.gl_swap_window();
}
