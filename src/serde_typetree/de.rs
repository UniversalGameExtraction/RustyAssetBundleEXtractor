use std::io::{Cursor, Read, Seek};
use std::marker::PhantomData;

use super::{Error, Result};
use byteorder::{ByteOrder, ReadBytesExt};
use serde::Deserialize;
use serde::de::{DeserializeSeed, Error as _, IntoDeserializer, MapAccess, SeqAccess, Visitor};

use crate::read_ext::{ReadSeekUrexExt, ReadUrexExt};
use crate::serde_typetree::error::ErrorImpl;
use crate::typetree::TypeTreeNode;

/// A structure that deserializes typetree data into Rust values.
pub struct Deserializer<'cx, R, B> {
    typetree: &'cx TypeTreeNode,
    reader: &'cx mut R,
    marker: PhantomData<B>,
}

/// Deserialize an instance of type `T` from a a slice.
pub fn from_slice<'de, T: Deserialize<'de>, B: ByteOrder>(
    slice: &'de [u8],
    typetree: &'de TypeTreeNode,
) -> Result<T> {
    let mut reader = Cursor::new(slice);
    T::deserialize(&mut Deserializer::<_, B>::from_reader(
        &mut reader,
        typetree,
    ))
}

/// Deserialize an instance of type `T` from an I/O stream.
pub fn from_reader<'de, T: Deserialize<'de>, B: ByteOrder>(
    reader: &mut (impl Read + Seek),
    typetree: &TypeTreeNode,
) -> Result<T> {
    let deserializer = &mut Deserializer::<_, B>::from_reader(reader, typetree);
    #[cfg(feature = "serde_path_to_error")]
    return serde_path_to_error::deserialize::<_, T>(deserializer)
        .map_err(|e| Error::custom(format!("{}", e)));
    #[cfg(not(feature = "serde_path_to_error"))]
    T::deserialize(deserializer)
}

impl<'a, R: Read + Seek, B: ByteOrder> Deserializer<'a, R, B> {
    pub fn from_reader(reader: &'a mut R, typetree: &'a TypeTreeNode) -> Self {
        Deserializer {
            typetree,
            reader,
            marker: PhantomData,
        }
    }
}

macro_rules! deserialize_by {
    ($name:ident, $pat:pat, $read:path, $visit:ident) => {
        fn $name<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            let $pat = self.typetree.m_Type.as_str() else {
                return Err(Error::custom(format_args!(
                    "invalid type: {}, expected {} (at {} {})",
                    &self.typetree.m_Type,
                    &stringify!($name),
                    self.typetree.m_Type,
                    self.typetree.m_Name,
                )));
            };
            let value = $read(&mut self.reader)?;
            if self.typetree.requires_align() {
                self.reader.align4()?;
            }

            visitor.$visit(value)
        }
    };
}
macro_rules! deserialize_unsupported {
    ($name:ident, $expected:literal) => {
        fn $name<V>(self, _: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            Err(Error::custom(format_args!(
                "invalid type: {}, expected {} (at {} {})",
                &self.typetree.m_Type, &$expected, &self.typetree.m_Type, &self.typetree.m_Name,
            )))
        }
    };
}

