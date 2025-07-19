use crate::{
    common::{determine_image_type, determine_image_type_from_str, image_type_id},
    Sharp,
};
use libvips::bindings::{
    vips_cache_get_max, vips_cache_get_max_files, vips_cache_get_max_mem, vips_cache_get_size, vips_cache_set_max, vips_cache_set_max_files, vips_cache_set_max_mem, vips_concurrency_get,
    vips_concurrency_set, vips_error_clear, vips_init, vips_leak_set, vips_thread_shutdown, vips_tracked_get_files, vips_tracked_get_mem, vips_tracked_get_mem_highwater,
};
use std::ffi::CString;

#[derive(Debug, Clone, Default)]
pub struct Memory {
    pub current: u64,
    pub high: u64,
    pub max: u64,
}

#[derive(Debug, Clone, Default)]
pub struct Files {
    pub current: i32,
    pub max: i32,
}

#[derive(Debug, Clone, Default)]
pub struct Items {
    pub current: i32,
    pub max: i32,
}

#[derive(Debug, Clone, Default)]
pub struct CacheResult {
    pub memory: Memory,
    pub files: Files,
    pub items: Items,
}

impl Sharp {
    /*
     * Get file type.
     */
    pub fn get_file_type(&self) -> String {
        if !self.options.input.file.is_empty() {
            let image_type = determine_image_type_from_str(&self.options.input.file);
            return image_type_id(image_type);
        }

        if !self.options.input.buffer.is_empty() {
            let image_type = determine_image_type(&self.options.input.buffer);
            return image_type_id(image_type);
        }

        String::new()
    }

    /**
     * Gets or, when options are provided, sets the limits of _libvips'_ operation cache.
     * Existing entries in the cache will be trimmed after any change in limits.
     * This method always returns cache statistics,
     * useful for determining how much working memory is required for a particular task.
     *
     * @example
     * const stats = sharp.cache();
     * @example
     * sharp.cache(false);
     *
     * @param {Object|boolean} [options=true] - Object with the following attributes, or boolean where true uses default cache settings and false removes all caching
     * @returns {Object}
     */
    pub fn cache(cache: bool) -> CacheResult {
        if cache {
            Self::set_cache(50, 20, 100)
        } else {
            Self::set_cache(0, 0, 0)
        }
    }

    /**
     * Gets or, when options are provided, sets the limits of _libvips'_ operation cache.
     * Existing entries in the cache will be trimmed after any change in limits.
     * This method always returns cache statistics,
     * useful for determining how much working memory is required for a particular task.
     *
     * @example
     * const stats = sharp.cache();
     * @example
     * sharp.cache( { items: 200 } );
     * sharp.cache( { files: 0 } );
     *
     * @param {number} [options.memory=50] - is the maximum memory in MB to use for this cache
     * @param {number} [options.files=20] - is the maximum number of files to hold open
     * @param {number} [options.items=100] - is the maximum number of operations to cache
     * @returns {Object}
     */
    pub fn set_cache(memory: u64, files: i32, items: i32) -> CacheResult {
        unsafe {
            // Set memory limit
            vips_cache_set_max_mem(memory);
            // Set file limit
            vips_cache_set_max_files(files);
            // Set items limit
            vips_cache_set_max(items);

            let mut result = CacheResult::default();
            // Get memory stats
            result.memory.current = vips_tracked_get_mem() / 1048576;
            result.memory.high = vips_tracked_get_mem_highwater() / 1048576;
            result.memory.max = vips_cache_get_max_mem() / 1048576;

            // Get file stats
            result.files.current = vips_tracked_get_files();
            result.files.max = vips_cache_get_max_files();

            // Get item stats
            result.items.current = vips_cache_get_size();
            result.items.max = vips_cache_get_max();

            result
        }
    }

    /*
     * Set size of thread pool
     */
    pub fn set_concurrency(max: i32) {
        unsafe { vips_concurrency_set(max) };
    }

    /*
     * Get size of thread pool
     */
    pub fn get_concurrency() -> i32 {
        unsafe { vips_concurrency_get() }
    }
}

pub(crate) fn init(name: &str, detect_leak: bool) -> Result<i32, String> {
    let cstring = new_c_string(name);
    if let Ok(c_name) = cstring {
        let res = unsafe { vips_init(c_name.as_ptr()) };
        let result = if res == 0 {
            Ok(res)
        } else {
            Err("Failed to init libvips".to_string())
        };
        unsafe {
            if detect_leak {
                vips_leak_set(1);
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
