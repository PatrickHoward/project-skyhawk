pub mod camera;
pub mod clock;
pub mod demos;
pub mod input;
pub mod math;
pub mod mem;
pub mod renderer;

use input::{Input, InputMapping};

use math::{matrix::Mat4f32, rotation::Axis, Vec2f32, Vec3f32};

use clock::Clock;

use camera::Camera;

use renderer::{
    color::Color,
    open_gl::*,
    opengl::buffer,
    shader::{GLShaderProgram, GlShaderUniform},
    vertex::{AsGLVert, GLVert, Vertex},
};

use std::ffi::CString;

use sdl2::{event::Event, video::Window, EventPump};

pub fn start() {
    let points: [Vec3f32; 36] = demos::multibox::get_verticies();
    let colors: [Color; 36] = [Color::white(); 36];
    let tex_cords: [Vec2f32; 36] = demos::multibox::get_texture_coordinates();
    let cube_pos = demos::multibox::get_cube_positions();

    let mut verts: Vec<GLVert> = vec![];
    for i in 0..points.len() {
        verts.push(Vertex::new(points[i], colors[i], tex_cords[i]).as_glvert());
    }

    // Initialize SDL and OpenGL
    let sdl = sdl2::init().expect("Failed to initialize SDL!");
    let video_subsystem = sdl.video().unwrap();
    let timer = sdl.timer().unwrap();

    let gl_attributes = video_subsystem.gl_attr();
    gl_attributes.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attributes.set_context_version(3, 2);

    let window = video_subsystem
        .window("Hello GL", 1024, 768)
        // .allow_highdpi()
        .position_centered()
        .fullscreen_desktop()
        .opengl()
        .build()
        .unwrap();

    let _gl_context = make_context(&window);

    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // -----

    // Create Textures
    let box_texture = GlTexture::from_path(std::path::Path::new("container.jpg"));
    let ferris_texture = GlTexture::from_path(std::path::Path::new("ferris.png"));
    // -----

    // Create shaders
    let vert_shader =
        renderer::shader::VertexShader::new(&CString::new(include_str!("triangle.vert")).unwrap())
            .expect("Failed to create vertex shader");

    let frag_shader = renderer::shader::FragmentShader::new(
        &CString::new(include_str!("triangle.frag")).unwrap(),
    )
    .expect("Failed to create fragment shader");

    let shader_program = renderer::shader::GLShaderProgram::new(frag_shader, vert_shader)
        .expect("Failed to create Shader program");
    // ----

    let clear_color = Color::black().as_tuple();

    // Create buffers
    let ebo = buffer::ElementBuffer::new();
    let vbo = buffer::ArrayBuffer::new();
    let vao = buffer::VertexArray::new();
    // ----

    let mut view = Mat4f32::identity();
    view.translate(Vec3f32::new(0.0f32, 0.0f32, -3.0f32));

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

    let mut clock = Clock::new(timer.performance_counter());
    let mut camera = Camera::new();
    let mut ev_pump = sdl.event_pump().unwrap();

    sdl.mouse().capture(true);
    sdl.mouse().show_cursor(false);

    'main: loop {
        clock.throttle(60);
        clock.tick(timer.performance_counter(), timer.performance_frequency());

        let exit = process_sdl_events(&mut input, &mut ev_pump);

        const EXIT_PROGRAM: InputMapping =
            InputMapping::Keyboard(sdl2::keyboard::Scancode::Escape as i32);
        if input.mapping_down(EXIT_PROGRAM) || exit {
            break 'main;
        }

        // TODO: Create window abstraction so values here are not hard coded.
        sdl.mouse().warp_mouse_in_window(&window, 1024 / 2, 768 / 2);
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
            &cube_pos,
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
    cube_pos: &[Vec3f32; 10],
    window: &Window,
) {
    unsafe {
        gl::Viewport(0, 0, 1680, 1050);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT)
    };

    shader_program.use_program();

    let texture_a = GlShaderUniform::new(shader_program, "texture_a");
    let texture_b = GlShaderUniform::new(shader_program, "texture_b");

    texture_a.set_i32(0);
    texture_b.set_i32(1);

    unsafe {
        gl::ActiveTexture(gl::TEXTURE0);
    }
    box_texture.bind();

    unsafe {
        gl::ActiveTexture(gl::TEXTURE1);
    }
    ferris_texture.bind();

    vao.bind();
    // ebo.bind();

    gl_view.set_mat(&camera.get_viewmatrix());
    gl_projection.set_mat(&camera.get_perspectivematrix());

    let mut i = 0;
    for trans in cube_pos.iter() {
        let mut model = Mat4f32::identity();
        model.translate(*trans);
        model.rotate(
            20.0f32 * i as f32,
            Axis::ARBITRARY(Vec3f32::new(1.0f32, 0.3f32, 0.5f32)),
        );

        gl_model.set_mat(&model);

        unsafe {
            // gl::DrawElements(
            //     gl::TRIANGLES,
            //     6,
            //     gl::UNSIGNED_INT,
            //     0 as *const gl::types::GLvoid,
            // );
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        i += 1;
    }

    window.gl_swap_window();
}
