use std::{
    cell::RefCell,
    io::{stdout, Write},
};

// `println!` replacement that does not use thread-local storage
pub fn log<S: AsRef<str>>(s: S) {
    stdout()
        .write_all(format!("[{:x}] ", unsafe { libc::pthread_self() }).as_bytes())
        .unwrap();
    stdout().write_all(s.as_ref().as_bytes()).unwrap();
    stdout().write_all(b"\n").unwrap();
}
struct Sentinel;

impl Sentinel {
    fn new() -> Self {
        log("Sentinel::new");
        Self
    }
}

impl Drop for Sentinel {
    fn drop(&mut self) {
        log("Sentinel::drop");
    }
}

thread_local! {
    static SENTINEL: RefCell<Option<Sentinel>> = {
        RefCell::new(None)
    }
}

pub fn use_sentinel() {
    SENTINEL.with(|s| {
        log("SENTINEL.with");
        let mut s = s.borrow_mut();
        if s.is_none() {
            log("SENTINEL is none");
            *s = Some(Sentinel::new())
        } else {
            log("SENTINEL is some");
        }
    })
}
