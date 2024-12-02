//wtf!!!!!!
/*use std::io::BufReader;

use positioned_io::Cursor;
use zstd::Decoder;
pub struct RangedReader<'a> {
    inner: Decoder<'a, BufReader<Cursor<std::fs::File>>>,
    pos: usize,
    end: u64,
}
impl RangedReader<'_> {
    pub fn new(file: std::fs::File, start: u64, end: u64) -> Self {
        let reader = Decoder::new({
            let mut cur = Cursor::new(file);
            cur.set_position(start);
            cur
        })
        .unwrap();
        Self { inner: reader, end,pos: start as usize }
    }
}
impl std::io::Read for RangedReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes_count = usize::min(self.end as usize - buf.len() - self.pos,buf.len());
        self.pos = self.inner.;
        self.inner.read(&mut buf[..bytes_count])?;
        Ok(bytes_count)
    }
}*/
