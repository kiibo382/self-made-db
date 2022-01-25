use std::convert::TryInto;
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, SeekFrom};
use std::path::Path;

use zerocopy::{AsBytes, FromBytes};

pub const PAGE_SIZE: usize = 4096;

// https://zenn.dev/mebiusbox/books/22d4c1ed9b0003/viewer/497a21#%F0%9F%93%8C-derive%E5%B1%9E%E6%80%A7
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, FromBytes, AsBytes)]
// https://doc.rust-jp.rs/rust-nomicon-ja/repr-rust.html
// https://doc.rust-lang.org/nomicon/repr-rust.html
#[repr(C)]
pub struct PageId(pub u64);
impl PageId {
    pub const INVALID_PAGE_ID: PageId = PageId(u64::MAX);

    pub fn valid(self) -> Option<PageId> {
        if self == Self::INVALID_PAGE_ID {
            None
        } else {
            Some(self)
        }
    }

    pub fn to_u64(self) -> u64 {
        self.0
    }
}

// traitごとに実装を分ける
// ref: https://zenn.dev/mebiusbox/books/22d4c1ed9b0003/viewer/0e7a37#%F0%9F%93%8C-%E3%83%A1%E3%82%BD%E3%83%83%E3%83%89
impl Default for PageId {
    fn default() -> Self {
        Self::INVALID_PAGE_ID
    }
}

impl From<Option<PageId>> for PageId {
    fn from(page_id: Option<PageId>) -> Self {
        page_id.unwrap_or_default()
    }
}

impl From<&[u8]> for PageId {
    fn from(bytes: &[u8]) -> Self {
        let arr = bytes.try_into().unwrap();
        PageId(u64::from_ne_bytes(arr))
    }
}

pub struct DiskManager {
  heap_file: File,
  next_page_id: u64,
}

impl DiskManager {
  pub fn new(heap_file: File) -> io::Result<Self> {
      // ?演算子はErrをうけとったときに、関数の戻り値をErrにする
      // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
      let heap_file_size = heap_file.metadata()?.len();
      let next_page_id = heap_file_size / PAGE_SIZE as u64;
      Ok(Self {
          heap_file,
          next_page_id,
      })
  }

  pub fn open(heap_file_path: impl AsRef<Path>) -> io::Result<Self> {
      let heap_file = OpenOptions::new()
          .read(true)
          .write(true)
          .create(true)
          .open(heap_file_path)?;
      Self::new(heap_file)
  }

  pub fn read_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> io::Result<()> {
      let offset = PAGE_SIZE as u64 * page_id.to_u64();
      self.heap_file.seek(SeekFrom::Start(offset))?;
      self.heap_file.read_exact(data)
  }

  pub fn write_page_data(&mut self, page_id: PageId, data: &[u8]) -> io::Result<()> {
      let offset = PAGE_SIZE as u64 * page_id.to_u64();
      self.heap_file.seek(SeekFrom::Start(offset))?;
      self.heap_file.write_all(data)
  }

  pub fn allocate_page(&mut self) -> PageId {
      let page_id = self.next_page_id;
      self.next_page_id += 1;
      PageId(page_id)
  }

  pub fn sync(&mut self) -> io::Result<()> {
      self.heap_file.flush()?;
      self.heap_file.sync_all()
  }
}
