// TODO: logcat only in debug mode
macro_rules! logcat {
    ($val:expr) => {
        println!("Debug: {}", $val);
    };
}

pub(crate) use logcat;