#![allow(unused_variables)]
use std::io::{Cursor, Seek, Write};
use std::marker::PhantomData;

use crate::files::serializedfile::Endianness;
use crate::write_ext::WriteSeekExt;
use byteorder::{ByteOrder, WriteBytesExt};
use serde::Serialize;
use serde::de::Error as _;
use serde::ser::Impossible;

use crate::{serde_typetree::Error, typetree::TypeTreeNode};

pub fn to_vec<T: Serialize + ?Sized, B: ByteOrder + 'static>(
    value: &T,
    typetree: &TypeTreeNode,
) -> Result<Vec<u8>, Error> {
    let mut writer = Cursor::new(Vec::with_capacity(128));
    to_writer::<_, _, B>(&mut writer, value, typetree)?;

    Ok(writer.into_inner())
}

pub fn to_vec_endianed<T: Serialize + ?Sized>(
    value: &T,
    typetree: &TypeTreeNode,
    endianness: Endianness,
) -> Result<Vec<u8>, Error> {
    match endianness {
        Endianness::Little => to_vec::<T, byteorder::LE>(value, typetree),
        Endianness::Big => to_vec::<T, byteorder::BE>(value, typetree),
    }
}

pub fn to_writer<W: Write + Seek, T: Serialize + ?Sized, B: ByteOrder + 'static>(
    writer: &mut W,
    value: &T,
    typetree: &TypeTreeNode,
) -> Result<(), Error> {
    value.serialize(&mut Serializer::<_, B>::new(writer, typetree))
}

pub struct Serializer<'cx, W, B> {
    pub typetree: &'cx TypeTreeNode,
    pub writer: &'cx mut W,
    pub marker: PhantomData<B>,
    pub requires_align: bool,
}

impl<'cx, W, B> Serializer<'cx, W, B> {
    pub fn new(writer: &'cx mut W, typetree: &'cx TypeTreeNode) -> Self {
        Serializer {
            typetree,
            writer,
            marker: PhantomData,
            requires_align: typetree.requires_align(),
        }
    }
}

