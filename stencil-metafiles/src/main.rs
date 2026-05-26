use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::path::PathBuf;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use tracing::debug;

use libvisio_bridge::{InputStream, Painter};


const FORBIDDEN_FILENAME_CHARS_SORTED: [char; 9] = [
    '"', '*', '/', ':', '<', '>', '?', '\\', '|',
];


fn normalize_filename(potential_filename: &str) -> String {
    let mut ret = String::new();
    for c in potential_filename.chars() {
        if c < ' ' || c > '~' {
            // not printable ASCII
            ret.push('_');
            continue;
        }
        if FORBIDDEN_FILENAME_CHARS_SORTED.binary_search(&c).is_ok() {
            // forbidden character
            ret.push('_');
            continue;
        }
        ret.push(c);
    }
    ret
}


struct LoggingPainter<P: Painter> {
    inner: P,
}
impl<P: Painter> LoggingPainter<P> {
    pub fn new(inner: P) -> Self {
        Self {
            inner,
        }
    }

    pub fn into_inner(self) -> P {
        self.inner
    }
}
impl<P: Painter> Painter for LoggingPainter<P> {
    fn start_document(&mut self, properties: HashMap<String, String>) {
        debug!("start_document properties={:?}", properties);
        self.inner.start_document(properties)
    }

    fn end_document(&mut self) {
        debug!("end_document");
        self.inner.end_document()
    }

    fn start_page(&mut self, properties: HashMap<String, String>) {
        debug!("start_page properties={:?}", properties);
        self.inner.start_page(properties)
    }

    fn end_page(&mut self) {
        debug!("end_page");
        self.inner.end_page()
    }

    fn start_master_page(&mut self, properties: HashMap<String, String>) {
        debug!("start_master_page: {:?}", properties);
        self.inner.start_master_page(properties)
    }

    fn end_master_page(&mut self) {
        debug!("end_master_page");
        self.inner.end_master_page()
    }

    fn start_layer(&mut self, properties: HashMap<String, String>) {
        debug!("start_layer: {:?}", properties);
        self.inner.start_layer(properties)
    }

    fn end_layer(&mut self) {
        debug!("end_layer");
        self.inner.end_layer()
    }

    fn start_embedded_graphics(&mut self, properties: HashMap<String, String>) {
        debug!("start_embedded_graphics: {:?}", properties);
        self.inner.start_embedded_graphics(properties)
    }

    fn end_embedded_graphics(&mut self) {
        debug!("end_embedded_graphics");
        self.inner.end_embedded_graphics()
    }

    fn open_group(&mut self, properties: HashMap<String, String>) {
        debug!("open_group: {:?}", properties);
        self.inner.open_group(properties)
    }

    fn close_group(&mut self) {
        debug!("close_group");
        self.inner.close_group()
    }

    fn start_text_object(&mut self, properties: HashMap<String, String>) {
        debug!("start_text_object: {:?}", properties);
        self.inner.start_text_object(properties)
    }

    fn end_text_object(&mut self) {
        debug!("end_text_object");
        self.inner.end_text_object()
    }

    fn start_table_object(&mut self, properties: HashMap<String, String>) {
        debug!("start_table_object: {:?}", properties);
        self.inner.start_table_object(properties)
    }

    fn end_table_object(&mut self) {
        debug!("end_table_object");
        self.inner.end_table_object()
    }

    fn open_table_row(&mut self, properties: HashMap<String, String>) {
        debug!("open_table_row: {:?}", properties);
        self.inner.open_table_row(properties)
    }

    fn close_table_row(&mut self) {
        debug!("close_table_row");
        self.inner.close_table_row()
    }

    fn open_table_cell(&mut self, properties: HashMap<String, String>) {
        debug!("open_table_cell: {:?}", properties);
        self.inner.open_table_cell(properties)
    }

    fn close_table_cell(&mut self) {
        debug!("close_table_cell");
        self.inner.close_table_cell()
    }

    fn open_ordered_list_level(&mut self, properties: HashMap<String, String>) {
        debug!("open_ordered_list_level: {:?}", properties);
        self.inner.open_ordered_list_level(properties)
    }

    fn close_ordered_list_level(&mut self) {
        debug!("close_ordered_list_level");
        self.inner.close_ordered_list_level()
    }

    fn open_unordered_list_level(&mut self, properties: HashMap<String, String>) {
        debug!("open_unordered_list_level: {:?}", properties);
        self.inner.open_unordered_list_level(properties)
    }

    fn close_unordered_list_level(&mut self) {
        debug!("close_unordered_list_level");
        self.inner.close_unordered_list_level()
    }

    fn open_list_element(&mut self, properties: HashMap<String, String>) {
        debug!("open_list_element: {:?}", properties);
        self.inner.open_list_element(properties)
    }

    fn close_list_element(&mut self) {
        debug!("close_list_element");
        self.inner.close_list_element()
    }

    fn open_paragraph(&mut self, properties: HashMap<String, String>) {
        debug!("open_paragraph: {:?}", properties);
        self.inner.open_paragraph(properties)
    }

    fn close_paragraph(&mut self) {
        debug!("close_paragraph");
        self.inner.close_paragraph()
    }

    fn open_span(&mut self, properties: HashMap<String, String>) {
        debug!("open_span: {:?}", properties);
        self.inner.open_span(properties)
    }

    fn close_span(&mut self) {
        debug!("close_span");
        self.inner.close_span()
    }

