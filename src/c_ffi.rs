use super::PiBenchData;
use libc::c_char;
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn text_to_json(text: *const c_char) -> *mut c_char {
    let c_str = unsafe {
        assert!(!text.is_null());
        CStr::from_ptr(text).to_str().unwrap()
    };
    let rv = PiBenchData::from_text(c_str).unwrap().to_json();
    let c_str_rv = CString::new(rv).unwrap();
    c_str_rv.into_raw()
}

#[no_mangle]
pub extern "C" fn free_json_str(val: *mut c_char) {
    unsafe {
        if val.is_null() {
            return;
        }
        CString::from_raw(val);
    }
}
