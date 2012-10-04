use stb_image::*;
use stb_image::bindgen::*;
use libc::types::os::arch::c95::c_int;
use ptr::{is_null, to_unsafe_ptr};
use str::as_c_str;
use vec::as_imm_buf;
use vec::raw::from_buf;

pub struct Image {
    width: uint,
    height: uint,
    depth: uint,
    data: ~[u8],
}

pub fn new_image(width: uint, height: uint, depth: uint, -data: ~[u8]) -> Image {
    Image {
        width : width,
        height : height,
        depth : depth,
        data : data,
    }
}

pub fn load(path: ~str) -> Option<Image> unsafe {
    do task::unkillable {
        let mut width = 0 as c_int;
        let mut height = 0 as c_int;
        let mut depth = 0 as c_int;
        let force_depth = 4 as c_int;
        let buffer = as_c_str(path, |bytes| {
            stbi_load(bytes, to_unsafe_ptr(&width), to_unsafe_ptr(&height), to_unsafe_ptr(&depth),
                      force_depth)
        });

        if is_null(buffer) {
            None
        } else {
            // FIXME: Shouldn't copy; instead we should use a sendable resource. They
            // aren't particularly safe yet though.
            let data = from_buf(buffer, (width * height * force_depth) as uint);
            libc::free(buffer as *c_void);
            Some(new_image(width as uint, height as uint, force_depth as uint, data))
        }
    }
}

pub fn load_from_memory(buffer: &[u8]) -> Option<Image> unsafe {
    do task::unkillable {
        let mut width = 0 as c_int;
        let mut height = 0 as c_int;
        let mut depth = 0 as c_int;
        let force_depth = 4 as c_int;
        let buffer = as_imm_buf(buffer, |bytes, len| {
            stbi_load_from_memory(bytes, len as c_int, to_unsafe_ptr(&width),
                                  to_unsafe_ptr(&height), to_unsafe_ptr(&depth), force_depth)
        });

        if is_null(buffer) {
            None
        } else {
            // FIXME: Shouldn't copy; instead we should use a sendable resource. They
            // aren't particularly safe yet though.
            let data = from_buf(buffer, (width * height * force_depth) as uint);
            libc::free(buffer as *c_void);
            Some(new_image(width as uint, height as uint, force_depth as uint, data))
        }
    }
}
