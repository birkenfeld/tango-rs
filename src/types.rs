use std::ffi::{CStr, CString};
use std::mem;
use std::slice;

use libc;
use time::{get_time, Timespec};
use c_tango as c;


pub unsafe fn string_from(ptr: *const i8) -> String {
    CStr::from_ptr(ptr).to_string_lossy().into_owned()
}

#[allow(non_camel_case_types)]
type voidptr = *mut libc::c_void;


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


#[derive(Debug)]
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

    State(TangoDevState),
    Encoded(DevEncoded),
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
            // XXX: for all arrays this copies the data, instead of reusing the
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
            CommandData::String(v) => {
                let cstr = CString::new(v).unwrap();  // XXX not null-free
                *content.string_val() = cstr.into_raw();
                TangoDataType::String
            }
            _ => unimplemented!()  // XXX Array types, Encoded, etc.
        };
        c::CommandData { arg_type: tag as u32, cmd_data: content }
    }

    pub unsafe fn free_c_data(mut data: c::CommandData) {
        match TangoDataType::from_c(data.arg_type) {
            TangoDataType::Void |
            TangoDataType::Boolean |
            TangoDataType::Short |
            TangoDataType::Long |
            TangoDataType::Float |
            TangoDataType::Double |
            TangoDataType::UShort |
            TangoDataType::ULong => {}
            TangoDataType::String => {
                drop(CString::from_raw(*data.cmd_data.string_val()));
            }
            _ => unimplemented!()  // XXX Array types, Encoded, etc.
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


#[derive(Debug)]
pub struct AttributeData {
    pub data: AttrValue,
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
            data: data,
            format: AttrDataFormat::Scalar,
            quality: AttrQuality::Valid,
            name: name.into(),
            dim_x: 1,
            dim_y: 0,
            time_stamp: get_time(),
        }
    }

    pub unsafe fn from_c(mut attr_data: c::AttributeData, free: bool) -> AttributeData {
        let tag = TangoDataType::from_c(attr_data.data_type);
        let mut data = attr_data.attr_data;
        let res = if AttrDataFormat::from_c(attr_data.data_format) == AttrDataFormat::Scalar {
            match tag {
                TangoDataType::Boolean => AttrValue::Boolean(*(*data.bool_arr()).sequence != 0),
                TangoDataType::UChar => AttrValue::Char(*(*data.char_arr()).sequence),
                TangoDataType::Short => AttrValue::Short(*(*data.short_arr()).sequence),
                TangoDataType::UShort => AttrValue::UShort(*(*data.ushort_arr()).sequence),
                TangoDataType::Long => AttrValue::Long(*(*data.long_arr()).sequence),
                TangoDataType::ULong => AttrValue::ULong(*(*data.ulong_arr()).sequence),
                TangoDataType::Long64 => AttrValue::Long64(*(*data.long64_arr()).sequence),
                TangoDataType::ULong64 => AttrValue::ULong64(*(*data.ulong64_arr()).sequence),
                TangoDataType::Float => AttrValue::Float(*(*data.float_arr()).sequence),
                TangoDataType::Double => AttrValue::Double(*(*data.double_arr()).sequence),
                TangoDataType::State =>
                    AttrValue::State(TangoDevState::from_c(*(*data.state_arr()).sequence)),
                TangoDataType::String => {
                    let raw = *(*data.string_arr()).sequence;
                    let len = libc::strlen(raw);
                    AttrValue::String(Vec::from(slice::from_raw_parts(raw as *mut u8, len)))
                },
                TangoDataType::Encoded => {
                    let raw = *(*data.encoded_arr()).sequence;
                    AttrValue::Encoded(
                        (string_from(raw.encoded_format),
                         Vec::from(slice::from_raw_parts(raw.encoded_data as *mut u8,
                                                         raw.encoded_length as usize))))
                },
                _ => panic!("data type {:?} not allowed for attributes", tag)
            }
        } else {
            macro_rules! impl_simple {
                ($alt:ident, $arr:ident) => {
                    AttrValue::$alt({
                        let ptr = *data.$arr();
                        Vec::from(slice::from_raw_parts(ptr.sequence, ptr.length as usize))
                    })
                }
            }

            match tag {
                TangoDataType::Boolean => AttrValue::BooleanArray({
                    let ptr = *data.bool_arr();
                    slice::from_raw_parts(ptr.sequence, ptr.length as usize)
                        .iter().map(|&v| v != 0).collect()
                }),
                TangoDataType::UChar => impl_simple!(CharArray, char_arr),
                TangoDataType::Short => impl_simple!(ShortArray, short_arr),
                TangoDataType::UShort => impl_simple!(UShortArray, ushort_arr),
                TangoDataType::Long => impl_simple!(LongArray, long_arr),
                TangoDataType::ULong => impl_simple!(ULongArray, ulong_arr),
                TangoDataType::Long64 => impl_simple!(Long64Array, long64_arr),
                TangoDataType::ULong64 => impl_simple!(ULong64Array, ulong64_arr),
                TangoDataType::Float => impl_simple!(FloatArray, float_arr),
                TangoDataType::Double => impl_simple!(DoubleArray, double_arr),
                TangoDataType::State => AttrValue::StateArray({
                    let ptr = *data.state_arr();
                    slice::from_raw_parts(ptr.sequence, ptr.length as usize)
                        .iter().map(|&v| TangoDevState::from_c(v)).collect()
                }),
                TangoDataType::String => AttrValue::StringArray({
                    let ptr = *data.string_arr();
                    let mut res = Vec::with_capacity(ptr.length as usize);
                    for i in 0..ptr.length {
                        let raw = *ptr.sequence.offset(i as isize);
                        let len = libc::strlen(raw);
                        res.push(Vec::from(slice::from_raw_parts(raw as *mut u8, len)));
                    }
                    res
                }),
                TangoDataType::Encoded => AttrValue::EncodedArray({
                    let ptr = *data.encoded_arr();
                    let mut res = Vec::with_capacity(ptr.length as usize);
                    for i in 0..ptr.length {
                        let raw = *ptr.sequence.offset(i as isize);
                        res.push((string_from(raw.encoded_format),
                                  Vec::from(slice::from_raw_parts(raw.encoded_data as *mut u8,
                                                                  raw.encoded_length as usize))));
                    }
                    res
                }),
                _ => panic!("data type {:?} not allowed for attributes", tag)
            }
        };
        let res = AttributeData {
            data: res,
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
                    (*array).sequence = libc::malloc(mem::size_of::<$ctype>()) as *mut $ctype;
                    *(*array).sequence = $val as $ctype;
                    TangoDataType::$alt
                }
            }
        }

        let tag = match self.data {
            AttrValue::Boolean(v) => impl_simple!(v, Boolean, bool_arr, libc::c_uchar),
            AttrValue::Char(v) => impl_simple!(v, UChar, char_arr, u8),
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
                (*array).sequence = libc::malloc(mem::size_of::<*mut i8>()) as *mut *mut i8;
                *(*array).sequence = CString::new(v).unwrap().into_raw();  // XXX not null-free
                TangoDataType::String
            }
            AttrValue::Encoded((format, data)) => {
                let mut array = content.encoded_arr();
                (*array).length = 1;
                (*array).sequence = libc::malloc(mem::size_of::<c::TangoDevEncoded>()) as *mut c::TangoDevEncoded;
                (*(*array).sequence).encoded_format = CString::new(format).unwrap().into_raw();
                (*(*array).sequence).encoded_length = data.len() as u32;
                (*(*array).sequence).encoded_data = Box::into_raw(data.into_boxed_slice()) as *mut u8;
                TangoDataType::Encoded
            }
            _ => unimplemented!()  // XXX: implement array values
        };
        c::AttributeData {
            name: CString::new(self.name).unwrap().into_raw(),
            data_type: tag as u32,
            data_format: self.format as u32,
            attr_data: content,
            quality: self.quality as u32,
            dim_x: self.dim_x as i32,
            dim_y: self.dim_y as i32,
            time_stamp: c::Struct_timeval { tv_sec: self.time_stamp.sec,
                                            tv_usec: self.time_stamp.nsec as i64 / 1000 }
        }
    }

    pub unsafe fn free_c_data(mut data: c::AttributeData) {
        match TangoDataType::from_c(data.data_type) {
            TangoDataType::Boolean => libc::free((*data.attr_data.bool_arr()).sequence as voidptr),
            TangoDataType::UChar => libc::free((*data.attr_data.char_arr()).sequence as voidptr),
            TangoDataType::Short => libc::free((*data.attr_data.short_arr()).sequence as voidptr),
            TangoDataType::UShort => libc::free((*data.attr_data.ushort_arr()).sequence as voidptr),
            TangoDataType::Long => libc::free((*data.attr_data.long_arr()).sequence as voidptr),
            TangoDataType::ULong => libc::free((*data.attr_data.ulong_arr()).sequence as voidptr),
            TangoDataType::Long64 => libc::free((*data.attr_data.long64_arr()).sequence as voidptr),
            TangoDataType::ULong64 => libc::free((*data.attr_data.ulong64_arr()).sequence as voidptr),
            TangoDataType::Float => libc::free((*data.attr_data.float_arr()).sequence as voidptr),
            TangoDataType::Double => libc::free((*data.attr_data.double_arr()).sequence as voidptr),
            TangoDataType::State => libc::free((*data.attr_data.state_arr()).sequence as voidptr),
            TangoDataType::String => {
                drop(CString::from_raw((*data.attr_data.string_arr()).sequence as *mut i8));
                libc::free((*data.attr_data.string_arr()).sequence as voidptr);
            }
            TangoDataType::Encoded => {
                let ptr = (*data.attr_data.encoded_arr()).sequence as *mut c::TangoDevEncoded;
                drop(CString::from_raw((*ptr).encoded_format as *mut i8));
                drop(Box::from_raw((*ptr).encoded_data));
                libc::free(ptr as voidptr);
            }
            val => panic!("invalid attribute data tag={:?}", val)
        }
    }
}


#[derive(Debug)]
pub enum AttrValue {
    Boolean(bool),
    Char(u8),
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
    CharArray(Vec<u8>),
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
