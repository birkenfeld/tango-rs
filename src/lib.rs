extern crate libc;
extern crate time;

pub mod c;


macro_rules! tango_call {
    ($call:ident, $res:expr, $($args:expr),+) => {
        {
            let mut error = c::ErrorStack::default();
            let success = unsafe {
                c::$call($($args,)+ &mut error)
            };
            if success == 0 {
                Err(TangoError::from_stack(error))
            } else {
                Ok($res)
            }
        }
    }
}


pub mod types;
pub mod proxy;
pub mod error;

pub use types::*;
pub use proxy::*;
pub use error::*;
