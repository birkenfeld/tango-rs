use std::error;
use std::ffi::CStr;
use std::fmt;
use std::ptr;

use c_tango as c;

use super::types::ErrSeverity;

pub type TangoResult<R> = Result<R, TangoError>;


#[derive(Debug)]
pub struct TangoFailure {
    pub desc: String,
    pub reason: String,
    pub origin: String,
    pub severity: ErrSeverity,
}

#[derive(Debug)]
pub struct TangoError {
    pub failures: Vec<TangoFailure>,
}

impl fmt::Display for TangoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "TangoError[\n"));
        for fail in &self.failures {
            try!(write!(f, "DevFailed[\n"));
            try!(write!(f, "      desc = {}\n", fail.desc));
            try!(write!(f, "    origin = {}\n", fail.origin));
            try!(write!(f, "    reason = {}[\n", fail.reason));
            try!(write!(f, "  severity = {:?}]\n\n", fail.severity));
        }
        write!(f, "]\n")
    }
}

impl error::Error for TangoError {
    fn description(&self) -> &str {
        if self.failures.is_empty() {
            ""
        } else {
            &self.failures[0].desc
        }
    }
}

impl TangoError {
    pub fn from_stack(mut stack: c::ErrorStack) -> TangoError {
        let mut seq = Vec::with_capacity(stack.length as usize);
        for i in 0..stack.length {
            unsafe {
                let df = ptr::read(stack.sequence.offset(i as isize));
                let fail = TangoFailure {
                    desc: CStr::from_ptr(df.desc).to_string_lossy().into_owned(),
                    reason: CStr::from_ptr(df.reason).to_string_lossy().into_owned(),
                    origin: CStr::from_ptr(df.origin).to_string_lossy().into_owned(),
                    severity: ErrSeverity::from_c(df.severity)
                };
                seq.push(fail);
            }
        }
        unsafe { c::tango_free_ErrorStack(&mut stack); }
        TangoError { failures: seq }
    }
}
