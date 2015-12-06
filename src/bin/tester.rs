extern crate tango;

use std::cmp::min;

fn main() {
    let mut dev = tango::DeviceProxy::new("tango://localhost:10000/sys/tg_test/1").unwrap();
    let instr = tango::CommandData::from_str("This is a minimal Tango test client.");
    let argout = dev.command_inout("DevString", instr).unwrap();
    println!("Command exec result: {}", argout.into_string().unwrap());
    let cmd = dev.command_query("DevString").unwrap();
    println!("Command query: {:?}", cmd);
    let cmds = dev.command_list_query().unwrap();
    println!("Command list: {:?} commands", cmds.len());
    let attrs = dev.get_attribute_list().unwrap();
    println!("Attribute name list: {:?} etc.", &attrs[..min(3, attrs.len())]);
    let aconfig = dev.get_attribute_config(&["State", "Status"]).unwrap();
    println!("Attribute config: {:?}", aconfig);
    let aconfig = dev.attribute_list_query().unwrap();
    println!("Attribute config list: {:?} attrs", aconfig.len());
    dev.write_attribute(tango::AttributeData::simple(
        "float_scalar", tango::AttrValue::Float(42.42))).unwrap();
    println!("Attribute write ok");
    let val = dev.read_attribute("float_scalar").unwrap();
    println!("Attribute value readback: {:?}", val);
    dev.write_attributes(vec![tango::AttributeData::simple(
        "float_scalar", tango::AttrValue::Float(69.69))]).unwrap();
    println!("Attribute write list ok");
    let vals = dev.read_attributes(&["float_scalar"]).unwrap();
    println!("Attribute value list readback: {:?}", vals);
}
