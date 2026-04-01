pub(crate) mod painter;
pub(crate) mod stream;


use std::ffi::c_void;

use crate::bridge::painter::{Painter, make_rust_painter_funcs};
use crate::bridge::stream::{InputStream, make_rust_input_stream_funcs};
use crate::glue;


pub fn is_supported(stream: &mut Box<dyn InputStream>) -> bool {
    let stream_funcs = make_rust_input_stream_funcs(false);
    let stream_user_ptr = &raw mut *stream as *mut c_void;
    let stream = unsafe {
        glue::visio_glue_new_input_stream(stream_funcs, stream_user_ptr)
    };

    let result = unsafe {
        glue::visio_glue_document_is_supported(stream)
    };

    unsafe {
        glue::visio_glue_input_stream_free(stream)
    };

    result
}

pub fn parse(stream: &mut Box<dyn InputStream>, painter: &mut Box<dyn Painter>) -> bool {
    let stream_funcs = make_rust_input_stream_funcs(false);
    let stream_user_ptr = &raw mut *stream as *mut c_void;
    let stream = unsafe {
        glue::visio_glue_new_input_stream(stream_funcs, stream_user_ptr)
    };

    let painter_funcs = make_rust_painter_funcs(false);
    let painter_user_ptr = &raw mut *painter as *mut c_void;
    let painter = unsafe {
        glue::visio_glue_new_painter(painter_funcs, painter_user_ptr)
    };

    let result = unsafe {
        glue::visio_glue_document_parse(stream, painter)
    };

    unsafe {
        glue::visio_glue_painter_free(painter)
    };
    unsafe {
        glue::visio_glue_input_stream_free(stream)
    };

    result
}

pub fn parse_stencils(stream: &mut Box<dyn InputStream>, painter: &mut Box<dyn Painter>) -> bool {
    let stream_funcs = make_rust_input_stream_funcs(false);
    let stream_user_ptr = &raw mut *stream as *mut c_void;
    let stream = unsafe {
        glue::visio_glue_new_input_stream(stream_funcs, stream_user_ptr)
    };

    let painter_funcs = make_rust_painter_funcs(false);
    let painter_user_ptr = &raw mut *painter as *mut c_void;
    let painter = unsafe {
        glue::visio_glue_new_painter(painter_funcs, painter_user_ptr)
    };

    let result = unsafe {
        glue::visio_glue_document_parse_stencils(stream, painter)
    };

    unsafe {
        glue::visio_glue_painter_free(painter)
    };
    unsafe {
        glue::visio_glue_input_stream_free(stream)
    };

    result
}
