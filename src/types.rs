use std::ffi::{CStr, CString};
use std::slice;

use libc;
use time::{get_time, Timespec};
use c_tango as c;


pub unsafe fn string_from(ptr: *const i8) -> String {
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
            c::DEV_STATE => TangoDataType::State,
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
            CommandData::CharArray(s) => Some(s),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            CommandData::String(s) => String::from_utf8(s).ok(),
            CommandData::CharArray(s) => String::from_utf8(s).ok(),
            _ => None,
        }
    }

    pub unsafe fn from_c(mut cmd_data: c::CommandData) -> CommandData {
        let tag = TangoDataType::from_c(cmd_data.arg_type);
        let mut data = cmd_data.cmd_data;
        let res = match tag {
            TangoDataType::Void => CommandData::Void,
            TangoDataType::Boolean => CommandData::Boolean(*data.bool_val() != 0),
            TangoDataType::Short => CommandData::Short(*data.short_val()),
            TangoDataType::Long | TangoDataType::Int => CommandData::Long(*data.long_val()),
            TangoDataType::Float => CommandData::Float(*data.float_val()),
            TangoDataType::Double => CommandData::Double(*data.double_val()),
            TangoDataType::UShort => CommandData::UShort(*data.ushort_val()),
            TangoDataType::ULong => CommandData::ULong(*data.ulong_val()),
            TangoDataType::UChar => unimplemented!(),  // only for attribute arrays
            TangoDataType::Long64 => CommandData::Long64(*data.long64_val()),
            TangoDataType::ULong64 => CommandData::ULong64(*data.ulong64_val()),
            TangoDataType::State => CommandData::State(TangoDevState::from_c(*data.state_val())),
            // note: for all arrays this copies the data, instead of reusing the
            // existing allocation
            TangoDataType::String | TangoDataType::ConstString => CommandData::String({
                let ptr = *data.string_val();
                let len = libc::strlen(ptr);
                Vec::from(slice::from_raw_parts(ptr as *mut u8, len))
            }),
            TangoDataType::CharArray => CommandData::CharArray({
                let ptr = *data.char_arr();
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            TangoDataType::ShortArray => CommandData::ShortArray({
                let ptr = *data.short_arr();
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            TangoDataType::LongArray => CommandData::LongArray({
                let ptr = *data.long_arr();
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            TangoDataType::FloatArray => CommandData::FloatArray({
                let ptr = *data.float_arr();
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            TangoDataType::DoubleArray => CommandData::DoubleArray({
                let ptr = *data.double_arr();
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            TangoDataType::UShortArray => CommandData::UShortArray({
                let ptr = *data.ushort_arr();
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            TangoDataType::ULongArray => CommandData::ULongArray({
                let ptr = *data.ulong_arr();
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            TangoDataType::StringArray => CommandData::StringArray({
                let ptr = *data.string_arr();
                let mut res = Vec::with_capacity(ptr.length as usize);
                for i in 0..ptr.length {
                    let raw = *ptr.sequence.offset(i as isize);
                    let len = libc::strlen(raw);
                    res.push(Vec::from(slice::from_raw_parts(raw as *mut u8, len)));
                }
                res
            }),
            TangoDataType::LongStringArray => {
                let ptr = *data.long_string_arr();
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
                let ptr = *data.double_string_arr();
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
                let ptr = *data.bool_arr();
                slice::from_raw_parts(ptr.sequence, ptr.length as usize)
                    .iter().map(|&v| v != 0).collect()
            }),
            TangoDataType::Long64Array => CommandData::Long64Array({
                let ptr = *data.long64_arr();
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            TangoDataType::ULong64Array => CommandData::ULong64Array({
                let ptr = *data.ulong64_arr();
                Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
            }),
            TangoDataType::Encoded => {
                let ptr = *data.encoded_val();
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
        let mut content = c::TangoCommandData::default();

        macro_rules! impl_array {
            ($val:ident, $alt:ident, $arr:ident, $ctype:ty) => {
                {
                    let array = content.$arr();
                    (*array).length = $val.len() as u32;
                    (*array).sequence = Box::into_raw($val.into_boxed_slice()) as *mut $ctype;
                    TangoDataType::$alt
                }
            }
        }

        let tag = match self {
            CommandData::Void => {
                TangoDataType::Void
            }
            CommandData::Boolean(v) => {
                *content.bool_val() = v as u8;
                TangoDataType::Boolean
            }
            CommandData::Short(v) => {
                *content.short_val() = v;
                TangoDataType::Short
            }
            CommandData::Long(v) => {
                *content.long_val() = v;
                TangoDataType::Long
            }
            CommandData::Float(v) => {
                *content.float_val() = v;
                TangoDataType::Float
            }
            CommandData::Double(v) => {
                *content.double_val() = v;
                TangoDataType::Double
            }
            CommandData::UShort(v) => {
                *content.ushort_val() = v;
                TangoDataType::UShort
            }
            CommandData::ULong(v) => {
                *content.ulong_val() = v;
                TangoDataType::ULong
            }
            CommandData::Long64(v) => {
                *content.long64_val() = v;
                TangoDataType::Long64
            }
            CommandData::ULong64(v) => {
                *content.ulong64_val() = v;
                TangoDataType::ULong64
            }
            CommandData::String(v) => {
                let cstr = cstring_from(v);
                *content.string_val() = cstr.into_raw();
                TangoDataType::String
            }
            CommandData::Encoded((format, data)) => {
                let ptr = content.encoded_val();
                (*ptr).encoded_format = cstring_from(format).into_raw();
                (*ptr).encoded_length = data.len() as u32;
                (*ptr).encoded_data = Box::into_raw(data.into_boxed_slice()) as *mut u8;
                TangoDataType::Encoded
            }
            CommandData::BooleanArray(v) => impl_array!(v, BooleanArray, bool_arr, libc::c_uchar),
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
                let array = content.string_arr();
                let mut vec = Vec::with_capacity(v.len());
                (*array).length = v.len() as u32;
                for s in v.into_iter() {
                    vec.push(cstring_from(s).into_raw());
                }
                (*array).sequence = Box::into_raw(vec.into_boxed_slice()) as *mut *mut i8;
                TangoDataType::StringArray
            },
            CommandData::LongStringArray(vl, vs) => {
                let array = content.long_string_arr();
                (*array).long_length = vl.len() as u32;
                (*array).long_sequence = Box::into_raw(vl.into_boxed_slice()) as *mut i32;
                let mut vec = Vec::with_capacity(vs.len());
                (*array).string_length = vs.len() as u32;
                for s in vs.into_iter() {
                    vec.push(cstring_from(s).into_raw());
                }
                (*array).string_sequence = Box::into_raw(vec.into_boxed_slice()) as *mut *mut i8;
                TangoDataType::LongStringArray
            },
            CommandData::DoubleStringArray(vd, vs) => {
                let array = content.double_string_arr();
                (*array).double_length = vd.len() as u32;
                (*array).double_sequence = Box::into_raw(vd.into_boxed_slice()) as *mut f64;
                let mut vec = Vec::with_capacity(vs.len());
                (*array).string_length = vs.len() as u32;
                for s in vs.into_iter() {
                    vec.push(cstring_from(s).into_raw());
                }
                (*array).string_sequence = Box::into_raw(vec.into_boxed_slice()) as *mut *mut i8;
                TangoDataType::DoubleStringArray
            },
            CommandData::State(_) => panic!("Cannot send input argument of type State")
        };
        c::CommandData { arg_type: tag as u32, cmd_data: content }
    }

    pub unsafe fn free_c_data(cmd_data: c::CommandData) {
        let mut data = cmd_data.cmd_data;
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
                drop(CString::from_raw(*data.string_val()));
            }
            TangoDataType::Encoded => {
                drop(CString::from_raw((*data.encoded_val()).encoded_format));
                drop(Box::from_raw((*data.encoded_val()).encoded_data));
            }
            TangoDataType::BooleanArray => drop(Box::from_raw((*data.bool_arr()).sequence)),
            TangoDataType::CharArray => drop(Box::from_raw((*data.char_arr()).sequence)),
            TangoDataType::ShortArray => drop(Box::from_raw((*data.short_arr()).sequence)),
            TangoDataType::UShortArray => drop(Box::from_raw((*data.ushort_arr()).sequence)),
            TangoDataType::LongArray => drop(Box::from_raw((*data.long_arr()).sequence)),
            TangoDataType::ULongArray => drop(Box::from_raw((*data.ulong_arr()).sequence)),
            TangoDataType::Long64Array => drop(Box::from_raw((*data.long64_arr()).sequence)),
            TangoDataType::ULong64Array => drop(Box::from_raw((*data.ulong64_arr()).sequence)),
            TangoDataType::FloatArray => drop(Box::from_raw((*data.float_arr()).sequence)),
            TangoDataType::DoubleArray => drop(Box::from_raw((*data.double_arr()).sequence)),
            TangoDataType::StringArray => {
                for i in 0..(*data.string_arr()).length {
                    drop(CString::from_raw(*(*data.string_arr()).sequence.offset(i as isize) as *mut i8));
                }
                drop(Box::from_raw((*data.string_arr()).sequence));
            }
            TangoDataType::LongStringArray => {
                for i in 0..(*data.long_string_arr()).string_length {
                    drop(CString::from_raw(*(*data.long_string_arr())
                                           .string_sequence.offset(i as isize) as *mut i8));
                }
                drop(Box::from_raw((*data.long_string_arr()).string_sequence));
                drop(Box::from_raw((*data.long_string_arr()).long_sequence));
            }
            TangoDataType::DoubleStringArray => {
                for i in 0..(*data.double_string_arr()).string_length {
                    drop(CString::from_raw(*(*data.double_string_arr())
                                           .string_sequence.offset(i as isize) as *mut i8));
                }
                drop(Box::from_raw((*data.double_string_arr()).string_sequence));
                drop(Box::from_raw((*data.double_string_arr()).double_sequence));
            }
        }
    }
}


#[derive(Debug)]
pub struct AttributeInfo {
    pub name: String,
    pub writable: AttrWriteType,
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
        let mut data = attr_data.attr_data;
        let is_scalar = AttrDataFormat::from_c(attr_data.data_format) == AttrDataFormat::Scalar;
        let (res_read, res_written) = if is_scalar {
            macro_rules! impl_simple {
                ($alt:ident, $arr:ident) => {
                    {
                        let ptr = *data.$arr();
                        ($alt(*ptr.sequence), $alt(*ptr.sequence.offset(1)))
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
                TangoDataType::Boolean =>
                    (Boolean(*(*data.bool_arr()).sequence != 0),
                     Boolean(*(*data.bool_arr()).sequence.offset(1) != 0)),
                TangoDataType::State =>
                    (State(TangoDevState::from_c(*(*data.state_arr()).sequence)),
                     State(TangoDevState::from_c(*(*data.state_arr()).sequence.offset(1)))),
                TangoDataType::String => {
                    let rawr = *(*data.string_arr()).sequence;
                    let lenr = libc::strlen(rawr);
                    let raww = *(*data.string_arr()).sequence.offset(1);
                    let lenw = libc::strlen(raww);
                    (String(Vec::from(slice::from_raw_parts(rawr as *mut u8, lenr))),
                     String(Vec::from(slice::from_raw_parts(raww as *mut u8, lenw))))
                },
                TangoDataType::Encoded => {
                    let rawr = *(*data.encoded_arr()).sequence;
                    let raww = *(*data.encoded_arr()).sequence.offset(1);
                    (Encoded(
                        (string_from(rawr.encoded_format),
                         Vec::from(slice::from_raw_parts(rawr.encoded_data as *mut u8,
                                                         rawr.encoded_length as usize)))),
                     Encoded(
                        (string_from(raww.encoded_format),
                         Vec::from(slice::from_raw_parts(raww.encoded_data as *mut u8,
                                                         raww.encoded_length as usize)))))
                },
                _ => panic!("data type {:?} not allowed for attributes", tag)
            }
        } else {
            macro_rules! impl_simple {
                ($alt:ident, $arr:ident) => {
                    {
                        let ptr = *data.$arr();
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
                TangoDataType::Boolean => {
                    let ptr = *data.bool_arr();
                    let slice = slice::from_raw_parts(ptr.sequence, ptr.length as usize);
                    let (p1, p2) = slice.split_at(attr_data.nb_read as usize);
                    (BooleanArray(p1.iter().map(|&v| v != 0).collect()),
                     BooleanArray(p2.iter().map(|&v| v != 0).collect()))
                },
                TangoDataType::State => {
                    let ptr = *data.state_arr();
                    let slice = slice::from_raw_parts(ptr.sequence, ptr.length as usize);
                    let (p1, p2) = slice.split_at(attr_data.nb_read as usize);
                    (StateArray(p1.iter().map(|&v| TangoDevState::from_c(v)).collect()),
                     StateArray(p2.iter().map(|&v| TangoDevState::from_c(v)).collect()))
                },
                TangoDataType::String => {
                    let ptr = *data.string_arr();
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
                    let ptr = *data.encoded_arr();
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
            time_stamp: Timespec::new(attr_data.time_stamp.tv_sec,
                                      1000 * attr_data.time_stamp.tv_usec as i32),
        };
        if free {
            c::tango_free_AttributeData(&mut attr_data);
        }
        res
    }

    pub unsafe fn into_c(self) -> c::AttributeData {
        let mut content = c::TangoAttributeData::default();

        macro_rules! impl_simple {
            ($val:ident, $alt:ident, $arr:ident, $ctype:ty) => {
                {
                    let array = content.$arr();
                    (*array).length = 1;
                    (*array).sequence = Box::into_raw(Box::new($val)) as *mut $ctype;
                    TangoDataType::$alt
                }
            }
        }

        macro_rules! impl_array {
            ($val:ident, $alt:ident, $arr:ident, $ctype:ty) => {
                {
                    let array = content.$arr();
                    (*array).length = $val.len() as u32;
                    (*array).sequence = Box::into_raw($val.into_boxed_slice()) as *mut $ctype;
                    TangoDataType::$alt
                }
            }
        }

        let tag = match self.data {
            AttrValue::Boolean(v) => impl_simple!(v, Boolean, bool_arr, libc::c_uchar),
            AttrValue::UChar(v) => impl_simple!(v, UChar, char_arr, u8),
            AttrValue::Short(v) => impl_simple!(v, Short, short_arr, i16),
            AttrValue::UShort(v) => impl_simple!(v, UShort, ushort_arr, u16),
            AttrValue::Long(v) => impl_simple!(v, Long, long_arr, i32),
            AttrValue::ULong(v) => impl_simple!(v, ULong, ulong_arr, u32),
            AttrValue::Long64(v) => impl_simple!(v, Long64, long64_arr, i64),
            AttrValue::ULong64(v) => impl_simple!(v, ULong64, ulong64_arr, u64),
            AttrValue::Float(v) => impl_simple!(v, Float, float_arr, f32),
            AttrValue::Double(v) => impl_simple!(v, Double, double_arr, f64),
            AttrValue::State(v) => impl_simple!(v, State, state_arr, u32),
            AttrValue::String(v) => {
                let array = content.string_arr();
                (*array).length = 1;
                let mut vec = Vec::with_capacity(1);
                vec.push(cstring_from(v).into_raw());
                (*array).sequence = Box::into_raw(vec.into_boxed_slice()) as *mut *mut i8;
                TangoDataType::String
            }
            AttrValue::Encoded((format, data)) => {
                let mut array = content.encoded_arr();
                (*array).length = 1;
                let mut vec = Vec::with_capacity(1);
                let encoded = c::TangoDevEncoded {
                    encoded_format: cstring_from(format).into_raw(),
                    encoded_length: data.len() as u32,
                    encoded_data: Box::into_raw(data.into_boxed_slice()) as *mut u8,
                };
                vec.push(Box::into_raw(Box::new(encoded)));
                (*array).sequence = Box::into_raw(vec.into_boxed_slice()) as *mut c::TangoDevEncoded;
                TangoDataType::Encoded
            }
            AttrValue::BooleanArray(v) => impl_array!(v, Boolean, bool_arr, libc::c_uchar),
            AttrValue::UCharArray(v) => impl_array!(v, UChar, char_arr, u8),
            AttrValue::ShortArray(v) => impl_array!(v, Short, short_arr, i16),
            AttrValue::UShortArray(v) => impl_array!(v, UShort, ushort_arr, u16),
            AttrValue::LongArray(v) => impl_array!(v, Long, long_arr, i32),
            AttrValue::ULongArray(v) => impl_array!(v, ULong, ulong_arr, u32),
            AttrValue::Long64Array(v) => impl_array!(v, Long64, long64_arr, i64),
            AttrValue::ULong64Array(v) => impl_array!(v, ULong64, ulong64_arr, u64),
            AttrValue::FloatArray(v) => impl_array!(v, Float, float_arr, f32),
            AttrValue::DoubleArray(v) => impl_array!(v, Double, double_arr, f64),
            AttrValue::StateArray(v) => impl_array!(v, State, state_arr, u32),
            AttrValue::StringArray(v) => {
                let array = content.string_arr();
                let mut vec = Vec::with_capacity(v.len());
                (*array).length = v.len() as u32;
                for s in v.into_iter() {
                    vec.push(cstring_from(s).into_raw());
                }
                (*array).sequence = Box::into_raw(vec.into_boxed_slice()) as *mut *mut i8;
                TangoDataType::String
            }
            AttrValue::EncodedArray(v) => {
                let array = content.encoded_arr();
                (*array).length = v.len() as u32;
                let mut vec = Vec::with_capacity(v.len());
                for (format, data) in v.into_iter() {
                    let encoded = c::TangoDevEncoded {
                        encoded_format: cstring_from(format).into_raw(),
                        encoded_length: data.len() as u32,
                        encoded_data: Box::into_raw(data.into_boxed_slice()) as *mut u8,
                    };
                    vec.push(Box::into_raw(Box::new(encoded)));
                }
                (*array).sequence = Box::into_raw(vec.into_boxed_slice()) as *mut c::TangoDevEncoded;
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
            time_stamp: c::Struct_timeval { tv_sec: self.time_stamp.sec,
                                            tv_usec: self.time_stamp.nsec as i64 / 1000 }
        }
    }

    pub unsafe fn free_c_data(attr_data: c::AttributeData) {
        let mut data = attr_data.attr_data;
        drop(CString::from_raw(attr_data.name));
        match TangoDataType::from_c(attr_data.data_type) {
            TangoDataType::Boolean => drop(Box::from_raw((*data.bool_arr()).sequence)),
            TangoDataType::UChar => drop(Box::from_raw((*data.char_arr()).sequence)),
            TangoDataType::Short => drop(Box::from_raw((*data.short_arr()).sequence)),
            TangoDataType::UShort => drop(Box::from_raw((*data.ushort_arr()).sequence)),
            TangoDataType::Long => drop(Box::from_raw((*data.long_arr()).sequence)),
            TangoDataType::ULong => drop(Box::from_raw((*data.ulong_arr()).sequence)),
            TangoDataType::Long64 => drop(Box::from_raw((*data.long64_arr()).sequence)),
            TangoDataType::ULong64 => drop(Box::from_raw((*data.ulong64_arr()).sequence)),
            TangoDataType::Float => drop(Box::from_raw((*data.float_arr()).sequence)),
            TangoDataType::Double => drop(Box::from_raw((*data.double_arr()).sequence)),
            TangoDataType::State => drop(Box::from_raw((*data.state_arr()).sequence)),
            TangoDataType::String => {
                for i in 0..(*data.string_arr()).length {
                    drop(CString::from_raw(*(*data.string_arr()).sequence.offset(i as isize) as *mut i8));
                }
                drop(Box::from_raw((*data.string_arr()).sequence));
            }
            TangoDataType::Encoded => {
                for i in 0..(*data.encoded_arr()).length {
                    let ptr = (*data.encoded_arr()).sequence.offset(i as isize) as *mut c::TangoDevEncoded;
                    drop(CString::from_raw((*ptr).encoded_format as *mut i8));
                    drop(Box::from_raw((*ptr).encoded_data));
                }
                drop(Box::from_raw((*data.encoded_arr()).sequence));
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
        match *self {
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

            AttrValue::BooleanArray(ref v) => v.len(),
            AttrValue::UCharArray(ref v) => v.len(),
            AttrValue::ShortArray(ref v) => v.len(),
            AttrValue::UShortArray(ref v) => v.len(),
            AttrValue::LongArray(ref v) => v.len(),
            AttrValue::ULongArray(ref v) => v.len(),
            AttrValue::Long64Array(ref v) => v.len(),
            AttrValue::ULong64Array(ref v) => v.len(),
            AttrValue::FloatArray(ref v) => v.len(),
            AttrValue::DoubleArray(ref v) => v.len(),
            AttrValue::StringArray(ref v) => v.len(),
            AttrValue::StateArray(ref v) => v.len(),
            AttrValue::EncodedArray(ref v) => v.len(),
        }
    }
}
