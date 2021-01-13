use crate::{math::Vec2, renderer::opengl::make_context};

use sdl2::{
    video::{GLContext, Window},
    TimerSubsystem, VideoSubsystem,
};

pub enum GraphicsLibrary {
    OPENGL(u8, u8),
}

pub struct WindowContextConfig {
    pub size: Vec2,
    pub caption: &'static str,
    pub graphics_library: GraphicsLibrary,
}

pub struct WindowContext {
    pub window: Window,
    pub video: VideoSubsystem,
    pub timer: TimerSubsystem,
    pub sdl: sdl2::Sdl,

    _ctx: GLContext, // This has to be kept alive for the duration of the program
}

impl WindowContext {
    pub fn new(config: WindowContextConfig) -> Self {
        let sdl = sdl2::init().expect("Failed to initialize SDL!");
        let video = sdl.video().unwrap();
        let timer = sdl.timer().unwrap();

        if let GraphicsLibrary::OPENGL(major, minor) = config.graphics_library {
            let gl_attributes = video.gl_attr();
            gl_attributes.set_context_profile(sdl2::video::GLProfile::Core);
            gl_attributes.set_context_version(major, minor);
        }

        let window = video
            .window(config.caption, config.size.x as u32, config.size.y as u32)
            // .allow_highdpi()
            .position_centered()
            .fullscreen_desktop()
            .opengl()
            .build()
            .unwrap();

        let ctx = make_context(&window);
        let _gl = gl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

        WindowContext {
            window,
            video,
            _ctx: ctx,
            timer,
            sdl,
        }
    }
}
