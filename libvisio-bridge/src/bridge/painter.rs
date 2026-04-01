use std::ffi::{CStr, c_char, c_void};
use std::collections::HashMap;

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


fn collect_prop_list(prop_list: *const glue::visio_glue_property_list) -> HashMap<String, String> {
    let iterator = unsafe {
        glue::visio_glue_property_list_iterate(prop_list)
    };

    let mut ret = HashMap::new();

    if iterator.is_null() {
        return ret;
    }

    loop {
        let got_item = unsafe {
            glue::visio_glue_property_list_iterator_advance(iterator)
        };
        if !got_item {
            break;
        }

        let key_ptr = unsafe {
            glue::visio_glue_property_list_iterator_key(iterator)
        };
        let key_cstr = unsafe {
            CStr::from_ptr(key_ptr)
        };
        let key = key_cstr.to_string_lossy().into_owned();

        let mut value_ptr = unsafe {
            glue::visio_glue_property_list_iterator_value(iterator)
        };
        let value = if value_ptr.value.is_null() {
            String::new()
        } else {
            let value_cstr = unsafe {
                CStr::from_ptr(value_ptr.value)
            };
            value_cstr.to_string_lossy().into_owned()
        };
        unsafe {
            glue::visio_glue_property_value_free(&raw mut value_ptr)
        };

        ret.insert(key, value);
    }

    unsafe {
        glue::visio_glue_property_list_iterator_free(iterator)
    };

    ret
}


macro_rules! impl_bridge {
    ($name:ident) => {
        extern "C" fn $name(user_ptr: *mut c_void) {
            let painter_ptr = user_ptr as *mut Box<dyn Painter>;
            unsafe { &mut *painter_ptr }.$name()
        }
    };
}
macro_rules! impl_bridge_pl {
    ($name:ident) => {
        extern "C" fn $name(user_ptr: *mut c_void, prop_list: *const glue::visio_glue_property_list) {
            let painter_ptr = user_ptr as *mut Box<dyn Painter>;
            let properties = collect_prop_list(prop_list);
            unsafe { &mut *painter_ptr }.$name(properties)
        }
    };
}
macro_rules! impl_bridge_str {
    ($name:ident) => {
        extern "C" fn $name(user_ptr: *mut c_void, text: *const c_char, length: usize) {
            let painter_ptr = user_ptr as *mut Box<dyn Painter>;
            let bs = unsafe { std::slice::from_raw_parts(text as *const u8, length) };
            let string = String::from_utf8_lossy(bs).into_owned();
            unsafe { &mut *painter_ptr }.$name(string)
        }
    };
}

extern "C" fn destroy(user_ptr: *mut c_void) {
    let painter_ptr = user_ptr as *mut Box<dyn Painter>;

    // re-box that (yay double-boxing)
    let painter_box = unsafe { Box::from_raw(painter_ptr) };

    // drop it, which drops the Painter
    drop(painter_box);
}

extern "C" fn noop_destroy(_user_ptr: *mut c_void) {
}


impl_bridge_pl!(start_document);
impl_bridge!(end_document);
impl_bridge_pl!(start_page);
impl_bridge!(end_page);
impl_bridge_pl!(start_master_page);
impl_bridge!(end_master_page);
impl_bridge_pl!(start_layer);
impl_bridge!(end_layer);
impl_bridge_pl!(start_embedded_graphics);
impl_bridge!(end_embedded_graphics);
impl_bridge_pl!(open_group);
impl_bridge!(close_group);
impl_bridge_pl!(start_text_object);
impl_bridge!(end_text_object);
impl_bridge_pl!(start_table_object);
impl_bridge!(end_table_object);
impl_bridge_pl!(open_table_row);
impl_bridge!(close_table_row);
impl_bridge_pl!(open_table_cell);
impl_bridge!(close_table_cell);
impl_bridge_pl!(open_ordered_list_level);
impl_bridge!(close_ordered_list_level);
impl_bridge_pl!(open_unordered_list_level);
impl_bridge!(close_unordered_list_level);
impl_bridge_pl!(open_list_element);
impl_bridge!(close_list_element);
impl_bridge_pl!(open_paragraph);
impl_bridge!(close_paragraph);
impl_bridge_pl!(open_span);
impl_bridge!(close_span);
impl_bridge_pl!(open_link);
impl_bridge!(close_link);

impl_bridge_pl!(set_document_metadata);
impl_bridge_pl!(define_embedded_font);
impl_bridge_pl!(set_style);
impl_bridge_pl!(draw_rectangle);
impl_bridge_pl!(draw_ellipse);
impl_bridge_pl!(draw_polygon);
impl_bridge_pl!(draw_polyline);
impl_bridge_pl!(draw_path);
impl_bridge_pl!(draw_graphic_object);
impl_bridge_pl!(draw_connector);
impl_bridge_pl!(insert_covered_table_cell);
impl_bridge_pl!(insert_field);
impl_bridge_pl!(define_paragraph_style);
impl_bridge_pl!(define_character_style);

impl_bridge!(insert_tab);
impl_bridge!(insert_space);
impl_bridge!(insert_line_break);

impl_bridge_str!(insert_text);

pub fn make_rust_painter_funcs(drop_on_destroy: bool) -> glue::visio_glue_painter_funcs {
    glue::visio_glue_painter_funcs {
        destroy: if drop_on_destroy { Some(destroy) } else { Some(noop_destroy) },
        start_document: Some(start_document),
        end_document: Some(end_document),
        start_page: Some(start_page),
        end_page: Some(end_page),
        start_master_page: Some(start_master_page),
        end_master_page: Some(end_master_page),
        start_layer: Some(start_layer),
        end_layer: Some(end_layer),
        start_embedded_graphics: Some(start_embedded_graphics),
        end_embedded_graphics: Some(end_embedded_graphics),
        open_group: Some(open_group),
        close_group: Some(close_group),
        start_text_object: Some(start_text_object),
        end_text_object: Some(end_text_object),
        start_table_object: Some(start_table_object),
        end_table_object: Some(end_table_object),
        open_table_row: Some(open_table_row),
        close_table_row: Some(close_table_row),
        open_table_cell: Some(open_table_cell),
        close_table_cell: Some(close_table_cell),
        open_ordered_list_level: Some(open_ordered_list_level),
        close_ordered_list_level: Some(close_ordered_list_level),
        open_unordered_list_level: Some(open_unordered_list_level),
        close_unordered_list_level: Some(close_unordered_list_level),
        open_list_element: Some(open_list_element),
        close_list_element: Some(close_list_element),
        open_paragraph: Some(open_paragraph),
        close_paragraph: Some(close_paragraph),
        open_span: Some(open_span),
        close_span: Some(close_span),
        open_link: Some(open_link),
        close_link: Some(close_link),
        set_document_metadata: Some(set_document_metadata),
        define_embedded_font: Some(define_embedded_font),
        set_style: Some(set_style),
        draw_rectangle: Some(draw_rectangle),
        draw_ellipse: Some(draw_ellipse),
        draw_polygon: Some(draw_polygon),
        draw_polyline: Some(draw_polyline),
        draw_path: Some(draw_path),
        draw_graphic_object: Some(draw_graphic_object),
        draw_connector: Some(draw_connector),
        insert_covered_table_cell: Some(insert_covered_table_cell),
        insert_field: Some(insert_field),
        define_paragraph_style: Some(define_paragraph_style),
        define_character_style: Some(define_character_style),
        insert_tab: Some(insert_tab),
        insert_space: Some(insert_space),
        insert_line_break: Some(insert_line_break),
        insert_text: Some(insert_text),
    }
}
