extern crate napi_sys;

use std::{ffi::CString, ptr};

use napi_sys::*;

#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(
    env: *const napi_env,
    exports: *mut napi_value,
) -> *mut napi_value {
    let k = CString::new("foo").unwrap();
    let v = CString::new("bar").unwrap();
    let mut str = ptr::null_mut();
    assert!(napi_create_string_utf8(env, v.as_ptr(), 3, &mut str) == 0);
    assert!(napi_set_named_property(env, exports, k.as_ptr(), str) == 0);
    exports
}
