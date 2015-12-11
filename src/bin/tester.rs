extern crate tango;

use std::cmp::min;

fn main() {
    let mut dev = tango::DeviceProxy::new("tango://localhost:10000/sys/tg_test/1").unwrap();

    let instr = tango::CommandData::from_str("This is a minimal Tango test client.");
    let argout = dev.command_inout("DevString", instr).unwrap();
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

    dev.write_attribute(tango::AttributeData::simple(
        "float_scalar", tango::AttrValue::Float(42.42))).unwrap();
    println!("Attribute write ok");

    let val = dev.read_attribute("float_scalar").unwrap();
    println!("Attribute value readback: {:?}", val);
    drop(val);

    dev.write_attributes(vec![tango::AttributeData::simple(
        "float_scalar", tango::AttrValue::Float(69.69))]).unwrap();
    println!("Attribute write list ok");

    let vals = dev.read_attributes(&["float_scalar"]).unwrap();
    println!("Attribute value list readback: {:?}", vals);
    drop(vals);

    test_commands(&mut dev);
    test_attributes(&mut dev);

    drop(dev);
}

fn test_commands(dev: &mut tango::DeviceProxy) {
    use tango::CommandData::*;
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
        ("DevString", tango::CommandData::from_str("str")),
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
                                                  vec![vec![b'a', b'b']])),
        ("DevVarDoubleStringArray", DoubleStringArray(vec![-5.0, 1.0, 0.0],
                                                      vec![vec![b'a', b'b']])),
        // no test methods for: DevEncoded, DevVarBooleanArray
        ];
    for (cmd, data) in tests {
        println!("{}", cmd);
        let res = dev.command_inout(cmd, data.clone()).unwrap();
        assert_eq!(res, data);
    }
    // test special types
    println!("DevState");
    let res = dev.command_inout("State", Void).unwrap();
    assert!(res == State(tango::TangoDevState::Running) ||
            res == State(tango::TangoDevState::Fault));
    // test exceptions
    println!("errors");
    let res = dev.command_inout("NotPresent", Void).unwrap_err();
    assert_eq!(res.failures[0].reason, "API_CommandNotFound");
    let res = dev.command_inout("DevBoolean", Void).unwrap_err();
    assert_eq!(res.failures[0].reason, "API_IncompatibleCmdArgumentType");
}

fn test_attributes(dev: &mut tango::DeviceProxy) {
    use tango::AttrValue::*;
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
    dev.read_attributes(&read_tests).unwrap();
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
        ("string_scalar", String(b"0000000000000000".to_vec())),
        ("string_spectrum", StringArray(vec![vec![b'a', b'b'],
                                             vec![b'c'], vec![b'd']])),
        ];
    for (attr, data) in write_tests {
        println!("write {}", attr);
        dev.write_attribute(tango::AttributeData::simple(attr, data.clone())).unwrap();
        let res = dev.read_attribute(attr).unwrap();
        assert_eq!(res.written_data.unwrap(), data);
    }
    println!("errors");
    let res = dev.read_attribute("not_present").unwrap_err();
    assert_eq!(res.failures[0].reason, "API_AttrNotFound");
    let res = dev.write_attribute(tango::AttributeData::simple("boolean_scalar", Short(10))).unwrap_err();
    assert_eq!(res.failures[0].reason, "API_IncompatibleAttrDataType");
}
