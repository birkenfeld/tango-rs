use std::ffi::{CStr, CString};
use std::{fmt, mem, slice};
use libc::{self, c_char, c_long};
use time::{get_time, Timespec};

use crate::c;


pub unsafe fn string_from(ptr: *const c_char) -> String {
    CStr::from_ptr(ptr).to_string_lossy().into_owned()
}

pub fn cstring_from<I: Into<Vec<u8>>>(v: I) -> CString {
    match CString::new(v) {
        Ok(s) => s,
        Err(err) => {
            let end = err.nul_position();
            let v = err.into_vec();
            CString::new(&v[..end]).unwrap()
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TangoDataType {
    Void,
    Boolean,
    Short,
    Long,
    Float,
    Double,
    UShort,
    ULong,
    String,
    CharArray,
    ShortArray,
    LongArray,
    FloatArray,
    DoubleArray,
    UShortArray,
    ULongArray,
    StringArray,
    LongStringArray,
    DoubleStringArray,
    State,
    ConstString,
    BooleanArray,
    UChar,
    Long64,
    ULong64,
    Long64Array,
    ULong64Array,
    Int,
    Encoded,
}

impl TangoDataType {
    fn from_c(val: c::TangoDataType) -> TangoDataType {
        match val {
            c::TangoDataType_DEV_VOID => TangoDataType::Void,
            c::TangoDataType_DEV_BOOLEAN => TangoDataType::Boolean,
            c::TangoDataType_DEV_SHORT => TangoDataType::Short,
            c::TangoDataType_DEV_LONG => TangoDataType::Long,
            c::TangoDataType_DEV_FLOAT => TangoDataType::Float,
            c::TangoDataType_DEV_DOUBLE => TangoDataType::Double,
            c::TangoDataType_DEV_USHORT => TangoDataType::UShort,
            c::TangoDataType_DEV_ULONG => TangoDataType::ULong,
            c::TangoDataType_DEV_STRING => TangoDataType::String,
            c::TangoDataType_DEVVAR_CHARARRAY => TangoDataType::CharArray,
            c::TangoDataType_DEVVAR_SHORTARRAY => TangoDataType::ShortArray,
            c::TangoDataType_DEVVAR_LONGARRAY => TangoDataType::LongArray,
            c::TangoDataType_DEVVAR_FLOATARRAY => TangoDataType::FloatArray,
            c::TangoDataType_DEVVAR_DOUBLEARRAY => TangoDataType::DoubleArray,
            c::TangoDataType_DEVVAR_USHORTARRAY => TangoDataType::UShortArray,
            c::TangoDataType_DEVVAR_ULONGARRAY => TangoDataType::ULongArray,
            c::TangoDataType_DEVVAR_STRINGARRAY => TangoDataType::StringArray,
            c::TangoDataType_DEVVAR_LONGSTRINGARRAY => TangoDataType::LongStringArray,
            c::TangoDataType_DEVVAR_DOUBLESTRINGARRAY => TangoDataType::DoubleStringArray,
            c::TangoDataType_DEV_STATE => TangoDataType::State,
            c::TangoDataType_CONST_DEV_STRING => TangoDataType::ConstString,
            c::TangoDataType_DEVVAR_BOOLEANARRAY => TangoDataType::BooleanArray,
            c::TangoDataType_DEV_UCHAR => TangoDataType::UChar,
            c::TangoDataType_DEV_LONG64 => TangoDataType::Long64,
            c::TangoDataType_DEV_ULONG64 => TangoDataType::ULong64,
            c::TangoDataType_DEVVAR_LONG64ARRAY => TangoDataType::Long64Array,
            c::TangoDataType_DEVVAR_ULONG64ARRAY => TangoDataType::ULong64Array,
            c::TangoDataType_DEV_INT => TangoDataType::Int,
            c::TangoDataType_DEV_ENCODED => TangoDataType::Encoded,
            _ => panic!("unknown Tango data type tag={:?}", val)
        }
    }
}

pub type DevEncoded = (String, Vec<u8>);


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TangoDevState {
    On,
    Off,
    Close,
    Open,
    Insert,
    Extract,
    Moving,
    Standby,
    Fault,
    Init,
    Running,
    Alarm,
    Disable,
    Unknown,
}

impl TangoDevState {
    fn from_c(val: c::TangoDevState) -> TangoDevState {
        match val {
            c::TangoDevState_ON => TangoDevState::On,
            c::TangoDevState_OFF => TangoDevState::Off,
            c::TangoDevState_CLOSE => TangoDevState::Close,
            c::TangoDevState_OPEN => TangoDevState::Open,
            c::TangoDevState_INSERT => TangoDevState::Insert,
            c::TangoDevState_EXTRACT => TangoDevState::Extract,
            c::TangoDevState_MOVING => TangoDevState::Moving,
            c::TangoDevState_STANDBY => TangoDevState::Standby,
            c::TangoDevState_FAULT => TangoDevState::Fault,
            c::TangoDevState_INIT => TangoDevState::Init,
            c::TangoDevState_RUNNING => TangoDevState::Running,
            c::TangoDevState_ALARM => TangoDevState::Alarm,
            c::TangoDevState_DISABLE => TangoDevState::Disable,
            c::TangoDevState_UNKNOWN => TangoDevState::Unknown,
            _ => unreachable!("no TangoDevState for {}", val)
        }
    }
}

impl fmt::Display for TangoDevState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(match self {
            TangoDevState::On => "ON",
            TangoDevState::Off => "OFF",
            TangoDevState::Close => "CLOSE",
            TangoDevState::Open => "OPEN",
            TangoDevState::Insert => "INSERT",
            TangoDevState::Extract => "EXTRACT",
            TangoDevState::Moving => "MOVING",
            TangoDevState::Standby => "STANDBY",
            TangoDevState::Fault => "FAULT",
            TangoDevState::Init => "INIT",
            TangoDevState::Running => "RUNNING",
            TangoDevState::Alarm => "ALARM",
            TangoDevState::Disable => "DISABLE",
            TangoDevState::Unknown => "UNKNOWN",
        })
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AttrQuality {
    Valid,
    Invalid,
    Alarm,
    Changing,
    Warning,
}

impl AttrQuality {
    fn from_c(val: c::AttrQuality) -> AttrQuality {
        match val {
            c::AttrQuality_ATTR_VALID => AttrQuality::Valid,
            c::AttrQuality_ATTR_INVALID => AttrQuality::Invalid,
            c::AttrQuality_ATTR_ALARM => AttrQuality::Alarm,
            c::AttrQuality_ATTR_CHANGING => AttrQuality::Changing,
            c::AttrQuality_ATTR_WARNING => AttrQuality::Warning,
            _ => unreachable!("no TangoAttrQuality for {}", val)
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AttrWriteType {
    Read,
    ReadWithWrite,
    Write,
    ReadWrite,
}

impl AttrWriteType {
    fn from_c(val: c::AttrWriteType) -> AttrWriteType {
        match val {
            c::AttrWriteType_READ => AttrWriteType::Read,
            c::AttrWriteType_READ_WITH_WRITE => AttrWriteType::ReadWithWrite,
            c::AttrWriteType_WRITE => AttrWriteType::Write,
            c::AttrWriteType_READ_WRITE => AttrWriteType::ReadWrite,
            _ => unreachable!("no TangoAttrWriteType for {}", val)
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AttrDataFormat {
    Scalar,
    Spectrum,
    Image,
}

impl AttrDataFormat {
    fn from_c(val: c::AttrDataFormat) -> AttrDataFormat {
        match val {
            c::AttrDataFormat_SCALAR => AttrDataFormat::Scalar,
            c::AttrDataFormat_SPECTRUM => AttrDataFormat::Spectrum,
            c::AttrDataFormat_IMAGE => AttrDataFormat::Image,
            _ => unreachable!("no TangoAttrDataFormat for {}", val)
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DispLevel {
    Operator,
    Expert,
}

impl DispLevel {
    fn from_c(val: c::DispLevel) -> DispLevel {
        match val {
            c::DispLevel_OPERATOR => DispLevel::Operator,
            c::DispLevel_EXPERT => DispLevel::Expert,
            _ => unreachable!("no TangoDispLevel for {}", val)
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ErrSeverity {
    Warn,
    Err,
    Panic,
}

impl ErrSeverity {
    pub fn from_c(val: c::ErrSeverity) -> ErrSeverity {
        match val {
            c::ErrSeverity_WARN => ErrSeverity::Warn,
            c::ErrSeverity_ERR => ErrSeverity::Err,
            c::ErrSeverity_PANIC => ErrSeverity::Panic,
            _ => unreachable!("no TangoErrSeverity for {}", val)
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DevSource {
    Dev,
    Cache,
    CacheDev,
}

impl DevSource {
    pub fn from_c(val: c::DevSource) -> DevSource {
        match val {
            c::DevSource_DEV => DevSource::Dev,
            c::DevSource_CACHE => DevSource::Cache,
            c::DevSource_CACHE_DEV => DevSource::CacheDev,
            _ => unreachable!("no TangoDevSource for {}", val)
        }
    }
}


#[derive(Debug)]
pub struct CommandInfo {
    pub name: String,
    pub in_type: TangoDataType,
    pub out_type: TangoDataType,
    pub in_type_desc: String,
    pub out_type_desc: String,
    pub disp_level: DispLevel,
}

impl CommandInfo {
    pub unsafe fn from_c(mut info: c::CommandInfo, free: bool) -> CommandInfo {
        let res = CommandInfo {
            name: string_from(info.cmd_name),
            in_type: TangoDataType::from_c(info.in_type as u32),
            out_type: TangoDataType::from_c(info.out_type as u32),
            in_type_desc: string_from(info.in_type_desc),
            out_type_desc: string_from(info.out_type_desc),
            disp_level: DispLevel::from_c(info.disp_level),
        };
        if free {
            c::tango_free_CommandInfo(&mut info);
        }
        res
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum CommandData {
    Void,

    Boolean(bool),
    Short(i16),
    Long(i32),
    Long64(i64),
    UShort(u16),
    ULong(u32),
    ULong64(u64),

    Float(f32),
    Double(f64),

    String(Vec<u8>),
    State(TangoDevState),
    Encoded(DevEncoded),

    BooleanArray(Vec<bool>),
    CharArray(Vec<u8>),
    ShortArray(Vec<i16>),
    LongArray(Vec<i32>),
    Long64Array(Vec<i64>),
    UShortArray(Vec<u16>),
    ULongArray(Vec<u32>),
    ULong64Array(Vec<u64>),
    FloatArray(Vec<f32>),
    DoubleArray(Vec<f64>),

    StringArray(Vec<Vec<u8>>),
    LongStringArray(Vec<i32>, Vec<Vec<u8>>),
    DoubleStringArray(Vec<f64>, Vec<Vec<u8>>),
}

impl CommandData {
    pub fn from_str(s: &str) -> CommandData {
        CommandData::String(s.to_owned().into_bytes())
    }

    pub fn into_bool(self) -> Result<bool, Self> {
        match self {
            CommandData::Boolean(v) => Ok(v),
            _ => Err(self),
        }
    }

    pub fn into_i32(self) -> Result<i32, Self> {
        match self {
            CommandData::Boolean(v) => Ok(v as i32),
            CommandData::Short(v) => Ok(v as i32),
            CommandData::Long(v) => Ok(v),
            CommandData::UShort(v) => Ok(v as i32),
            _ => Err(self),
        }
    }

    pub fn into_i64(self) -> Result<i64, Self> {
        match self {
            CommandData::Boolean(v) => Ok(v as i64),
            CommandData::Short(v) => Ok(v as i64),
            CommandData::Long(v) => Ok(v as i64),
            CommandData::Long64(v) => Ok(v),
            CommandData::UShort(v) => Ok(v as i64),
            CommandData::ULong(v) => Ok(v as i64),
            _ => Err(self),
        }
    }

    pub fn into_u32(self) -> Result<u32, Self> {
        match self {
            CommandData::Boolean(v) => Ok(v as u32),
            CommandData::Short(v) => Ok(v as u32),
            CommandData::UShort(v) => Ok(v as u32),
            CommandData::ULong(v) => Ok(v as u32),
            _ => Err(self),
        }
    }

    pub fn into_u64(self) -> Result<u64, Self> {
        match self {
            CommandData::Boolean(v) => Ok(v as u64),
            CommandData::Short(v) => Ok(v as u64),
            CommandData::Long(v) => Ok(v as u64),
            CommandData::UShort(v) => Ok(v as u64),
            CommandData::ULong(v) => Ok(v as u64),
            CommandData::ULong64(v) => Ok(v),
            _ => Err(self),
        }
    }

    pub fn into_f32(self) -> Result<f32, Self> {
        match self {
            CommandData::Boolean(v) => Ok(v as i32 as f32),
            CommandData::Short(v) => Ok(v as f32),
            CommandData::Long(v) => Ok(v as f32),
            CommandData::Long64(v) => Ok(v as f32),
            CommandData::UShort(v) => Ok(v as f32),
            CommandData::ULong(v) => Ok(v as f32),
            CommandData::ULong64(v) => Ok(v as f32),
            CommandData::Float(v) => Ok(v),
            CommandData::Double(v) => Ok(v as f32),
            _ => Err(self),
        }
    }

    pub fn into_f64(self) -> Result<f64, Self> {
        match self {
            CommandData::Boolean(v) => Ok(v as i32 as f64),
            CommandData::Short(v) => Ok(v as f64),
            CommandData::Long(v) => Ok(v as f64),
            CommandData::Long64(v) => Ok(v as f64),
            CommandData::UShort(v) => Ok(v as f64),
            CommandData::ULong(v) => Ok(v as f64),
            CommandData::ULong64(v) => Ok(v as f64),
            CommandData::Float(v) => Ok(v as f64),
            CommandData::Double(v) => Ok(v),
            _ => Err(self),
        }
    }

    pub fn into_bytes(self) -> Result<Vec<u8>, Self> {
        match self {
            CommandData::String(s) => Ok(s),
            CommandData::CharArray(s) => Ok(s),
            _ => Err(self),
        }
    }

    pub fn into_string(self) -> Result<String, Self> {
        match self {
            CommandData::String(s) => String::from_utf8(s).map_err(
                |e| CommandData::String(e.into_bytes())),
            CommandData::CharArray(s) => String::from_utf8(s).map_err(
                |e| CommandData::CharArray(e.into_bytes())),
            _ => Err(self),
        }
    }

    pub unsafe fn from_c(mut cmd_data: c::CommandData) -> CommandData {
        let tag = TangoDataType::from_c(cmd_data.arg_type);
        let data = cmd_data.cmd_data;
        let res = match tag {
            TangoDataType::Void => CommandData::Void,
            TangoDataType::Boolean => CommandData::Boolean(data.bool_val),
            TangoDataType::Short => CommandData::Short(data.short_val),
            TangoDataType::Long | TangoDataType::Int => CommandData::Long(data.long_val),
            TangoDataType::Float => CommandData::Float(data.float_val),
            TangoDataType::Double => CommandData::Double(data.double_val),
            TangoDataType::UShort => CommandData::UShort(data.ushort_val),
            TangoDataType::ULong => CommandData::ULong(data.ulong_val),
            TangoDataType::UChar => unimplemented!(),  // only for attribute arrays
            TangoDataType::Long64 => CommandData::Long64(data.long64_val),
            TangoDataType::ULong64 => CommandData::ULong64(data.ulong64_val),
            TangoDataType::State => CommandData::State(TangoDevState::from_c(data.state_val)),
            // note: for all arrays this copies the data, instead of reusing the
            // existing allocation
            TangoDataType::String | TangoDataType::ConstString => CommandData::String({
                let ptr = data.string_val;
                let len = libc::strlen(ptr);
                Vec::from(slice::from_raw_parts(ptr as *mut u8, len))
            }),
            TangoDataType::CharArray => CommandData::CharArray({
                let ptr = data.char_arr;
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            TangoDataType::ShortArray => CommandData::ShortArray({
                let ptr = data.short_arr;
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            TangoDataType::LongArray => CommandData::LongArray({
                let ptr = data.long_arr;
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            TangoDataType::FloatArray => CommandData::FloatArray({
                let ptr = data.float_arr;
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            TangoDataType::DoubleArray => CommandData::DoubleArray({
                let ptr = data.double_arr;
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            TangoDataType::UShortArray => CommandData::UShortArray({
                let ptr = data.ushort_arr;
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            TangoDataType::ULongArray => CommandData::ULongArray({
                let ptr = data.ulong_arr;
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            TangoDataType::StringArray => CommandData::StringArray({
                let ptr = data.string_arr;
                let mut res = Vec::with_capacity(ptr.length as usize);
                for i in 0..ptr.length {
                    let raw = *ptr.sequence.offset(i as isize);
                    let len = libc::strlen(raw);
                    res.push(Vec::from(slice::from_raw_parts(raw as *mut u8, len)));
                }
                res
            }),
            TangoDataType::LongStringArray => {
                let ptr = data.long_string_arr;
                let mut strvec = Vec::with_capacity(ptr.string_length as usize);
                for i in 0..ptr.string_length {
                    let raw = *ptr.string_sequence.offset(i as isize);
                    let len = libc::strlen(raw);
                    strvec.push(Vec::from(slice::from_raw_parts(raw as *mut u8, len)));
                }
                CommandData::LongStringArray(
                    Vec::from(slice::from_raw_parts(ptr.long_sequence, ptr.long_length as usize)),
                    strvec
                )
            },
            TangoDataType::DoubleStringArray => {
                let ptr = data.double_string_arr;
                let mut strvec = Vec::with_capacity(ptr.string_length as usize);
                for i in 0..ptr.string_length {
                    let raw = *ptr.string_sequence.offset(i as isize);
                    let len = libc::strlen(raw);
                    strvec.push(Vec::from(slice::from_raw_parts(raw as *mut u8, len)));
                }
                CommandData::DoubleStringArray(
                    Vec::from(slice::from_raw_parts(ptr.double_sequence, ptr.double_length as usize)),
                    strvec
                )
            },
            TangoDataType::BooleanArray => CommandData::BooleanArray({
                let ptr = data.bool_arr;
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            TangoDataType::Long64Array => CommandData::Long64Array({
                let ptr = data.long64_arr;
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            TangoDataType::ULong64Array => CommandData::ULong64Array({
                let ptr = data.ulong64_arr;
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            TangoDataType::Encoded => {
                let ptr = data.encoded_val;
                let format = string_from(ptr.encoded_format);
                CommandData::Encoded((
                    format,
                    Vec::from(slice::from_raw_parts(ptr.encoded_data, ptr.encoded_length as usize))))
            },
        };
        c::tango_free_CommandData(&mut cmd_data);
        res
    }

    pub unsafe fn into_c(self) -> c::CommandData {
        let mut content = mem::zeroed::<c::TangoCommandData>();

        macro_rules! impl_array {
            ($val:ident, $alt:ident, $arr:ident, $ctype:ty) => {
                {
                    let array = &mut content.$arr;
                    array.length = $val.len() as u32;
                    array.sequence = Box::into_raw($val.into_boxed_slice()) as *mut $ctype;
                    TangoDataType::$alt
                }
            }
        }

        let tag = match self {
            CommandData::Void => {
                TangoDataType::Void
            }
            CommandData::Boolean(v) => {
                content.bool_val = v;
                TangoDataType::Boolean
            }
            CommandData::Short(v) => {
                content.short_val = v;
                TangoDataType::Short
            }
            CommandData::Long(v) => {
                content.long_val = v;
                TangoDataType::Long
            }
            CommandData::Float(v) => {
                content.float_val = v;
                TangoDataType::Float
            }
            CommandData::Double(v) => {
                content.double_val = v;
                TangoDataType::Double
            }
            CommandData::UShort(v) => {
                content.ushort_val = v;
                TangoDataType::UShort
            }
            CommandData::ULong(v) => {
                content.ulong_val = v;
                TangoDataType::ULong
            }
            CommandData::Long64(v) => {
                content.long64_val = v;
                TangoDataType::Long64
            }
            CommandData::ULong64(v) => {
                content.ulong64_val = v;
                TangoDataType::ULong64
            }
            CommandData::String(v) => {
                let cstr = cstring_from(v);
                content.string_val = cstr.into_raw();
                TangoDataType::String
            }
            CommandData::Encoded((format, data)) => {
                let ptr = &mut content.encoded_val;
                ptr.encoded_format = cstring_from(format).into_raw();
                ptr.encoded_length = data.len() as u32;
                ptr.encoded_data = Box::into_raw(data.into_boxed_slice()) as *mut u8;
                TangoDataType::Encoded
            }
            CommandData::BooleanArray(v) => impl_array!(v, BooleanArray, bool_arr, bool),
            CommandData::CharArray(v) => impl_array!(v, CharArray, char_arr, u8),
            CommandData::ShortArray(v) => impl_array!(v, ShortArray, short_arr, i16),
            CommandData::UShortArray(v) => impl_array!(v, UShortArray, ushort_arr, u16),
            CommandData::LongArray(v) => impl_array!(v, LongArray, long_arr, i32),
            CommandData::ULongArray(v) => impl_array!(v, ULongArray, ulong_arr, u32),
            CommandData::Long64Array(v) => impl_array!(v, Long64Array, long64_arr, i64),
            CommandData::ULong64Array(v) => impl_array!(v, ULong64Array, ulong64_arr, u64),
            CommandData::FloatArray(v) => impl_array!(v, FloatArray, float_arr, f32),
            CommandData::DoubleArray(v) => impl_array!(v, DoubleArray, double_arr, f64),
            CommandData::StringArray(v) => {
                let array = &mut content.string_arr;
                let mut vec = Vec::with_capacity(v.len());
                array.length = v.len() as u32;
                for s in v.into_iter() {
                    vec.push(cstring_from(s).into_raw());
                }
                array.sequence = Box::into_raw(vec.into_boxed_slice()) as *mut *mut c_char;
                TangoDataType::StringArray
            },
            CommandData::LongStringArray(vl, vs) => {
                let array = &mut content.long_string_arr;
                array.long_length = vl.len() as u32;
                array.long_sequence = Box::into_raw(vl.into_boxed_slice()) as *mut i32;
                let mut vec = Vec::with_capacity(vs.len());
                array.string_length = vs.len() as u32;
                for s in vs.into_iter() {
                    vec.push(cstring_from(s).into_raw());
                }
                array.string_sequence = Box::into_raw(vec.into_boxed_slice()) as *mut *mut c_char;
                TangoDataType::LongStringArray
            },
            CommandData::DoubleStringArray(vd, vs) => {
                let array = &mut content.double_string_arr;
                array.double_length = vd.len() as u32;
                array.double_sequence = Box::into_raw(vd.into_boxed_slice()) as *mut f64;
                let mut vec = Vec::with_capacity(vs.len());
                array.string_length = vs.len() as u32;
                for s in vs.into_iter() {
                    vec.push(cstring_from(s).into_raw());
                }
                array.string_sequence = Box::into_raw(vec.into_boxed_slice()) as *mut *mut c_char;
                TangoDataType::DoubleStringArray
            },
            CommandData::State(_) => panic!("Cannot send input argument of type State")
        };
        c::CommandData { arg_type: tag as u32, cmd_data: content }
    }

    pub unsafe fn free_c_data(cmd_data: c::CommandData) {
        let data = cmd_data.cmd_data;
        match TangoDataType::from_c(cmd_data.arg_type) {
            TangoDataType::Void |
            TangoDataType::Boolean |
            TangoDataType::UChar |
            TangoDataType::Short |
            TangoDataType::Long |
            TangoDataType::Int |
            TangoDataType::Float |
            TangoDataType::Double |
            TangoDataType::UShort |
            TangoDataType::ULong |
            TangoDataType::Long64 |
            TangoDataType::ULong64 |
            TangoDataType::State => {}
            TangoDataType::String | TangoDataType::ConstString => {
                drop(CString::from_raw(data.string_val));
            }
            TangoDataType::Encoded => {
                drop(CString::from_raw(data.encoded_val.encoded_format));
                drop(Box::from_raw(data.encoded_val.encoded_data));
            }
            TangoDataType::BooleanArray => drop(Box::from_raw(data.bool_arr.sequence)),
            TangoDataType::CharArray => drop(Box::from_raw(data.char_arr.sequence)),
            TangoDataType::ShortArray => drop(Box::from_raw(data.short_arr.sequence)),
            TangoDataType::UShortArray => drop(Box::from_raw(data.ushort_arr.sequence)),
            TangoDataType::LongArray => drop(Box::from_raw(data.long_arr.sequence)),
            TangoDataType::ULongArray => drop(Box::from_raw(data.ulong_arr.sequence)),
            TangoDataType::Long64Array => drop(Box::from_raw(data.long64_arr.sequence)),
            TangoDataType::ULong64Array => drop(Box::from_raw(data.ulong64_arr.sequence)),
            TangoDataType::FloatArray => drop(Box::from_raw(data.float_arr.sequence)),
            TangoDataType::DoubleArray => drop(Box::from_raw(data.double_arr.sequence)),
            TangoDataType::StringArray => {
                for i in 0..data.string_arr.length {
                    drop(CString::from_raw(*data.string_arr.sequence.offset(i as isize) as *mut c_char));
                }
                drop(Box::from_raw(data.string_arr.sequence));
            }
            TangoDataType::LongStringArray => {
                for i in 0..data.long_string_arr.string_length {
                    drop(CString::from_raw(*data.long_string_arr
                                           .string_sequence.offset(i as isize) as *mut c_char));
                }
                drop(Box::from_raw(data.long_string_arr.string_sequence));
                drop(Box::from_raw(data.long_string_arr.long_sequence));
            }
            TangoDataType::DoubleStringArray => {
                for i in 0..data.double_string_arr.string_length {
                    drop(CString::from_raw(*data.double_string_arr
                                           .string_sequence.offset(i as isize) as *mut c_char));
                }
                drop(Box::from_raw(data.double_string_arr.string_sequence));
                drop(Box::from_raw(data.double_string_arr.double_sequence));
            }
        }
    }
}

impl fmt::Display for CommandData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandData::Void => f.pad("<Void>"),
            CommandData::Boolean(v) => fmt::Display::fmt(&v, f),
            CommandData::Short(v) => fmt::Display::fmt(&v, f),
            CommandData::UShort(v) => fmt::Display::fmt(&v, f),
            CommandData::Long(v) => fmt::Display::fmt(&v, f),
            CommandData::ULong(v) => fmt::Display::fmt(&v, f),
            CommandData::Long64(v) => fmt::Display::fmt(&v, f),
            CommandData::ULong64(v) => fmt::Display::fmt(&v, f),
            CommandData::Float(v) => fmt::Display::fmt(&v, f),
            CommandData::Double(v) => fmt::Display::fmt(&v, f),
            CommandData::String(v) => fmt::Display::fmt(&String::from_utf8_lossy(v), f),
            CommandData::State(v) => fmt::Display::fmt(&v, f),
            CommandData::Encoded(_) => f.pad("<DevEncoded>"),

            CommandData::BooleanArray(v) => slice_display(v, |x| x, f),
            CommandData::CharArray(v) => slice_display(v, |x| x, f),
            CommandData::ShortArray(v) => slice_display(v, |x| x, f),
            CommandData::UShortArray(v) => slice_display(v, |x| x, f),
            CommandData::LongArray(v) => slice_display(v, |x| x, f),
            CommandData::ULongArray(v) => slice_display(v, |x| x, f),
            CommandData::Long64Array(v) => slice_display(v, |x| x, f),
            CommandData::ULong64Array(v) => slice_display(v, |x| x, f),
            CommandData::FloatArray(v) => slice_display(v, |x| x, f),
            CommandData::DoubleArray(v) => slice_display(v, |x| x, f),
            CommandData::StringArray(v) => slice_display(v, |x| String::from_utf8_lossy(x), f),
            CommandData::LongStringArray(vi, vs) => {
                slice_display(vi, |x| x, f)?;
                slice_display(vs, |x| String::from_utf8_lossy(x), f)
            },
            CommandData::DoubleStringArray(vd, vs) => {
                slice_display(vd, |x| x, f)?;
                slice_display(vs, |x| String::from_utf8_lossy(x), f)
            },
        }
    }
}

impl fmt::LowerHex for CommandData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandData::Void => f.pad("<Void>"),
            CommandData::Boolean(v) => fmt::Display::fmt(&v, f),
            CommandData::Short(v) => fmt::LowerHex::fmt(&v, f),
            CommandData::UShort(v) => fmt::LowerHex::fmt(&v, f),
            CommandData::Long(v) => fmt::LowerHex::fmt(&v, f),
            CommandData::ULong(v) => fmt::LowerHex::fmt(&v, f),
            CommandData::Long64(v) => fmt::LowerHex::fmt(&v, f),
            CommandData::ULong64(v) => fmt::LowerHex::fmt(&v, f),
            CommandData::Float(v) => fmt::Display::fmt(&v, f),
            CommandData::Double(v) => fmt::Display::fmt(&v, f),
            CommandData::String(v) => fmt::Display::fmt(&String::from_utf8_lossy(v), f),
            CommandData::State(v) => fmt::Display::fmt(&v, f),
            CommandData::Encoded(_) => f.pad("<DevEncoded>"),

            CommandData::BooleanArray(v) => slice_display(v, |x| x, f),
            CommandData::CharArray(v) => slice_lower_hex(v, |x| x, f),
            CommandData::ShortArray(v) => slice_lower_hex(v, |x| x, f),
            CommandData::UShortArray(v) => slice_lower_hex(v, |x| x, f),
            CommandData::LongArray(v) => slice_lower_hex(v, |x| x, f),
            CommandData::ULongArray(v) => slice_lower_hex(v, |x| x, f),
            CommandData::Long64Array(v) => slice_lower_hex(v, |x| x, f),
            CommandData::ULong64Array(v) => slice_lower_hex(v, |x| x, f),
            CommandData::FloatArray(v) => slice_display(v, |x| x, f),
            CommandData::DoubleArray(v) => slice_display(v, |x| x, f),
            CommandData::StringArray(v) => slice_display(v, |x| String::from_utf8_lossy(x), f),
            CommandData::LongStringArray(vi, vs) => {
                slice_lower_hex(vi, |x| x, f)?;
                slice_display(vs, |x| String::from_utf8_lossy(x), f)
            },
            CommandData::DoubleStringArray(vd, vs) => {
                slice_display(vd, |x| x, f)?;
                slice_display(vs, |x| String::from_utf8_lossy(x), f)
            },
        }
    }
}

impl fmt::UpperHex for CommandData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandData::Void => f.pad("<Void>"),
            CommandData::Boolean(v) => fmt::Display::fmt(&v, f),
            CommandData::Short(v) => fmt::UpperHex::fmt(&v, f),
            CommandData::UShort(v) => fmt::UpperHex::fmt(&v, f),
            CommandData::Long(v) => fmt::UpperHex::fmt(&v, f),
            CommandData::ULong(v) => fmt::UpperHex::fmt(&v, f),
            CommandData::Long64(v) => fmt::UpperHex::fmt(&v, f),
            CommandData::ULong64(v) => fmt::UpperHex::fmt(&v, f),
            CommandData::Float(v) => fmt::Display::fmt(&v, f),
            CommandData::Double(v) => fmt::Display::fmt(&v, f),
            CommandData::String(v) => fmt::Display::fmt(&String::from_utf8_lossy(v), f),
            CommandData::State(v) => fmt::Display::fmt(&v, f),
            CommandData::Encoded(_) => f.pad("<DevEncoded>"),

            CommandData::BooleanArray(v) => slice_display(v, |x| x, f),
            CommandData::CharArray(v) => slice_upper_hex(v, |x| x, f),
            CommandData::ShortArray(v) => slice_upper_hex(v, |x| x, f),
            CommandData::UShortArray(v) => slice_upper_hex(v, |x| x, f),
            CommandData::LongArray(v) => slice_upper_hex(v, |x| x, f),
            CommandData::ULongArray(v) => slice_upper_hex(v, |x| x, f),
            CommandData::Long64Array(v) => slice_upper_hex(v, |x| x, f),
            CommandData::ULong64Array(v) => slice_upper_hex(v, |x| x, f),
            CommandData::FloatArray(v) => slice_display(v, |x| x, f),
            CommandData::DoubleArray(v) => slice_display(v, |x| x, f),
            CommandData::StringArray(v) => slice_display(v, |x| String::from_utf8_lossy(x), f),
            CommandData::LongStringArray(vi, vs) => {
                slice_upper_hex(vi, |x| x, f)?;
                slice_display(vs, |x| String::from_utf8_lossy(x), f)
            },
            CommandData::DoubleStringArray(vd, vs) => {
                slice_display(vd, |x| x, f)?;
                slice_display(vs, |x| String::from_utf8_lossy(x), f)
            },
        }
    }
}

impl fmt::LowerExp for CommandData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandData::Void => f.pad("<Void>"),
            CommandData::Boolean(v) => fmt::Display::fmt(&v, f),
            CommandData::Short(v) => fmt::Display::fmt(&v, f),
            CommandData::UShort(v) => fmt::Display::fmt(&v, f),
            CommandData::Long(v) => fmt::Display::fmt(&v, f),
            CommandData::ULong(v) => fmt::Display::fmt(&v, f),
            CommandData::Long64(v) => fmt::Display::fmt(&v, f),
            CommandData::ULong64(v) => fmt::Display::fmt(&v, f),
            CommandData::Float(v) => fmt::LowerExp::fmt(&v, f),
            CommandData::Double(v) => fmt::LowerExp::fmt(&v, f),
            CommandData::String(v) => fmt::Display::fmt(&String::from_utf8_lossy(v), f),
            CommandData::State(v) => fmt::Display::fmt(&v, f),
            CommandData::Encoded(_) => f.pad("<DevEncoded>"),

            CommandData::BooleanArray(v) => slice_display(v, |x| x, f),
            CommandData::CharArray(v) => slice_display(v, |x| x, f),
            CommandData::ShortArray(v) => slice_display(v, |x| x, f),
            CommandData::UShortArray(v) => slice_display(v, |x| x, f),
            CommandData::LongArray(v) => slice_display(v, |x| x, f),
            CommandData::ULongArray(v) => slice_display(v, |x| x, f),
            CommandData::Long64Array(v) => slice_display(v, |x| x, f),
            CommandData::ULong64Array(v) => slice_display(v, |x| x, f),
            CommandData::FloatArray(v) => slice_lower_exp(v, |x| x, f),
            CommandData::DoubleArray(v) => slice_lower_exp(v, |x| x, f),
            CommandData::StringArray(v) => slice_display(v, |x| String::from_utf8_lossy(x), f),
            CommandData::LongStringArray(vi, vs) => {
                slice_display(vi, |x| x, f)?;
                slice_display(vs, |x| String::from_utf8_lossy(x), f)
            },
            CommandData::DoubleStringArray(vd, vs) => {
                slice_lower_exp(vd, |x| x, f)?;
                slice_display(vs, |x| String::from_utf8_lossy(x), f)
            },
        }
    }
}

impl fmt::UpperExp for CommandData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandData::Void => f.pad("<Void>"),
            CommandData::Boolean(v) => fmt::Display::fmt(&v, f),
            CommandData::Short(v) => fmt::Display::fmt(&v, f),
            CommandData::UShort(v) => fmt::Display::fmt(&v, f),
            CommandData::Long(v) => fmt::Display::fmt(&v, f),
            CommandData::ULong(v) => fmt::Display::fmt(&v, f),
            CommandData::Long64(v) => fmt::Display::fmt(&v, f),
            CommandData::ULong64(v) => fmt::Display::fmt(&v, f),
            CommandData::Float(v) => fmt::UpperExp::fmt(&v, f),
            CommandData::Double(v) => fmt::UpperExp::fmt(&v, f),
            CommandData::String(v) => fmt::Display::fmt(&String::from_utf8_lossy(v), f),
            CommandData::State(v) => fmt::Display::fmt(&v, f),
            CommandData::Encoded(_) => f.pad("<DevEncoded>"),

            CommandData::BooleanArray(v) => slice_display(v, |x| x, f),
            CommandData::CharArray(v) => slice_display(v, |x| x, f),
            CommandData::ShortArray(v) => slice_display(v, |x| x, f),
            CommandData::UShortArray(v) => slice_display(v, |x| x, f),
            CommandData::LongArray(v) => slice_display(v, |x| x, f),
            CommandData::ULongArray(v) => slice_display(v, |x| x, f),
            CommandData::Long64Array(v) => slice_display(v, |x| x, f),
            CommandData::ULong64Array(v) => slice_display(v, |x| x, f),
            CommandData::FloatArray(v) => slice_upper_exp(v, |x| x, f),
            CommandData::DoubleArray(v) => slice_upper_exp(v, |x| x, f),
            CommandData::StringArray(v) => slice_display(v, |x| String::from_utf8_lossy(x), f),
            CommandData::LongStringArray(vi, vs) => {
                slice_display(vi, |x| x, f)?;
                slice_display(vs, |x| String::from_utf8_lossy(x), f)
            },
            CommandData::DoubleStringArray(vd, vs) => {
                slice_upper_exp(vd, |x| x, f)?;
                slice_display(vs, |x| String::from_utf8_lossy(x), f)
            },
        }
    }
}


#[derive(Debug)]
pub struct AttributeInfo {
    pub name: String,
    pub writable: AttrWriteType,
    pub data_type: TangoDataType,
    pub data_format: AttrDataFormat,
    pub max_dim_x: usize,
    pub max_dim_y: usize,
    pub description: String,
    pub label: String,
    pub unit: String,
    pub standard_unit: String,
    pub display_unit: String,
    pub format: String,
    pub min_value: String,
    pub max_value: String,
    pub min_alarm: String,
    pub max_alarm: String,
    pub writable_attr_name: String,
    pub disp_level: DispLevel,
}

impl AttributeInfo {
    pub unsafe fn from_c(info: c::AttributeInfo) -> AttributeInfo {
        AttributeInfo {
            name: string_from(info.name),
            writable: AttrWriteType::from_c(info.writable),
            data_type: TangoDataType::from_c(info.data_type),
            data_format: AttrDataFormat::from_c(info.data_format),
            max_dim_x: info.max_dim_x as usize,
            max_dim_y: info.max_dim_y as usize,
            description: string_from(info.description),
            label: string_from(info.label),
            unit: string_from(info.unit),
            standard_unit: string_from(info.standard_unit),
            display_unit: string_from(info.display_unit),
            format: string_from(info.format),
            min_value: string_from(info.min_value),
            max_value: string_from(info.max_value),
            min_alarm: string_from(info.min_alarm),
            max_alarm: string_from(info.max_alarm),
            writable_attr_name: string_from(info.writable_attr_name),
            disp_level: DispLevel::from_c(info.disp_level),
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct AttributeData {
    pub data: AttrValue,
    pub written_data: Option<AttrValue>,
    pub format: AttrDataFormat,
    pub quality: AttrQuality,
    pub name: String,
    pub dim_x: usize,
    pub dim_y: usize,
    pub time_stamp: Timespec,
}

impl AttributeData {
    pub fn simple(name: &str, data: AttrValue) -> AttributeData {
        AttributeData {
            dim_x: data.len(),
            dim_y: 0,
            data: data,
            written_data: None,              // doesn't actually matter for writing
            format: AttrDataFormat::Scalar,  // doesn't actually matter for writing
            quality: AttrQuality::Valid,
            name: name.into(),
            time_stamp: get_time(),
        }
    }

    pub unsafe fn from_c(mut attr_data: c::AttributeData, free: bool) -> AttributeData {
        use AttrValue::*;

        let tag = TangoDataType::from_c(attr_data.data_type);
        let data = attr_data.attr_data;
        let is_scalar = AttrDataFormat::from_c(attr_data.data_format) == AttrDataFormat::Scalar;
        let (res_read, res_written) = if is_scalar {
            macro_rules! impl_simple {
                ($alt:ident, $arr:ident) => {
                    {
                        let ptr = data.$arr;
                        if ptr.length == 1 {
                            ($alt(*ptr.sequence), $alt(Default::default()))
                        } else {
                            ($alt(*ptr.sequence), $alt(*ptr.sequence.offset(1)))
                        }
                    }
                }
            }

            match tag {
                TangoDataType::UChar => impl_simple!(UChar, char_arr),
                TangoDataType::Short => impl_simple!(Short, short_arr),
                TangoDataType::UShort => impl_simple!(UShort, ushort_arr),
                TangoDataType::Long => impl_simple!(Long, long_arr),
                TangoDataType::ULong => impl_simple!(ULong, ulong_arr),
                TangoDataType::Long64 => impl_simple!(Long64, long64_arr),
                TangoDataType::ULong64 => impl_simple!(ULong64, ulong64_arr),
                TangoDataType::Float => impl_simple!(Float, float_arr),
                TangoDataType::Double => impl_simple!(Double, double_arr),
                TangoDataType::Boolean => impl_simple!(Boolean, bool_arr),
                TangoDataType::State => {
                    let r = TangoDevState::from_c(*data.state_arr.sequence);
                    let w = if data.string_arr.length == 1 {
                        TangoDevState::Unknown
                    } else {
                        TangoDevState::from_c(*data.state_arr.sequence.offset(1))
                    };
                    (State(r), State(w))
                }
                TangoDataType::String => {
                    let rawr = *data.string_arr.sequence;
                    let lenr = libc::strlen(rawr);
                    let r = Vec::from(slice::from_raw_parts(rawr as *mut u8, lenr));
                    let w = if data.string_arr.length == 1 {
                        Vec::new()
                    } else {
                        let raww = *data.string_arr.sequence.offset(1);
                        let lenw = libc::strlen(raww);
                        Vec::from(slice::from_raw_parts(raww as *mut u8, lenw))
                    };
                    (String(r), String(w))
                },
                TangoDataType::Encoded => {
                    let rawr = *data.encoded_arr.sequence;
                    let r = (string_from(rawr.encoded_format),
                             Vec::from(slice::from_raw_parts(rawr.encoded_data as *mut u8,
                                                             rawr.encoded_length as usize)));
                    let w = if data.string_arr.length == 1 {
                        ("".into(), Vec::new())
                    } else {
                        let raww = *data.encoded_arr.sequence.offset(1);
                        (string_from(raww.encoded_format),
                         Vec::from(slice::from_raw_parts(raww.encoded_data as *mut u8,
                                                         raww.encoded_length as usize)))
                    };
                    (Encoded(r), Encoded(w))
                },
                _ => panic!("data type {:?} not allowed for attributes", tag)
            }
        } else {
            macro_rules! impl_simple {
                ($alt:ident, $arr:ident) => {
                    {
                        let ptr = data.$arr;
                        let slice = slice::from_raw_parts(ptr.sequence, ptr.length as usize);
                        let (p1, p2) = slice.split_at(attr_data.nb_read as usize);
                        ($alt(Vec::from(p1)), $alt(Vec::from(p2)))
                    }
                }
            }

            match tag {
                TangoDataType::UChar => impl_simple!(UCharArray, char_arr),
                TangoDataType::Short => impl_simple!(ShortArray, short_arr),
                TangoDataType::UShort => impl_simple!(UShortArray, ushort_arr),
                TangoDataType::Long => impl_simple!(LongArray, long_arr),
                TangoDataType::ULong => impl_simple!(ULongArray, ulong_arr),
                TangoDataType::Long64 => impl_simple!(Long64Array, long64_arr),
                TangoDataType::ULong64 => impl_simple!(ULong64Array, ulong64_arr),
                TangoDataType::Float => impl_simple!(FloatArray, float_arr),
                TangoDataType::Double => impl_simple!(DoubleArray, double_arr),
                TangoDataType::Boolean => impl_simple!(BooleanArray, bool_arr),
                TangoDataType::State => {
                    let ptr = data.state_arr;
                    let slice = slice::from_raw_parts(ptr.sequence, ptr.length as usize);
                    let (p1, p2) = slice.split_at(attr_data.nb_read as usize);
                    (StateArray(p1.iter().map(|&v| TangoDevState::from_c(v)).collect()),
                     StateArray(p2.iter().map(|&v| TangoDevState::from_c(v)).collect()))
                },
                TangoDataType::String => {
                    let ptr = data.string_arr;
                    let mut res = Vec::with_capacity(ptr.length as usize);
                    for i in 0..ptr.length {
                        let raw = *ptr.sequence.offset(i as isize);
                        let len = libc::strlen(raw);
                        res.push(Vec::from(slice::from_raw_parts(raw as *mut u8, len)));
                    }
                    let res2 = res.split_off(attr_data.nb_read as usize);
                    (StringArray(res), StringArray(res2))
                },
                TangoDataType::Encoded => {
                    let ptr = data.encoded_arr;
                    let mut res = Vec::with_capacity(ptr.length as usize);
                    for i in 0..ptr.length {
                        let raw = *ptr.sequence.offset(i as isize);
                        res.push((string_from(raw.encoded_format),
                                  Vec::from(slice::from_raw_parts(raw.encoded_data as *mut u8,
                                                                  raw.encoded_length as usize))));
                    }
                    let res2 = res.split_off(attr_data.nb_read as usize);
                    (EncodedArray(res), EncodedArray(res2))
                },
                _ => panic!("data type {:?} not allowed for attributes", tag)
            }
        };
        let res = AttributeData {
            data: res_read,
            written_data: Some(res_written),
            format: AttrDataFormat::from_c(attr_data.data_format),
            quality: AttrQuality::from_c(attr_data.quality),
            name: string_from(attr_data.name),
            dim_x: attr_data.dim_x as usize,
            dim_y: attr_data.dim_y as usize,
            time_stamp: Timespec::new(attr_data.time_stamp.tv_sec.into(),
                                      1000 * attr_data.time_stamp.tv_usec as i32),
        };
        if free {
            c::tango_free_AttributeData(&mut attr_data);
        }
        res
    }

    pub unsafe fn into_c(self) -> c::AttributeData {
        let mut content = mem::zeroed::<c::TangoAttributeData>();

        macro_rules! impl_simple {
            ($val:ident, $alt:ident, $arr:ident, $ctype:ty) => {
                {
                    let array = &mut content.$arr;
                    array.length = 1;
                    array.sequence = Box::into_raw(Box::new($val)) as *mut $ctype;
                    TangoDataType::$alt
                }
            }
        }

        macro_rules! impl_array {
            ($val:ident, $alt:ident, $arr:ident, $ctype:ty) => {
                {
                    let array = &mut content.$arr;
                    array.length = $val.len() as u32;
                    array.sequence = Box::into_raw($val.into_boxed_slice()) as *mut $ctype;
                    TangoDataType::$alt
                }
            }
        }

        let tag = match self.data {
            AttrValue::Boolean(v) => impl_simple!(v, Boolean, bool_arr, bool),
            AttrValue::UChar(v) => impl_simple!(v, UChar, char_arr, u8),
            AttrValue::Short(v) => impl_simple!(v, Short, short_arr, i16),
            AttrValue::UShort(v) => impl_simple!(v, UShort, ushort_arr, u16),
            AttrValue::Long(v) => impl_simple!(v, Long, long_arr, i32),
            AttrValue::ULong(v) => impl_simple!(v, ULong, ulong_arr, u32),
            AttrValue::Long64(v) => impl_simple!(v, Long64, long64_arr, i64),
            AttrValue::ULong64(v) => impl_simple!(v, ULong64, ulong64_arr, u64),
            AttrValue::Float(v) => impl_simple!(v, Float, float_arr, f32),
            AttrValue::Double(v) => impl_simple!(v, Double, double_arr, f64),
            AttrValue::State(v) => impl_simple!(v, State, state_arr, c::TangoDevState),
            AttrValue::String(v) => {
                let array = &mut content.string_arr;
                array.length = 1;
                let mut vec = Vec::with_capacity(1);
                vec.push(cstring_from(v).into_raw());
                array.sequence = Box::into_raw(vec.into_boxed_slice()) as *mut *mut c_char;
                TangoDataType::String
            }
            AttrValue::Encoded((format, data)) => {
                let array = &mut content.encoded_arr;
                array.length = 1;
                let mut vec = Vec::with_capacity(1);
                let encoded = c::TangoDevEncoded {
                    encoded_format: cstring_from(format).into_raw(),
                    encoded_length: data.len() as u32,
                    encoded_data: Box::into_raw(data.into_boxed_slice()) as *mut u8,
                };
                vec.push(Box::into_raw(Box::new(encoded)));
                array.sequence = Box::into_raw(vec.into_boxed_slice()) as *mut c::TangoDevEncoded;
                TangoDataType::Encoded
            }
            AttrValue::BooleanArray(v) => impl_array!(v, Boolean, bool_arr, bool),
            AttrValue::UCharArray(v) => impl_array!(v, UChar, char_arr, u8),
            AttrValue::ShortArray(v) => impl_array!(v, Short, short_arr, i16),
            AttrValue::UShortArray(v) => impl_array!(v, UShort, ushort_arr, u16),
            AttrValue::LongArray(v) => impl_array!(v, Long, long_arr, i32),
            AttrValue::ULongArray(v) => impl_array!(v, ULong, ulong_arr, u32),
            AttrValue::Long64Array(v) => impl_array!(v, Long64, long64_arr, i64),
            AttrValue::ULong64Array(v) => impl_array!(v, ULong64, ulong64_arr, u64),
            AttrValue::FloatArray(v) => impl_array!(v, Float, float_arr, f32),
            AttrValue::DoubleArray(v) => impl_array!(v, Double, double_arr, f64),
            AttrValue::StateArray(v) => impl_array!(v, State, state_arr, c::TangoDevState),
            AttrValue::StringArray(v) => {
                let array = &mut content.string_arr;
                let mut vec = Vec::with_capacity(v.len());
                array.length = v.len() as u32;
                for s in v.into_iter() {
                    vec.push(cstring_from(s).into_raw());
                }
                array.sequence = Box::into_raw(vec.into_boxed_slice()) as *mut *mut c_char;
                TangoDataType::String
            }
            AttrValue::EncodedArray(v) => {
                let array = &mut content.encoded_arr;
                array.length = v.len() as u32;
                let mut vec = Vec::with_capacity(v.len());
                for (format, data) in v.into_iter() {
                    let encoded = c::TangoDevEncoded {
                        encoded_format: cstring_from(format).into_raw(),
                        encoded_length: data.len() as u32,
                        encoded_data: Box::into_raw(data.into_boxed_slice()) as *mut u8,
                    };
                    vec.push(Box::into_raw(Box::new(encoded)));
                }
                array.sequence = Box::into_raw(vec.into_boxed_slice()) as *mut c::TangoDevEncoded;
                TangoDataType::Encoded
            }
        };
        c::AttributeData {
            name: cstring_from(self.name).into_raw(),
            data_type: tag as u32,
            data_format: self.format as u32,
            attr_data: content,
            nb_read: 0,  // doesn't matter for writing
            quality: self.quality as u32,
            dim_x: self.dim_x as i32,
            dim_y: self.dim_y as i32,
            time_stamp: c::timeval { tv_sec: self.time_stamp.sec as c_long,
                                     tv_usec: self.time_stamp.nsec as c_long / 1000 }
        }
    }

    pub unsafe fn free_c_data(attr_data: c::AttributeData) {
        let data = attr_data.attr_data;
        drop(CString::from_raw(attr_data.name));
        match TangoDataType::from_c(attr_data.data_type) {
            TangoDataType::Boolean => drop(Box::from_raw(data.bool_arr.sequence)),
            TangoDataType::UChar => drop(Box::from_raw(data.char_arr.sequence)),
            TangoDataType::Short => drop(Box::from_raw(data.short_arr.sequence)),
            TangoDataType::UShort => drop(Box::from_raw(data.ushort_arr.sequence)),
            TangoDataType::Long => drop(Box::from_raw(data.long_arr.sequence)),
            TangoDataType::ULong => drop(Box::from_raw(data.ulong_arr.sequence)),
            TangoDataType::Long64 => drop(Box::from_raw(data.long64_arr.sequence)),
            TangoDataType::ULong64 => drop(Box::from_raw(data.ulong64_arr.sequence)),
            TangoDataType::Float => drop(Box::from_raw(data.float_arr.sequence)),
            TangoDataType::Double => drop(Box::from_raw(data.double_arr.sequence)),
            TangoDataType::State => drop(Box::from_raw(data.state_arr.sequence)),
            TangoDataType::String => {
                for i in 0..data.string_arr.length {
                    drop(CString::from_raw(*data.string_arr.sequence.offset(i as isize) as *mut c_char));
                }
                drop(Box::from_raw(data.string_arr.sequence));
            }
            TangoDataType::Encoded => {
                for i in 0..data.encoded_arr.length {
                    let ptr = data.encoded_arr.sequence.offset(i as isize) as *mut c::TangoDevEncoded;
                    drop(CString::from_raw((*ptr).encoded_format as *mut c_char));
                    drop(Box::from_raw((*ptr).encoded_data));
                }
                drop(Box::from_raw(data.encoded_arr.sequence));
            }
            val => panic!("invalid attribute data tag={:?}", val)
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum AttrValue {
    Boolean(bool),
    UChar(u8),
    Short(i16),
    UShort(u16),
    Long(i32),
    ULong(u32),
    Long64(i64),
    ULong64(u64),
    Float(f32),
    Double(f64),
    String(Vec<u8>),
    State(TangoDevState),
    Encoded(DevEncoded),

    BooleanArray(Vec<bool>),
    UCharArray(Vec<u8>),
    ShortArray(Vec<i16>),
    UShortArray(Vec<u16>),
    LongArray(Vec<i32>),
    ULongArray(Vec<u32>),
    Long64Array(Vec<i64>),
    ULong64Array(Vec<u64>),
    FloatArray(Vec<f32>),
    DoubleArray(Vec<f64>),
    StringArray(Vec<Vec<u8>>),
    StateArray(Vec<TangoDevState>),
    EncodedArray(Vec<DevEncoded>),
}

impl AttrValue {
    pub fn len(&self) -> usize {
        match self {
            AttrValue::Boolean(_) |
            AttrValue::UChar(_) |
            AttrValue::Short(_) |
            AttrValue::UShort(_) |
            AttrValue::Long(_) |
            AttrValue::ULong(_) |
            AttrValue::Long64(_) |
            AttrValue::ULong64(_) |
            AttrValue::Float(_) |
            AttrValue::Double(_) |
            AttrValue::String(_) |
            AttrValue::State(_) |
            AttrValue::Encoded(_) => 1,

            AttrValue::BooleanArray(v) => v.len(),
            AttrValue::UCharArray(v) => v.len(),
            AttrValue::ShortArray(v) => v.len(),
            AttrValue::UShortArray(v) => v.len(),
            AttrValue::LongArray(v) => v.len(),
            AttrValue::ULongArray(v) => v.len(),
            AttrValue::Long64Array(v) => v.len(),
            AttrValue::ULong64Array(v) => v.len(),
            AttrValue::FloatArray(v) => v.len(),
            AttrValue::DoubleArray(v) => v.len(),
            AttrValue::StringArray(v) => v.len(),
            AttrValue::StateArray(v) => v.len(),
            AttrValue::EncodedArray(v) => v.len(),
        }
    }

    pub fn into_bool(self) -> Result<bool, Self> {
        match self {
            AttrValue::Boolean(v) => Ok(v),
            _ => Err(self),
        }
    }

    pub fn into_i32(self) -> Result<i32, Self> {
        match self {
            AttrValue::Boolean(v) => Ok(v as i32),
            AttrValue::UChar(v) => Ok(v as i32),
            AttrValue::Short(v) => Ok(v as i32),
            AttrValue::UShort(v) => Ok(v as i32),
            AttrValue::Long(v) => Ok(v),
            _ => Err(self),
        }
    }

    pub fn into_i64(self) -> Result<i64, Self> {
        match self {
            AttrValue::Boolean(v) => Ok(v as i64),
            AttrValue::UChar(v) => Ok(v as i64),
            AttrValue::Short(v) => Ok(v as i64),
            AttrValue::UShort(v) => Ok(v as i64),
            AttrValue::Long(v) => Ok(v as i64),
            AttrValue::ULong(v) => Ok(v as i64),
            AttrValue::Long64(v) => Ok(v),
            _ => Err(self),
        }
    }

    pub fn into_u32(self) -> Result<u32, Self> {
        match self {
            AttrValue::Boolean(v) => Ok(v as u32),
            AttrValue::UChar(v) => Ok(v as u32),
            AttrValue::Short(v) => Ok(v as u32),
            AttrValue::UShort(v) => Ok(v as u32),
            AttrValue::ULong(v) => Ok(v as u32),
            _ => Err(self),
        }
    }

    pub fn into_u64(self) -> Result<u64, Self> {
        match self {
            AttrValue::Boolean(v) => Ok(v as u64),
            AttrValue::UChar(v) => Ok(v as u64),
            AttrValue::Short(v) => Ok(v as u64),
            AttrValue::UShort(v) => Ok(v as u64),
            AttrValue::Long(v) => Ok(v as u64),
            AttrValue::ULong(v) => Ok(v as u64),
            AttrValue::ULong64(v) => Ok(v),
            _ => Err(self),
        }
    }

    pub fn into_f32(self) -> Result<f32, Self> {
        match self {
            AttrValue::Boolean(v) => Ok(v as i32 as f32),
            AttrValue::UChar(v) => Ok(v as f32),
            AttrValue::Short(v) => Ok(v as f32),
            AttrValue::Long(v) => Ok(v as f32),
            AttrValue::Long64(v) => Ok(v as f32),
            AttrValue::UShort(v) => Ok(v as f32),
            AttrValue::ULong(v) => Ok(v as f32),
            AttrValue::ULong64(v) => Ok(v as f32),
            AttrValue::Float(v) => Ok(v),
            AttrValue::Double(v) => Ok(v as f32),
            _ => Err(self),
        }
    }

    pub fn into_f64(self) -> Result<f64, Self> {
        match self {
            AttrValue::Boolean(v) => Ok(v as i32 as f64),
            AttrValue::UChar(v) => Ok(v as f64),
            AttrValue::Short(v) => Ok(v as f64),
            AttrValue::Long(v) => Ok(v as f64),
            AttrValue::Long64(v) => Ok(v as f64),
            AttrValue::UShort(v) => Ok(v as f64),
            AttrValue::ULong(v) => Ok(v as f64),
            AttrValue::ULong64(v) => Ok(v as f64),
            AttrValue::Float(v) => Ok(v as f64),
            AttrValue::Double(v) => Ok(v),
            _ => Err(self),
        }
    }

    pub fn into_bytes(self) -> Result<Vec<u8>, Self> {
        match self {
            AttrValue::String(s) => Ok(s),
            AttrValue::Encoded((_, s)) => Ok(s),
            _ => Err(self),
        }
    }

    pub fn into_string(self) -> Result<String, Self> {
        match self {
            AttrValue::String(s) => String::from_utf8(s).map_err(
                |e| AttrValue::String(e.into_bytes())),
            _ => Err(self),
        }
    }
}

impl fmt::Display for AttrValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AttrValue::Boolean(v) => fmt::Display::fmt(&v, f),
            AttrValue::UChar(v) => fmt::Display::fmt(&v, f),
            AttrValue::Short(v) => fmt::Display::fmt(&v, f),
            AttrValue::UShort(v) => fmt::Display::fmt(&v, f),
            AttrValue::Long(v) => fmt::Display::fmt(&v, f),
            AttrValue::ULong(v) => fmt::Display::fmt(&v, f),
            AttrValue::Long64(v) => fmt::Display::fmt(&v, f),
            AttrValue::ULong64(v) => fmt::Display::fmt(&v, f),
            AttrValue::Float(v) => fmt::Display::fmt(&v, f),
            AttrValue::Double(v) => fmt::Display::fmt(&v, f),
            AttrValue::String(v) => fmt::Display::fmt(&String::from_utf8_lossy(v), f),
            AttrValue::State(v) => fmt::Display::fmt(&v, f),
            AttrValue::Encoded(_) => f.pad("<DevEncoded>"),

            AttrValue::BooleanArray(v) => slice_display(v, |x| x, f),
            AttrValue::UCharArray(v) => slice_display(v, |x| x, f),
            AttrValue::ShortArray(v) => slice_display(v, |x| x, f),
            AttrValue::UShortArray(v) => slice_display(v, |x| x, f),
            AttrValue::LongArray(v) => slice_display(v, |x| x, f),
            AttrValue::ULongArray(v) => slice_display(v, |x| x, f),
            AttrValue::Long64Array(v) => slice_display(v, |x| x, f),
            AttrValue::ULong64Array(v) => slice_display(v, |x| x, f),
            AttrValue::FloatArray(v) => slice_display(v, |x| x, f),
            AttrValue::DoubleArray(v) => slice_display(v, |x| x, f),
            AttrValue::StringArray(v) => slice_display(v, |x| String::from_utf8_lossy(x), f),
            AttrValue::StateArray(v) => slice_display(v, |x| x, f),
            AttrValue::EncodedArray(v) => slice_display(v, |_| "<DevEncoded>", f),
        }
    }
}

impl fmt::LowerHex for AttrValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AttrValue::Boolean(v) => fmt::Display::fmt(&v, f),
            AttrValue::UChar(v) => fmt::LowerHex::fmt(&v, f),
            AttrValue::Short(v) => fmt::LowerHex::fmt(&v, f),
            AttrValue::UShort(v) => fmt::LowerHex::fmt(&v, f),
            AttrValue::Long(v) => fmt::LowerHex::fmt(&v, f),
            AttrValue::ULong(v) => fmt::LowerHex::fmt(&v, f),
            AttrValue::Long64(v) => fmt::LowerHex::fmt(&v, f),
            AttrValue::ULong64(v) => fmt::LowerHex::fmt(&v, f),
            AttrValue::Float(v) => fmt::Display::fmt(&v, f),
            AttrValue::Double(v) => fmt::Display::fmt(&v, f),
            AttrValue::String(v) => fmt::Display::fmt(&String::from_utf8_lossy(v), f),
            AttrValue::State(v) => fmt::Display::fmt(&v, f),
            AttrValue::Encoded(_) => f.pad("<DevEncoded>"),

            AttrValue::BooleanArray(v) => slice_display(v, |x| x, f),
            AttrValue::UCharArray(v) => slice_lower_hex(v, |x| x, f),
            AttrValue::ShortArray(v) => slice_lower_hex(v, |x| x, f),
            AttrValue::UShortArray(v) => slice_lower_hex(v, |x| x, f),
            AttrValue::LongArray(v) => slice_lower_hex(v, |x| x, f),
            AttrValue::ULongArray(v) => slice_lower_hex(v, |x| x, f),
            AttrValue::Long64Array(v) => slice_lower_hex(v, |x| x, f),
            AttrValue::ULong64Array(v) => slice_lower_hex(v, |x| x, f),
            AttrValue::FloatArray(v) => slice_display(v, |x| x, f),
            AttrValue::DoubleArray(v) => slice_display(v, |x| x, f),
            AttrValue::StringArray(v) => slice_display(v, |x| String::from_utf8_lossy(x), f),
            AttrValue::StateArray(v) => slice_display(v, |x| x, f),
            AttrValue::EncodedArray(v) => slice_display(v, |_| "<DevEncoded>", f),
        }
    }
}

impl fmt::UpperHex for AttrValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AttrValue::Boolean(v) => fmt::Display::fmt(&v, f),
            AttrValue::UChar(v) => fmt::UpperHex::fmt(&v, f),
            AttrValue::Short(v) => fmt::UpperHex::fmt(&v, f),
            AttrValue::UShort(v) => fmt::UpperHex::fmt(&v, f),
            AttrValue::Long(v) => fmt::UpperHex::fmt(&v, f),
            AttrValue::ULong(v) => fmt::UpperHex::fmt(&v, f),
            AttrValue::Long64(v) => fmt::UpperHex::fmt(&v, f),
            AttrValue::ULong64(v) => fmt::UpperHex::fmt(&v, f),
            AttrValue::Float(v) => fmt::Display::fmt(&v, f),
            AttrValue::Double(v) => fmt::Display::fmt(&v, f),
            AttrValue::String(v) => fmt::Display::fmt(&String::from_utf8_lossy(v), f),
            AttrValue::State(v) => fmt::Display::fmt(&v, f),
            AttrValue::Encoded(_) => f.pad("<DevEncoded>"),

            AttrValue::BooleanArray(v) => slice_display(v, |x| x, f),
            AttrValue::UCharArray(v) => slice_upper_hex(v, |x| x, f),
            AttrValue::ShortArray(v) => slice_upper_hex(v, |x| x, f),
            AttrValue::UShortArray(v) => slice_upper_hex(v, |x| x, f),
            AttrValue::LongArray(v) => slice_upper_hex(v, |x| x, f),
            AttrValue::ULongArray(v) => slice_upper_hex(v, |x| x, f),
            AttrValue::Long64Array(v) => slice_upper_hex(v, |x| x, f),
            AttrValue::ULong64Array(v) => slice_upper_hex(v, |x| x, f),
            AttrValue::FloatArray(v) => slice_display(v, |x| x, f),
            AttrValue::DoubleArray(v) => slice_display(v, |x| x, f),
            AttrValue::StringArray(v) => slice_display(v, |x| String::from_utf8_lossy(x), f),
            AttrValue::StateArray(v) => slice_display(v, |x| x, f),
            AttrValue::EncodedArray(v) => slice_display(v, |_| "<DevEncoded>", f),
        }
    }
}

impl fmt::LowerExp for AttrValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AttrValue::Boolean(v) => fmt::Display::fmt(&v, f),
            AttrValue::UChar(v) => fmt::Display::fmt(&v, f),
            AttrValue::Short(v) => fmt::Display::fmt(&v, f),
            AttrValue::UShort(v) => fmt::Display::fmt(&v, f),
            AttrValue::Long(v) => fmt::Display::fmt(&v, f),
            AttrValue::ULong(v) => fmt::Display::fmt(&v, f),
            AttrValue::Long64(v) => fmt::Display::fmt(&v, f),
            AttrValue::ULong64(v) => fmt::Display::fmt(&v, f),
            AttrValue::Float(v) => fmt::LowerExp::fmt(&v, f),
            AttrValue::Double(v) => fmt::LowerExp::fmt(&v, f),
            AttrValue::String(v) => fmt::Display::fmt(&String::from_utf8_lossy(v), f),
            AttrValue::State(v) => fmt::Display::fmt(&v, f),
            AttrValue::Encoded(_) => f.pad("<DevEncoded>"),

            AttrValue::BooleanArray(v) => slice_display(v, |x| x, f),
            AttrValue::UCharArray(v) => slice_display(v, |x| x, f),
            AttrValue::ShortArray(v) => slice_display(v, |x| x, f),
            AttrValue::UShortArray(v) => slice_display(v, |x| x, f),
            AttrValue::LongArray(v) => slice_display(v, |x| x, f),
            AttrValue::ULongArray(v) => slice_display(v, |x| x, f),
            AttrValue::Long64Array(v) => slice_display(v, |x| x, f),
            AttrValue::ULong64Array(v) => slice_display(v, |x| x, f),
            AttrValue::FloatArray(v) => slice_lower_exp(v, |x| x, f),
            AttrValue::DoubleArray(v) => slice_lower_exp(v, |x| x, f),
            AttrValue::StringArray(v) => slice_display(v, |x| String::from_utf8_lossy(x), f),
            AttrValue::StateArray(v) => slice_display(v, |x| x, f),
            AttrValue::EncodedArray(v) => slice_display(v, |_| "<DevEncoded>", f),
        }
    }
}

impl fmt::UpperExp for AttrValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AttrValue::Boolean(v) => fmt::Display::fmt(&v, f),
            AttrValue::UChar(v) => fmt::Display::fmt(&v, f),
            AttrValue::Short(v) => fmt::Display::fmt(&v, f),
            AttrValue::UShort(v) => fmt::Display::fmt(&v, f),
            AttrValue::Long(v) => fmt::Display::fmt(&v, f),
            AttrValue::ULong(v) => fmt::Display::fmt(&v, f),
            AttrValue::Long64(v) => fmt::Display::fmt(&v, f),
            AttrValue::ULong64(v) => fmt::Display::fmt(&v, f),
            AttrValue::Float(v) => fmt::UpperExp::fmt(&v, f),
            AttrValue::Double(v) => fmt::UpperExp::fmt(&v, f),
            AttrValue::String(v) => fmt::Display::fmt(&String::from_utf8_lossy(v), f),
            AttrValue::State(v) => fmt::Display::fmt(&v, f),
            AttrValue::Encoded(_) => f.pad("<DevEncoded>"),

            AttrValue::BooleanArray(v) => slice_display(v, |x| x, f),
            AttrValue::UCharArray(v) => slice_display(v, |x| x, f),
            AttrValue::ShortArray(v) => slice_display(v, |x| x, f),
            AttrValue::UShortArray(v) => slice_display(v, |x| x, f),
            AttrValue::LongArray(v) => slice_display(v, |x| x, f),
            AttrValue::ULongArray(v) => slice_display(v, |x| x, f),
            AttrValue::Long64Array(v) => slice_display(v, |x| x, f),
            AttrValue::ULong64Array(v) => slice_display(v, |x| x, f),
            AttrValue::FloatArray(v) => slice_upper_exp(v, |x| x, f),
            AttrValue::DoubleArray(v) => slice_upper_exp(v, |x| x, f),
            AttrValue::StringArray(v) => slice_display(v, |x| String::from_utf8_lossy(x), f),
            AttrValue::StateArray(v) => slice_display(v, |x| x, f),
            AttrValue::EncodedArray(v) => slice_display(v, |_| "<DevEncoded>", f),
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct DbDatum {
    pub name: String,
    pub data: PropertyValue,
    pub wrong_data_type: bool,
    request_type: Option<TangoDataType>,
}

impl DbDatum {
    pub fn new(name: &str, data: PropertyValue) -> DbDatum {
        DbDatum {
            name: name.into(),
            data: data,
            wrong_data_type: false,
            request_type: None,
        }
    }

    pub fn for_request(name: &str, typ: TangoDataType) -> DbDatum {
        DbDatum {
            name: name.into(),
            data: PropertyValue::Empty,
            wrong_data_type: false,
            request_type: Some(typ),
        }
    }

    pub fn name_only(name: &str) -> DbDatum {
        DbDatum {
            name: name.into(),
            data: PropertyValue::Empty,
            wrong_data_type: false,
            request_type: None
        }
    }

    pub unsafe fn from_c(mut db_datum: c::DbDatum, free: bool) -> DbDatum {
        let data = if db_datum.is_empty {
            PropertyValue::Empty
        } else {
            let data = db_datum.prop_data;
            match TangoDataType::from_c(db_datum.data_type) {
                TangoDataType::Boolean => PropertyValue::Boolean(data.bool_val),
                TangoDataType::UChar => PropertyValue::UChar(data.char_val),
                TangoDataType::Short => PropertyValue::Short(data.short_val),
                TangoDataType::UShort => PropertyValue::UShort(data.ushort_val),
                TangoDataType::Long | TangoDataType::Int => PropertyValue::Long(data.long_val),
                TangoDataType::ULong => PropertyValue::ULong(data.ulong_val),
                TangoDataType::Long64 => PropertyValue::Long64(data.long64_val),
                TangoDataType::ULong64 => PropertyValue::ULong64(data.ulong64_val),
                TangoDataType::Float => PropertyValue::Float(data.float_val),
                TangoDataType::Double => PropertyValue::Double(data.double_val),
                TangoDataType::String | TangoDataType::ConstString => PropertyValue::String({
                    let ptr = data.string_val;
                    let len = libc::strlen(ptr);
                    Vec::from(slice::from_raw_parts(ptr as *mut u8, len))
                }),
                TangoDataType::ShortArray => PropertyValue::ShortArray({
                    let ptr = data.short_arr;
                    Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
                }),
                TangoDataType::UShortArray => PropertyValue::UShortArray({
                    let ptr = data.ushort_arr;
                    Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
                }),
                TangoDataType::LongArray => PropertyValue::LongArray({
                    let ptr = data.long_arr;
                    Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
                }),
                TangoDataType::ULongArray => PropertyValue::ULongArray({
                    let ptr = data.ulong_arr;
                    Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
                }),
                TangoDataType::Long64Array => PropertyValue::Long64Array({
                    let ptr = data.long64_arr;
                    Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
                }),
                TangoDataType::ULong64Array => PropertyValue::ULong64Array({
                    let ptr = data.ulong64_arr;
                    Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
                }),
                TangoDataType::FloatArray => PropertyValue::FloatArray({
                    let ptr = data.float_arr;
                    Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
                }),
                TangoDataType::DoubleArray => PropertyValue::DoubleArray({
                    let ptr = data.double_arr;
                    Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
                }),
                TangoDataType::StringArray => PropertyValue::StringArray({
                    let ptr = data.string_arr;
                    let mut res = Vec::with_capacity(ptr.length as usize);
                    for i in 0..ptr.length {
                        let raw = *ptr.sequence.offset(i as isize);
                        let len = libc::strlen(raw);
                        res.push(Vec::from(slice::from_raw_parts(raw as *mut u8, len)));
                    }
                    res
                }),
                _ => panic!("data type {:?} not allowed for attributes", db_datum.data_type)
            }
        };
        let res = DbDatum {
            name: string_from(db_datum.property_name),
            data: data,
            wrong_data_type: db_datum.wrong_data_type,
            request_type: None,
        };
        if free {
            c::tango_free_DbDatum(&mut db_datum);
        }
        res
    }

    pub unsafe fn into_c(self) -> (c::DbDatum, CString) {
        let mut content = mem::zeroed::<c::DbDatum>();

        // Since the property name string is sometimes overwritten, we have to store
        // a reference to it somewhere else to be able to free it.
        let name_string = cstring_from(self.name);
        content.property_name = name_string.as_ptr() as *mut c_char;

        macro_rules! impl_array {
            ($val:ident, $alt:ident, $arr:ident, $ctype:ty) => {
                {
                    let array = &mut content.prop_data.$arr;
                    array.length = $val.len() as u32;
                    array.sequence = Box::into_raw($val.into_boxed_slice()) as *mut $ctype;
                    TangoDataType::$alt as u32
                }
            }
        }

        if self.wrong_data_type {
            content.wrong_data_type = true;
        }

        if let Some(typ) = self.request_type {
            content.data_type = typ as u32;
        } else {
            let tag = match self.data {
                PropertyValue::Empty => {
                    content.is_empty = true;
                    0
                }
                PropertyValue::Boolean(v) => {
                    content.prop_data.bool_val = v;
                    TangoDataType::Boolean as u32
                }
                PropertyValue::Short(v) => {
                    content.prop_data.short_val = v;
                    TangoDataType::Short as u32
                }
                PropertyValue::Long(v) => {
                    content.prop_data.long_val = v;
                    TangoDataType::Long as u32
                }
                PropertyValue::Float(v) => {
                    content.prop_data.float_val = v;
                    TangoDataType::Float as u32
                }
                PropertyValue::Double(v) => {
                    content.prop_data.double_val = v;
                    TangoDataType::Double as u32
                }
                PropertyValue::UShort(v) => {
                    content.prop_data.ushort_val = v;
                    TangoDataType::UShort as u32
                }
                PropertyValue::ULong(v) => {
                    content.prop_data.ulong_val = v;
                    TangoDataType::ULong as u32
                }
                PropertyValue::Long64(v) => {
                    content.prop_data.long64_val = v;
                    TangoDataType::Long64 as u32
                }
                PropertyValue::ULong64(v) => {
                    content.prop_data.ulong64_val = v;
                    TangoDataType::ULong64 as u32
                }
                PropertyValue::String(v) => {
                    let cstr = cstring_from(v);
                    content.prop_data.string_val = cstr.into_raw();
                    TangoDataType::String as u32
                }
                PropertyValue::ShortArray(v) => impl_array!(v, ShortArray, short_arr, i16),
                PropertyValue::UShortArray(v) => impl_array!(v, UShortArray, ushort_arr, u16),
                PropertyValue::LongArray(v) => impl_array!(v, LongArray, long_arr, i32),
                PropertyValue::ULongArray(v) => impl_array!(v, ULongArray, ulong_arr, u32),
                PropertyValue::Long64Array(v) => impl_array!(v, Long64Array, long64_arr, i64),
                PropertyValue::ULong64Array(v) => impl_array!(v, ULong64Array, ulong64_arr, u64),
                PropertyValue::FloatArray(v) => impl_array!(v, FloatArray, float_arr, f32),
                PropertyValue::DoubleArray(v) => impl_array!(v, DoubleArray, double_arr, f64),
                PropertyValue::StringArray(v) => {
                    let array = &mut content.prop_data.string_arr;
                    let mut vec = Vec::with_capacity(v.len());
                    array.length = v.len() as u32;
                    for s in v.into_iter() {
                        vec.push(cstring_from(s).into_raw());
                    }
                    array.sequence = Box::into_raw(vec.into_boxed_slice()) as *mut *mut c_char;
                    TangoDataType::StringArray as u32
                },
                _ => panic!("Cannot send property value of type {:?}", self.data)
            };
            content.data_type = tag;
        }
        (content, name_string)
    }

    pub unsafe fn free_c_data(db_datum: c::DbDatum) {
        let data = db_datum.prop_data;
        match TangoDataType::from_c(db_datum.data_type) {
            TangoDataType::Void |
            TangoDataType::Boolean |
            TangoDataType::UChar |
            TangoDataType::Short |
            TangoDataType::Long |
            TangoDataType::Int |
            TangoDataType::Float |
            TangoDataType::Double |
            TangoDataType::UShort |
            TangoDataType::ULong |
            TangoDataType::Long64 |
            TangoDataType::ULong64 |
            TangoDataType::State => {}
            TangoDataType::String | TangoDataType::ConstString => {
                drop(CString::from_raw(data.string_val));
            }
            TangoDataType::ShortArray => drop(Box::from_raw(data.short_arr.sequence)),
            TangoDataType::UShortArray => drop(Box::from_raw(data.ushort_arr.sequence)),
            TangoDataType::LongArray => drop(Box::from_raw(data.long_arr.sequence)),
            TangoDataType::ULongArray => drop(Box::from_raw(data.ulong_arr.sequence)),
            TangoDataType::Long64Array => drop(Box::from_raw(data.long64_arr.sequence)),
            TangoDataType::ULong64Array => drop(Box::from_raw(data.ulong64_arr.sequence)),
            TangoDataType::FloatArray => drop(Box::from_raw(data.float_arr.sequence)),
            TangoDataType::DoubleArray => drop(Box::from_raw(data.double_arr.sequence)),
            TangoDataType::StringArray => {
                for i in 0..data.string_arr.length {
                    drop(CString::from_raw(*data.string_arr.sequence.offset(i as isize) as *mut c_char));
                }
                drop(Box::from_raw(data.string_arr.sequence));
            }
            _ => unreachable!()
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    Empty,

    Boolean(bool),
    UChar(u8),
    Short(i16),
    UShort(u16),
    Long(i32),
    ULong(u32),
    Long64(i64),
    ULong64(u64),
    Float(f32),
    Double(f64),
    String(Vec<u8>),

    ShortArray(Vec<i16>),
    UShortArray(Vec<u16>),
    LongArray(Vec<i32>),
    ULongArray(Vec<u32>),
    Long64Array(Vec<i64>),
    ULong64Array(Vec<u64>),
    FloatArray(Vec<f32>),
    DoubleArray(Vec<f64>),
    StringArray(Vec<Vec<u8>>),
}

impl PropertyValue {
    pub fn len(&self) -> usize {
        use PropertyValue::*;
        match self {
            ShortArray(a) => a.len(),
            UShortArray(a) => a.len(),
            LongArray(a) => a.len(),
            ULongArray(a) => a.len(),
            Long64Array(a) => a.len(),
            ULong64Array(a) => a.len(),
            FloatArray(a) => a.len(),
            DoubleArray(a) => a.len(),
            StringArray(a) => a.len(),
            Empty => 0,
            _ => 1,
        }
    }

    pub fn into_bool(self) -> Result<bool, Self> {
        match self {
            PropertyValue::Boolean(v) => Ok(v),
            _ => Err(self),
        }
    }

    pub fn into_i32(self) -> Result<i32, Self> {
        match self {
            PropertyValue::Boolean(v) => Ok(v as i32),
            PropertyValue::UChar(v) => Ok(v as i32),
            PropertyValue::Short(v) => Ok(v as i32),
            PropertyValue::UShort(v) => Ok(v as i32),
            PropertyValue::Long(v) => Ok(v),
            _ => Err(self),
        }
    }

    pub fn into_i64(self) -> Result<i64, Self> {
        match self {
            PropertyValue::Boolean(v) => Ok(v as i64),
            PropertyValue::UChar(v) => Ok(v as i64),
            PropertyValue::Short(v) => Ok(v as i64),
            PropertyValue::UShort(v) => Ok(v as i64),
            PropertyValue::Long(v) => Ok(v as i64),
            PropertyValue::ULong(v) => Ok(v as i64),
            PropertyValue::Long64(v) => Ok(v),
            _ => Err(self),
        }
    }

    pub fn into_u32(self) -> Result<u32, Self> {
        match self {
            PropertyValue::Boolean(v) => Ok(v as u32),
            PropertyValue::UChar(v) => Ok(v as u32),
            PropertyValue::Short(v) => Ok(v as u32),
            PropertyValue::UShort(v) => Ok(v as u32),
            PropertyValue::ULong(v) => Ok(v as u32),
            _ => Err(self),
        }
    }

    pub fn into_u64(self) -> Result<u64, Self> {
        match self {
            PropertyValue::Boolean(v) => Ok(v as u64),
            PropertyValue::UChar(v) => Ok(v as u64),
            PropertyValue::Short(v) => Ok(v as u64),
            PropertyValue::UShort(v) => Ok(v as u64),
            PropertyValue::Long(v) => Ok(v as u64),
            PropertyValue::ULong(v) => Ok(v as u64),
            PropertyValue::ULong64(v) => Ok(v),
            _ => Err(self),
        }
    }

    pub fn into_f32(self) -> Result<f32, Self> {
        match self {
            PropertyValue::Boolean(v) => Ok(v as i32 as f32),
            PropertyValue::UChar(v) => Ok(v as f32),
            PropertyValue::Short(v) => Ok(v as f32),
            PropertyValue::Long(v) => Ok(v as f32),
            PropertyValue::Long64(v) => Ok(v as f32),
            PropertyValue::UShort(v) => Ok(v as f32),
            PropertyValue::ULong(v) => Ok(v as f32),
            PropertyValue::ULong64(v) => Ok(v as f32),
            PropertyValue::Float(v) => Ok(v),
            PropertyValue::Double(v) => Ok(v as f32),
            _ => Err(self),
        }
    }

    pub fn into_f64(self) -> Result<f64, Self> {
        match self {
            PropertyValue::Boolean(v) => Ok(v as i32 as f64),
            PropertyValue::UChar(v) => Ok(v as f64),
            PropertyValue::Short(v) => Ok(v as f64),
            PropertyValue::Long(v) => Ok(v as f64),
            PropertyValue::Long64(v) => Ok(v as f64),
            PropertyValue::UShort(v) => Ok(v as f64),
            PropertyValue::ULong(v) => Ok(v as f64),
            PropertyValue::ULong64(v) => Ok(v as f64),
            PropertyValue::Float(v) => Ok(v as f64),
            PropertyValue::Double(v) => Ok(v),
            _ => Err(self),
        }
    }

    pub fn into_bytes(self) -> Result<Vec<u8>, Self> {
        match self {
            PropertyValue::String(s) => Ok(s),
            _ => Err(self),
        }
    }

    pub fn into_string(self) -> Result<String, Self> {
        match self {
            PropertyValue::String(s) => String::from_utf8(s).map_err(
                |e| PropertyValue::String(e.into_bytes())),
            _ => Err(self),
        }
    }
}

impl fmt::Display for PropertyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PropertyValue::Empty => f.pad("<Empty>"),
            PropertyValue::Boolean(v) => fmt::Display::fmt(&v, f),
            PropertyValue::UChar(v) => fmt::Display::fmt(&v, f),
            PropertyValue::Short(v) => fmt::Display::fmt(&v, f),
            PropertyValue::UShort(v) => fmt::Display::fmt(&v, f),
            PropertyValue::Long(v) => fmt::Display::fmt(&v, f),
            PropertyValue::ULong(v) => fmt::Display::fmt(&v, f),
            PropertyValue::Long64(v) => fmt::Display::fmt(&v, f),
            PropertyValue::ULong64(v) => fmt::Display::fmt(&v, f),
            PropertyValue::Float(v) => fmt::Display::fmt(&v, f),
            PropertyValue::Double(v) => fmt::Display::fmt(&v, f),
            PropertyValue::String(v) => fmt::Display::fmt(&String::from_utf8_lossy(v), f),

            PropertyValue::ShortArray(v) => slice_display(v, |x| x, f),
            PropertyValue::UShortArray(v) => slice_display(v, |x| x, f),
            PropertyValue::LongArray(v) => slice_display(v, |x| x, f),
            PropertyValue::ULongArray(v) => slice_display(v, |x| x, f),
            PropertyValue::Long64Array(v) => slice_display(v, |x| x, f),
            PropertyValue::ULong64Array(v) => slice_display(v, |x| x, f),
            PropertyValue::FloatArray(v) => slice_display(v, |x| x, f),
            PropertyValue::DoubleArray(v) => slice_display(v, |x| x, f),
            PropertyValue::StringArray(v) => slice_display(v, |x| String::from_utf8_lossy(x), f),
        }
    }
}

impl fmt::LowerHex for PropertyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PropertyValue::Empty => f.pad("<Empty>"),
            PropertyValue::Boolean(v) => fmt::Display::fmt(&v, f),
            PropertyValue::UChar(v) => fmt::LowerHex::fmt(&v, f),
            PropertyValue::Short(v) => fmt::LowerHex::fmt(&v, f),
            PropertyValue::UShort(v) => fmt::LowerHex::fmt(&v, f),
            PropertyValue::Long(v) => fmt::LowerHex::fmt(&v, f),
            PropertyValue::ULong(v) => fmt::LowerHex::fmt(&v, f),
            PropertyValue::Long64(v) => fmt::LowerHex::fmt(&v, f),
            PropertyValue::ULong64(v) => fmt::LowerHex::fmt(&v, f),
            PropertyValue::Float(v) => fmt::Display::fmt(&v, f),
            PropertyValue::Double(v) => fmt::Display::fmt(&v, f),
            PropertyValue::String(v) => fmt::Display::fmt(&String::from_utf8_lossy(v), f),

            PropertyValue::ShortArray(v) => slice_lower_hex(v, |x| x, f),
            PropertyValue::UShortArray(v) => slice_lower_hex(v, |x| x, f),
            PropertyValue::LongArray(v) => slice_lower_hex(v, |x| x, f),
            PropertyValue::ULongArray(v) => slice_lower_hex(v, |x| x, f),
            PropertyValue::Long64Array(v) => slice_lower_hex(v, |x| x, f),
            PropertyValue::ULong64Array(v) => slice_lower_hex(v, |x| x, f),
            PropertyValue::FloatArray(v) => slice_display(v, |x| x, f),
            PropertyValue::DoubleArray(v) => slice_display(v, |x| x, f),
            PropertyValue::StringArray(v) => slice_display(v, |x| String::from_utf8_lossy(x), f),
        }
    }
}

impl fmt::UpperHex for PropertyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PropertyValue::Empty => f.pad("<Empty>"),
            PropertyValue::Boolean(v) => fmt::Display::fmt(&v, f),
            PropertyValue::UChar(v) => fmt::UpperHex::fmt(&v, f),
            PropertyValue::Short(v) => fmt::UpperHex::fmt(&v, f),
            PropertyValue::UShort(v) => fmt::UpperHex::fmt(&v, f),
            PropertyValue::Long(v) => fmt::UpperHex::fmt(&v, f),
            PropertyValue::ULong(v) => fmt::UpperHex::fmt(&v, f),
            PropertyValue::Long64(v) => fmt::UpperHex::fmt(&v, f),
            PropertyValue::ULong64(v) => fmt::UpperHex::fmt(&v, f),
            PropertyValue::Float(v) => fmt::Display::fmt(&v, f),
            PropertyValue::Double(v) => fmt::Display::fmt(&v, f),
            PropertyValue::String(v) => fmt::Display::fmt(&String::from_utf8_lossy(v), f),

            PropertyValue::ShortArray(v) => slice_upper_hex(v, |x| x, f),
            PropertyValue::UShortArray(v) => slice_upper_hex(v, |x| x, f),
            PropertyValue::LongArray(v) => slice_upper_hex(v, |x| x, f),
            PropertyValue::ULongArray(v) => slice_upper_hex(v, |x| x, f),
            PropertyValue::Long64Array(v) => slice_upper_hex(v, |x| x, f),
            PropertyValue::ULong64Array(v) => slice_upper_hex(v, |x| x, f),
            PropertyValue::FloatArray(v) => slice_display(v, |x| x, f),
            PropertyValue::DoubleArray(v) => slice_display(v, |x| x, f),
            PropertyValue::StringArray(v) => slice_display(v, |x| String::from_utf8_lossy(x), f),
        }
    }
}

impl fmt::LowerExp for PropertyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PropertyValue::Empty => f.pad("<Empty>"),
            PropertyValue::Boolean(v) => fmt::Display::fmt(&v, f),
            PropertyValue::UChar(v) => fmt::Display::fmt(&v, f),
            PropertyValue::Short(v) => fmt::Display::fmt(&v, f),
            PropertyValue::UShort(v) => fmt::Display::fmt(&v, f),
            PropertyValue::Long(v) => fmt::Display::fmt(&v, f),
            PropertyValue::ULong(v) => fmt::Display::fmt(&v, f),
            PropertyValue::Long64(v) => fmt::Display::fmt(&v, f),
            PropertyValue::ULong64(v) => fmt::Display::fmt(&v, f),
            PropertyValue::Float(v) => fmt::LowerExp::fmt(&v, f),
            PropertyValue::Double(v) => fmt::LowerExp::fmt(&v, f),
            PropertyValue::String(v) => fmt::Display::fmt(&String::from_utf8_lossy(v), f),

            PropertyValue::ShortArray(v) => slice_display(v, |x| x, f),
            PropertyValue::UShortArray(v) => slice_display(v, |x| x, f),
            PropertyValue::LongArray(v) => slice_display(v, |x| x, f),
            PropertyValue::ULongArray(v) => slice_display(v, |x| x, f),
            PropertyValue::Long64Array(v) => slice_display(v, |x| x, f),
            PropertyValue::ULong64Array(v) => slice_display(v, |x| x, f),
            PropertyValue::FloatArray(v) => slice_lower_exp(v, |x| x, f),
            PropertyValue::DoubleArray(v) => slice_lower_exp(v, |x| x, f),
            PropertyValue::StringArray(v) => slice_display(v, |x| String::from_utf8_lossy(x), f),
        }
    }
}

impl fmt::UpperExp for PropertyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PropertyValue::Empty => f.pad("<Empty>"),
            PropertyValue::Boolean(v) => fmt::Display::fmt(&v, f),
            PropertyValue::UChar(v) => fmt::Display::fmt(&v, f),
            PropertyValue::Short(v) => fmt::Display::fmt(&v, f),
            PropertyValue::UShort(v) => fmt::Display::fmt(&v, f),
            PropertyValue::Long(v) => fmt::Display::fmt(&v, f),
            PropertyValue::ULong(v) => fmt::Display::fmt(&v, f),
            PropertyValue::Long64(v) => fmt::Display::fmt(&v, f),
            PropertyValue::ULong64(v) => fmt::Display::fmt(&v, f),
            PropertyValue::Float(v) => fmt::UpperExp::fmt(&v, f),
            PropertyValue::Double(v) => fmt::UpperExp::fmt(&v, f),
            PropertyValue::String(v) => fmt::Display::fmt(&String::from_utf8_lossy(v), f),

            PropertyValue::ShortArray(v) => slice_display(v, |x| x, f),
            PropertyValue::UShortArray(v) => slice_display(v, |x| x, f),
            PropertyValue::LongArray(v) => slice_display(v, |x| x, f),
            PropertyValue::ULongArray(v) => slice_display(v, |x| x, f),
            PropertyValue::Long64Array(v) => slice_display(v, |x| x, f),
            PropertyValue::ULong64Array(v) => slice_display(v, |x| x, f),
            PropertyValue::FloatArray(v) => slice_upper_exp(v, |x| x, f),
            PropertyValue::DoubleArray(v) => slice_upper_exp(v, |x| x, f),
            PropertyValue::StringArray(v) => slice_display(v, |x| String::from_utf8_lossy(x), f),
        }
    }
}


macro_rules! impl_slice_fmter {
    ($fn:ident, $tn:ident) => {
        fn $fn<'a, T, D: fmt::$tn>(slice: &'a [T], mkdisp: impl Fn(&'a T) -> D,
                                   f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[")?;
            for (i, item) in slice.iter().enumerate() {
                if i != 0 {
                    write!(f, ", ")?;
                }
                fmt::$tn::fmt(&mkdisp(item), f)?;
            }
            write!(f, "]")
        }
    }
}

impl_slice_fmter!(slice_display, Display);
impl_slice_fmter!(slice_lower_hex, LowerHex);
impl_slice_fmter!(slice_upper_hex, UpperHex);
impl_slice_fmter!(slice_lower_exp, LowerExp);
impl_slice_fmter!(slice_upper_exp, UpperExp);
