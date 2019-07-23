use std::ffi::CString;
use std::mem;
use std::ptr;
use libc::{c_char, c_void};

use crate::c;
use crate::error::{TangoResult, TangoError};
use crate::types::*;


pub struct DatabaseProxy {
    ptr: *mut c_void,
}

impl Drop for DatabaseProxy {
    fn drop(&mut self) {
        let error_stack = unsafe { c::tango_delete_database_proxy(self.ptr) };
        if !error_stack.is_null() {
            // we need to construct the error to deallocate the stack
            drop(TangoError::from_stack(error_stack));
        }
    }
}

impl DatabaseProxy {
    pub fn new() -> TangoResult<DatabaseProxy> {
        let mut dev = ptr::null_mut();
        tango_call!(tango_create_database_proxy,
                    DatabaseProxy { ptr: dev },
                    &mut dev)
    }

    pub fn get_device_exported(&self, name_filter: &str) -> TangoResult<DbDatum> {
        let c_filter = CString::new(name_filter).unwrap();
        let mut db_datum = unsafe { mem::zeroed() };
        tango_call!(tango_get_device_exported,
                    unsafe { DbDatum::from_c(db_datum, true) },
                    self.ptr, c_filter.as_ptr() as *mut c_char, &mut db_datum)
    }

    pub fn get_device_exported_for_class(&self, class_name: &str) -> TangoResult<DbDatum> {
        let c_class = CString::new(class_name).unwrap();
        let mut db_datum = unsafe { mem::zeroed() };
        tango_call!(tango_get_device_exported_for_class,
                    unsafe { DbDatum::from_c(db_datum, true) },
                    self.ptr, c_class.as_ptr() as *mut c_char, &mut db_datum)
    }

    pub fn get_object_list(&self, name_filter: &str) -> TangoResult<DbDatum> {
        let c_filter = CString::new(name_filter).unwrap();
        let mut db_datum = unsafe { mem::zeroed() };
        tango_call!(tango_get_object_list,
                    unsafe { DbDatum::from_c(db_datum, true) },
                    self.ptr, c_filter.as_ptr() as *mut c_char, &mut db_datum)
    }

    pub fn get_object_property_list(&self, obj_name: &str, name_filter: &str) -> TangoResult<DbDatum> {
        let c_name = CString::new(obj_name).unwrap();
        let c_filter = CString::new(name_filter).unwrap();
        let mut db_datum = unsafe { mem::zeroed() };
        tango_call!(tango_get_object_property_list,
                    unsafe { DbDatum::from_c(db_datum, true) },
                    self.ptr, c_name.as_ptr() as *mut c_char,
                    c_filter.as_ptr() as *mut c_char, &mut db_datum)
    }

    pub fn get_property(&self, obj_name: &str, prop_list: Vec<DbDatum>) -> TangoResult<Vec<DbDatum>> {
        let c_name = CString::new(obj_name).unwrap();
        let mut db_data = unsafe { mem::zeroed::<c::DbData>() };
        let mut ptr_vec = Vec::with_capacity(prop_list.len());
        let mut cstr_vec = Vec::with_capacity(prop_list.len());
        db_data.length = prop_list.len() as u32;
        for datum in prop_list {
            let (datum, cstr) = unsafe { datum.into_c() };
            ptr_vec.push(datum);
            cstr_vec.push(cstr);
        }
        db_data.sequence = ptr_vec.as_mut_ptr();
        tango_call!(tango_get_property, (), self.ptr,
                    c_name.as_ptr() as *mut c_char, &mut db_data)?;
        let mut res = Vec::with_capacity(db_data.length as usize);
        unsafe {
            for i in 0..db_data.length {
                let db_datum = ptr::read(db_data.sequence.offset(i as isize));
                res.push(DbDatum::from_c(db_datum, false));
            }
            c::tango_free_DbData(&mut db_data);
        }
        Ok(res)
    }

    pub fn put_property(&mut self, obj_name: &str, prop_list: Vec<DbDatum>) -> TangoResult<()> {
        let c_name = CString::new(obj_name).unwrap();
        let mut db_data = unsafe { mem::zeroed::<c::DbData>() };
        let mut ptr_vec = Vec::with_capacity(prop_list.len());
        let mut cstr_vec = Vec::with_capacity(prop_list.len());
        db_data.length = prop_list.len() as u32;
        for datum in prop_list {
            let (datum, cstr) = unsafe { datum.into_c() };
            ptr_vec.push(datum);
            cstr_vec.push(cstr);
        }
        db_data.sequence = ptr_vec.as_mut_ptr();
        let res = tango_call!(tango_put_property, (),
                              self.ptr, c_name.as_ptr() as *mut c_char, &mut db_data);
        unsafe {
            for ptr in ptr_vec {
                DbDatum::free_c_data(ptr);
            }
        }
        res
    }

    pub fn delete_property(&mut self, obj_name: &str, prop_list: &[&str]) -> TangoResult<()> {
        let c_name = CString::new(obj_name).unwrap();
        let mut db_data = unsafe { mem::zeroed::<c::DbData>() };
        let mut ptr_vec = Vec::with_capacity(prop_list.len());
        let mut cstr_vec = Vec::with_capacity(prop_list.len());
        db_data.length = prop_list.len() as u32;
        for prop in prop_list {
            let datum = DbDatum::name_only(prop);
            let (datum, cstr) = unsafe { datum.into_c() };
            ptr_vec.push(datum);
            cstr_vec.push(cstr);
        }
        db_data.sequence = ptr_vec.as_mut_ptr();
        let res = tango_call!(tango_delete_property, (),
                              self.ptr, c_name.as_ptr() as *mut c_char, &mut db_data);
        unsafe {
            for ptr in ptr_vec {
                DbDatum::free_c_data(ptr);
            }
        }
        res
    }
}