    fn open_link(&mut self, properties: HashMap<String, String>) {
        debug!("open_link: {:?}", properties);
        self.inner.open_link(properties)
    }

    fn close_link(&mut self) {
        debug!("close_link");
        self.inner.close_link()
    }

    fn set_document_metadata(&mut self, properties: HashMap<String, String>) {
        debug!("set_document_metadata: {:?}", properties);
        self.inner.set_document_metadata(properties)
    }

    fn define_embedded_font(&mut self, properties: HashMap<String, String>) {
        debug!("define_embedded_font: {:?}", properties);
        self.inner.define_embedded_font(properties)
    }

    fn set_style(&mut self, properties: HashMap<String, String>) {
        debug!("set_style: {:?}", properties);
        self.inner.set_style(properties)
    }

    fn draw_rectangle(&mut self, properties: HashMap<String, String>) {
        debug!("draw_rectangle: {:?}", properties);
        self.inner.draw_rectangle(properties)
    }

    fn draw_ellipse(&mut self, properties: HashMap<String, String>) {
        debug!("draw_ellipse: {:?}", properties);
        self.inner.draw_ellipse(properties)
    }

    fn draw_polygon(&mut self, properties: HashMap<String, String>) {
        debug!("draw_polygon: {:?}", properties);
        self.inner.draw_polygon(properties)
    }

    fn draw_polyline(&mut self, properties: HashMap<String, String>) {
        debug!("draw_polyline: {:?}", properties);
        self.inner.draw_polyline(properties)
    }

    fn draw_path(&mut self, properties: HashMap<String, String>) {
        debug!("draw_path: {:?}", properties);
        self.inner.draw_path(properties)
    }

    fn draw_graphic_object(&mut self, properties: HashMap<String, String>) {
        debug!("draw_graphic_object: {:?}", properties);
        self.inner.draw_graphic_object(properties)
    }

    fn draw_connector(&mut self, properties: HashMap<String, String>) {
        debug!("draw_connector: {:?}", properties);
        self.inner.draw_connector(properties)
    }

    fn insert_covered_table_cell(&mut self, properties: HashMap<String, String>) {
        debug!("insert_covered_table_cell: {:?}", properties);
        self.inner.insert_covered_table_cell(properties)
    }

    fn insert_field(&mut self, properties: HashMap<String, String>) {
        debug!("insert_field: {:?}", properties);
        self.inner.insert_field(properties)
    }

    fn define_paragraph_style(&mut self, properties: HashMap<String, String>) {
        debug!("define_paragraph_style: {:?}", properties);
        self.inner.define_paragraph_style(properties)
    }

    fn define_character_style(&mut self, properties: HashMap<String, String>) {
        debug!("define_character_style: {:?}", properties);
        self.inner.define_character_style(properties)
    }

    fn insert_tab(&mut self) {
        debug!("insert_tab");
        self.inner.insert_tab()
    }

    fn insert_space(&mut self) {
        debug!("insert_space");
        self.inner.insert_space()
    }

    fn insert_line_break(&mut self) {
        debug!("insert_line_break");
        self.inner.insert_line_break()
    }

    fn insert_text(&mut self, string: String) {
        debug!("insert_text");
        self.inner.insert_text(string)
    }
}


struct EmfPainter {
    output_dir: PathBuf,
    page_name: String,
    page_number: usize,
    graphic_on_page_number: usize,
}
impl EmfPainter {
    pub fn new<P: Into<PathBuf>>(output_dir: P) -> Self {
        Self {
            output_dir: output_dir.into(),
            page_name: String::with_capacity(0),
            page_number: 0,
            graphic_on_page_number: 0,
        }
    }
}
impl Painter for EmfPainter {
    fn start_page(&mut self, properties: HashMap<String, String>) {
        self.page_number += 1;
        self.graphic_on_page_number = 0;
        self.page_name = properties.get("draw:name")
            .cloned()
            .unwrap_or_else(|| format!("Symbol{}", self.page_number));
    }

    fn end_page(&mut self) {
        self.page_name.clear();
    }

    fn draw_graphic_object(&mut self, properties: HashMap<String, String>) {
        let mime_type = properties
            .get("librevenge:mime-type")
            .map(|mt| mt.as_str());
        match mime_type {
            Some("image/emf")|Some("image/png") => {
                let binary_data_b64_opt = properties.get("office:binary-data");
                let Some(binary_data_b64) = binary_data_b64_opt
                    else { return };
                let Ok(binary_data) = STANDARD.decode(binary_data_b64)
                    else { return };

                let extension = match mime_type {
                    Some("image/emf") => "emf",
                    Some("image/png") => "png",
                    _ => unreachable!(),
                };
                let output_filename = format!(
                    "{}_{}.{}",
                    normalize_filename(&self.page_name),
                    self.graphic_on_page_number,
                    extension,
                );
                self.graphic_on_page_number += 1;

                let mut output_path = self.output_dir.clone();
                output_path.push(&output_filename);
                std::fs::write(&output_path, &binary_data)
                    .expect("failed to write graphic");
            },
            _ => {},
        }
    }
}


fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

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

    let in_file = File::open(&args[1])
        .expect("failed to open input file");
    let in_reader = libvisio_bridge::ReadStream::new(in_file);
    let mut in_reader_box: Box<dyn InputStream> = Box::new(in_reader);

    let mut painter: Box<dyn Painter> = Box::new(LoggingPainter::new(EmfPainter::new(&args[2])));

    libvisio_bridge::parse_stencils(&mut in_reader_box, &mut painter);
}
