use sentinel::{log, use_sentinel};
use std::{error::Error, ffi::CStr, ffi::CString};

use libc::c_void;

struct Handle(*mut c_void);

unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}

// #[no_mangle]
// #[allow(clippy::no_safety_doc)]
// pub fn __cxa_thread_atexit_impl() {
//     sentinel::log("eschewing __cxa_thread_atexit_impl");
// }

fn main() -> Result<(), Box<dyn Error>> {
    let lib_name = std::env::args()
        .nth(1)
        .expect("Usage: run path/to/libcheer.so");
    let lib_name = CString::new(lib_name)?;

    for _ in 0..2 {
        let lib_name = lib_name.clone();
        std::thread::spawn(move || {
            round(&lib_name).unwrap();
        })
        .join()
        .unwrap();
    }

    Ok(())
}

fn round(lib_name: &CStr) -> Result<(), Box<dyn Error>> {
    assert!(!is_lib_loaded(), "lib should not be loaded yet");

    log("loading noop library");
    let noop_name = CString::new("./libnoop/target/debug/libnoop.so").unwrap();
    let mut lmid: libc::Lmid_t = 0;
    let noop_handle =
        unsafe { libc::dlmopen(libc::LM_ID_NEWLM, noop_name.as_ptr(), libc::RTLD_NOW) };
    assert!(!noop_handle.is_null());

    unsafe {
        libc::dlinfo(
            noop_handle,
            libc::RTLD_DI_LMID,
            &mut lmid as *mut i64 as *mut c_void,
        )
    };
    log(format!("lmid = {:x}", lmid));

    // let handle = unsafe { libc::dlopen(lib_name.as_ptr(), libc::RTLD_NOW) };
    log("loading cheer library");
    let handle = unsafe { libc::dlmopen(lmid, lib_name.as_ptr(), libc::RTLD_NOW) };
    let handle = Handle(handle);

    assert!(
        !handle.0.is_null(),
        "dlopen should open the library successfully"
    );
    assert!(is_lib_loaded(), "lib should be loaded by now");

    let sym_name = "cheer";
    let sym_name = CString::new(sym_name)?;
    let cheer = unsafe { libc::dlsym(handle.0, sym_name.as_ptr()) };
    assert!(!cheer.is_null(), "lib should contain 'cheer' symbol");

    let cheer: unsafe extern "C" fn() = unsafe { std::mem::transmute(cheer) };

    // std::thread::spawn(move || {
    use_sentinel("runner");

    log("thread: starting");
    unsafe {
        cheer();
    }
    unsafe {
        cheer();
    }
    log("thread: exiting");
    // })
    // .join()
    // .unwrap();

    log("library: closing");
    unsafe {
        libc::dlclose(handle.0);
    }

    assert!(!is_lib_loaded(), "lib should have unloaded by now");

    Ok(())
}

fn is_lib_loaded() -> bool {
    let path = format!("/proc/{pid}/maps", pid = std::process::id());
    std::fs::read_to_string(&path)
        .unwrap()
        .contains("libcheer.so")
}
