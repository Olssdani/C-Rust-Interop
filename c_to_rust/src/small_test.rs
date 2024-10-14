use std::{env, ffi::CStr, ffi::CString, os::raw::c_char};
//Example 1
#[no_mangle]
pub unsafe extern "C" fn add(a: i32, b: i32) -> i32 {
    return a + b + 2;
}

//Example 2
#[repr(C)]
#[derive(Default)]
pub struct TestStruct {
    x: f32,
    y: f64,
    z: i32,
}

#[no_mangle]
pub unsafe extern "C" fn add_togheter(data: &TestStruct) -> f64 {
    data.x as f64 + data.y + data.z as f64
}

//Example 3
#[no_mangle]
pub unsafe extern "C" fn get_env(
    env_variable: *const c_char,
    output: *mut c_char,
    size: i32,
) -> i32 {
    if env_variable.is_null() {
        return -1;
    }

    let Ok(var) = CString::from(CStr::from_ptr(env_variable))
        .to_str()
        .and_then(|op| Ok(op.to_string()))
    else {
        return -1;
    };

    let Ok(env_res) = env::var(var) else {
        return -1;
    };

    let string_size = env_res.len() as i32;

    if output.is_null() {
        return string_size;
    }

    if size != string_size {
        return -1;
    }

    let env_res_bytes = env_res.as_bytes();
    std::ptr::copy_nonoverlapping(
        env_res_bytes.as_ptr(),
        output as *mut u8,
        string_size as usize,
    );

    return string_size;
}
