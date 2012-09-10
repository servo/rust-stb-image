import stb_image::*;
import stb_image::bindgen::*;
import libc::types::os::arch::c95::c_int;
import ptr::{addr_of, is_null};
import str::as_c_str;
import vec::as_buf;
import vec::unsafe::from_buf;

struct Image {
    width: uint,
    height: uint,
    depth: uint,
    data: ~[u8],
}

fn new_image(width: uint, height: uint, depth: uint, -data: ~[u8]) -> Image {
    Image {
        width : width,
        height : height,
        depth : depth,
        data : data,
    }
}

fn load(path: ~str) -> Option<Image> unsafe {
    do task::unkillable {
        let mut width = 0 as c_int;
        let mut height = 0 as c_int;
        let mut depth = 0 as c_int;
        let force_depth = 4 as c_int;
        let buffer = as_c_str(path, |bytes| {
            stbi_load(bytes, addr_of(width), addr_of(height), addr_of(depth), force_depth)
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

fn load_from_memory(buffer: &[u8]) -> Option<Image> unsafe {
    do task::unkillable {
        let mut width = 0 as c_int;
        let mut height = 0 as c_int;
        let mut depth = 0 as c_int;
        let force_depth = 4 as c_int;
        let buffer = as_buf(buffer, |bytes, len| {
            stbi_load_from_memory(bytes, len as c_int, addr_of(width), addr_of(height), addr_of(depth), force_depth)
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
