extern crate web_sys;

mod game;

mod lib {
    #[macro_export]
    macro_rules! log {
        ( $( $t:tt )* ) => {
            web_sys::console::log_1(&format!( $( $t )* ).into());
        }
    }
}

