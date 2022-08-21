use crate::_1_getting_started::init_window;

pub fn main_1_1_2() {
    init_window(|| {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    });
}