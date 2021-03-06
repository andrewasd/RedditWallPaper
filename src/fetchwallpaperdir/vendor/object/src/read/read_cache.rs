use std::boxed::Box;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{Read, Seek, SeekFrom};
use std::mem;

use crate::read::ReadRef;

/// An implementation of `ReadRef` for data in a stream that implements
/// `Read + Seek`.
///
/// Contains a cache of read-only blocks of data, allowing references to
/// them to be returned. Entries in the cache are never removed.
/// Entries are keyed on the offset and size of the read.
/// Currently overlapping reads are considered separate reads.
#[derive(Debug)]
pub struct ReadCache<R: Read + Seek> {
    cache: RefCell<ReadCacheInternal<R>>,
}

#[derive(Debug)]
struct ReadCacheInternal<R: Read + Seek> {
    read: R,
    bufs: HashMap<(u64, u64), Box<[u8]>>,
}

impl<R: Read + Seek> ReadCache<R> {
    /// Create an empty `ReadCache` for the given stream.
    pub fn new(read: R) -> Self {
        ReadCache {
            cache: RefCell::new(ReadCacheInternal {
                read,
                bufs: HashMap::new(),
            }),
        }
    }

    /// Return an implementation of `ReadRef` that restricts reads
    /// to the given range of the stream.
    pub fn range<'a>(&'a self, offset: u64, size: u64) -> ReadCacheRange<'a, R> {
        ReadCacheRange {
            r: self,
            offset,
            size,
        }
    }

    /// Free buffers used by the cache.
    pub fn clear(&mut self) {
        self.cache.borrow_mut().bufs.clear();
    }

    /// Unwrap this `ReadCache<R>`, returning the underlying reader.
    pub fn into_inner(self) -> R {
        self.cache.into_inner().read
    }
}

impl<'a, R: Read + Seek> ReadRef<'a> for &'a ReadCache<R> {
    fn len(self) -> Result<u64, ()> {
        let cache = &mut *self.cache.borrow_mut();
        cache.read.seek(SeekFrom::End(0)).map_err(|_| ())
    }

    fn read_bytes_at(self, offset: u64, size: u64) -> Result<&'a [u8], ()> {
        if size == 0 {
            return Ok(&[]);
        }
        let cache = &mut *self.cache.borrow_mut();
        let buf = match cache.bufs.entry((offset, size)) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let size = size.try_into().map_err(|_| ())?;
                cache
                    .read
                    .seek(SeekFrom::Start(offset as u64))
                    .map_err(|_| ())?;
                let mut bytes = vec![0; size].into_boxed_slice();
                cache.read.read_exact(&mut bytes).map_err(|_| ())?;
                entry.insert(bytes)
            }
        };
        // Extend the lifetime to that of self.
        // This is OK because we never mutate or remove entries.
        Ok(unsafe { mem::transmute::<&[u8], &[u8]>(buf) })
    }
}

/// An implementation of `ReadRef` for a range of data in a stream that
/// implements `Read + Seek`.
///
/// Shares an underlying `ReadCache` with a lifetime of `'a`.
#[derive(Debug)]
pub struct ReadCacheRange<'a, R: Read + Seek> {
    r: &'a ReadCache<R>,
    offset: u64,
    size: u64,
}

impl<'a, R: Read + Seek> Clone for ReadCacheRange<'a, R> {
    fn clone(&self) -> Self {
        Self {
            r: self.r,
            offset: self.offset,
            size: self.size,
        }
    }
}

impl<'a, R: Read + Seek> Copy for ReadCacheRange<'a, R> {}

impl<'a, R: Read + Seek> ReadRef<'a> for ReadCacheRange<'a, R> {
    fn len(self) -> Result<u64, ()> {
        Ok(self.size)
    }

    fn read_bytes_at(self, offset: u64, size: u64) -> Result<&'a [u8], ()> {
        if size == 0 {
            return Ok(&[]);
        }
        let end = offset.checked_add(size).ok_or(())?;
        if end > self.size {
            return Err(());
        }
        let r_offset = self.offset.checked_add(offset).ok_or(())?;
        self.r.read_bytes_at(r_offset, size)
    }
}
