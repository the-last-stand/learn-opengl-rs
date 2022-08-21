use glfw::{Action, Context, init, Key, OpenGlProfileHint, Window, WindowHint, WindowMode};

extern crate gl;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main_1_1_1() {
    init_window();
}

/// 创建一个黑色的窗口，点击 `Esc` 键会关闭窗口。
/// fixme 拖动调整窗口的时候会有并不会是全黑
pub fn init_window() {
    // initialize and configure
    let mut fw = init(glfw::LOG_ERRORS).unwrap();
    fw.window_hint(WindowHint::ContextVersionMajor(3));
    fw.window_hint(WindowHint::ContextVersionMinor(3));
    fw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    #[cfg(target_os = "macos")]
    fw.window_hint(WindowHint::OpenGlForwardCompat(true));

    // window creation
    let (mut window, _events) = fw.create_window(
        SCR_WIDTH, SCR_HEIGHT, "LearnOpenGL", WindowMode::Windowed)
        .expect("Failed to create GLFW window");
    window.make_current();
    window.set_all_polling(true);
    // 这个库默认是注册了 framebuffer_size_callback 回调，然后只暴露是否开启这个开关
    window.set_framebuffer_size_polling(true);

    // load all openGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // render loop
    while !window.should_close() {
        // handle input to terminate or loop
        process_input(&mut window);

        window.swap_buffers();
        fw.poll_events();
    }
}

/// press `Esc` to close
fn process_input(window: &mut Window) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
    }
}