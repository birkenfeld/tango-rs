use std::ffi::CString;
use std::ptr;

use libc;

use super::c;
use super::error::{TangoResult, TangoError};
use super::types::*;


pub struct DeviceProxy {
    ptr: *mut libc::c_void,
}

impl Drop for DeviceProxy {
    fn drop(&mut self) {
        let mut error = c::ErrorStack::default();
        let success = unsafe { c::tango_delete_device_proxy(&mut self.ptr, &mut error) };
        if success == 0 {
            // we need to construct the error to deallocate the stack
            drop(TangoError::from_stack(error));
        }
    }
}

impl DeviceProxy {
    pub fn new(address: &str) -> TangoResult<DeviceProxy> {
        let mut dev = ptr::null_mut();
        let address = CString::new(address).unwrap();
        tango_call!(tango_create_device_proxy,
                    DeviceProxy { ptr: dev },
                    address.into_raw(), &mut dev)
    }

    pub fn get_timeout(&self) -> TangoResult<i32> {
        let mut res = 0;
        tango_call!(tango_get_timeout_millis,
                    res,
                    self.ptr, &mut res)
    }

    pub fn set_timeout(&mut self, timeout: i32) -> TangoResult<()> {
        tango_call!(tango_set_timeout_millis,
                    (),
                    self.ptr, timeout)
    }

    pub fn get_source(&self) -> TangoResult<DevSource> {
        let mut source = 0 as libc::c_uint;
        tango_call!(tango_get_source,
                    DevSource::from_c(source),
                    self.ptr, &mut source)
    }

    pub fn set_source(&mut self, source: DevSource) -> TangoResult<()> {
        tango_call!(tango_set_source,
                    (),
                    self.ptr, source as u32)
    }

    pub fn lock(&mut self) -> TangoResult<()> {
        tango_call!(tango_lock, (), self.ptr)
    }

    pub fn unlock(&mut self) -> TangoResult<()> {
        tango_call!(tango_unlock, (), self.ptr)
    }

    pub fn is_locked(&self) -> TangoResult<bool> {
        let mut res = 0;
        tango_call!(tango_is_locked,
                    res != 0,
                    self.ptr, &mut res)
    }

    pub fn is_locked_by_me(&self) -> TangoResult<bool> {
        let mut res = 0;
        tango_call!(tango_is_locked_by_me,
                    res != 0,
                    self.ptr, &mut res)
    }

    pub fn locking_status(&self) -> TangoResult<String> {
        let mut resptr = ptr::null_mut();
        tango_call!(tango_locking_status,
                    {
                        let res = string_from(resptr);
                        libc::free(resptr as *mut libc::c_void);
                        res
                    },
                    self.ptr, &mut resptr)
    }

    pub fn command_query(&self, cmd_name: &str) -> TangoResult<CommandInfo> {
        let c_name = CString::new(cmd_name).unwrap().as_ptr() as *mut i8;
        let mut cmdinfo = c::CommandInfo::default();
        tango_call!(tango_command_query,
                    CommandInfo::from_c(cmdinfo, true),
                    self.ptr, c_name, &mut cmdinfo)
    }

    pub fn command_list_query(&self) -> TangoResult<Vec<CommandInfo>> {
        let mut cmdinfolist = c::CommandInfoList::default();
        try!(tango_call!(tango_command_list_query,
                         (),
                         self.ptr, &mut cmdinfolist));
        let mut res = Vec::with_capacity(cmdinfolist.length as usize);
        unsafe {
            for i in 0..cmdinfolist.length {
                let cmd_ptr = ptr::read(cmdinfolist.sequence.offset(i as isize));
                res.push(CommandInfo::from_c(cmd_ptr, false));
            }
            c::tango_free_CommandInfoList(&mut cmdinfolist);
        }
        Ok(res)
    }

    pub fn command_inout(&mut self, cmd_name: &str, argin: CommandData) -> TangoResult<CommandData> {
        let c_name = CString::new(cmd_name).unwrap().as_ptr() as *mut i8;
        let mut argin = unsafe { argin.into_c() };
        let mut argout = c::CommandData::default();
        let res = tango_call!(tango_command_inout,
                              CommandData::from_c(argout),
                              self.ptr, c_name, &mut argin, &mut argout);
        unsafe { CommandData::free_c_data(argin) };
        res
    }

    pub fn get_attribute_list(&self) -> TangoResult<Vec<String>> {
        unimplemented!()
    }

    pub fn get_attribute_config(&self) -> TangoResult<Vec<(String, Vec<AttributeInfo>)>> {
        unimplemented!()
    }

    pub fn attribute_list_query(&self) -> TangoResult<Vec<Vec<AttributeInfo>>> {
        unimplemented!()
    }

    pub fn read_attribute(&mut self, attr_name: &str) -> TangoResult<AttributeData> {
        unimplemented!()
    }

    pub fn write_attribute(&mut self, attr_data: AttributeData) -> TangoResult<()> {
        unimplemented!()
    }

    pub fn read_attributes(&mut self, attr_names: Vec<String>) -> TangoResult<Vec<AttributeData>> {
        unimplemented!()
    }

    pub fn write_attributes(&mut self, attr_data: Vec<AttributeData>) -> TangoResult<()> {
        unimplemented!()
    }
}
