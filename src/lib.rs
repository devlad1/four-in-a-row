extern crate web_sys;
extern crate enum_map;

mod game;

mod lib {
    #[macro_export]
    macro_rules! log {
        ( $( $t:tt )* ) => {
            web_sys::console::log_1(&format!( $( $t )* ).into());
        }
    }

    pub fn max<T: PartialOrd>(a: T, b: T) -> T {
        if a > b {
            a
        } else { 
            b
        }
    }
}