impl<'de, R: Read + Seek, B: ByteOrder> serde::Deserializer<'de> for &mut Deserializer<'_, R, B> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        /*println!(
            "{} {} {}",
            self.reader.stream_position()?,
            self.typetree.m_Type,
            self.typetree.m_Name
        );*/
        match self.typetree.m_Type.as_str() {
            "bool" => self.deserialize_bool(visitor),
            "UInt8" => self.deserialize_u8(visitor),
            "UInt16" | "unsigned short" => self.deserialize_u16(visitor),
            "UInt32" | "unsigned int" | "Type*" => self.deserialize_u32(visitor),
            "UInt64" | "unsigned long long" | "FileSize" => self.deserialize_u64(visitor),
            "SInt8" => self.deserialize_i8(visitor),
            "SInt16" | "short" => self.deserialize_i16(visitor),
            "SInt32" | "int" => self.deserialize_i32(visitor),
            "SInt64" | "long long" => self.deserialize_i64(visitor),
            "float" => self.deserialize_f32(visitor),
            "double" => self.deserialize_f64(visitor),
            "char" => self.deserialize_char(visitor),
            "string" => self.deserialize_string(visitor),
            "map" => self.deserialize_map(visitor),
            "Type" => self.deserialize_unit(visitor), // TODO?
            // "TypelessData" => visitor.visit_byte_buf(self.reader.read_bytes::<B>()?),
            "TypelessData" => visitor.visit_seq(ByteSeqDeserializer {
                data: self.reader.read_bytes::<B>()?.into_iter(),
            }),
            "ReferencedObject" | "ReferencedObjectData" | "ManagedReferencesRegistry" => todo!(),
            _ if self.typetree.children.len() == 1
                && &self.typetree.children[0].m_Type == "Array" =>
            {
                self.deserialize_seq(visitor)
            }
            _ => {
                if self.typetree.children.is_empty() {
                    return visitor.visit_unit();
                }
                visitor.visit_map(StructDeserializer {
                    typetree: self.typetree,
                    reader: self.reader,
                    next_index: 0,
                    marker: self.marker,
                })
            }
        }
    }

    deserialize_by!(deserialize_bool, "bool", ReadUrexExt::read_bool, visit_bool);

    deserialize_by!(deserialize_u8, "UInt8", ReadBytesExt::read_u8, visit_u8);
    deserialize_by!(
        deserialize_u16,
        ("UInt16" | "unsigned short"),
        ReadBytesExt::read_u16::<B>,
        visit_u16
    );
    deserialize_by!(
        deserialize_u32,
        ("UInt32" | "unsigned int" | "Type*"),
        ReadBytesExt::read_u32<B>,
        visit_u32
    );
    deserialize_by!(
        deserialize_u64,
        ("UInt64" | "unsigned long long" | "FileSize"),
        ReadBytesExt::read_u64<B>,
        visit_u64
    );

    deserialize_by!(deserialize_i8, "SInt8", ReadBytesExt::read_i8, visit_i8);
    deserialize_by!(
        deserialize_i16,
        ("SInt16" | "short"),
        ReadBytesExt::read_i16::<B>,
        visit_i16
    );
    deserialize_by!(
        deserialize_i32,
        ("SInt32" | "int"),
        ReadBytesExt::read_i32<B>,
        visit_i32
    );
    deserialize_by!(
        deserialize_i64,
        ("SInt64" | "long long"),
        ReadBytesExt::read_i64<B>,
        visit_i64
    );

    deserialize_by!(
        deserialize_f32,
        "float",
        ReadBytesExt::read_f32<B>,
        visit_f32
    );
    deserialize_by!(
        deserialize_f64,
        "double",
        ReadBytesExt::read_f64<B>,
        visit_f64
    );

    deserialize_by!(deserialize_char, "char", ReadUrexExt::read_char, visit_char);

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        ensure_type(self.typetree, "string")?;
        let data = self.reader.read_string::<B>()?;
        self.reader.align4()?;

        visitor.visit_string(data)
    }

    deserialize_unsupported!(deserialize_bytes, "bytes");
    deserialize_unsupported!(deserialize_byte_buf, "byte buffer");

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_some(&mut *self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _: &'static str, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::invalid_type(
            serde::de::Unexpected::Other(&self.typetree.m_Type),
            &"unit struct",
        ))
    }

    fn deserialize_newtype_struct<V>(
        self,
        _: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // vector m_Component
        //   Array Array
        //     int size
        //     ComponentPair data

        if self.typetree.children.len() != 1 || self.typetree.children[0].m_Type != "Array" {
            return Err(Error::invalid_type(
                serde::de::Unexpected::Other(&self.typetree.m_Type),
                &"a list",
            ));
        };
        let array = &self.typetree.children[0];

        let length = self.reader.read_array_len::<B>()?;
        let item_type = &array.children[1];

        let mut count = length;
        let sequence = visitor.visit_seq(SeqDeserializer {
            item_de: &mut Deserializer {
                typetree: item_type,
                reader: self.reader.by_ref(),
                marker: self.marker,
            },
            count: &mut count,
        });
        if count != 0 {
            return Err(Error::custom(format_args!(
                "Deserialize implementation did not exhaust sequence: length is {length} but only read {} size",
                length - count,
                //self.typetree.dump_pretty()
            )));
        }

        if self.typetree.requires_align() || array.requires_align() {
            self.reader.align4()?;
        }

        sequence
    }

    fn deserialize_tuple<V>(self, _: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(StructDeserializer::new(self))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _: &'static str,
        _: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(StructDeserializer::new(self))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // map m_Container
        //  Array Array
        //   int size
        //   pair data
        //     TYPE first
        //     TYPE second
        if self.typetree.m_Type != "map" {
            return Err(Error::invalid_type(
                serde::de::Unexpected::Other(&self.typetree.m_Type),
                &"a map",
            ));
        };

        let size = self.reader.read_array_len::<B>()?;
        let pair = &self.typetree.children[0].children[1];
        let key_type = &pair.children[0];
        let value_type = &pair.children[1];

        let map = visitor.visit_map(MapDeserializer {
            reader: self.reader.by_ref(),
            key_type,
            value_type,
            count: size,
            marker: self.marker,
        });

        if self.typetree.requires_align() || pair.requires_align() {
            self.reader.align4()?;
        }

        map
    }

    fn deserialize_struct<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_map(StructDeserializer::new(self))
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        _: &'static [&'static str],
        _: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::new(ErrorImpl::Enum(name)))
    }

    deserialize_unsupported!(deserialize_identifier, "identifier");

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct SeqDeserializer<'cx, R, B> {
    item_de: &'cx mut Deserializer<'cx, R, B>,
    count: &'cx mut usize,
}

