use hawk_engine::{
    math::Vec2,
    renderer::window::sdl::{GraphicsLibrary, WindowContextConfig},
};

fn main() {
    let window_config = WindowContextConfig {
        size: Vec2::new(1024.0, 768.0),
        caption: "Hello GL",
        graphics_library: GraphicsLibrary::OPENGL(3, 2),
    };

    hawk_engine::start(window_config);
}
