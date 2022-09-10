use window::window_manager::GlWindow;

use window::WindowContainer;

mod window;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
    hello_window();
}

fn hello_window() {
    let mut window = GlWindow::new(SCR_WIDTH, SCR_HEIGHT, "Hello Window");
    window.show();
}
