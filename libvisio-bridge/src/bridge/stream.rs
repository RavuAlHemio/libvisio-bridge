use std::ffi::{CStr, CString, c_char, c_int, c_long, c_uint, c_ulong, c_void};
use std::ptr::{null, null_mut};
use std::sync::LazyLock;

use crate::glue;


pub trait InputStream {
    fn is_structured(&mut self) -> bool { false }
    fn sub_stream_count(&mut self) -> usize { 0 }
    fn sub_stream_name(&mut self, _stream_id: usize) -> Option<CString> { None }
    fn sub_stream_exists(&mut self, _stream_name: &CStr) -> bool { false }
    fn sub_stream_by_id(&mut self, _stream_id: usize) -> Option<Box<dyn InputStream>> { None }
    fn sub_stream_by_name(&mut self, _stream_name: &CStr) -> Option<Box<dyn InputStream>> { None }
    fn read(&mut self, _num_bytes: usize) -> &[u8] { &[] }
    fn seek(&mut self, _offset: c_long, _seek_type: glue::visio_glue_seek_type) -> c_int { -1 }
    fn tell(&mut self) -> c_long { -1 }
    fn is_end(&mut self) -> bool { true }
}

extern "C" fn destroy(user_ptr: *mut c_void) {
    let stream_ptr = user_ptr as *mut Box<dyn InputStream>;

    // re-box that (yay double-boxing)
    let stream_box = unsafe { Box::from_raw(stream_ptr) };

    // drop it, which drops the InputStream
    drop(stream_box);
}

extern "C" fn noop_destroy(_user_ptr: *mut c_void) {
}

extern "C" fn is_structured(user_ptr: *mut c_void) -> bool {
    let stream_ptr = user_ptr as *mut Box<dyn InputStream>;
    unsafe { &mut *stream_ptr }.is_structured()
}

extern "C" fn sub_stream_count(user_ptr: *mut c_void) -> c_uint {
    let stream_ptr = user_ptr as *mut Box<dyn InputStream>;
    unsafe { &mut *stream_ptr }.sub_stream_count().try_into().unwrap()
}

static mut SUB_STREAM_NAME_BUF: LazyLock<CString> = LazyLock::new(|| CString::new("").unwrap());
extern "C" fn sub_stream_name(user_ptr: *mut c_void, stream_id: c_uint) -> *const c_char {
    let stream_ptr = user_ptr as *mut Box<dyn InputStream>;
    let stream_id_usize: usize = stream_id.try_into().unwrap();
    let ret_opt = unsafe { &mut *stream_ptr }.sub_stream_name(stream_id_usize);

    if let Some(ret) = ret_opt {
        unsafe {
            *SUB_STREAM_NAME_BUF = ret;

            // Rust doesn't like SUB_STREAM_NAME_BUF.as_ptr()
            let raw_buf = &raw mut SUB_STREAM_NAME_BUF;
            (&mut *raw_buf).as_ptr()
        }
    } else {
        null()
    }
}

extern "C" fn sub_stream_exists(user_ptr: *mut c_void, name: *const c_char) -> bool {
    let stream_ptr = user_ptr as *mut Box<dyn InputStream>;
    let name_c_str = unsafe { CStr::from_ptr(name) };
    unsafe { &mut *stream_ptr }.sub_stream_exists(name_c_str)
}

extern "C" fn sub_stream_by_id(user_ptr: *mut c_void, stream_id: c_uint) -> *mut glue::visio_glue_input_stream {
    let stream_ptr = user_ptr as *mut Box<dyn InputStream>;
    let stream_id_usize = stream_id.try_into().unwrap();
    let substream_opt = unsafe { &mut *stream_ptr }.sub_stream_by_id(stream_id_usize);
    if let Some(substream) = substream_opt {
        let stream_ref = Box::leak(Box::new(substream));
        let stream_ptr = &raw mut *stream_ref;
        unsafe {
            glue::visio_glue_new_input_stream(
                make_rust_input_stream_funcs(true),
                stream_ptr as *mut c_void,
            )
        }
    } else {
        null_mut()
    }
}

extern "C" fn sub_stream_by_name(user_ptr: *mut c_void, name: *const c_char) -> *mut glue::visio_glue_input_stream {
    let stream_ptr = user_ptr as *mut Box<dyn InputStream>;
    let name_c_str = unsafe { CStr::from_ptr(name) };
    let substream_opt = unsafe { &mut *stream_ptr }.sub_stream_by_name(name_c_str);
    if let Some(substream) = substream_opt {
        let stream_ref = Box::leak(Box::new(substream));
        let stream_ptr = &raw mut *stream_ref;
        unsafe {
            glue::visio_glue_new_input_stream(
                make_rust_input_stream_funcs(true),
                stream_ptr as *mut c_void,
            )
        }
    } else {
        null_mut()
    }
}

extern "C" fn read(user_ptr: *mut c_void, num_bytes: c_ulong, num_bytes_read: *mut c_ulong) -> *const u8 {
    let stream_ptr = user_ptr as *mut Box<dyn InputStream>;
    let num_bytes_usize: usize = num_bytes.try_into().unwrap();
    let slice = unsafe { &mut *stream_ptr }.read(num_bytes_usize);
    unsafe {
        *num_bytes_read = slice.len().try_into().unwrap()
    };
    slice.as_ptr()
}

extern "C" fn seek(user_ptr: *mut c_void, offset: c_long, seek_type: c_int) -> c_int {
    const CUR: c_int = glue::visio_glue_seek_type::Cur as c_int;
    const START: c_int = glue::visio_glue_seek_type::Start as c_int;
    const END: c_int = glue::visio_glue_seek_type::End as c_int;

    let glue_seek_type = match seek_type {
        CUR => glue::visio_glue_seek_type::Cur,
        START => glue::visio_glue_seek_type::Start,
        END => glue::visio_glue_seek_type::End,
        _ => return -1,
    };

    let stream_ptr = user_ptr as *mut Box<dyn InputStream>;
    unsafe { &mut *stream_ptr }.seek(offset.into(), glue_seek_type)
}

extern "C" fn tell(user_ptr: *mut c_void) -> c_long {
    let stream_ptr = user_ptr as *mut Box<dyn InputStream>;
    unsafe { &mut *stream_ptr }.tell()
}

extern "C" fn is_end(user_ptr: *mut c_void) -> bool {
    let stream_ptr = user_ptr as *mut Box<dyn InputStream>;
    unsafe { &mut *stream_ptr }.is_end()
}


pub fn make_rust_input_stream_funcs(drop_on_destroy: bool) -> glue::visio_glue_input_stream_funcs {
    glue::visio_glue_input_stream_funcs {
        destroy: if drop_on_destroy { Some(destroy) } else { Some(noop_destroy) },
        is_structured: Some(is_structured),
        sub_stream_count: Some(sub_stream_count),
        sub_stream_name: Some(sub_stream_name),
        sub_stream_exists: Some(sub_stream_exists),
        sub_stream_by_id: Some(sub_stream_by_id),
        sub_stream_by_name: Some(sub_stream_by_name),
        read: Some(read),
        seek: Some(seek),
        tell: Some(tell),
        is_end: Some(is_end),
    }
}
