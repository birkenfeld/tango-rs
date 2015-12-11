extern crate libc;
extern crate tango;
extern crate c_tango as c;

use std::ffi::*;
use std::ptr;

fn main() {
    unsafe {
        let mut dev: *mut libc::c_void = ptr::null_mut();
        let dn = CString::new("tango://localhost:10000/test/benchmark/echo").unwrap();
        let cn = CString::new("Echo").unwrap();
        c::tango_create_device_proxy(dn.into_raw(), &mut dev);
        for _ in 0..2000 {
            let instr = CString::new("This is a minimal Tango test client.").unwrap();
            let mut input = c::TangoCommandData::default();
            ptr::write(input.string_val(), instr.as_ptr() as *mut i8);
            let mut argin = c::CommandData { arg_type: c::DEV_STRING,
                                             cmd_data: input };
            let mut argout = c::CommandData::default();
            c::tango_command_inout(dev, cn.as_ptr() as *mut i8,
                                   &mut argin, &mut argout);
            let outstr = CStr::from_ptr(ptr::read(argout.cmd_data.string_val()).offset(0));
            println!("{}", outstr.to_string_lossy());
            c::tango_free_CommandData(&mut argout);
        }
        c::tango_delete_device_proxy(dev);
    }
}
