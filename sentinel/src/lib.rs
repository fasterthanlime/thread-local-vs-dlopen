use colored::*;
use libc::c_void;
use std::cell::RefCell;

fn color(s: &str, x: u64) -> ColoredString {
    match (x >> 8) % 25 {
        0 => s.red(),
        1 => s.green(),
        2 => s.yellow(),
        3 => s.blue(),
        4 => s.magenta(),
        5 => s.purple(),
        6 => s.bright_red(),
        7 => s.bright_green(),
        8 => s.bright_yellow(),
        9 => s.bright_blue(),
        10 => s.bright_magenta(),
        11 => s.bright_purple(),
        12 => s.on_red(),
        13 => s.on_green(),
        14 => s.on_yellow(),
        15 => s.on_blue(),
        16 => s.on_magenta(),
        17 => s.on_purple(),
        18 => s.on_bright_red().black(),
        19 => s.on_bright_green().black(),
        20 => s.on_bright_yellow().black(),
        21 => s.on_bright_blue().black(),
        22 => s.on_bright_magenta().black(),
        23 => s.on_bright_purple().black(),
        _ => s.on_cyan(),
    }
}

fn write_stdout(buf: &[u8]) {
    unsafe {
        libc::write(libc::STDOUT_FILENO, buf.as_ptr() as _, buf.len());
    }
}

// `println!` replacement that does not use thread-local storage
pub fn log<S: AsRef<str>>(s: S) {
    let tid = unsafe { libc::pthread_self() };
    let prefix = format!("{:x}", tid);
    let prefix = color(&prefix, tid);
    let prefix = format!("{}", prefix);
    write_stdout(prefix.as_bytes());
    write_stdout(b" ");
    write_stdout(s.as_ref().as_bytes());
    write_stdout(b"\n");
}
struct Sentinel(*mut c_void);

impl Sentinel {
    fn new() -> Self {
        let res = Self(unsafe { libc::malloc(8) });
        log(format!("Sentinel::new() -> {:?}", res.0));
        res
    }
}

impl Drop for Sentinel {
    fn drop(&mut self) {
        log(format!("Sentinel::drop({:?})", self.0));
        unsafe { libc::free(self.0) }
    }
}

thread_local! {
    static SENTINEL: RefCell<Option<Sentinel>> = {
        RefCell::new(None)
    }
}

pub fn use_sentinel(dso: &str) {
    log("==============================");
    log(format!("{}: use_sentinel start", dso));

    SENTINEL.with(|s| {
        let mut s = s.borrow_mut();
        if s.is_none() {
            log("sentinel: initializing");
            *s = Some(Sentinel::new());
        } else {
            log("sentinel: already initialized");
        }
    });

    log(format!("{}: use_sentinel stop", dso));
    log("==============================");
}
