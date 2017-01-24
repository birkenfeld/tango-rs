extern crate tango;

use tango::*;

#[test]
fn proxy_api() {
    use std::cmp::min;

    let mut dev = DeviceProxy::new("tango://localhost:10000/sys/tg_test/1")
        .expect("Could not proxy to sys/tg_test/1, is a database running on localhost?");

    let instr = CommandData::from_str("This is a minimal Tango test client.");
    let argout = dev.command_inout("DevString", instr)
        .expect("Could not execute command on sys/tg_test/1, is the TangoTest server running?");
    println!("Command exec result: {}", argout.into_string().unwrap());

    let cmd = dev.command_query("DevString").unwrap();
    println!("Command query: {:?}", cmd);
    drop(cmd);

    let cmds = dev.command_list_query().unwrap();
    println!("Command list: {:?} commands", cmds.len());
    drop(cmds);

    let attrs = dev.get_attribute_list().unwrap();
    println!("Attribute name list: {:?} etc.", &attrs[..min(3, attrs.len())]);
    drop(attrs);

    let aconfig = dev.get_attribute_config(&["State", "Status"]).unwrap();
    println!("Attribute config: {:?}", aconfig[0]);
    drop(aconfig);

    let aconfigs = dev.attribute_list_query().unwrap();
    println!("Attribute config list: {:?} attrs", aconfigs.len());
    drop(aconfigs);

    dev.write_attribute(AttributeData::simple(
        "float_scalar", AttrValue::Float(42.42))).unwrap();
    println!("Attribute write ok");

    let val = dev.read_attribute("float_scalar").unwrap();
    println!("Attribute value readback: {:?}", val);
    drop(val);

    dev.write_attributes(vec![AttributeData::simple(
        "float_scalar", AttrValue::Float(69.69))]).unwrap();
    println!("Attribute write list ok");

    let vals = dev.read_attributes(&["float_scalar"]).unwrap();
    println!("Attribute value list readback: {:?}", vals);
    drop(vals);

}

#[test]
fn proxy_commands() {
    use tango::CommandData::*;

    let mut dev = DeviceProxy::new("tango://localhost:10000/sys/tg_test/1")
        .expect("Could not proxy to sys/tg_test/1, is a database running on localhost?");

    // test all types
    println!("\nTesting commands for all data types:");
    let tests = vec![
        ("DevVoid", Void),
        ("DevBoolean", Boolean(true)),
        ("DevShort", Short(-147)),
        ("DevLong", Long(-(1 << 20))),
        ("DevFloat", Float(42.42)),
        ("DevDouble", Double(123.456790123752)),
        ("DevUShort", UShort(137)),
        ("DevULong", ULong(1 << 20)),
        ("DevLong64", Long64(-(1 << 60))),
        ("DevULong64", ULong64(1 << 60)),
        ("DevString", CommandData::from_str("some_str_ing")),
        ("DevVarCharArray", CharArray(vec![1, 5, 7])),
        ("DevVarShortArray", ShortArray(vec![-5, 1, 0])),
        ("DevVarUShortArray", UShortArray(vec![5, 1, 0])),
        ("DevVarLongArray", LongArray(vec![-(1 << 20), 1, 0])),
        ("DevVarULongArray", ULongArray(vec![1 << 30, 1, 0])),
        ("DevVarLong64Array", Long64Array(vec![-(1 << 60), 1, 0])),
        ("DevVarULong64Array", ULong64Array(vec![1 << 60, 1, 0])),
        ("DevVarFloatArray", FloatArray(vec![-42.4, 0.0, 80.123])),
        ("DevVarDoubleArray", DoubleArray(vec![-5.0, 1.0, 0.0])),
        ("DevVarStringArray", StringArray(vec![vec![b'a', b'b'],
                                               vec![b'c'], vec![b'd']])),
        ("DevVarLongStringArray", LongStringArray(vec![-5, 1, 0, 1],
                                                  vec![vec![b'a', b'b'], vec![b'c']])),
        ("DevVarDoubleStringArray", DoubleStringArray(vec![-5.0, 1.0, 0.0],
                                                     vec![vec![b'a', b'b'], vec![b'c']])),
        // no test methods for: DevEncoded, DevVarBooleanArray
        ];
    for (cmd, data) in tests {
        println!("{}", cmd);
        let res = dev.command_inout(cmd, data.clone())
                     .expect("Could not execute command on sys/tg_test/1, is \
                              the TangoTest server running?");
        assert_eq!(res, data);
    }
    // test special types
    println!("DevState");
    let res = dev.command_inout("State", Void).unwrap();
    assert!(res == State(TangoDevState::Running) ||
            res == State(TangoDevState::Fault));
    // test exceptions
    println!("errors");
    let res = dev.command_inout("NotPresent", Void).unwrap_err();
    assert_eq!(res.failures[0].reason, "API_CommandNotFound");
    let res = dev.command_inout("DevBoolean", Void).unwrap_err();
    assert_eq!(res.failures[0].reason, "API_IncompatibleCmdArgumentType");
}