impl<'a, 'cx, W: Write + Seek, B: ByteOrder + 'static> serde::Serializer
    for &'a mut Serializer<'cx, W, B>
{
    type Ok = ();

    type Error = Error;

    type SerializeSeq = SerializerSeq<'a, W, B>;

    type SerializeTuple = SerializerTuple<'a, W, B>;

    type SerializeTupleStruct = Self;

    type SerializeTupleVariant = Self;

    type SerializeMap = SerializerMap<'a, W, B>;

    type SerializeStruct = SerializerStruct<'a, W, B>;

    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        ensure_type(self.typetree, "bool")?;
        self.writer.write_u8(v as u8)?;

        self.align_if(self.requires_align)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        todo!("{}", self.typetree.dump())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        match self.typetree.m_Type.as_str() {
            "SInt16" | "short" => self.writer.write_i16::<B>(v)?,
            _ => {
                return Err(Error::custom(format_args!(
                    "invalid type: {}, expected {} {}",
                    "i16", self.typetree.m_Type, self.typetree.m_Name,
                )));
            }
        }

        self.align_if(self.requires_align)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        match self.typetree.m_Type.as_str() {
            // "UInt8" => self.writer.write_u8(v as u8)?,
            // "UInt16" | "unsigned short" => self.writer.write_u16::<B>(v as u16)?,
            // "UInt32" | "unsigned int" | "Type*" => self.writer.write_u32::<B>(v as u32)?,
            // // "SInt8" => self.writer.write_i8(v as i8)?,
            // "SInt16" | "short" => self.writer.write_i16::<B>(v as i16)?,
            "SInt32" | "int" => self.writer.write_i32::<B>(v)?,
            _ => {
                return Err(Error::custom(format_args!(
                    "invalid type: {}, expected {} {}",
                    "i32", self.typetree.m_Type, self.typetree.m_Name,
                )));
            }
        }

        self.align_if(self.requires_align)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        match self.typetree.m_Type.as_str() {
            // "UInt8" => self.writer.write_u8(v as u8)?,
            // "UInt16" | "unsigned short" => self.writer.write_u16::<B>(v as u16)?,
            // "UInt32" | "unsigned int" | "Type*" => self.writer.write_u32::<B>(v as u32)?,
            // "UInt64" | "unsigned long long" | "FileSize" => self.writer.write_u64::<B>(v as u64)?,
            "SInt8" => self.writer.write_i8(v as i8)?,
            "SInt16" | "short" => self.writer.write_i16::<B>(v as i16)?,
            "SInt32" | "int" => self.writer.write_i32::<B>(v as i32)?,
            "SInt64" | "long long" => self.writer.write_i64::<B>(v)?,
            _ => {
                return Err(Error::custom(format_args!(
                    "invalid type: {}, expected {} {}",
                    "i64", self.typetree.m_Type, self.typetree.m_Name,
                )));
            }
        }

        self.align_if(self.requires_align)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        match self.typetree.m_Type.as_str() {
            "UInt8" => self.writer.write_u8(v)?,
            _ => {
                return Err(Error::custom(format_args!(
                    "invalid type: {}, expected {} {}",
                    "u8", self.typetree.m_Type, self.typetree.m_Name,
                )));
            }
        }

        self.align_if(self.requires_align)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        match self.typetree.m_Type.as_str() {
            // "UInt8" => self.writer.write_u8(v as u8)?,
            "UInt16" | "unsigned short" => self.writer.write_u16::<B>(v)?,
            // "SInt8" => self.writer.write_i8(v as i8)?,
            // "SInt16" | "short" => self.writer.write_i16::<B>(v as i16)?,
            _ => {
                return Err(Error::custom(format_args!(
                    "invalid type: {}, expected {} {}",
                    "u16", self.typetree.m_Type, self.typetree.m_Name,
                )));
            }
        }

        self.align_if(self.requires_align)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        match self.typetree.m_Type.as_str() {
            // "UInt8" => self.writer.write_u8(v as u8)?,
            // "UInt16" | "unsigned short" => self.writer.write_u16::<B>(v as u16)?,
            "UInt32" | "unsigned int" | "Type*" => self.writer.write_u32::<B>(v)?,
            // "SInt8" => self.writer.write_i8(v as i8)?,
            // "SInt16" | "short" => self.writer.write_i16::<B>(v as i16)?,
            // "SInt32" | "int" => self.writer.write_i32::<B>(v as i32)?,
            _ => {
                return Err(Error::custom(format_args!(
                    "invalid type: {}, expected {} {}",
                    "u32", self.typetree.m_Type, self.typetree.m_Name,
                )));
            }
        }

        self.align_if(self.requires_align)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        match self.typetree.m_Type.as_str() {
            "UInt8" => self.writer.write_u8(v as u8)?,
            "UInt16" | "unsigned short" => self.writer.write_u16::<B>(v as u16)?,
            "UInt32" | "unsigned int" | "Type*" => self.writer.write_u32::<B>(v as u32)?,
            "UInt64" | "unsigned long long" | "FileSize" => self.writer.write_u64::<B>(v)?,
            "SInt8" => self.writer.write_i8(v as i8)?,
            "SInt16" | "short" => self.writer.write_i16::<B>(v as i16)?,
            "SInt32" | "int" => self.writer.write_i32::<B>(v as i32)?,
            "SInt64" | "long long" => self.writer.write_i64::<B>(v as i64)?,
            _ => {
                return Err(Error::custom(format_args!(
                    "invalid type: {}, expected {} {}",
                    "u64", self.typetree.m_Type, self.typetree.m_Name,
                )));
            }
        }

        self.align_if(self.requires_align)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        match self.typetree.m_Type.as_str() {
            "float" => self.writer.write_f32::<B>(v)?,
            _ => {
                return Err(Error::custom(format_args!(
                    "invalid type: {}, expected {} {}",
                    "float", self.typetree.m_Type, self.typetree.m_Name,
                )));
            }
        }

        self.align_if(self.requires_align)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        match self.typetree.m_Type.as_str() {
            "float" => self.serialize_f32(v as f32)?,
            "double" => self.writer.write_f64::<B>(v)?,
            _ => {
                return Err(Error::custom(format_args!(
                    "invalid type: {}, expected {} {}",
                    "double", self.typetree.m_Type, self.typetree.m_Name,
                )));
            }
        }

        self.align_if(self.requires_align)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!("{}", self.typetree.dump())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        assert!(!self.requires_align);
        ensure_type(self.typetree, "string")?;
        self.writer.write_i32::<B>(v.len() as i32)?;
        self.writer.write_all(v.as_bytes())?;
        self.align_if(true)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!("{}", self.typetree.dump())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!("{}", self.typetree.dump())
    }

    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        todo!("{}", self.typetree.dump())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        match self.typetree.m_Type.as_str() {
            "float" => Err(Error::custom(
                "Trying to serialize unit as float. This can happen with e.g. serde_json and infinity/NaN",
            )),
            "Type" => Ok(()),
            _ => ensure_type(self.typetree, "unit"),
        }
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!("{}", self.typetree.dump())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!("{}", self.typetree.dump())
    }

    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        todo!("{}", self.typetree.dump())
    }

    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        todo!("{}", self.typetree.dump())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let len = len.expect("todo");

        if self.typetree.m_Type == "TypelessData" {
            self.writer.write_i32::<B>(len as i32)?;
            let byte_type = &self.typetree.children[1];
            assert!(!byte_type.requires_align());
            return Ok(SerializerSeq {
                item_type: byte_type,
                item_type_align: false,
                writer: self.writer,
                marker: self.marker,
                seq_align: self.typetree.requires_align(),
            });
        }

        if self.typetree.children.len() != 1 || self.typetree.children[0].m_Type != "Array" {
            return Err(Error::invalid_type(
                serde::de::Unexpected::Other(&self.typetree.m_Type),
                &"a list",
            ));
        };

        let array_type = &self.typetree.children[0];
        let item_type = &array_type.children[1];
        let requires_align = self.typetree.requires_align() || array_type.requires_align();

        self.writer.write_i32::<B>(len as i32)?;
        Ok(SerializerSeq {
            item_type: &array_type.children[1],
            item_type_align: item_type.requires_align(),
            writer: self.writer,
            marker: self.marker,
            seq_align: requires_align,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(SerializerTuple {
            typetree: self.typetree,
            writer: self.writer,
            requires_align: self.requires_align,
            marker: self.marker,

            index: 0,
        })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!("{}", self.typetree.dump())
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!("{}", self.typetree.dump())
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        match self.typetree.m_Type.as_str() {
            "vector" => todo!(),
            "array" => todo!(),
            "pair" => todo!(),
            "ReferencedObject" | "ReferencedObjectData" | "ManagedReferencesRegistry" => todo!(),
            "map" => {
                self.writer.write_i32::<B>(len.expect("TODO") as i32)?;

                let array_type = &self.typetree.children[0];
                let pair_type = &array_type.children[1];
                let key_type = &pair_type.children[0];
                let value_type = &pair_type.children[1];

                Ok(SerializerMap {
                    typetree: self.typetree,
                    writer: self.writer,
                    requires_align: self.requires_align,
                    marker: PhantomData,
                    mode: SerializerMapMode::Map {
                        key_type,
                        value_type,
                    },
                })
            }
            // "ComponentPair" => todo!("{}", self.typetree.dump()),
            "string" => todo!(),
            _ => Ok(SerializerMap {
                typetree: self.typetree,
                writer: self.writer,
                requires_align: self.requires_align,
                marker: PhantomData,
                mode: SerializerMapMode::Typed {
                    last_item: None,
                    index: 0,
                },
            }),
        }
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(SerializerStruct {
            typetree: self.typetree,
            writer: self.writer,
            requires_align: self.requires_align,
            marker: self.marker,

            index: 0,
        })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!("{}", self.typetree.dump())
    }
}

