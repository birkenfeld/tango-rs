extern crate gcc;

use std::env;

fn main() {
    let tango_root = env::var("TANGO_ROOT").expect("Set TANGO_ROOT to the directory \
                                                    containing Tango includes and libs!");
    let include_1 = tango_root.clone() + "/include";
    let include_2 = tango_root.clone() + "/include/tango";
    let libdir = tango_root + "/lib";
    gcc::Config::new()
        .cpp(true)
        .file("src/c_tango/c_tango_const.c")
        .file("src/c_tango/c_tango_proxy.c")
        .file("src/c_tango/c_tango_command.c")
        .file("src/c_tango/c_tango_attribute.c")
        .file("src/c_tango/c_tango_dbase.c")
        .include("src/c_tango")
        .include(include_1)
        .include(include_2)
        .flag("-std=c++0x")
        .compile("libc_tango.a");
    println!("cargo:rustc-link-search=native={}", libdir);
}
