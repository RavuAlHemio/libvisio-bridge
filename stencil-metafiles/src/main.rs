use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::ffi::{CString, OsString};

use libvisio_bridge::{Painter, VisioFile};


struct LaziestPainter;
impl Painter for LaziestPainter {
    fn start_document(&mut self, properties: HashMap<String, String>) {
        println!("start_document({:?})", properties);
    }

    fn end_document(&mut self) {
        println!("end_document()");
    }

    fn start_page(&mut self, properties: HashMap<String, String>) {
        println!("start_page({:?})", properties);
    }

    fn end_page(&mut self) {
        println!("end_page()");
    }

    fn start_master_page(&mut self, properties: HashMap<String, String>) {
        println!("start_master_page({:?})", properties);
    }

    fn end_master_page(&mut self) {
        println!("end_master_page()");
    }

    fn start_layer(&mut self, properties: HashMap<String, String>) {
        println!("start_layer({:?})", properties);
    }

    fn end_layer(&mut self) {
        println!("end_layer()");
    }

    fn start_embedded_graphics(&mut self, properties: HashMap<String, String>) {
        println!("start_embedded_graphics({:?})", properties);
    }

    fn end_embedded_graphics(&mut self) {
        println!("end_embedded_graphics()");
    }

    fn open_group(&mut self, properties: HashMap<String, String>) {
        println!("open_group({:?})", properties);
    }

    fn close_group(&mut self) {
        println!("close_group()");
    }

    fn start_text_object(&mut self, properties: HashMap<String, String>) {
        println!("start_text_object({:?})", properties);
    }

    fn end_text_object(&mut self) {
        println!("end_text_object()");
    }

    fn start_table_object(&mut self, properties: HashMap<String, String>) {
        println!("start_table_object({:?})", properties);
    }

    fn end_table_object(&mut self) {
        println!("end_table_object()");
    }

    fn open_table_row(&mut self, properties: HashMap<String, String>) {
        println!("open_table_row({:?})", properties);
    }

    fn close_table_row(&mut self) {
        println!("close_table_row()");
    }

    fn open_table_cell(&mut self, properties: HashMap<String, String>) {
        println!("open_table_cell({:?})", properties);
    }

    fn close_table_cell(&mut self) {
        println!("close_table_cell()");
    }

    fn open_ordered_list_level(&mut self, properties: HashMap<String, String>) {
        println!("open_ordered_list_level({:?})", properties);
    }

    fn close_ordered_list_level(&mut self) {
        println!("close_ordered_list_level()");
    }

    fn open_unordered_list_level(&mut self, properties: HashMap<String, String>) {
        println!("open_unordered_list_level({:?})", properties);
    }

    fn close_unordered_list_level(&mut self) {
        println!("close_unordered_list_level()");
    }

    fn open_list_element(&mut self, properties: HashMap<String, String>) {
        println!("open_list_element({:?})", properties);
    }

    fn close_list_element(&mut self) {
        println!("close_list_element()");
    }

    fn open_paragraph(&mut self, properties: HashMap<String, String>) {
        println!("open_paragraph({:?})", properties);
    }

    fn close_paragraph(&mut self) {
        println!("close_paragraph()");
    }

    fn open_span(&mut self, properties: HashMap<String, String>) {
        println!("open_span({:?})", properties);
    }

    fn close_span(&mut self) {
        println!("close_span()");
    }

    fn open_link(&mut self, properties: HashMap<String, String>) {
        println!("open_link({:?})", properties);
    }

    fn close_link(&mut self) {
        println!("close_link()");
    }

    fn set_document_metadata(&mut self, properties: HashMap<String, String>) {
        println!("set_document_metadata({:?})", properties);
    }

    fn define_embedded_font(&mut self, properties: HashMap<String, String>) {
        println!("define_embedded_font({:?})", properties);
    }

    fn set_style(&mut self, properties: HashMap<String, String>) {
        println!("set_style({:?})", properties);
    }

    fn draw_rectangle(&mut self, properties: HashMap<String, String>) {
        println!("draw_rectangle({:?})", properties);
    }

    fn draw_ellipse(&mut self, properties: HashMap<String, String>) {
        println!("draw_ellipse({:?})", properties);
    }

    fn draw_polygon(&mut self, properties: HashMap<String, String>) {
        println!("draw_polygon({:?})", properties);
    }

    fn draw_polyline(&mut self, properties: HashMap<String, String>) {
        println!("draw_polyline({:?})", properties);
    }

    fn draw_path(&mut self, properties: HashMap<String, String>) {
        println!("draw_path({:?})", properties);
    }

    fn draw_graphic_object(&mut self, properties: HashMap<String, String>) {
        println!("draw_graphic_object({:?})", properties);
    }

    fn draw_connector(&mut self, properties: HashMap<String, String>) {
        println!("draw_connector({:?})", properties);
    }

    fn insert_covered_table_cell(&mut self, properties: HashMap<String, String>) {
        println!("insert_covered_table_cell({:?})", properties);
    }

    fn insert_field(&mut self, properties: HashMap<String, String>) {
        println!("insert_field({:?})", properties);
    }

    fn define_paragraph_style(&mut self, properties: HashMap<String, String>) {
        println!("define_paragraph_style({:?})", properties);
    }

    fn define_character_style(&mut self, properties: HashMap<String, String>) {
        println!("define_character_style({:?})", properties);
    }

    fn insert_tab(&mut self) {
        println!("insert_tab()");
    }

    fn insert_space(&mut self) {
        println!("insert_space()");
    }

    fn insert_line_break(&mut self) {
        println!("insert_line_break()");
    }

    fn insert_text(&mut self, _string: String) {}
}


fn main() {
    let args: Vec<OsString> = env::args_os().collect();
    if args.len() != 3 {
        eprintln!(
            "Usage: {} INFILE OUTDIR",
            args
                .get(0)
                .map(|a| a.to_string_lossy())
                .unwrap_or(Cow::Borrowed("stencil-metafiles")),
        );
        return;
    }

    let infile_c = CString::new(args[1].as_encoded_bytes())
        .expect("input file contains NUL bytes");

    let mut visio_file = VisioFile::new(&infile_c)
        .expect("failed to load Visio file");
    let mut painter: Box<dyn Painter> = Box::new(LaziestPainter);
    visio_file.parse_stencils(&mut painter);

    // TODO: extraction logic
}
