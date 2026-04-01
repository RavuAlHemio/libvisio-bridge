use std::ffi::{CStr, CString, c_int, c_long};
use std::io::{Cursor, Read, Seek, SeekFrom};

use cfb::CompoundFile;

use crate::{InputStream, glue::visio_glue_seek_type};


pub struct ReadStream<R: Read + Seek> {
    reader: R,
    read_buf: Box<[u8]>,
}
impl<R: Read + Seek> ReadStream<R> {
    pub fn new(reader: R) -> Self {
        let read_buf = vec![0u8; 4*1024*1024].into_boxed_slice();
        Self {
            reader,
            read_buf,
        }
    }
}
impl<R: Read + Seek> InputStream for ReadStream<R> {
    fn is_structured(&mut self) -> bool {
        CompoundFile::open(&mut self.reader).is_ok()
    }

    fn sub_stream_count(&mut self) -> usize {
        let Ok(cf) = CompoundFile::open(&mut self.reader)
            else { return 0 };
        cf.walk()
            .count()
    }

    fn sub_stream_name(&mut self, stream_id: usize) -> Option<CString> {
        let cf = CompoundFile::open(&mut self.reader).ok()?;
        let entry = cf.walk().nth(stream_id)?;
        let name = CString::new(entry.name()).ok()?;
        Some(name)
    }

    fn sub_stream_exists(&mut self, stream_name: &CStr) -> bool {
        let stream_name_no_nul = stream_name.to_bytes();
        let Ok(cf) = CompoundFile::open(&mut self.reader)
            else { return false };
        cf.walk()
            .any(|e| e.name().as_bytes() == stream_name_no_nul)
    }

    fn sub_stream_by_id(&mut self, stream_id: usize) -> Option<Box<dyn InputStream>> {
        let mut cf = CompoundFile::open(&mut self.reader).ok()?;
        let entry = cf.walk().nth(stream_id)?;
        let mut stream = cf.open_stream(entry.path()).ok()?;
        let mut stream_bytes = Vec::new();
        stream.read_to_end(&mut stream_bytes).ok()?;

        let stream_cursor = Cursor::new(stream_bytes);
        let inner_stream = ReadStream::new(stream_cursor);
        Some(Box::new(inner_stream))
    }

    fn sub_stream_by_name(&mut self, stream_name: &CStr) -> Option<Box<dyn InputStream>> {
        let stream_name_no_nul = stream_name.to_bytes();
        let mut cf = CompoundFile::open(&mut self.reader).ok()?;
        let entry = cf.walk()
            .filter(|e| e.name().as_bytes() == stream_name_no_nul)
            .nth(0)?;
        let mut stream = cf.open_stream(entry.path()).ok()?;
        let mut stream_bytes = Vec::new();
        stream.read_to_end(&mut stream_bytes).ok()?;

        let stream_cursor = Cursor::new(stream_bytes);
        let inner_stream = ReadStream::new(stream_cursor);
        Some(Box::new(inner_stream))
    }

    fn read(&mut self, num_bytes: usize) -> &[u8] {
        if num_bytes == 0 {
            return &[];
        }

        let read_this_many = num_bytes.min(self.read_buf.len());
        match self.reader.read(&mut self.read_buf[..read_this_many]) {
            Ok(bytes_read) => &self.read_buf[..bytes_read],
            Err(_) => &[],
        }
    }

    #[allow(irrefutable_let_patterns)]
    fn seek(&mut self, offset: c_long, seek_type: visio_glue_seek_type) -> c_int {
        match seek_type {
            visio_glue_seek_type::Start => {
                let Ok(offset_u64) = offset.try_into()
                    else { return -1 };
                match self.reader.seek(SeekFrom::Start(offset_u64)) {
                    Ok(new_pos) => if new_pos > (c_int::MAX as u64) {
                        c_int::MAX
                    } else {
                        new_pos as c_int
                    },
                    Err(_) => -1,
                }
            },
            visio_glue_seek_type::End => {
                let Ok(offset_i64) = offset.try_into()
                    else { return -1 };
                match self.reader.seek(SeekFrom::End(offset_i64)) {
                    Ok(new_pos) => if new_pos > (c_int::MAX as u64) {
                        c_int::MAX
                    } else {
                        new_pos as c_int
                    },
                    Err(_) => -1,
                }
            },
            visio_glue_seek_type::Cur => {
                let Ok(offset_i64) = offset.try_into()
                    else { return -1 };
                match self.reader.seek(SeekFrom::Current(offset_i64)) {
                    Ok(new_pos) => if new_pos > (c_int::MAX as u64) {
                        c_int::MAX
                    } else {
                        new_pos as c_int
                    },
                    Err(_) => -1,
                }
            },
        }
    }

    fn tell(&mut self) -> c_long {
        match self.reader.seek(SeekFrom::Current(0)) {
            Ok(new_pos) => match new_pos.try_into() {
                Ok(np_signed) => np_signed,
                Err(_) => -1,
            },
            Err(_) => -1,
        }
    }

    fn is_end(&mut self) -> bool {
        let mut buf = [0u8; 1];
        match self.reader.read(&mut buf) {
            Ok(0) => true,
            Ok(1) => {
                // go back
                if let Err(_) = self.reader.seek(SeekFrom::Current(-1)) {
                    // seeking failed, give up
                    return true;
                }
                false
            },
            Ok(_) => unreachable!(),
            Err(_) => true, // an error means that this is the end
        }
    }
}
