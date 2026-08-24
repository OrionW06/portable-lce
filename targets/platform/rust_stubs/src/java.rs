use std::io::{Read, Write, Result};

#[derive(Debug, Clone)]
pub struct Buffer {
    pub capacity: usize,
    pub position: usize,
    pub limit: usize,
}

impl Buffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            position: 0,
            limit: capacity,
        }
    }

    pub fn clear(&mut self) {
        self.position = 0;
        self.limit = self.capacity;
    }

    pub fn flip(&mut self) {
        self.limit = self.position;
        self.position = 0;
    }

    pub fn remaining(&self) -> usize {
        self.limit.saturating_sub(self.position)
    }

    pub fn has_remaining(&self) -> bool {
        self.remaining() > 0
    }
}

#[derive(Debug, Clone)]
pub struct ByteBuffer {
    pub buffer: Buffer,
    pub data: Vec<u8>,
}

impl ByteBuffer {
    pub fn allocate(capacity: usize) -> Self {
        Self {
            buffer: Buffer::new(capacity),
            data: vec![0u8; capacity],
        }
    }

    pub fn put(&mut self, byte: u8) {
        if self.buffer.position < self.buffer.limit {
            self.data[self.buffer.position] = byte;
            self.buffer.position += 1;
        }
    }

    pub fn get(&mut self) -> u8 {
        if self.buffer.position < self.buffer.limit {
            let byte = self.data[self.buffer.position];
            self.buffer.position += 1;
            byte
        } else {
            0
        }
    }
}

#[derive(Debug, Clone)]
pub struct FloatBuffer {
    pub buffer: Buffer,
    pub data: Vec<f32>,
}

impl FloatBuffer {
    pub fn allocate(capacity: usize) -> Self {
        Self {
            buffer: Buffer::new(capacity),
            data: vec![0.0f32; capacity],
        }
    }

    pub fn put(&mut self, val: f32) {
        if self.buffer.position < self.buffer.limit {
            self.data[self.buffer.position] = val;
            self.buffer.position += 1;
        }
    }

    pub fn get(&mut self) -> f32 {
        if self.buffer.position < self.buffer.limit {
            let val = self.data[self.buffer.position];
            self.buffer.position += 1;
            val
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntBuffer {
    pub buffer: Buffer,
    pub data: Vec<i32>,
}

impl IntBuffer {
    pub fn allocate(capacity: usize) -> Self {
        Self {
            buffer: Buffer::new(capacity),
            data: vec![0i32; capacity],
        }
    }

    pub fn put(&mut self, val: i32) {
        if self.buffer.position < self.buffer.limit {
            self.data[self.buffer.position] = val;
            self.buffer.position += 1;
        }
    }

    pub fn get(&mut self) -> i32 {
        if self.buffer.position < self.buffer.limit {
            let val = self.data[self.buffer.position];
            self.buffer.position += 1;
            val
        } else {
            0
        }
    }
}

pub struct ByteArrayInputStream {
    data: Vec<u8>,
    offset: usize,
}

impl ByteArrayInputStream {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data, offset: 0 }
    }
}

impl Read for ByteArrayInputStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let avail = self.data.len().saturating_sub(self.offset);
        let to_copy = buf.len().min(avail);
        if to_copy > 0 {
            buf[..to_copy].copy_from_slice(&self.data[self.offset..self.offset + to_copy]);
            self.offset += to_copy;
        }
        Ok(to_copy)
    }
}

#[derive(Default)]
pub struct ByteArrayOutputStream {
    pub buf: Vec<u8>,
}

impl ByteArrayOutputStream {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Write for ByteArrayOutputStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

pub struct DataInputStream<R: Read> {
    inner: R,
}

impl<R: Read> DataInputStream<R> {
    pub fn new(inner: R) -> Self {
        Self { inner }
    }

    pub fn read_byte(&mut self) -> Result<i8> {
        let mut buf = [0u8; 1];
        self.inner.read_exact(&mut buf)?;
        Ok(buf[0] as i8)
    }

    pub fn read_short(&mut self) -> Result<i16> {
        let mut buf = [0u8; 2];
        self.inner.read_exact(&mut buf)?;
        Ok(i16::from_be_bytes(buf))
    }

    pub fn read_int(&mut self) -> Result<i32> {
        let mut buf = [0u8; 4];
        self.inner.read_exact(&mut buf)?;
        Ok(i32::from_be_bytes(buf))
    }
}

pub struct DataOutputStream<W: Write> {
    inner: W,
}

impl<W: Write> DataOutputStream<W> {
    pub fn new(inner: W) -> Self {
        Self { inner }
    }

    pub fn write_byte(&mut self, val: i8) -> Result<()> {
        self.inner.write_all(&[val as u8])
    }

    pub fn write_short(&mut self, val: i16) -> Result<()> {
        self.inner.write_all(&val.to_be_bytes())
    }

    pub fn write_int(&mut self, val: i32) -> Result<()> {
        self.inner.write_all(&val.to_be_bytes())
    }
}

pub struct BufferedOutputStream<W: Write> {
    inner: W,
    buffer: Vec<u8>,
    capacity: usize,
}

impl<W: Write> BufferedOutputStream<W> {
    pub fn new(inner: W, capacity: usize) -> Self {
        Self {
            inner,
            buffer: Vec::with_capacity(capacity),
            capacity,
        }
    }
}

impl<W: Write> Write for BufferedOutputStream<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if self.buffer.len() + buf.len() >= self.capacity {
            self.flush()?;
        }
        if buf.len() >= self.capacity {
            self.inner.write_all(buf)?;
        } else {
            self.buffer.extend_from_slice(buf);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        if !self.buffer.is_empty() {
            self.inner.write_all(&self.buffer)?;
            self.buffer.clear();
        }
        self.inner.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_java_buffers() {
        let mut bb = ByteBuffer::allocate(10);
        bb.put(0x12);
        bb.put(0x34);
        bb.buffer.flip();
        assert_eq!(bb.get(), 0x12);
        assert_eq!(bb.get(), 0x34);

        let mut fb = FloatBuffer::allocate(5);
        fb.put(1.23);
        fb.buffer.flip();
        assert_eq!(fb.get(), 1.23);

        let mut ib = IntBuffer::allocate(5);
        ib.put(42);
        ib.buffer.flip();
        assert_eq!(ib.get(), 42);
    }

    #[test]
    fn test_java_data_streams() {
        let mut baos = ByteArrayOutputStream::new();
        {
            let mut dos = DataOutputStream::new(&mut baos);
            dos.write_byte(12).unwrap();
            dos.write_short(1024).unwrap();
            dos.write_int(65536).unwrap();
        }

        let mut bais = ByteArrayInputStream::new(baos.buf);
        let mut dis = DataInputStream::new(&mut bais);

        assert_eq!(dis.read_byte().unwrap(), 12);
        assert_eq!(dis.read_short().unwrap(), 1024);
        assert_eq!(dis.read_int().unwrap(), 65536);
    }
}
