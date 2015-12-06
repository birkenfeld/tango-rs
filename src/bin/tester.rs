extern crate tango;

fn main() {
    let mut dev = tango::DeviceProxy::new("tango://localhost:10000/test/benchmark/echo").unwrap();
    let instr = tango::CommandData::from_str("This is a minimal Tango test client.");
    let argout = dev.command_inout("Echo", instr).unwrap();
    println!("{}", argout.into_string().unwrap());
    let cmd = dev.command_query("Echo").unwrap();
    println!("Echo cmd: {:?}", cmd);
    let cmds = dev.command_list_query().unwrap();
    println!("Echo cmds: {:?}", cmds);
}
