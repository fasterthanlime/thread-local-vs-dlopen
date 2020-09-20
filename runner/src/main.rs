use sentinel::{log, use_sentinel};
use std::{error::Error, ffi::CString};

use libc::c_void;

struct Handle(*mut c_void);

unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}

fn main() -> Result<(), Box<dyn Error>> {
    let lib_name = std::env::args()
        .nth(1)
        .expect("Usage: run path/to/libcheer.so");
    log(format!("loading library {:?}", lib_name));
    let lib_name = CString::new(lib_name)?;

    assert!(!is_lib_loaded(), "lib should not be loaded yet");

    let handle = unsafe { libc::dlopen(lib_name.as_ptr(), libc::RTLD_NOW) };
    let handle = Handle(handle);

    assert!(
        !handle.0.is_null(),
        "dlopen should open the library successfully"
    );
    assert!(is_lib_loaded(), "lib should be loaded by now");

    let sym_name = "cheer";
    log(format!("looking up symbol {:?}", sym_name));
    let sym_name = CString::new(sym_name)?;
    let cheer = unsafe { libc::dlsym(handle.0, sym_name.as_ptr()) };
    assert!(!cheer.is_null(), "lib should contain 'cheer' symbol");

    let cheer: unsafe extern "C" fn() = unsafe { std::mem::transmute(cheer) };
    log("calling cheer");

    std::thread::spawn(move || {
        use_sentinel();

        unsafe {
            cheer();
        }

        log("closing library");
        unsafe {
            libc::dlclose(handle.0);
        }
        assert!(!is_lib_loaded(), "lib should have unloaded by now");
    })
    .join()
    .unwrap();

    Ok(())
}

fn is_lib_loaded() -> bool {
    let path = format!("/proc/{pid}/maps", pid = std::process::id());
    std::fs::read_to_string(&path)
        .unwrap()
        .contains("libcheer.so")
}
