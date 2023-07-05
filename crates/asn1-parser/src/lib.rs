#![no_std]

extern crate alloc;

#[macro_use]
mod macros;

mod asn1;
mod constructors;
mod error;
mod length;
mod reader;
mod string;
mod tag;
mod writer;

pub use asn1::{Asn1, Asn1Type};
pub use constructors::*;
pub use error::Error;
use reader::Reader;
pub use string::*;
pub use tag::Tag;
use writer::Writer;

pub type Asn1Result<T> = Result<T, Error>;

/// General trait for decoding asn1 entities.
pub trait Asn1Decode<'data>: Sized {
    /// Check if the provided tag belongs to decoding implementation.
    fn compare_tags(tag: &Tag) -> bool;

    /// Decodes the asn1 entity using provided Reader.
    fn decode(reader: &mut Reader<'data>) -> Asn1Result<Self>;

    /// Decodes the asn1 entity using provided Reader.
    fn decode_asn1(reader: &mut Reader<'data>) -> Asn1Result<Asn1<'data>>;
}

/// General trait for encoding asn1 entities
pub trait Asn1Encode {
    /// Returns needed buffer size for asn1 entity encoding
    fn needed_buf_size(&self) -> usize;

    /// Encodes asn1 entity into provided buffer
    fn encode_buff(&self, buf: &mut [u8]) -> Asn1Result<()> {
        self.encode(&mut Writer::new(buf))
    }

    //// Encodes asn1 entity into provided writer
    fn encode(&self, writer: &mut Writer) -> Asn1Result<()>;
}

/// Every asn1 entity should implement this trait.
pub trait Asn1Entity {
    /// Returns asn1 tag of the entity
    fn tag(&self) -> &Tag;
}
