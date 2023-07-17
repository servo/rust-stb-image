// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use stb_image::*;

use libc::{c_int, c_void};
use std::convert::AsRef;
use std::ffi::CString;
use std::path::Path;
use std::slice;

pub struct Image<T> {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub data: Vec<T>,
}

impl<T> Image<T> {
    pub fn new(width: usize, height: usize, depth: usize, data: Vec<T>) -> Image<T> {
        Image::<T> {
            width,
            height,
            depth,
            data,
        }
    }
}

pub enum LoadResult {
    Error(String),
    ImageU8(Image<u8>),
    ImageF32(Image<f32>),
}

pub fn load<T: AsRef<Path>>(path: T) -> LoadResult {
    let force_depth = 0;
    load_with_depth(path, force_depth, false)
}

fn load_internal<T: Clone>(buf: *mut T, w: c_int, h: c_int, d: c_int) -> Image<T> {
    unsafe {
        // FIXME: Shouldn't copy; instead we should use a sendable resource. They
        // aren't particularly safe yet though.
        let data = slice::from_raw_parts(buf, (w * h * d) as usize).to_vec();
        stbi_image_free(buf as *mut c_void);
        Image::<T> {
            width: w as usize,
            height: h as usize,
            depth: d as usize,
            data,
        }
    }
}

pub fn load_with_depth<T: AsRef<Path>>(
    path: T,
    force_depth: usize,
    convert_hdr: bool,
) -> LoadResult {
    let mut width = 0 as c_int;
    let mut height = 0 as c_int;
    let mut depth = 0 as c_int;
    let force_depth = force_depth as c_int;
    let path_as_cstr = match path.as_ref().as_os_str().to_str() {
        Some(s) => match CString::new(s.as_bytes()) {
            Ok(s) => s,
            Err(_) => return LoadResult::Error("path contains null character".to_string()),
        },
        None => return LoadResult::Error("path is not valid utf8".to_string()),
    };
    unsafe {
        let bytes = path_as_cstr.as_ptr();
        if !convert_hdr && stbi_is_hdr(bytes) != 0 {
            let buffer = stbi_loadf(bytes, &mut width, &mut height, &mut depth, force_depth);
            if buffer.is_null() {
                LoadResult::Error("stbi_loadf failed".to_string())
            } else {
                let actual_depth = if force_depth != 0 { force_depth } else { depth };
                LoadResult::ImageF32(load_internal(buffer, width, height, actual_depth))
            }
        } else {
            let buffer = stbi_load(bytes, &mut width, &mut height, &mut depth, force_depth);
            if buffer.is_null() {
                LoadResult::Error("stbi_load failed".to_string())
            } else {
                let actual_depth = if force_depth != 0 { force_depth } else { depth };
                LoadResult::ImageU8(load_internal(buffer, width, height, actual_depth))
            }
        }
    }
}

pub fn load_from_memory(buffer: &[u8]) -> LoadResult {
    let force_depth = 0;
    load_from_memory_with_depth(buffer, force_depth, false)
}

pub fn load_from_memory_with_depth(
    buffer: &[u8],
    force_depth: usize,
    convert_hdr: bool,
) -> LoadResult {
    unsafe {
        let mut width = 0 as c_int;
        let mut height = 0 as c_int;
        let mut depth = 0 as c_int;
        let force_depth = force_depth as c_int;
        if !convert_hdr && stbi_is_hdr_from_memory(buffer.as_ptr(), buffer.len() as c_int) != 0 {
            let buffer = stbi_loadf_from_memory(
                buffer.as_ptr(),
                buffer.len() as c_int,
                &mut width,
                &mut height,
                &mut depth,
                force_depth,
            );
            if buffer.is_null() {
                LoadResult::Error("stbi_loadf_from_memory failed".to_string())
            } else {
                let actual_depth = if force_depth != 0 { force_depth } else { depth };
                LoadResult::ImageF32(load_internal(buffer, width, height, actual_depth))
            }
        } else {
            let buffer = stbi_load_from_memory(
                buffer.as_ptr(),
                buffer.len() as c_int,
                &mut width,
                &mut height,
                &mut depth,
                force_depth,
            );
            if buffer.is_null() {
                LoadResult::Error("stbi_load_from_memory failed".to_string())
            } else {
                let actual_depth = if force_depth != 0 { force_depth } else { depth };
                LoadResult::ImageU8(load_internal(buffer, width, height, actual_depth))
            }
        }
    }
}
