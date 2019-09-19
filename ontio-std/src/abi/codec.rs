use super::Error;
use super::{Decoder2, Encoder};
use super::{Sink};

use crate::abi::{Source};
use crate::cmp;
use crate::prelude::*;
use crate::types::{Address, H256, U256};

impl<'a> Decoder2<'a> for u8 {
    fn decode2(source: &mut Source<'a>) -> Result<Self, Error> {
        source.read_byte()
    }
}

impl<'a> Decoder2<'a> for &'a Address {
    fn decode2(source: &mut Source<'a>) -> Result<Self, Error> {
        source.read_address()
    }
}

impl<'a> Decoder2<'a> for Address {
    fn decode2(source: &mut Source<'a>) -> Result<Self, Error> {
        let mut addr = Address::zero();
        source.read_into(addr.as_mut())?;
        Ok(addr)
    }
}

impl<'a> Decoder2<'a> for &'a [u8] {
    fn decode2(source: &mut Source<'a>) -> Result<Self, Error> {
        source.read_bytes()
    }
}
//
//impl<'a> Decoder2<'a> for Vec<u8> {
//    fn decode2(source: &mut Source<'a>) -> Result<Self, Error> {
//        Ok(source.read_bytes()?.to_vec())
//    }
//}

impl<'a, T: Decoder2<'a>> Decoder2<'a> for Vec<T> {
    fn decode2(source: &mut Source<'a>) -> Result<Self, Error> {
        let len = source.read_varuint()?;
        let mut value = Vec::with_capacity(cmp::min(len, 1024) as usize);
        for _i in 0..len {
            value.push(source.read::<T>()?);
        }

        Ok(value)
    }
}

impl<'a> Decoder2<'a> for &'a str {
    fn decode2(source: &mut Source<'a>) -> Result<Self, Error> {
        let buf = source.read_bytes()?;
        str::from_utf8(buf).map_err(|_| Error::InvalidUtf8)
    }
}

impl<'a> Decoder2<'a> for String {
    fn decode2(source: &mut Source<'a>) -> Result<Self, Error> {
        let s: &str = source.read()?;
        Ok(s.to_string())
    }
}

impl<'a> Decoder2<'a> for u16 {
    fn decode2(source: &mut Source<'a>) -> Result<Self, Error> {
        source.read_u16()
    }
}

impl<'a> Decoder2<'a> for u32 {
    fn decode2(source: &mut Source<'a>) -> Result<Self, Error> {
        source.read_u32()
    }
}

impl<'a> Decoder2<'a> for u64 {
    fn decode2(source: &mut Source<'a>) -> Result<Self, Error> {
        source.read_u64()
    }
}

impl<'a> Decoder2<'a> for bool {
    fn decode2(source: &mut Source<'a>) -> Result<Self, Error> {
        source.read_bool()
    }
}

impl<'a> Decoder2<'a> for &'a H256 {
    fn decode2(source: &mut Source<'a>) -> Result<Self, Error> {
        source.read_h256()
    }
}

impl<'a> Decoder2<'a> for U256 {
    fn decode2(source: &mut Source<'a>) -> Result<Self, Error> {
        source.read_u256()
    }
}

impl<'a> Decoder2<'a> for u128 {
    fn decode2(source: &mut Source<'a>) -> Result<Self, Error> {
        source.read_u128()
    }
}

impl Encoder for u8 {
    fn encode(&self, sink: &mut Sink) {
        sink.write_byte(*self)
    }
}

impl Encoder for u16 {
    fn encode(&self, sink: &mut Sink) {
        sink.write_u16(*self)
    }
}

impl Encoder for u32 {
    fn encode(&self, sink: &mut Sink) {
        sink.write_u32(*self)
    }
}

impl Encoder for u128 {
    fn encode(&self, sink: &mut Sink) {
        sink.write_bytes(&self.to_le_bytes())
    }
}


