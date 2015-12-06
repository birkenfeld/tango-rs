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
                    unsafe {
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
                    unsafe { CommandInfo::from_c(cmdinfo, true) },
                    self.ptr, c_name, &mut cmdinfo)
    }

    pub fn command_list_query(&self) -> TangoResult<Vec<CommandInfo>> {
        let mut infolist = c::CommandInfoList::default();
        try!(tango_call!(tango_command_list_query, (),
                         self.ptr, &mut infolist));
        let mut res = Vec::with_capacity(infolist.length as usize);
        unsafe {
            for i in 0..infolist.length {
                let cmd_ptr = ptr::read(infolist.sequence.offset(i as isize));
                res.push(CommandInfo::from_c(cmd_ptr, false));
            }
            c::tango_free_CommandInfoList(&mut infolist);
        }
        Ok(res)
    }

    pub fn command_inout(&mut self, cmd_name: &str, argin: CommandData) -> TangoResult<CommandData> {
        let c_name = CString::new(cmd_name).unwrap().as_ptr() as *mut i8;
        let mut argin = unsafe { argin.into_c() };
        let mut argout = c::CommandData::default();
        let res = tango_call!(tango_command_inout,
                              unsafe { CommandData::from_c(argout) },
                              self.ptr, c_name, &mut argin, &mut argout);
        unsafe { CommandData::free_c_data(argin) };
        res
    }

    pub fn get_attribute_list(&self) -> TangoResult<Vec<String>> {
        let mut namelist = c::VarStringArray::default();
        try!(tango_call!(tango_get_attribute_list, (),
                         self.ptr, &mut namelist));
        let mut res = Vec::with_capacity(namelist.length as usize);
        unsafe {
            for i in 0..namelist.length {
                let name = ptr::read(namelist.sequence.offset(i as isize));
                res.push(string_from(name));
            }
            c::tango_free_VarStringArray(&mut namelist);
        }
        Ok(res)
    }

    pub fn get_attribute_config(&self, attr_names: &[&str]) -> TangoResult<Vec<AttributeInfo>> {
        let mut namelist = c::VarStringArray::default();
        let mut infolist = c::AttributeInfoList::default();
        let mut ptr_vec = Vec::with_capacity(attr_names.len());
        for name in attr_names {
            ptr_vec.push(CString::new(*name).unwrap().into_raw());
        }
        namelist.length = attr_names.len() as u32;
        namelist.sequence = ptr_vec.as_mut_ptr();
        try!(tango_call!(tango_get_attribute_config, (),
                         self.ptr, &mut namelist, &mut infolist));
        let mut res = Vec::with_capacity(infolist.length as usize);
        unsafe {
            for i in 0..infolist.length {
                let info = ptr::read(infolist.sequence.offset(i as isize));
                res.push(AttributeInfo::from_c(info));
            }
            c::tango_free_AttributeInfoList(&mut infolist);
            for ptr in ptr_vec {
                drop(CString::from_raw(ptr));
            }
        }
        Ok(res)
    }

    pub fn attribute_list_query(&self) -> TangoResult<Vec<AttributeInfo>> {
        let mut infolist = c::AttributeInfoList::default();
        try!(tango_call!(tango_attribute_list_query, (),
                         self.ptr, &mut infolist));
        let mut res = Vec::with_capacity(infolist.length as usize);
        unsafe {
            for i in 0..infolist.length {
                let info = ptr::read(infolist.sequence.offset(i as isize));
                res.push(AttributeInfo::from_c(info));
            }
            c::tango_free_AttributeInfoList(&mut infolist);
        }
        Ok(res)
    }

    pub fn read_attribute(&mut self, attr_name: &str) -> TangoResult<AttributeData> {
        let c_name = CString::new(attr_name).unwrap().as_ptr() as *mut i8;
        let mut data = c::AttributeData::default();
        tango_call!(tango_read_attribute,
                    unsafe { AttributeData::from_c(data, true) },
                    self.ptr, c_name, &mut data)
    }

    pub fn write_attribute(&mut self, attr_data: AttributeData) -> TangoResult<()> {
        let mut data = unsafe { attr_data.into_c() };
        let res = tango_call!(tango_write_attribute, (),
                              self.ptr, &mut data);
        unsafe { AttributeData::free_c_data(data) };
        res
    }

    pub fn read_attributes(&mut self, attr_names: &[&str]) -> TangoResult<Vec<AttributeData>> {
        let mut namelist = c::VarStringArray::default();
        let mut datalist = c::AttributeDataList::default();
        let mut ptr_vec = Vec::with_capacity(attr_names.len());
        for name in attr_names {
            ptr_vec.push(CString::new(*name).unwrap().into_raw());
        }
        namelist.length = attr_names.len() as u32;
        namelist.sequence = ptr_vec.as_mut_ptr();
        try!(tango_call!(tango_read_attributes, (),
                         self.ptr, &mut namelist, &mut datalist));
        let mut res = Vec::with_capacity(datalist.length as usize);
        unsafe {
            for i in 0..datalist.length {
                let data = ptr::read(datalist.sequence.offset(i as isize));
                res.push(AttributeData::from_c(data, false));
            }
            c::tango_free_AttributeDataList(&mut datalist);
            for ptr in ptr_vec {
                drop(CString::from_raw(ptr));
            }
        }
        Ok(res)
    }

    pub fn write_attributes(&mut self, attr_data: Vec<AttributeData>) -> TangoResult<()> {
        let mut datalist = c::AttributeDataList::default();
        let mut ptr_vec = Vec::with_capacity(attr_data.len());
        datalist.length = attr_data.len() as u32;
        for data in attr_data {
            ptr_vec.push(unsafe { data.into_c() });
        }
        datalist.sequence = ptr_vec.as_mut_ptr();
        try!(tango_call!(tango_write_attributes, (),
                         self.ptr, &mut datalist));
        unsafe {
            for ptr in ptr_vec {
                AttributeData::free_c_data(ptr);
            }
        }
        Ok(())
    }
}
