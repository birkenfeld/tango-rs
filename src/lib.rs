extern crate libc;
extern crate time;
extern crate c_tango;


macro_rules! tango_call {
    ($call:ident, $res:expr, $($args:expr),+) => {
        {
            let error_stack = unsafe {
                c::$call($($args,)+)
            };
            if !error_stack.is_null() {
                Err(TangoError::from_stack(error_stack))
            } else {
                Ok($res)
            }
        }
    }
}


pub mod types;
pub mod error;
pub mod proxy;
pub mod dbase;

pub use types::*;
pub use error::*;
pub use proxy::*;
pub use dbase::*;
