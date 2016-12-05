#![feature(test)]

extern crate libc;
extern crate tango;
extern crate c_tango as c;
extern crate test;

use std::ffi::*;
use std::ptr;

#[bench]
fn wrapped(b: &mut test::Bencher) {
    let mut dev = tango::DeviceProxy::new("tango://localhost:10000/sys/tg_test/1").unwrap();
    let s = "This is a minimal Tango test client.";
    b.iter(|| {
        let instr = tango::CommandData::from_str(s);
        let argout = dev.command_inout("DevString", instr).unwrap();
        assert_eq!(argout.into_string().unwrap(), s);
    });
}

#[bench]
fn raw(b: &mut test::Bencher) {
    unsafe {
        let mut dev: *mut libc::c_void = ptr::null_mut();
        let dn = CString::new("tango://localhost:10000/sys/tg_test/1").unwrap();
        let cn = CString::new("DevString").unwrap();
        let s = "This is a minimal Tango test client.";
        c::tango_create_device_proxy(dn.as_ptr() as *mut i8, &mut dev);
        b.iter(|| {
            let instr = CString::new(s).unwrap();
            let mut input = c::TangoCommandData::default();
            ptr::write(input.string_val(), instr.as_ptr() as *mut i8);
            let mut argin = c::CommandData { arg_type: c::DEV_STRING,
                                             cmd_data: input };
            let mut argout = c::CommandData::default();
            c::tango_command_inout(dev, cn.as_ptr() as *mut i8,
                                   &mut argin, &mut argout);
            let outstr = CStr::from_ptr(ptr::read(argout.cmd_data.string_val()).offset(0));
            assert_eq!(outstr.to_string_lossy(), s);
            c::tango_free_CommandData(&mut argout);
        });
        c::tango_delete_device_proxy(dev);
    }
}