#[test]
fn proxy_attributes() {
    use tango::AttrValue::*;

    let mut dev = DeviceProxy::new("tango://localhost:10000/sys/tg_test/1")
        .expect("Could not proxy to sys/tg_test/1, is a database running on localhost?");

    // test all attributes
    println!("\nTesting attributes for all data types:");
    let read_tests = vec![
        "boolean_scalar",
        "boolean_spectrum",
        "uchar_scalar",
        "uchar_spectrum",
        "short_scalar",
        "short_spectrum",
        "ushort_scalar",
        "ushort_spectrum",
        "long_scalar",
        "long_spectrum",
        "ulong_scalar",
        "ulong_spectrum_ro",
        "long64_scalar",
        "long64_spectrum_ro",
        "ulong64_scalar",
        "ulong64_spectrum_ro",
        "float_scalar",
        "float_spectrum",
        "double_scalar",
        "double_spectrum",
        "string_scalar",
        "string_spectrum",
        ];
    println!("read all");
    dev.read_attributes(&read_tests)
        .expect("Could not read attrs on sys/tg_test/1, is the TangoTest server running?");
    let write_tests = vec![
        ("boolean_scalar", Boolean(false)),
        ("boolean_spectrum", BooleanArray(vec![true, false, false])),
        ("uchar_scalar", UChar(152)),
        ("uchar_spectrum", UCharArray(vec![111, 152, 255])),
        ("short_scalar", Short(-1000)),
        ("short_spectrum", ShortArray(vec![-1000, 1000])),
        ("ushort_scalar", UShort(10000)),
        ("ushort_spectrum", UShortArray(vec![10000, 20000])),
        ("long_scalar", Long(1 << 30)),
        ("long_spectrum", LongArray(vec![1, 2])),
        ("ulong_scalar", ULong(1 << 31)),
        ("long64_scalar", Long64(1 << 32)),
        ("ulong64_scalar", ULong64(1 << 62)),
        ("float_scalar", Float(42.42)),
        ("float_spectrum", FloatArray(vec![1.0, 2.0, 3.0])),
        ("double_scalar", Double(42.424242)),
        ("double_spectrum", DoubleArray(vec![4.0, 5.0, 6.0])),
        ("string_scalar", String(b"0102040800000000".to_vec())),
        ("string_spectrum", StringArray(vec![vec![b'a', b'b'],
                                             vec![b'c'], vec![b'd']])),
        ];
    for (attr, data) in write_tests {
        println!("write {}", attr);
        dev.write_attribute(AttributeData::simple(attr, data.clone())).unwrap();
        let res = dev.read_attribute(attr).unwrap();
        assert_eq!(res.written_data.unwrap(), data);
    }
    println!("errors");
    let res = dev.read_attribute("not_present").unwrap_err();
    assert_eq!(res.failures[0].reason, "API_AttrNotFound");
    let res = dev.write_attribute(AttributeData::simple("boolean_scalar", Short(10))).unwrap_err();
    assert_eq!(res.failures[0].reason, "API_IncompatibleAttrDataType");
}

