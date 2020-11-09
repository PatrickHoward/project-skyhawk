pub mod math;
pub mod renderer;

use math::Vec3f32;

use renderer::{
    color::Color,
    open_gl,
    vertex::{AsGLVert, GLVert, Vertex},
};

use std::{ffi::CString};

pub fn start() {
    let points: [Vec3f32; 3] = [
        Vec3f32::new(-0.5f32, -0.5f32, 0.0f32),
        Vec3f32::new(0.5f32, -0.5f32, 0.0f32),
        Vec3f32::new(0.0f32, 0.5f32, 0.0f32),
    ];

    let colors: [Color; 3] = [Color::green(), Color::blue(), Color::green()];

    let mut verts: Vec<GLVert> = vec![];
    for i in 0..points.len() {
        verts.push(Vertex::new(points[i], colors[i]).as_glvert());
    }

    let sdl = sdl2::init().expect("Failed to initialize SDL!");
    let video_subsystem = sdl.video().unwrap();

    let gl_attributes = video_subsystem.gl_attr();
    gl_attributes.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attributes.set_context_version(4, 5);

    let window = video_subsystem
        .window("Hello GL", 800, 600)
        .opengl()
        .build()
        .unwrap();

    let _gl_context = open_gl::make_context(&window);

    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let vert_shader =
        renderer::shader::VertexShader::new(&CString::new(include_str!("triangle.vert")).unwrap())
            .expect("Failed to create vertex shader");

    let frag_shader = renderer::shader::FragmentShader::new(
        &CString::new(include_str!("triangle.frag")).unwrap(),
    )
    .expect("Failed to create fragment shader");

    let shader_program = renderer::shader::GLShaderProgram::new(frag_shader, vert_shader)
        .expect("Failed to create Shader program");

    let clear_color = Color::white().as_tuple();

    let vbo = open_gl::ArrayBuffer::new();

    vbo.bind();
    vbo.static_draw(&verts);
    vbo.unbind();

    let vao = open_gl::VertexArray::new();

    vao.bind();
    vbo.bind();

    GLVert::vertex_attr_pointer();

    vbo.unbind();
    vao.unbind();

    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::ClearColor(clear_color.0, clear_color.1, clear_color.2, clear_color.3);
    }

    let mut ev_pump = sdl.event_pump().unwrap();
    'main: loop {
        for ev in ev_pump.poll_iter() {
            match ev {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::KeyDown {
                    scancode: Some(sdl2::keyboard::Scancode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
        }

        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };

        shader_program.use_program();
        vao.bind();

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 1, 4);
        }

        window.gl_swap_window();
    }
}
