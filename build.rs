extern crate cc;

use std::io::Write;

fn main() {
    if cfg!(any(target_os = "linux", target_os = "android")) {
        println!("cargo:rustc-link-lib=dl");
    } else if cfg!(any(target_os = "freebsd", target_os = "dragonfly")) {
        println!("cargo:rustc-link-lib=c")
    } else if cfg!(any(
        target_os = "openbsd",
        target_os = "bitrig",
        target_os = "netbsd",
        target_os = "macos",
        target_os = "ios"
    )) {
        // netbsd claims dl* will be available to any dynamically linked binary, but I haven’t
        // found any libraries that have to be linked to on other platforms.
        // What happens if the executable is not linked up dynamically?
    } else if cfg!(any(target_os = "solaris", target_os = "haiku")) {
    } else if cfg!(target_os = "windows") {
    } else {
        writeln!(::std::io::stderr(), "Building for an unknown target_os.\n")
            .expect("could not report the error");
        ::std::process::exit(0xfc);
    }

    if cfg!(unix) {
        cc::Build::new()
            .file("src/os/unix/global_static.c")
            .compile("libloading.a");
    }
}
