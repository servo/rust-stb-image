import stb_image::*;
import stb_image::bindgen::*;
import libc::types::os::arch::c95::c_int;
import ptr::{addr_of, is_null};
import str::as_c_str;
import vec::unsafe::from_buf;

class image {
    let width: uint;
    let height: uint;
    let depth: uint;
    let data: ~[u8];

    new(width: uint, height: uint, depth: uint, -data: ~[u8]) {
        self.width = width;
        self.height = height;
        self.depth = depth;
        self.data = data;
    }
}

fn load(path: ~str) -> image unsafe {
    let mut width = 0 as c_int;
    let mut height = 0 as c_int;
    let mut depth = 0 as c_int;
    let buffer = as_c_str(path, |bytes| {
        stbi_load(bytes, addr_of(width), addr_of(height), addr_of(depth), 0 as c_int)
    });
    if is_null(buffer) {
        fail ~"failed to load image";
    }

    // FIXME: Shouldn't copy; instead we should use a sendable resource. They
    // aren't particularly safe yet though.
    let data = from_buf(buffer, (width * height * depth) as uint);
    return image(width as uint, height as uint, depth as uint, data);
}

