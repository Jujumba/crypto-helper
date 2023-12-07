use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::asn1::RawAsn1EntityData;
use crate::length::{len_size, read_len, write_len};
use crate::reader::{read_data, Reader};
use crate::writer::Writer;
use crate::{Asn1, Asn1Decoder, Asn1Encoder, Asn1Entity, Asn1Result, Asn1Type, Tag};

/// [OctetString](https://www.oss.com/asn1/resources/asn1-made-simple/asn1-quick-reference/octetstring.html)
///
/// The ASN.1 OCTET STRING type contains arbitrary strings of octets. This type is very similar to BIT STRING,
/// except that all values must be an integral number of eight bits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OctetString<'data> {
    id: u64,
    octets: Cow<'data, [u8]>,
    inner: Option<Asn1<'data>>,
}

pub type OwnedOctetString = OctetString<'static>;

impl OctetString<'_> {
    pub const TAG: Tag = Tag(4);

    /// Returns inner octets
    pub fn octets(&self) -> &[u8] {
        &self.octets
    }

    pub fn inner(&self) -> Option<&Asn1<'_>> {
        self.inner.as_ref()
    }

    /// Returns owned version of the [OctetString]
    pub fn to_owned(&self) -> OwnedOctetString {
        OctetString {
            id: self.id,
            octets: self.octets.to_vec().into(),
            inner: self.inner.as_ref().map(|inner| inner.to_owned()),
        }
    }

    pub fn new_owned(id: u64, octets: Vec<u8>) -> OwnedOctetString {
        let inner = Asn1Type::decode_asn1_buff(&octets).ok().map(|asn1| asn1.to_owned());
        OwnedOctetString {
            id,
            octets: Cow::Owned(octets),
            inner,
        }
    }
}

impl From<Vec<u8>> for OwnedOctetString {
    fn from(data: Vec<u8>) -> Self {
        let inner = Asn1Type::decode_asn1_buff(&data).ok().map(|asn1| asn1.to_owned());
        Self {
            id: 0,
            octets: Cow::Owned(data),
            inner,
        }
    }
}

impl<'data> Asn1Decoder<'data> for OctetString<'data> {
    fn compare_tags(tag: &Tag) -> bool {
        OctetString::TAG == *tag
    }

    fn decode(reader: &mut Reader<'data>) -> Asn1Result<Self> {
        check_tag!(in: reader);

        let (len, _len_range) = read_len(reader)?;

        let data = reader.read(len)?;

        let mut inner_reader = Reader::new(data);
        inner_reader.set_next_id(reader.next_id());
        let inner = Asn1Type::decode_asn1(&mut inner_reader).ok();

        reader.set_next_id(inner_reader.next_id());

        Ok(Self {
            id: reader.next_id(),
            octets: Cow::Borrowed(data),
            inner,
        })
    }

    fn decode_asn1(reader: &mut Reader<'data>) -> Asn1Result<Asn1<'data>> {
        let tag_position = reader.full_offset();
        let data_start = reader.position();
        check_tag!(in: reader);

        let (len, len_range) = read_len(reader)?;

        let (data, data_range) = read_data(reader, len)?;

        let mut inner_reader = Reader::new(data);
        inner_reader.set_next_id(reader.next_id());
        inner_reader.set_offset(reader.full_offset());
        let inner = Asn1Type::decode_asn1(&mut inner_reader).ok();

        reader.set_next_id(inner_reader.next_id());

        Ok(Asn1 {
            raw_data: RawAsn1EntityData {
                raw_data: Cow::Borrowed(reader.data_in_range(data_start..data_range.end)?),
                tag: tag_position,
                length: len_range,
                data: data_range,
            },
            asn1_type: Box::new(Asn1Type::OctetString(Self {
                id: reader.next_id(),
                octets: Cow::Borrowed(data),
                inner,
            })),
        })
    }
}

impl Asn1Entity for OctetString<'_> {
    fn tag(&self) -> Tag {
        OctetString::TAG
    }

    fn id(&self) -> u64 {
        self.id
    }
}

impl Asn1Encoder for OctetString<'_> {
    fn needed_buf_size(&self) -> usize {
        let data_len = self.octets.len();

        1 /* tag */ + len_size(data_len) + data_len
    }

    fn encode(&self, writer: &mut Writer) -> Asn1Result<()> {
        writer.write_byte(Self::TAG.into())?;
        write_len(self.octets.len(), writer)?;
        writer.write_slice(&self.octets)
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::Reader;
    use crate::{Asn1Decoder, Asn1Encoder, OctetString};

    #[test]
    fn example() {
        let raw = [4, 8, 0, 17, 34, 51, 68, 85, 102, 119];

        let octet_string = OctetString::decode_asn1(&mut Reader::new(&raw)).unwrap();

        assert_eq!(octet_string.raw_data.tag_position(), 0);
        assert_eq!(octet_string.raw_data.length_bytes(), &[8]);
        assert_eq!(octet_string.raw_data.length_range(), 1..2);
        assert_eq!(
            &raw[octet_string.raw_data.data_range()],
            &[0, 17, 34, 51, 68, 85, 102, 119]
        );

        let mut encoded = [0; 10];

        assert_eq!(octet_string.asn1().needed_buf_size(), 10);

        octet_string.asn1().encode_buff(&mut encoded).unwrap();

        assert_eq!(encoded, raw);
    }
}
