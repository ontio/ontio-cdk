mod list;

pub use self::list::ListStore;

use super::abi::{Decoder, Encoder, Sink, Source};
use super::prelude::*;
use super::runtime;

pub fn get<K: AsRef<[u8]>, T>(key: K) -> Option<T>
where
    for<'a> T: Decoder<'a> + 'static,
{
    runtime::storage_read(key.as_ref()).map(|val| {
        let mut source = Source::new(&val);
        source.read().unwrap()
    })
}

pub fn put<K: AsRef<[u8]>, T: Encoder>(key: K, val: T) {
    let mut sink = Sink::new(12);
    sink.write(val);
    runtime::storage_write(key.as_ref(), sink.bytes());
}

pub fn delete<K: AsRef<[u8]>>(key: K) {
    runtime::storage_delete(key.as_ref());
}
