extern crate gcc;
extern crate pkg_config;

use std::process;

fn main() {
    let tango_lib = match pkg_config::probe_library("tango") {
        Ok(lib) => lib,
        Err(err) => { print!("{}---", err); process::exit(1); }
    };
    let mut config = gcc::Build::new();
    config.cpp(true);
    config.flag("-std=c++0x");
    config.file("src/c_tango_proxy.c");
    config.file("src/c_tango_command.c");
    config.file("src/c_tango_attribute.c");
    config.file("src/c_tango_dbase.c");
    config.include("src");
    for path in tango_lib.include_paths {
        config.include(path);
    }
    config.compile("libc_tango.a");
}
