pub mod math;
pub mod renderer;

use math::{Vec3f32, Vec2f32, matrix::Mat4f32};

use renderer::{
    color::Color,
    open_gl,
    vertex::{AsGLVert, GLVert, Vertex},
};
use std::{
    ffi::CString,
};
use crate::math::matrix::Axis;

pub fn start() {
    let points: [Vec3f32; 4] = [
        Vec3f32::new(0.5, 0.5, 0.0),
        Vec3f32::new(0.5, -0.5, 0.0),
        Vec3f32::new(-0.5, -0.5, 0.0),
        Vec3f32::new(-0.5, 0.5, 0.0),
    ];

    let colors: [Color; 4] = [
        Color::white(), Color::white(), Color::white(), Color::white(),
    ];

    let tex_cords: [Vec2f32; 4] = [
        Vec2f32::new(1.0f32, 1.0f32),
        Vec2f32::new(1.0f32, 0.0f32),
        Vec2f32::new(0.0f32, 0.0f32),
        Vec2f32::new(0.0f32, 1.0f32),
    ];

    let indices: [u32; 6] = [
        0, 1, 3,
        1, 2, 3,
    ];

    let mut verts: Vec<GLVert> = vec![];
    for i in 0..points.len() {
        verts.push(Vertex::new(points[i], colors[i], tex_cords[i]).as_glvert());
    }

    let sdl = sdl2::init().expect("Failed to initialize SDL!");
    let video_subsystem = sdl.video().unwrap();

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

    let box_texture = open_gl::GlTexture::new(std::path::Path::new("container.jpg"));
    let ferris_texture = open_gl::GlTexture::new(std::path::Path::new("ferris.png"));

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

    let clear_color = Color::black().as_tuple();

    let ebo = open_gl::ElementBuffer::new();
    let vbo = open_gl::ArrayBuffer::new();
    let vao = open_gl::VertexArray::new();

    let mut transform = Mat4f32::identity();
    transform.rotate(20.0f32, Axis::Z);
    transform.scale(Vec3f32::new(0.5f32, 0.5f32, 0.5f32));

    let gl_transform = open_gl::GlTransform::new(
        transform,
        &shader_program,
        "transform",
    );

    vao.bind();

    vbo.bind();
    vbo.buffer_data(&verts);
    vbo.unbind();

    ebo.bind();
    ebo.buffer_data(&indices);

    vbo.bind();

    GLVert::vertex_attr_pointer();

    vbo.unbind();
    vao.unbind();

    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::ClearColor(clear_color.0, clear_color.1, clear_color.2, clear_color.3);
    }

    let mut wireframe_enable = false;

    let mut ev_pump = sdl.event_pump().unwrap();
    'main: loop {
        for ev in ev_pump.poll_iter() {
            match ev {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::KeyDown {
                    scancode: Some(key),
                    ..
                } => match key {
                    sdl2::keyboard::Scancode::Escape => break 'main,
                    sdl2::keyboard::Scancode::Space => unsafe {
                        if !wireframe_enable {
                            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                            wireframe_enable = true;
                        } else {
                            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                            wireframe_enable = false;
                        }
                   },
                    _ => {}
                }
                _ => {}
            }
        }

        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
        shader_program.use_program();

        shader_program.set_i32("texture_a", 0);
        shader_program.set_i32("texture_b", 1);

        unsafe { gl::ActiveTexture(gl::TEXTURE0); }
        box_texture.bind();

        unsafe { gl::ActiveTexture(gl::TEXTURE1); }
        ferris_texture.bind();

        vao.bind();
        ebo.bind();

        gl_transform.transform();

        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const gl::types::GLvoid);
        }

        window.gl_swap_window();
    }
}
