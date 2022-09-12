use glfw::{init, Action, Context, Glfw, Key, OpenGlProfileHint, WindowHint, WindowMode};

use crate::drawing::{Canvas, DARK_BLUE};
pub struct GlWindow {
    width: u32,
    height: u32,
    glfw: Glfw,
    is_visible: bool,
    pub title: String,
}

impl GlWindow {
    /// Creates a new [`GlWindow`].
    ///
    /// # Panics
    /// InitError
    /// Panics if init glfw failed.
    pub fn new(width: u32, height: u32, title: &str) -> GlWindow {
        // use init glfw
        let mut fw = init(glfw::LOG_ERRORS).unwrap();
        fw.window_hint(WindowHint::ContextVersion(3, 3));
        fw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

        #[cfg(target_os = "macos")]
        fw.window_hint(WindowHint::OpenGlForwardCompat(true));

        GlWindow {
            width: width,
            height: height,
            glfw: fw,
            is_visible: false,
            title: String::from(title),
        }
    }
}

pub(crate) trait WindowContainer {
    fn is_showing(&self) -> bool;

    /// show a window with `width` * `height`
    fn show(&mut self);

    fn dismiss(&mut self);
}

impl WindowContainer for GlWindow {
    fn is_showing(&self) -> bool {
        self.is_visible
    }

    fn show(&mut self) {
        let (mut window, _events) = self
            .glfw
            .create_window(
                self.width,
                self.height,
                self.title.as_str(),
                WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window");
        window.make_current();
        window.set_all_polling(true);

        // 这个库默认是注册了 framebuffer_size_callback 回调，然后只暴露是否开启这个开关
        window.set_framebuffer_size_polling(true);

        let canvas = Canvas::from(&mut window);

        self.is_visible = true;
        // render loop
        while self.is_visible {
            // press `Esc` to  
            if window.get_key(Key::Escape) == Action::Press {
                self.dismiss();
            }

            canvas.draw_background(DARK_BLUE);

            window.swap_buffers();

            // 这条很关键，驱动事件，绘制出窗口
            self.glfw.poll_events();
        }
    }

    fn dismiss(&mut self) {
        self.is_visible = false
    }
}
