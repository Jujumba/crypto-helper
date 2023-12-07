use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::str::from_utf8;

use crate::asn1::RawAsn1EntityData;
use crate::length::{len_size, read_len, write_len};
use crate::reader::{read_data, Reader};
use crate::writer::Writer;
use crate::{Asn1, Asn1Decoder, Asn1Encoder, Asn1Entity, Asn1Result, Asn1Type, Tag};

/// [Utf8String](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/utf8string.html)
///
/// The ASN.1 UTF8String type is used for handling Unicode characters. UniversalString and UTF8String both support the same character set,
/// however, their encoding is different.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Utf8String<'data> {
    id: u64,
    string: Cow<'data, str>,
}

pub type OwnedUtf8String = Utf8String<'static>;

impl Utf8String<'_> {
    pub const TAG: Tag = Tag(12);

    /// Returns inner raw data
    pub fn raw_data(&self) -> &[u8] {
        self.string.as_bytes()
    }

    /// Returnds inner string data
    pub fn string(&self) -> &str {
        &self.string
    }

    /// Returns owned version of the [BitString]
    pub fn to_owned(&self) -> OwnedUtf8String {
        Utf8String {
            id: self.id,
            string: self.string.to_string().into(),
        }
    }

    pub fn new_owned(id: u64, string: String) -> Self {
        Self {
            id,
            string: Cow::Owned(string),
        }
    }
}

impl From<String> for OwnedUtf8String {
    fn from(data: String) -> Self {
        Self {
            id: 0,
            string: Cow::Owned(data),
        }
    }
}

impl From<&'static str> for OwnedUtf8String {
    fn from(data: &'static str) -> Self {
        Self {
            id: 0,
            string: Cow::Borrowed(data),
        }
    }
}

impl<'data> Asn1Decoder<'data> for Utf8String<'data> {
    fn compare_tags(tag: &Tag) -> bool {
        Utf8String::TAG == *tag
    }

    fn decode(reader: &mut Reader<'data>) -> Asn1Result<Self> {
        check_tag!(in: reader);

        let (len, _len_range) = read_len(reader)?;

        let data = reader.read(len)?;

        Ok(Self {
            id: reader.next_id(),
            string: Cow::Borrowed(from_utf8(data)?),
        })
    }

    fn decode_asn1(reader: &mut Reader<'data>) -> Asn1Result<Asn1<'data>> {
        let tag_position = reader.full_offset();
        let data_start = reader.position();
        check_tag!(in: reader);

        let (len, len_range) = read_len(reader)?;

        let (data, data_range) = read_data(reader, len)?;

        Ok(Asn1 {
            raw_data: RawAsn1EntityData {
                raw_data: Cow::Borrowed(reader.data_in_range(data_start..data_range.end)?),
                tag: tag_position,
                length: len_range,
                data: data_range,
            },
            asn1_type: Box::new(Asn1Type::Utf8String(Self {
                id: reader.next_id(),
                string: Cow::Borrowed(from_utf8(data)?),
            })),
        })
    }
}

impl Asn1Entity for Utf8String<'_> {
    fn tag(&self) -> Tag {
        Utf8String::TAG
    }

    fn id(&self) -> u64 {
        self.id
    }
}

impl Asn1Encoder for Utf8String<'_> {
    fn needed_buf_size(&self) -> usize {
        let data_len = self.string.len();

        1 /* tag */ + len_size(data_len) + data_len
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(Self::TAG.into())?;
        write_len(self.string.len(), writer)?;
        writer.write_slice(self.string.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use alloc::borrow::Cow;

    use crate::reader::Reader;
    use crate::{Asn1Decoder, Asn1Encoder, Asn1Type, Utf8String};

    #[test]
    fn example() {
        let raw = [
            12, 15, 116, 104, 101, 98, 101, 115, 116, 116, 118, 97, 114, 121, 110, 107, 97,
        ];

        let utf8_string = Utf8String::decode_asn1(&mut Reader::new(&raw)).unwrap();

        assert_eq!(utf8_string.raw_data.tag_position(), 0);
        assert_eq!(utf8_string.raw_data.length_bytes(), &[15]);
        assert_eq!(utf8_string.raw_data.length_range(), 1..2);
        assert_eq!(&raw[utf8_string.raw_data.data_range()], b"thebesttvarynka");
        assert_eq!(
            utf8_string.asn1(),
            &Asn1Type::Utf8String(Utf8String {
                id: 0,
                string: Cow::Borrowed("thebesttvarynka"),
            })
        );

        let mut encoded = [0; 17];

        assert_eq!(utf8_string.asn1().needed_buf_size(), 17);

        utf8_string.asn1().encode_buff(&mut encoded).unwrap();

        assert_eq!(encoded, raw);
    }
}
