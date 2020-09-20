use sentinel::{log, use_sentinel};

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn cheer() {
    log("libcheer::cheer");
    use_sentinel();
}
