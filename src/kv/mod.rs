pub(crate) mod btree;
pub(crate) mod datafile;
pub(crate) mod filehandle;
pub(crate) mod keydir;
pub(crate) mod kvstore;

/* use std::ops::{Bound, RangeBounds};
pub trait KVEngine {

    fn set(&self, key: &[u8], value: &[u8]);

    fn get(&self, key: &[u8]) -> Option<&[u8]>;

    fn scan(&self, range: Range) -> Scan;
}

pub struct Range {
    start: Bound<Vec<u8>>,
    end: Bound<Vec<u8>>,
}

impl Range {
    pub fn from<R: RangeBounds<Vec<u8>>>(range: R) -> Self {
        Self {
            start: match range.start_bound() {
                Bound::Included(v) => Bound::Included(v.to_owned()),
                Bound::Excluded(v) => Bound::Excluded(v.to_owned()),
                Bound::Unbounded => Bound::Unbounded,
            },
            end: match range.end_bound() {
                Bound::Included(v) => Bound::Included(v.to_owned()),
                Bound::Excluded(v) => Bound::Excluded(v.to_owned()),
                Bound::Unbounded => Bound::Unbounded,
            },
        }
    }

    fn contains(&self, v: &[u8]) -> bool {
        (match &self.start {
            Bound::Included(start) => &**start <= v,
            Bound::Excluded(start) => &**start < v,
            Bound::Unbounded => true,
        }) && (match &self.end {
            Bound::Included(end) => v <= &**end,
            Bound::Excluded(end) => v < &**end,
            Bound::Unbounded => true,
        })
    }
}

impl RangeBounds<Vec<u8>> for Range {
    fn start_bound(&self) -> Bound<&Vec<u8>> {
        match &self.start {
            Bound::Included(v) => Bound::Included(v),
            Bound::Excluded(v) => Bound::Excluded(v),
            Bound::Unbounded => Bound::Unbounded,
        }
    }

    fn end_bound(&self) -> Bound<&Vec<u8>> {
        match &self.end {
            Bound::Included(v) => Bound::Included(v),
            Bound::Excluded(v) => Bound::Excluded(v),
            Bound::Unbounded => Bound::Unbounded,
        }
    }
}

pub type Scan = Box<dyn DoubleEndedIterator<Item = Result<(Vec<u8>, Vec<u8>)>>>; */
