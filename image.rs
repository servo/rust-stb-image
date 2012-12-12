use stb_image::*;
use stb_image::bindgen::*;
use libc::types::os::arch::c95::c_int;
use ptr::{is_null, to_unsafe_ptr};
use str::as_c_str;
use vec::as_imm_buf;
use vec::raw::from_buf_raw;

pub struct Image<T> {
    width   : uint,
    height  : uint,
    depth   : uint,
    data    : ~[T],
}

pub fn new_image<T>(width: uint, height: uint, depth: uint, data: ~[T]) -> Image<T> {
    Image::<T> {
        width   : width,
        height  : height,
        depth   : depth,
        data    : move data,
    }
}

enum LoadResult {
    Error,
    ImageU8(Image<u8>),
    ImageF32(Image<f32>),
}

pub fn load(path: ~str) -> LoadResult unsafe {
    let force_depth = 0;
    load_with_depth(move path, force_depth, false)
}


priv fn load_internal<T>(buf : *T, w : c_int, h : c_int, d : c_int) -> Image<T> unsafe   {
    // FIXME: Shouldn't copy; instead we should use a sendable resource. They
    // aren't particularly safe yet though.
    let data = from_buf_raw(buf, (w * h * d) as uint);
    libc::free(buf as *c_void);
    Image::<T>{
        width   : w as uint,
        height  : h as uint,
        depth   : d as uint,
        data    : data}
}

pub fn load_with_depth(path: ~str, force_depth: uint, convert_hdr:bool) -> LoadResult unsafe {
    do task::unkillable {
        let mut width   = 0 as c_int;
        let mut height  = 0 as c_int;
        let mut depth   = 0 as c_int;
        as_c_str(path, |bytes| {
            if !convert_hdr && stbi_is_hdr(bytes)!=0   {
                let buffer = stbi_loadf(
                    bytes, to_unsafe_ptr(&width), to_unsafe_ptr(&height),
                    to_unsafe_ptr(&depth), force_depth as c_int);
                if is_null(buffer) {
                    Error
                } else {
                    ImageF32( load_internal(buffer,width,height,depth) )
                }
            }else   {
                let buffer = stbi_load(
                    bytes, to_unsafe_ptr(&width), to_unsafe_ptr(&height),
                    to_unsafe_ptr(&depth), force_depth as c_int);
                if is_null(buffer) {
                    Error
                } else {
                    ImageU8( load_internal(buffer,width,height,depth) )
                }
            }
        })
    }
}

pub fn load_from_memory(buffer: &[u8]) -> LoadResult unsafe {
        let force_depth = 0;
        load_from_memory_with_depth(buffer, force_depth, false)
}

pub fn load_from_memory_with_depth(buffer: &[u8], force_depth: uint, convert_hdr:bool) -> LoadResult unsafe {
    do task::unkillable {
        let mut width = 0 as c_int;
        let mut height = 0 as c_int;
        let mut depth = 0 as c_int;
        as_imm_buf(buffer, |bytes, len| {
            if !convert_hdr && stbi_is_hdr_from_memory(bytes,len as c_int)!=0   {
                let buffer = stbi_loadf_from_memory(
                    bytes, len as c_int, to_unsafe_ptr(&width),
                    to_unsafe_ptr(&height), to_unsafe_ptr(&depth),
                    force_depth as c_int);
                if is_null(buffer) {
                    Error
                } else {
                    ImageF32( load_internal(buffer,width,height,depth) )
                }
            }else   {
                let buffer = stbi_load_from_memory(
                    bytes, len as c_int, to_unsafe_ptr(&width),
                    to_unsafe_ptr(&height), to_unsafe_ptr(&depth),
                    force_depth as c_int);
                if is_null(buffer) {
                    Error
                } else {
                    ImageU8( load_internal(buffer,width,height,depth) )
                }
            }
        })
    }
}