impl<W: Write + Seek, B> Serializer<'_, W, B> {
    fn align_if(&mut self, requires_align: bool) -> Result<(), Error> {
        if requires_align {
            self.writer.align::<4>()?;
        }
        Ok(())
    }
}

pub struct SerializerSeq<'cx, W, B> {
    item_type: &'cx TypeTreeNode,
    item_type_align: bool,
    writer: &'cx mut W,
    seq_align: bool,
    marker: PhantomData<B>,
}

impl<W: Write + Seek, B: ByteOrder + 'static> serde::ser::SerializeSeq for SerializerSeq<'_, W, B> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
        value.serialize(&mut Serializer {
            typetree: self.item_type,
            writer: self.writer,
            marker: self.marker,
            requires_align: self.item_type_align,
        })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.seq_align {
            self.writer.align::<4>()?;
        }
        Ok(())
    }
}

impl<W: Write, B: ByteOrder> serde::ser::SerializeTuple for &mut Serializer<'_, W, B> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
        todo!("{}", self.typetree.dump())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!("{}", self.typetree.dump())
    }
}
impl<W: Write, B: ByteOrder> serde::ser::SerializeTupleStruct for &mut Serializer<'_, W, B> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
        todo!("{}", self.typetree.dump())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!("{}", self.typetree.dump())
    }
}
impl<W: Write, B: ByteOrder> serde::ser::SerializeTupleVariant for &mut Serializer<'_, W, B> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
        todo!("{}", self.typetree.dump())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!("{}", self.typetree.dump())
    }
}