impl Encoder for i128 {
    fn encode(&self, sink: &mut Sink) {
        sink.write_bytes(&self.to_le_bytes())
    }
}

impl Encoder for u64 {
    fn encode(&self, sink: &mut Sink) {
        sink.write_u64(*self)
    }
}

impl Encoder for bool {
    fn encode(&self, sink: &mut Sink) {
        sink.write_bool(*self)
    }
}


impl Encoder for Address {
    fn encode(&self, sink: &mut Sink) {
        sink.write_bytes(self.as_ref())
    }
}


impl Encoder for H256 {
    fn encode(&self, sink: &mut Sink) {
        sink.write_bytes(self.as_ref())
    }
}


impl Encoder for U256 {
    fn encode(&self, sink: &mut Sink) {
        let mut buf = [0; 32];
        self.to_little_endian(&mut buf);
        sink.write_bytes(&buf)
    }
}


impl<T: Encoder> Encoder for Vec<T> {
    fn encode(&self, sink: &mut Sink) {
        self.as_slice().encode(sink);
    }
}

impl<T> Encoder for &[T]
where
    T: Encoder,
{
    fn encode(&self, sink: &mut Sink) {
        sink.write_varuint(self.len() as u64);
        for item in *self {
            sink.write(item);
        }
    }
}

impl Encoder for &str {
    fn encode(&self, sink: &mut Sink) {
        sink.write_varuint(self.len() as u64);
        sink.write_bytes(self.as_bytes());
    }
}

impl Encoder for String {
    fn encode(&self, sink: &mut Sink) {
        self.as_str().encode(sink)
    }
}

impl<T: Encoder> Encoder for &T {
    fn encode(&self, sink: &mut Sink) {
        (*self).encode(sink)
    }
}

macro_rules! impl_abi_codec_fixed_array {
    () => {};
    ($num:expr) => {
        impl<'a> Decoder2<'a> for [u8; $num] {
            fn decode2(source: &mut Source<'a>) -> Result<Self, Error> {
                let mut array = [0;$num];
                source.read_into(&mut array)?;
                Ok(array)
            }
        }

        impl Encoder for [u8; $num] {
            fn encode(&self, sink: &mut Sink) {
                sink.write_bytes(self)
            }
        }
    } ;
    ($num:expr, $($tail:expr),*) => {
        impl_abi_codec_fixed_array!($num);
        impl_abi_codec_fixed_array!($($tail),*);
     };
}

impl_abi_codec_fixed_array!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32
);

/// reference:
/// 1. https://github.com/rust-lang/rust/issues/24830
/// 2. https://github.com/rust-lang/rust/blob/8f5b5f94dcdb9884737dfbc8efd893d1d70f0b14/src/libcore/hash/mod.rs#L239
/// 3. https://github.com/rust-num/num/pull/89/files
macro_rules! for_each_tuple_ {
    ($m:ident !!) => {
        $m! { }
    };
    ($m:ident !! $h:ident, $($t:ident,)*) => {
        $m! { $h $($t)* }
        for_each_tuple_! { $m !! $($t,)* }
    }
}
macro_rules! for_each_tuple {
    ($($m:tt)*) => {
        macro_rules! m { $($m)* }
        for_each_tuple_! { m !! A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, }
    }
}

//trace_macros!(true);
for_each_tuple! {
    ($($item:ident)*) => {
        impl<'a, $($item: Decoder2<'a>),*> Decoder2<'a> for ($($item,)*) {
            fn decode2(_source: &mut Source<'a>) -> Result<Self, Error> {
                Ok(($(_source.read::<$item>()?,)*))
            }
        }

        impl<$($item: Encoder),*> Encoder for ($($item,)*) {
            fn encode(&self, _sink: &mut Sink) {
                #[allow(non_snake_case)]
                let ($($item,)*) = self;
                $(_sink.write($item);)*
            }
        }
    }
}

