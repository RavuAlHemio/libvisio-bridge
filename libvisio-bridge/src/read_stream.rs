use std::ffi::{CStr, CString, c_int, c_long};
use std::io::{Read, Seek, SeekFrom};

use crate::{InputStream, glue::visio_glue_seek_type};


pub struct ReadStream<R: Read + Seek> {
    reader: R,
    read_buf: Box<[u8]>,
    end_buf: Option<[u8; 1]>,
}
impl<R: Read + Seek> ReadStream<R> {
    pub fn new(reader: R) -> Self {
        let read_buf = vec![0u8; 4*1024*1024].into_boxed_slice();
        let end_buf = None;
        Self {
            reader,
            read_buf,
            end_buf,
        }
    }
}
impl<R: Read + Seek> InputStream for ReadStream<R> {
    fn is_structured(&mut self) -> bool {
        todo!();
    }

    fn sub_stream_count(&mut self) -> usize {
        todo!();
    }

    fn sub_stream_name(&mut self, _stream_id: usize) -> Option<CString> {
        todo!();
    }

    fn sub_stream_exists(&mut self, _stream_name: &CStr) -> bool {
        todo!();
    }

    fn sub_stream_by_id(&mut self, _stream_id: usize) -> Option<Box<dyn InputStream>> {
        todo!();
    }

    fn sub_stream_by_name(&mut self, _stream_name: &CStr) -> Option<Box<dyn InputStream>> {
        todo!();
    }

    fn read(&mut self, num_bytes: usize) -> &[u8] {
        if num_bytes == 0 {
            return &[];
        }

        if let Some(eb) = self.end_buf.as_ref() {
            // we previously read a byte into the is-this-the-end buffer
            // return that first and foremost
            return &eb[..];
        }

        let read_this_many = num_bytes.min(self.read_buf.len());
        match self.reader.read(&mut self.read_buf[..read_this_many]) {
            Ok(bytes_read) => {
                &self.read_buf[..bytes_read]
            },
            Err(_) => {
                &[]
            },
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
        if self.end_buf.is_some() {
            // there is still the byte from the end buffer to read
            return false;
        }

        let mut new_end_buf = [0u8; 1];
        match self.reader.read(&mut new_end_buf) {
            Ok(0) => true,
            Ok(1) => {
                // store it for later reading
                self.end_buf = Some(new_end_buf);
                false
            },
            Ok(_) => unreachable!(),
            Err(_) => true, // an error means that this is the end
        }
    }
}