pub struct SerializerMap<'cx, W, B> {
    typetree: &'cx TypeTreeNode,
    writer: &'cx mut W,
    requires_align: bool,
    marker: PhantomData<B>,

    mode: SerializerMapMode<'cx>,
}

enum SerializerMapMode<'cx> {
    Typed {
        last_item: Option<&'cx TypeTreeNode>,
        index: usize,
    },
    Map {
        key_type: &'cx TypeTreeNode,
        value_type: &'cx TypeTreeNode,
    },
}

impl<W: Write + Seek, B> SerializerMap<'_, W, B> {
    pub fn align_if(&mut self, requires_align: bool) -> Result<(), Error> {
        if requires_align {
            self.writer.align::<4>()?;
        }
        Ok(())
    }
}

impl<'cx, W: Write + Seek, B: ByteOrder + 'static> serde::ser::SerializeMap
    for SerializerMap<'cx, W, B>
{
    type Ok = ();

    type Error = Error;

    fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<(), Self::Error> {
        match &mut self.mode {
            SerializerMapMode::Typed { last_item, index } => {
                let mut key_name = String::new();
                key.serialize(StringSerializer {
                    string: &mut key_name,
                })?;

                let (found_index, item) = self
                    .typetree
                    .children
                    .iter()
                    .enumerate()
                    .find(|(_, child)| child.m_Name == key_name)
                    .unwrap();

                if found_index != *index {
                    let fields: Vec<_> = self
                        .typetree
                        .children
                        .iter()
                        .map(|child| format!("{} {}", child.m_Type, child.m_Name))
                        .collect();
                    return Err(Error::custom(format_args!(
                        "Unexpected field order in type {}:\nExpected {}\nFound {} at index {}",
                        self.typetree.m_Type,
                        fields.join(", "),
                        key_name,
                        *index,
                    )));
                }

                *last_item = Some(item);
                *index += 1;

                Ok(())
            }
            SerializerMapMode::Map { key_type, .. } => key.serialize(&mut Serializer {
                typetree: key_type,
                writer: self.writer,
                marker: self.marker,
                requires_align: key_type.requires_align(),
            }),
        }
    }

    fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
        match self.mode {
            SerializerMapMode::Typed { last_item, index } => {
                let child = last_item.unwrap();

                value.serialize(&mut Serializer {
                    typetree: child,
                    writer: self.writer,
                    marker: self.marker,
                    requires_align: child.requires_align(),
                })?;
                Ok(())
            }
            SerializerMapMode::Map { value_type, .. } => value.serialize(&mut Serializer {
                typetree: value_type,
                writer: self.writer,
                marker: self.marker,
                requires_align: value_type.requires_align(),
            }),
        }
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.align_if(self.requires_align)
    }
}

