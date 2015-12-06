use std::ffi::{CStr, CString};
use std::ptr;
use std::slice;

use libc;

use super::c;

pub unsafe fn string_from(ptr: *const i8) -> String {
    CStr::from_ptr(ptr).to_string_lossy().into_owned()
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
    DevState,
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
    fn from_c(val: u32) -> TangoDataType {
        match val {
            c::DEV_VOID => TangoDataType::Void,
            c::DEV_BOOLEAN => TangoDataType::Boolean,
            c::DEV_SHORT => TangoDataType::Short,
            c::DEV_LONG => TangoDataType::Long,
            c::DEV_FLOAT => TangoDataType::Float,
            c::DEV_DOUBLE => TangoDataType::Double,
            c::DEV_USHORT => TangoDataType::UShort,
            c::DEV_ULONG => TangoDataType::ULong,
            c::DEV_STRING => TangoDataType::String,
            c::DEVVAR_CHARARRAY => TangoDataType::CharArray,
            c::DEVVAR_SHORTARRAY => TangoDataType::ShortArray,
            c::DEVVAR_LONGARRAY => TangoDataType::LongArray,
            c::DEVVAR_FLOATARRAY => TangoDataType::FloatArray,
            c::DEVVAR_DOUBLEARRAY => TangoDataType::DoubleArray,
            c::DEVVAR_USHORTARRAY => TangoDataType::UShortArray,
            c::DEVVAR_ULONGARRAY => TangoDataType::ULongArray,
            c::DEVVAR_STRINGARRAY => TangoDataType::StringArray,
            c::DEVVAR_LONGSTRINGARRAY => TangoDataType::LongStringArray,
            c::DEVVAR_DOUBLESTRINGARRAY => TangoDataType::DoubleStringArray,
            c::DEV_STATE => TangoDataType::DevState,
            c::CONST_DEV_STRING => TangoDataType::ConstString,
            c::DEVVAR_BOOLEANARRAY => TangoDataType::BooleanArray,
            c::DEV_UCHAR => TangoDataType::UChar,
            c::DEV_LONG64 => TangoDataType::Long64,
            c::DEV_ULONG64 => TangoDataType::ULong64,
            c::DEVVAR_LONG64ARRAY => TangoDataType::Long64Array,
            c::DEVVAR_ULONG64ARRAY => TangoDataType::ULong64Array,
            c::DEV_INT => TangoDataType::Int,
            c::DEV_ENCODED => TangoDataType::Encoded,
            _ => panic!("unknown Tango data type tag={}", val)
        }
    }
}


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
    fn from_c(val: u32) -> TangoDevState {
        match val {
            c::ON => TangoDevState::On,
            c::OFF => TangoDevState::Off,
            c::CLOSE => TangoDevState::Close,
            c::OPEN => TangoDevState::Open,
            c::INSERT => TangoDevState::Insert,
            c::EXTRACT => TangoDevState::Extract,
            c::MOVING => TangoDevState::Moving,
            c::STANDBY => TangoDevState::Standby,
            c::FAULT => TangoDevState::Fault,
            c::INIT => TangoDevState::Init,
            c::RUNNING => TangoDevState::Running,
            c::ALARM => TangoDevState::Alarm,
            c::DISABLE => TangoDevState::Disable,
            c::UNKNOWN => TangoDevState::Unknown,
            _ => unimplemented!()
        }
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
    fn from_c(val: u32) -> AttrQuality {
        match val {
            c::ATTR_VALID => AttrQuality::Valid,
            c::ATTR_INVALID => AttrQuality::Invalid,
            c::ATTR_ALARM => AttrQuality::Alarm,
            c::ATTR_CHANGING => AttrQuality::Changing,
            c::ATTR_WARNING => AttrQuality::Warning,
            _ => unimplemented!()
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
    fn from_c(val: u32) -> AttrWriteType {
        match val {
            c::READ => AttrWriteType::Read,
            c::READ_WITH_WRITE => AttrWriteType::ReadWithWrite,
            c::WRITE => AttrWriteType::Write,
            c::READ_WRITE => AttrWriteType::ReadWrite,
            _ => unimplemented!()
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
    fn from_c(val: u32) -> AttrDataFormat {
        match val {
            c::SCALAR => AttrDataFormat::Scalar,
            c::SPECTRUM => AttrDataFormat::Spectrum,
            c::IMAGE => AttrDataFormat::Image,
            _ => unimplemented!()
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DispLevel {
    Operator,
    Expert,
}

impl DispLevel {
    fn from_c(val: u32) -> DispLevel {
        match val {
            c::OPERATOR => DispLevel::Operator,
            c::EXPERT => DispLevel::Expert,
            _ => unimplemented!()
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
            c::WARN => ErrSeverity::Warn,
            c::ERR => ErrSeverity::Err,
            c::PANIC => ErrSeverity::Panic,
            _ => unimplemented!()
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
    pub fn from_c(val: u32) -> DevSource {
        match val {
            c::DEV => DevSource::Dev,
            c::CACHE => DevSource::Cache,
            c::CACHE_DEV => DevSource::CacheDev,
            _ => unimplemented!()
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
    ConstString(Vec<u8>),

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

    DevState(TangoDevState),
    Encoded(String, Vec<u8>),
}

impl CommandData {
    pub fn from_str(s: &str) -> CommandData {
        CommandData::String(s.to_owned().into_bytes())
    }

    pub fn into_bool(self) -> Option<bool> {
        match self {
            CommandData::Boolean(v) => Some(v),
            _ => None,
        }
    }

    pub fn into_i32(self) -> Option<i32> {
        match self {
            CommandData::Boolean(v) => Some(v as i32),
            CommandData::Short(v) => Some(v as i32),
            CommandData::Long(v) => Some(v),
            CommandData::UShort(v) => Some(v as i32),
            _ => None,
        }
    }

    pub fn into_i64(self) -> Option<i64> {
        match self {
            CommandData::Boolean(v) => Some(v as i64),
            CommandData::Short(v) => Some(v as i64),
            CommandData::Long(v) => Some(v as i64),
            CommandData::Long64(v) => Some(v),
            CommandData::UShort(v) => Some(v as i64),
            CommandData::ULong(v) => Some(v as i64),
            _ => None,
        }
    }

    pub fn into_u32(self) -> Option<u32> {
        match self {
            CommandData::Boolean(v) => Some(v as u32),
            CommandData::Short(v) => Some(v as u32),
            CommandData::UShort(v) => Some(v as u32),
            CommandData::ULong(v) => Some(v as u32),
            _ => None,
        }
    }

    pub fn into_u64(self) -> Option<u64> {
        match self {
            CommandData::Boolean(v) => Some(v as u64),
            CommandData::Short(v) => Some(v as u64),
            CommandData::Long(v) => Some(v as u64),
            CommandData::UShort(v) => Some(v as u64),
            CommandData::ULong(v) => Some(v as u64),
            CommandData::ULong64(v) => Some(v),
            _ => None,
        }
    }

    pub fn into_bytes(self) -> Option<Vec<u8>> {
        match self {
            CommandData::String(s) => Some(s),
            CommandData::ConstString(s) => Some(s),
            CommandData::CharArray(s) => Some(s),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            CommandData::String(s) => String::from_utf8(s).ok(),
            CommandData::ConstString(s) => String::from_utf8(s).ok(),
            CommandData::CharArray(s) => String::from_utf8(s).ok(),
            _ => None,
        }
    }

    pub unsafe fn from_c(mut cmd_data: c::CommandData) -> CommandData {
        let tag = cmd_data.arg_type;
        let mut data = cmd_data.cmd_data;
        let res = match tag {
            c::DEV_VOID => CommandData::Void,
            c::DEV_BOOLEAN => CommandData::Boolean(ptr::read(data.bool_val()) != 0),
            c::DEV_SHORT => CommandData::Short(ptr::read(data.short_val())),
            c::DEV_LONG | c::DEV_INT => CommandData::Long(ptr::read(data.long_val())),
            c::DEV_FLOAT => CommandData::Float(ptr::read(data.float_val())),
            c::DEV_DOUBLE => CommandData::Double(ptr::read(data.double_val())),
            c::DEV_USHORT => CommandData::UShort(ptr::read(data.ushort_val())),
            c::DEV_ULONG => CommandData::ULong(ptr::read(data.ulong_val())),
            // XXX: for all arrays this copies the data, instead of reusing the
            // existing allocation
            c::DEV_STRING | c::CONST_DEV_STRING => CommandData::String({
                let raw = ptr::read(data.string_val());
                let len = libc::strlen(raw);
                Vec::from(slice::from_raw_parts(raw as *mut u8, len))
            }),
            c::DEVVAR_CHARARRAY => CommandData::CharArray({
                let ptr = ptr::read(data.char_arr());
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            c::DEVVAR_SHORTARRAY => CommandData::ShortArray({
                let ptr = ptr::read(data.short_arr());
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            c::DEVVAR_LONGARRAY => CommandData::LongArray({
                let ptr = ptr::read(data.long_arr());
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            c::DEVVAR_FLOATARRAY => CommandData::FloatArray({
                let ptr = ptr::read(data.float_arr());
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            c::DEVVAR_DOUBLEARRAY => CommandData::DoubleArray({
                let ptr = ptr::read(data.double_arr());
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            c::DEVVAR_USHORTARRAY => CommandData::UShortArray({
                let ptr = ptr::read(data.ushort_arr());
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            c::DEVVAR_ULONGARRAY => CommandData::ULongArray({
                let ptr = ptr::read(data.ulong_arr());
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            c::DEVVAR_STRINGARRAY => CommandData::StringArray({
                let ptr = ptr::read(data.string_arr());
                let mut res = Vec::with_capacity(ptr.length as usize);
                for i in 0..ptr.length {
                    let raw = ptr::read(ptr.sequence.offset(i as isize) as *const *const i8);
                    let len = libc::strlen(raw);
                    res.push(Vec::from(slice::from_raw_parts(raw as *mut u8, len)));
                }
                res
            }),
            c::DEVVAR_LONGSTRINGARRAY => {
                let ptr = ptr::read(data.long_string_arr());
                let mut strvec = Vec::with_capacity(ptr.string_length as usize);
                for i in 0..ptr.string_length {
                    let raw = ptr::read(ptr.string_sequence.offset(i as isize) as *const *const i8);
                    let len = libc::strlen(raw);
                    strvec.push(Vec::from(slice::from_raw_parts(raw as *mut u8, len)));
                }
                CommandData::LongStringArray(
                    Vec::from(slice::from_raw_parts(ptr.long_sequence, ptr.long_length as usize)),
                    strvec
                )
            },
            c::DEVVAR_DOUBLESTRINGARRAY => {
                let ptr = ptr::read(data.double_string_arr());
                let mut strvec = Vec::with_capacity(ptr.string_length as usize);
                for i in 0..ptr.string_length {
                    let raw = ptr::read(ptr.string_sequence.offset(i as isize) as *const *const i8);
                    let len = libc::strlen(raw);
                    strvec.push(Vec::from(slice::from_raw_parts(raw as *mut u8, len)));
                }
                CommandData::DoubleStringArray(
                    Vec::from(slice::from_raw_parts(ptr.double_sequence, ptr.double_length as usize)),
                    strvec
                )
            },
            c::DEV_STATE => CommandData::DevState(TangoDevState::from_c(
                ptr::read(data.state_val()))),
            c::DEVVAR_BOOLEANARRAY => CommandData::BooleanArray({
                let ptr = ptr::read(data.bool_arr());
                slice::from_raw_parts(ptr.sequence, ptr.length as usize).iter().map(|&v| v != 0).collect()
            }),
            c::DEV_UCHAR => unimplemented!(),  // only for attribute arrays
            c::DEV_LONG64 => CommandData::Long64(ptr::read(data.long64_val())),
            c::DEV_ULONG64 => CommandData::ULong64(ptr::read(data.ulong64_val())),
            c::DEVVAR_LONG64ARRAY => CommandData::Long64Array({
                let ptr = ptr::read(data.long64_arr());
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            c::DEVVAR_ULONG64ARRAY => CommandData::ULong64Array({
                let ptr = ptr::read(data.ulong64_arr());
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            c::DEV_ENCODED => {
                let ptr = ptr::read(data.encoded_val());
                let format = string_from(ptr.encoded_format);
                CommandData::Encoded(
                    format,
                    Vec::from(slice::from_raw_parts(ptr.encoded_data, ptr.encoded_length as usize)))
            },
            _ => unimplemented!()
        };
        c::tango_free_CommandData(&mut cmd_data);
        res
    }

    pub unsafe fn into_c(self) -> c::CommandData {
        let mut content = c::TangoCommandData::default();
        let tag = match self {
            CommandData::Void => TangoDataType::Void,
            CommandData::Boolean(v) => {
                ptr::write(content.bool_val(), v as u8);
                TangoDataType::Boolean
            }
            CommandData::Short(v) => {
                ptr::write(content.short_val(), v);
                TangoDataType::Short
            }
            CommandData::Long(v) => {
                ptr::write(content.long_val(), v);
                TangoDataType::Long
            }
            CommandData::Float(v) => {
                ptr::write(content.float_val(), v);
                TangoDataType::Float
            }
            CommandData::Double(v) => {
                ptr::write(content.double_val(), v);
                TangoDataType::Double
            }
            CommandData::UShort(v) => {
                ptr::write(content.ushort_val(), v);
                TangoDataType::UShort
            }
            CommandData::ULong(v) => {
                ptr::write(content.ulong_val(), v);
                TangoDataType::ULong
            }
            CommandData::String(v) => {
                let cstr = CString::new(v).unwrap();  // XXX
                ptr::write(content.string_val(), cstr.into_raw());
                TangoDataType::String
            }
            _ => unimplemented!()  // XXX
        };
        c::CommandData { arg_type: tag as u32, cmd_data: content }
    }

    pub unsafe fn free_c_data(mut data: c::CommandData) {
        match data.arg_type {
            c::DEV_VOID | c::DEV_BOOLEAN | c::DEV_SHORT | c::DEV_LONG |
            c::DEV_FLOAT | c::DEV_DOUBLE | c::DEV_USHORT | c::DEV_ULONG => {}
            c::DEV_STRING => {
                drop(CString::from_raw(ptr::read(data.cmd_data.string_val())));
            }
            _ => unimplemented!()
        }
    }
}


pub type AttributeInfo = ();
pub type AttributeData = ();
