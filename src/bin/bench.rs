extern crate tango;

fn main() {
    let mut dev = tango::DeviceProxy::new("tango://localhost:10000/test/benchmark/echo").unwrap();
    for _ in 0..2000 {
        let instr = tango::CommandData::from_str("This is a minimal Tango test client.");
        let argout = dev.command_inout("Echo", instr).unwrap();
        println!("{}", argout.into_string().unwrap());
    }
}