pub struct SerializerStruct<'cx, W, B> {
    typetree: &'cx TypeTreeNode,
    writer: &'cx mut W,
    requires_align: bool,
    marker: PhantomData<B>,

    index: usize,
}

impl<W: Write + Seek, B: ByteOrder + 'static> serde::ser::SerializeStruct
    for SerializerStruct<'_, W, B>
{
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let (found_index, child_type) = self
            .typetree
            .children
            .iter()
            .enumerate()
            .find(|(_, child)| child.m_Name == key)
            .unwrap();

        if found_index != self.index {
            let fields: Vec<_> = self
                .typetree
                .children
                .iter()
                .map(|child| format!("{} {}", child.m_Type, child.m_Name))
                .collect();
            return Err(Error::custom(format_args!(
                "Unexpected field order in type {}:\nExpected {}\nFound {} at index {}",
                self.typetree.m_Type,
                fields.join(", "),
                key,
                self.index,
            )));
        }

        value.serialize(&mut Serializer {
            typetree: child_type,
            writer: self.writer,
            marker: self.marker,
            requires_align: child_type.requires_align(),
        })?;

        self.index += 1;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.requires_align {
            self.writer.align::<4>()?;
        }
        Ok(())
    }
}

impl<W: Write, B: ByteOrder> serde::ser::SerializeStructVariant for &mut Serializer<'_, W, B> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        todo!("{}", self.typetree.dump())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!("{}", self.typetree.dump())
    }
}

pub struct SerializerTuple<'cx, W, B> {
    typetree: &'cx TypeTreeNode,
    writer: &'cx mut W,
    requires_align: bool,
    marker: PhantomData<B>,

    index: usize,
}

impl<W: Write + Seek, B: ByteOrder + 'static> serde::ser::SerializeTuple
    for SerializerTuple<'_, W, B>
{
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
        let child_type = &self.typetree.children[self.index];

        value.serialize(&mut Serializer {
            typetree: child_type,
            writer: self.writer,
            marker: self.marker,
            requires_align: child_type.requires_align(),
        })?;

        self.index += 1;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.requires_align {
            self.writer.align::<4>()?;
        }
        assert_eq!(self.index, self.typetree.children.len());

        Ok(())
    }
}

fn ensure_type(tt: &TypeTreeNode, expected: &str) -> Result<(), Error> {
    if tt.m_Type != expected {
        return Err(Error::custom(format_args!(
            "invalid type: {}, expected {} (at {} {})\n{}",
            tt.m_Type,
            &expected,
            tt.m_Type,
            tt.m_Name,
            tt.dump(),
        )));
    };

    Ok(())
}

struct StringSerializer<'a> {
    string: &'a mut String,
}
impl serde::Serializer for StringSerializer<'_> {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Impossible<(), Error>;

    type SerializeTuple = Impossible<(), Error>;

    type SerializeTupleStruct = Impossible<(), Error>;

    type SerializeTupleVariant = Impossible<(), Error>;

    type SerializeMap = Impossible<(), Error>;

    type SerializeStruct = Impossible<(), Error>;

    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.string.push_str(v);
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unreachable!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unreachable!()
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unreachable!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        unreachable!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unreachable!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unreachable!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unreachable!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unreachable!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        unreachable!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unreachable!()
    }
}
