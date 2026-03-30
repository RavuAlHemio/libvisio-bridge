mod painter;


use std::collections::HashMap;
use std::ffi::{CStr, c_void};
use std::ptr::null_mut;

use crate::bridge::painter::make_rust_painter;
use crate::glue;


pub trait Painter {
    fn start_document(&mut self, _properties: HashMap<String, String>) {}
    fn end_document(&mut self) {}
    fn start_page(&mut self, _properties: HashMap<String, String>) {}
    fn end_page(&mut self) {}
    fn start_master_page(&mut self, _properties: HashMap<String, String>) {}
    fn end_master_page(&mut self) {}
    fn start_layer(&mut self, _properties: HashMap<String, String>) {}
    fn end_layer(&mut self) {}
    fn start_embedded_graphics(&mut self, _properties: HashMap<String, String>) {}
    fn end_embedded_graphics(&mut self) {}
    fn open_group(&mut self, _properties: HashMap<String, String>) {}
    fn close_group(&mut self) {}
    fn start_text_object(&mut self, _properties: HashMap<String, String>) {}
    fn end_text_object(&mut self) {}
    fn start_table_object(&mut self, _properties: HashMap<String, String>) {}
    fn end_table_object(&mut self) {}
    fn open_table_row(&mut self, _properties: HashMap<String, String>) {}
    fn close_table_row(&mut self) {}
    fn open_table_cell(&mut self, _properties: HashMap<String, String>) {}
    fn close_table_cell(&mut self) {}
    fn open_ordered_list_level(&mut self, _properties: HashMap<String, String>) {}
    fn close_ordered_list_level(&mut self) {}
    fn open_unordered_list_level(&mut self, _properties: HashMap<String, String>) {}
    fn close_unordered_list_level(&mut self) {}
    fn open_list_element(&mut self, _properties: HashMap<String, String>) {}
    fn close_list_element(&mut self) {}
    fn open_paragraph(&mut self, _properties: HashMap<String, String>) {}
    fn close_paragraph(&mut self) {}
    fn open_span(&mut self, _properties: HashMap<String, String>) {}
    fn close_span(&mut self) {}
    fn open_link(&mut self, _properties: HashMap<String, String>) {}
    fn close_link(&mut self) {}

    fn set_document_metadata(&mut self, _properties: HashMap<String, String>) {}
    fn define_embedded_font(&mut self, _properties: HashMap<String, String>) {}
    fn set_style(&mut self, _properties: HashMap<String, String>) {}
    fn draw_rectangle(&mut self, _properties: HashMap<String, String>) {}
    fn draw_ellipse(&mut self, _properties: HashMap<String, String>) {}
    fn draw_polygon(&mut self, _properties: HashMap<String, String>) {}
    fn draw_polyline(&mut self, _properties: HashMap<String, String>) {}
    fn draw_path(&mut self, _properties: HashMap<String, String>) {}
    fn draw_graphic_object(&mut self, _properties: HashMap<String, String>) {}
    fn draw_connector(&mut self, _properties: HashMap<String, String>) {}
    fn insert_covered_table_cell(&mut self, _properties: HashMap<String, String>) {}
    fn insert_field(&mut self, _properties: HashMap<String, String>) {}
    fn define_paragraph_style(&mut self, _properties: HashMap<String, String>) {}
    fn define_character_style(&mut self, _properties: HashMap<String, String>) {}

    fn insert_tab(&mut self) {}
    fn insert_space(&mut self) {}
    fn insert_line_break(&mut self) {}

    fn insert_text(&mut self, _string: String) {}
}


pub struct VisioFile {
    inner: *mut glue::visio_glue_input_stream,
}
impl VisioFile {
    pub fn new(path: &CStr) -> Option<Self> {
        let inner = unsafe {
            glue::visio_glue_open_file(path.as_ptr())
        };
        if inner.is_null() {
            None
        } else {
            Some(Self {
                inner,
            })
        }
    }

    pub fn close(mut self) {
        if !self.inner.is_null() {
            unsafe {
                glue::visio_glue_close_file(self.inner);
            }
            self.inner = null_mut();
        }
    }

    pub fn is_supported(&mut self) -> bool {
        unsafe {
            glue::visio_glue_document_is_supported(self.inner)
        }
    }

    pub fn parse(&mut self, painter: &mut Box<dyn Painter>) -> bool {
        let mut rust_painter = make_rust_painter();
        let user_ptr = &raw mut *painter as *mut c_void;
        unsafe {
            glue::visio_glue_document_parse(self.inner, &mut rust_painter, user_ptr)
        }
    }

    pub fn parse_stencils(&mut self, painter: &mut Box<dyn Painter>) -> bool {
        let mut rust_painter = make_rust_painter();
        let user_ptr = &raw mut *painter as *mut c_void;
        unsafe {
            glue::visio_glue_document_parse_stencils(self.inner, &mut rust_painter, user_ptr)
        }
    }
}
impl Drop for VisioFile {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                glue::visio_glue_close_file(self.inner);
            }
            self.inner = null_mut();
        }
    }
}
