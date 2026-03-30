#![allow(non_camel_case_types)]


use std::ffi::{c_char, c_void};


macro_rules! opaque_type {
    ($name:ident) => {
        #[repr(C)]
        pub struct $name {
            _data: (),
            _marker: ::core::marker::PhantomData<(*mut u8, ::core::marker::PhantomPinned)>,
        }
    };
}

opaque_type!(visio_glue_input_stream);
opaque_type!(visio_glue_property_list);
opaque_type!(visio_glue_property_list_iterator);

pub type visio_glue_func = Option<extern "C" fn(user_ptr: *mut c_void)>;
pub type visio_glue_func_pl = Option<extern "C" fn(user_ptr: *mut c_void, prop_list: *const visio_glue_property_list)>;
pub type visio_glue_func_str = Option<extern "C" fn(user_ptr: *mut c_void, text: *const c_char, length: usize)>;

#[repr(C)]
pub struct visio_glue_painter {
    pub start_document: visio_glue_func_pl,
    pub end_document: visio_glue_func,
    pub start_page: visio_glue_func_pl,
    pub end_page: visio_glue_func,
    pub start_master_page: visio_glue_func_pl,
    pub end_master_page: visio_glue_func,
    pub start_layer: visio_glue_func_pl,
    pub end_layer: visio_glue_func,
    pub start_embedded_graphics: visio_glue_func_pl,
    pub end_embedded_graphics: visio_glue_func,
    pub open_group: visio_glue_func_pl,
    pub close_group: visio_glue_func,
    pub start_text_object: visio_glue_func_pl,
    pub end_text_object: visio_glue_func,
    pub start_table_object: visio_glue_func_pl,
    pub end_table_object: visio_glue_func,
    pub open_table_row: visio_glue_func_pl,
    pub close_table_row: visio_glue_func,
    pub open_table_cell: visio_glue_func_pl,
    pub close_table_cell: visio_glue_func,
    pub open_ordered_list_level: visio_glue_func_pl,
    pub close_ordered_list_level: visio_glue_func,
    pub open_unordered_list_level: visio_glue_func_pl,
    pub close_unordered_list_level: visio_glue_func,
    pub open_list_element: visio_glue_func_pl,
    pub close_list_element: visio_glue_func,
    pub open_paragraph: visio_glue_func_pl,
    pub close_paragraph: visio_glue_func,
    pub open_span: visio_glue_func_pl,
    pub close_span: visio_glue_func,
    pub open_link: visio_glue_func_pl,
    pub close_link: visio_glue_func,

    pub set_document_metadata: visio_glue_func_pl,
    pub define_embedded_font: visio_glue_func_pl,
    pub set_style: visio_glue_func_pl,
    pub draw_rectangle: visio_glue_func_pl,
    pub draw_ellipse: visio_glue_func_pl,
    pub draw_polygon: visio_glue_func_pl,
    pub draw_polyline: visio_glue_func_pl,
    pub draw_path: visio_glue_func_pl,
    pub draw_graphic_object: visio_glue_func_pl,
    pub draw_connector: visio_glue_func_pl,
    pub insert_covered_table_cell: visio_glue_func_pl,
    pub insert_field: visio_glue_func_pl,
    pub define_paragraph_style: visio_glue_func_pl,
    pub define_character_style: visio_glue_func_pl,

    pub insert_tab: visio_glue_func,
    pub insert_space: visio_glue_func,
    pub insert_line_break: visio_glue_func,

    pub insert_text: visio_glue_func_str,
}

#[repr(C)]
pub struct visio_glue_property_value {
    pub value: *mut c_char,
}

unsafe extern "C" {
    pub unsafe fn visio_glue_open_file(path: *const c_char) -> *mut visio_glue_input_stream;
    pub unsafe fn visio_glue_close_file(stream: *mut visio_glue_input_stream);

    pub unsafe fn visio_glue_document_is_supported(stream: *mut visio_glue_input_stream) -> bool;
    pub unsafe fn visio_glue_document_parse(stream: *mut visio_glue_input_stream, painter: *mut visio_glue_painter, user_ptr: *mut c_void) -> bool;
    pub unsafe fn visio_glue_document_parse_stencils(stream: *mut visio_glue_input_stream, painter: *mut visio_glue_painter, user_ptr: *mut c_void) -> bool;

    pub unsafe fn visio_glue_property_list_iterate(list: *const visio_glue_property_list) -> *mut visio_glue_property_list_iterator;
    pub unsafe fn visio_glue_property_list_iterator_free(iterator: *mut visio_glue_property_list_iterator);
    pub unsafe fn visio_glue_property_list_iterator_advance(iterator: *mut visio_glue_property_list_iterator) -> bool;
    pub unsafe fn visio_glue_property_list_iterator_key(iterator: *mut visio_glue_property_list_iterator) -> *const c_char;
    pub unsafe fn visio_glue_property_list_iterator_value(iterator: *mut visio_glue_property_list_iterator) -> visio_glue_property_value;
    pub unsafe fn visio_glue_property_value_free(value: *mut visio_glue_property_value);
}
