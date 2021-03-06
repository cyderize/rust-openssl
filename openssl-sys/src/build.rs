extern crate "pkg-config" as pkg_config;

use std::os;

fn main() {
	// pkg-config doesn't support Win64, so do this instead:
	if cfg!(all(target_os = "windows", target_word_size = "64")) {
		let flags = "-l crypto -l ssl -l gdi32 -l wsock32".to_string();
		println!("cargo:rustc-flags={}", flags);
		return;
	}
	
    // Without hackory, pkg-config will only look for host libraries.
    // So, abandon ship if we're cross compiling.
    if !pkg_config::target_supported() { return; }


    if pkg_config::find_library("openssl").is_err() {
        let mut flags = " -l crypto -l ssl".to_string();

        let target = os::getenv("TARGET").unwrap();

        let win_pos = target.find_str("windows")
                            .or(target.find_str("win32"))
                            .or(target.find_str("win64"));

        // It's fun, but it looks like win32 and win64 both
        // have all the libs with 32 sufix
        if win_pos.is_some() {
           flags.push_str(" -l gdi32 -l wsock32");
        }

        if target.find_str("android").is_some() {
            let path = os::getenv("OPENSSL_PATH").expect("Android does not provide openssl libraries, please \
                                                          build them yourselves (instructions in the README) \
                                                          and provide their location through $OPENSSL_PATH.");
            flags.push_str(format!(" -L {}", path).as_slice());
        }

        println!("cargo:rustc-flags={}", flags);
    }
}
