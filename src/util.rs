use crate::{
    common::{determine_image_type, determine_image_type_from_str, image_type_id},
    output::{AvailableFormat, AvailableFormatInput, AvailableFormatOutput},
    Sharp,
};
use rs_vips::{
    bindings::{g_type_from_name, vips_class_find, vips_type_find},
    Vips,
};
use std::{collections::HashMap, ffi::CString};

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
        // Set memory limit
        Vips::cache_set_max_mem(memory);
        // Set file limit
        Vips::cache_set_max_files(files);
        // Set items limit
        Vips::cache_set_max(items);

        let mut result = CacheResult::default();
        // Get memory stats
        result.memory.current = Vips::tracked_get_mem() / 1048576;
        result.memory.high = Vips::tracked_get_mem_highwater() / 1048576;
        result.memory.max = Vips::cache_get_max_mem() / 1048576;

        // Get file stats
        result.files.current = Vips::tracked_get_files();
        result.files.max = Vips::cache_get_max_files();

        // Get item stats
        result.items.current = Vips::cache_get_size();
        result.items.max = Vips::cache_get_max();

        result
    }

    /*
     * Set size of thread pool
     */
    pub fn set_concurrency(max: i32) {
        Vips::concurrency_set(max)
    }

    /*
     * Get size of thread pool
     */
    pub fn get_concurrency() -> i32 {
        Vips::concurrency_get()
    }
}

#[inline]
pub(crate) fn new_c_string(string: &str) -> Result<CString, String> {
    CString::new(string).map_err(|_| "Error initializing C string.".to_string())
}

pub(crate) struct VipsGuard;

impl Drop for VipsGuard {
    fn drop(&mut self) {
        Vips::error_clear();
        Vips::thread_shutdown();
    }
}

pub(crate) const G_TYPE_INT: &str = "gint";

pub(crate) fn get_g_type(name: &str) -> u64 {
    let type_name = new_c_string(name).unwrap();
    unsafe { g_type_from_name(type_name.as_ptr()) }
}

pub(crate) fn available_formats() -> HashMap<String, AvailableFormat> {
    Vips::init("sharp", false).unwrap();
    let mut formats = HashMap::new();
    [
        "jpeg",
        "png",
        "webp",
        "tiff",
        "magick",
        "openslide",
        "dz",
        "ppm",
        "fits",
        "gif",
        "svg",
        "heif",
        "pdf",
        "vips",
        "jp2k",
        "jxl",
        "rad",
        "dcraw",
    ]
    .into_iter()
    .for_each(|f| {
        let basename = new_c_string("VipsOperation").unwrap();
        // input
        let load = new_c_string(&format!("{f}load")).unwrap();
        let has_input_file =
            !unsafe { vips_class_find(basename.as_ptr(), load.as_ptr()) }.is_null();
        let load_buffer = new_c_string(&format!("{f}load_buffer")).unwrap();
        let has_input_buffer =
            !unsafe { vips_class_find(basename.as_ptr(), load_buffer.as_ptr()) }.is_null();
        let input = AvailableFormatInput {
            file: has_input_file,
            buffer: has_input_buffer,
            stream: has_input_buffer,
        };
        // Output
        let save = new_c_string(&format!("{f}save")).unwrap();
        let has_output_file = unsafe { vips_type_find(basename.as_ptr(), save.as_ptr()) } > 0;
        let save_buffer = new_c_string(&format!("{f}save_buffer")).unwrap();
        let has_output_buffer =
            unsafe { vips_type_find(basename.as_ptr(), save_buffer.as_ptr()) } > 0;
        let output = AvailableFormatOutput {
            file: has_output_file,
            buffer: has_output_buffer,
            stream: has_output_buffer,
        };

        formats.insert(
            f.to_string(),
            AvailableFormat {
                id: f.to_string(),
                input,
                output,
            },
        );
    });
    formats
}
