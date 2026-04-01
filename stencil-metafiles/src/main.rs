use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::ffi::{CString, OsString};
use std::path::PathBuf;

use libvisio_bridge::Painter;


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


struct EmfPainter {
    output_dir: PathBuf,
    page_name: String,
    page_number: usize,
    emf_on_page_number: usize,
}
impl EmfPainter {
    pub fn new<P: Into<PathBuf>>(output_dir: P) -> Self {
        Self {
            output_dir: output_dir.into(),
            page_name: String::with_capacity(0),
            page_number: 0,
            emf_on_page_number: 0,
        }
    }
}
impl Painter for EmfPainter {
    fn start_page(&mut self, properties: HashMap<String, String>) {
        self.page_number += 1;
        self.emf_on_page_number = 0;
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
            Some("image/emf") => {
                let binary_data_b64_opt = properties.get("office:binary-data");
                let Some(binary_data_b64) = binary_data_b64_opt
                    else { return };
                let Ok(binary_data) = STANDARD.decode(binary_data_b64)
                    else { return };

                let emf_filename = format!(
                    "{}_{}.emf",
                    normalize_filename(&self.page_name),
                    self.emf_on_page_number,
                );
                self.emf_on_page_number += 1;

                let mut emf_path = self.output_dir.clone();
                emf_path.push(&emf_filename);
                std::fs::write(&emf_path, &binary_data)
                    .expect("failed to write EMF");
            },
            _ => {},
        }
    }
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
    let mut painter: Box<dyn Painter> = Box::new(EmfPainter::new(&args[2]));
    visio_file.parse_stencils(&mut painter);
}