impl<'de, R: Read + Seek, B: ByteOrder> SeqAccess<'de> for SeqDeserializer<'_, R, B> {
    type Error = Error;

    fn next_element_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        if *self.count == 0 {
            return Ok(None);
        }
        *self.count -= 1;

        seed.deserialize(&mut *self.item_de).map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(*self.count)
    }
}

struct StructDeserializer<'cx, R, B> {
    typetree: &'cx TypeTreeNode,
    reader: &'cx mut R,
    next_index: usize,
    marker: PhantomData<B>,
}
impl<'cx, R, B> StructDeserializer<'cx, R, B> {
    fn new(de: &'cx mut Deserializer<'_, R, B>) -> Self {
        StructDeserializer {
            typetree: de.typetree,
            reader: de.reader,
            next_index: 0,
            marker: de.marker,
        }
    }
}

impl<'de, R: Read + Seek, B: ByteOrder> MapAccess<'de> for StructDeserializer<'_, R, B> {
    type Error = Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        if self.next_index >= self.typetree.children.len() {
            return Ok(None);
        }

        self.next_index += 1;
        let child = &self.typetree.children[self.next_index - 1];

        /*let name = match child.m_Name.strip_prefix("m_") {
            Some(rest) => {
                let mut str = String::with_capacity(rest.len());
                let mut chars = rest.chars();
                if let Some(first) = chars.next() {
                    str.push(first.to_ascii_lowercase());
                }
                str.extend(chars);
                Cow::Owned(str)
            }
            None => Cow::Borrowed(&child.m_Name),
        };*/

        seed.deserialize(FixedKeyDeserializer { key: &child.m_Name })
            .map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let child = &self.typetree.children[self.next_index - 1];

        seed.deserialize(&mut Deserializer {
            typetree: child,
            reader: self.reader.by_ref(),
            marker: self.marker,
        })
    }
}
impl<'de, R: Read + Seek, B: ByteOrder> SeqAccess<'de> for StructDeserializer<'_, R, B> {
    type Error = Error;

    fn next_element_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        if self.next_index >= self.typetree.children.len() {
            return Ok(None);
        }

        self.next_index += 1;
        let child = &self.typetree.children[self.next_index - 1];

        seed.deserialize(&mut Deserializer {
            typetree: child,
            reader: self.reader.by_ref(),
            marker: self.marker,
        })
        .map(Some)
    }
}

struct MapDeserializer<'cx, R, B> {
    reader: &'cx mut R,
    key_type: &'cx TypeTreeNode,
    value_type: &'cx TypeTreeNode,
    count: usize,
    marker: PhantomData<B>,
}

impl<'de, R: Read + Seek, B: ByteOrder> MapAccess<'de> for MapDeserializer<'_, R, B> {
    type Error = Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        if self.count == 0 {
            return Ok(None);
        }
        self.count -= 1;

        seed.deserialize(&mut Deserializer {
            typetree: self.key_type,
            reader: self.reader.by_ref(),
            marker: self.marker,
        })
        .map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut Deserializer {
            typetree: self.value_type,
            reader: self.reader.by_ref(),
            marker: self.marker,
        })
    }
}

struct FixedKeyDeserializer<'a> {
    key: &'a str,
}

impl<'de> serde::Deserializer<'de> for FixedKeyDeserializer<'_> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.key)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.key)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.key)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char string bytes byte_buf
        option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum ignored_any
    }
}

struct ByteSeqDeserializer {
    data: std::vec::IntoIter<u8>,
}
impl<'de> serde::de::SeqAccess<'de> for ByteSeqDeserializer {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.data.next() {
            Some(value) => seed.deserialize(value.into_deserializer()).map(Some),
            None => Ok(None),
        }
    }
}

fn ensure_type(tt: &TypeTreeNode, expected: &str) -> Result<(), Error> {
    if tt.m_Type != expected {
        return Err(Error::custom(format_args!(
            "invalid type: {}, expected {} (at {} {})",
            tt.m_Type, &expected, tt.m_Type, tt.m_Name
        )));
    };

    Ok(())
}
