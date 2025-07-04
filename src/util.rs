#![allow(dead_code)]
use libvips::bindings::{self, vips_error_clear, vips_thread_shutdown};
use std::ffi::CString;

pub(crate) fn progress_set(flag: bool) {
    unsafe {
        bindings::vips_progress_set(if flag {
            1
        } else {
            0
        });
    }
}

pub(crate) fn get_disc_threshold() -> u64 {
    unsafe { bindings::vips_get_disc_threshold() }
}

// pub(crate) fn version_string() -> Result<String, String> {
//     unsafe {
//         let version = CStr::from_ptr(bindings::vips_version_string());
//         version.to_str().map_err(|_| "Error initializing string".to_string())?.to_string()
//     }
// }

pub(crate) fn thread_shutdown() {
    unsafe {
        bindings::vips_thread_shutdown();
    }
}

// pub(crate) fn error_buffer() -> Result<&str, String> {
//     unsafe {
//         let buffer = CStr::from_ptr(bindings::vips_error_buffer());
//         buffer.to_str().map_err(|_| "Error initializing string".to_string())
//     }
// }

pub(crate) fn error(domain: &str, error: &str) -> Result<(), String> {
    unsafe {
        let c_str_error = new_c_string(error)?;
        let c_str_domain = new_c_string(domain)?;
        bindings::vips_error(c_str_domain.as_ptr(), c_str_error.as_ptr());
        Ok(())
    }
}

pub(crate) fn error_system(code: i32, domain: &str, error: &str) -> Result<(), String> {
    unsafe {
        let c_str_error = new_c_string(error)?;
        let c_str_domain = new_c_string(domain)?;
        bindings::vips_error_system(code, c_str_domain.as_ptr(), c_str_error.as_ptr());
        Ok(())
    }
}

pub(crate) fn freeze_error_buffer() {
    unsafe {
        bindings::vips_error_freeze();
    }
}

pub(crate) fn error_clear() {
    unsafe {
        bindings::vips_error_clear();
    }
}

pub(crate) fn error_thaw() {
    unsafe {
        bindings::vips_error_thaw();
    }
}

pub(crate) fn error_exit(error: &str) -> Result<(), String> {
    unsafe {
        let c_str_error = new_c_string(error)?;
        bindings::vips_error_exit(c_str_error.as_ptr());
    }
}

pub(crate) fn cache_print() {
    unsafe {
        bindings::vips_cache_print();
    }
}

pub(crate) fn cache_set_max(max: i32) {
    unsafe {
        bindings::vips_cache_set_max(max);
    }
}

pub(crate) fn cache_set_max_mem(max: u64) {
    unsafe {
        bindings::vips_cache_set_max_mem(max);
    }
}

pub(crate) fn cache_get_max() -> i32 {
    unsafe { bindings::vips_cache_get_max() }
}

pub(crate) fn cache_get_max_mem() -> u64 {
    unsafe { bindings::vips_cache_get_max_mem() }
}

pub(crate) fn cache_get_size() -> i32 {
    unsafe { bindings::vips_cache_get_size() }
}

pub(crate) fn cache_set_max_files(max: i32) {
    unsafe {
        bindings::vips_cache_set_max_files(max);
    }
}

pub(crate) fn cache_get_max_files() -> i32 {
    unsafe { bindings::vips_cache_get_max_files() }
}

pub(crate) fn vips_cache_set_dump(flag: bool) {
    unsafe {
        bindings::vips_cache_set_dump(if flag {
            1
        } else {
            0
        });
    }
}

pub(crate) fn vips_cache_set_trace(flag: bool) {
    unsafe {
        bindings::vips_cache_set_trace(if flag {
            1
        } else {
            0
        });
    }
}

/// set the number of worker threads for vips to operate
pub(crate) fn concurrency_set(max: i32) {
    unsafe {
        bindings::vips_concurrency_set(max);
    }
}

/// get the number of worker threads that vips is operating
pub(crate) fn concurrency_get() -> i32 {
    unsafe { bindings::vips_concurrency_get() }
}

pub(crate) fn tracked_get_mem() -> u64 {
    unsafe { bindings::vips_tracked_get_mem() }
}

pub(crate) fn tracked_get_mem_highwater() -> u64 {
    unsafe { bindings::vips_tracked_get_mem_highwater() }
}

pub(crate) fn tracked_get_allocs() -> i32 {
    unsafe { bindings::vips_tracked_get_allocs() }
}

pub(crate) fn pipe_read_limit_set(limit: i64) {
    unsafe {
        bindings::vips_pipe_read_limit_set(limit);
    }
}

pub(crate) fn init(name: &str, detect_leak: bool) -> Result<i32, String> {
    let cstring = new_c_string(name);
    if let Ok(c_name) = cstring {
        let res = unsafe { bindings::vips_init(c_name.as_ptr()) };
        let result = if res == 0 {
            Ok(res)
        } else {
            Err("Failed to init libvips".to_string())
        };
        unsafe {
            if detect_leak {
                bindings::vips_leak_set(1);
            };
        }
        result
    } else {
        Err("Failed to convert rust string to C string".to_string())
    }
}

#[inline]
pub(crate) fn new_c_string(string: &str) -> Result<CString, String> {
    CString::new(string).map_err(|_| "Error initializing C string.".to_string())
}

pub(crate) struct VipsGuard;

impl Drop for VipsGuard {
    fn drop(&mut self) {
        unsafe {
            vips_error_clear();
            vips_thread_shutdown();
        };
    }
}