#[test]
fn proxy_properties() {
    use tango::PropertyValue::*;
    use tango::TangoDataType;

    let mut dev = DeviceProxy::new("tango://localhost:10000/sys/tg_test/1")
        .expect("Could not proxy to sys/tg_test/1, is a database running on localhost?");

    println!("\nTesting properties for all data types:");
    let tests = vec![
        ("Boolean", Boolean(true), TangoDataType::Boolean),
        ("Short", Short(-147), TangoDataType::Short),
        ("Long", Long(-(1 << 20)), TangoDataType::Long),
        ("Float", Float(42.42), TangoDataType::Float),
        ("Double", Double(123.456790123752), TangoDataType::Double),
        ("UShort", UShort(137), TangoDataType::UShort),
        ("ULong", ULong(1 << 20), TangoDataType::ULong),
        ("Long64", Long64(-(1 << 60)), TangoDataType::Long64),
        ("ULong64", ULong64(1 << 60), TangoDataType::ULong64),
        ("String", String(b"some_str_ing".to_vec()), TangoDataType::String),
        ("ShortArray", ShortArray(vec![-5, 1, 0]), TangoDataType::ShortArray),
        ("UShortArray", UShortArray(vec![5, 1, 0]), TangoDataType::UShortArray),
        ("LongArray", LongArray(vec![-(1 << 20), 1, 0]), TangoDataType::LongArray),
        ("ULongArray", ULongArray(vec![1 << 30, 1, 0]), TangoDataType::ULongArray),
        ("Long64Array", Long64Array(vec![-(1 << 60), 1, 0]), TangoDataType::Long64Array),
        ("ULong64Array", ULong64Array(vec![1 << 60, 1, 0]), TangoDataType::ULong64Array),
        ("FloatArray", FloatArray(vec![-42.4, 0.0, 80.123]), TangoDataType::FloatArray),
        ("DoubleArray", DoubleArray(vec![-5.0, 1.0, 0.0]), TangoDataType::DoubleArray),
        ("StringArray", StringArray(vec![vec![b'a', b'b'],
                                         vec![b'c'], vec![b'd']]), TangoDataType::StringArray),
    ];
    for (prop, data, typ) in tests {
        println!("{}", prop);
        let put_prop = DbDatum::new(prop, data);
        dev.put_device_property(vec![put_prop.clone()]).unwrap();
        let req_prop = DbDatum::for_request(prop, typ);
        let res = dev.get_device_property(vec![req_prop]).unwrap();
        assert_eq!(res[0], put_prop);
    }
}

#[test]
fn database_api() {
    println!("\nTesting database proxy:");
    let mut db = DatabaseProxy::new()
        .expect("Could not get database proxy, is a database running on localhost?");

    let exported = db.get_device_exported("*").unwrap();
    println!("get_device_exported: {} devices", exported.data.len());
    drop(exported);

    let exported = db.get_device_exported_for_class("TangoTest").unwrap();
    println!("get_device_exported_for_class TangoTest: {} devices", exported.data.len());
    drop(exported);

    let prop = DbDatum::new("prop", PropertyValue::Long64(1));
    db.put_property("test_obj", vec![prop.clone()]).unwrap();
    println!("put_property");

    let obj_list = db.get_object_list("*").unwrap();
    println!("get_object_list: {:?} objects", obj_list.data.len());
    drop(obj_list);

    let req = DbDatum::for_request("prop", TangoDataType::Long64);
    let prop_return = db.get_property("test_obj", vec![req]).unwrap();
    println!("get_property");
    assert_eq!(prop_return, vec![prop]);
    drop(prop_return);

    db.delete_property("test_obj", &["prop"]).unwrap();
    println!("delete_property");

    let obj_prop_list = db.get_object_property_list("test_obj", "*").unwrap();
    println!("get_object_property_list: {:?} properties", obj_prop_list.data.len());
    assert_eq!(obj_prop_list.data.len(), 0);
    drop(obj_prop_list);
}
