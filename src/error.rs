use std::error;
use std::fmt;
use std::ptr;

use crate::c;
use crate::types::{ErrSeverity, string_from};

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
        write!(f, "TangoError[\n")?;
        for fail in &self.failures {
            write!(f, "DevFailed[\n")?;
            write!(f, "      desc = {}\n", fail.desc)?;
            write!(f, "    origin = {}\n", fail.origin)?;
            write!(f, "    reason = {}\n", fail.reason)?;
            write!(f, "  severity = {:?}]\n\n", fail.severity)?;
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
    pub fn from_stack(stackptr: *mut c::ErrorStack) -> TangoError {
        let stack = unsafe  { *stackptr };
        let mut seq = Vec::with_capacity(stack.length as usize);
        for i in 0..stack.length {
            unsafe {
                let df = ptr::read(stack.sequence.offset(i as isize));
                let fail = TangoFailure {
                    desc: string_from(df.desc),
                    reason: string_from(df.reason),
                    origin: string_from(df.origin),
                    severity: ErrSeverity::from_c(df.severity)
                };
                seq.push(fail);
            }
        }
        unsafe { c::tango_free_ErrorStack(stackptr); }
        TangoError { failures: seq }
    }
}
