use sentinel::use_sentinel;

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn cheer() {
    use_sentinel("libcheer");
}
