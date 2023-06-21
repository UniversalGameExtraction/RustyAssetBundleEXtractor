#![allow(clippy::redundant_closure_call)]
use crate::commonstring::COMMONSTRING;
use crate::read_ext::ReadSeekUrexExt;
use crate::read_ext::ReadUrexExt;
use bitflags::bitflags;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io::{Read, Seek};

bitflags! {
    struct TransferMetaFlags: i32 {
        const NO_TRANSFER_FLAGS = 0;
        /// Putting this mask in a transfer will make the variable be hidden in the property editor
        const HIDE_IN_EDITOR_MASK = 1 << 0;

        /// Makes a variable not editable in the property editor
        const NOT_EDITABLE_MASK = 1 << 4;

        /// There are 3 types of PPtrs: kStrongPPtrMask, default (weak pointer)
        /// a Strong PPtr forces the referenced object to be cloned.
        /// A Weak PPtr doesnt clone the referenced object, but if the referenced object is being cloned anyway (eg. If another (strong) pptr references this object)
        /// this PPtr will be remapped to the cloned object
        /// If an  object  referenced by a WeakPPtr is not cloned, it will stay the same when duplicating and cloning, but be NULLed when templating
        const STRONG_PPTR_MASK = 1 << 6;
        // unused  = 1 << 7,

        /// kEditorDisplaysCheckBoxMask makes an integer variable appear as a checkbox in the editor
        const EDITOR_DISPLAYS_CHECK_BOX_MASK = 1 << 8;

        // unused = 1 << 9,
        // unused = 1 << 10,

        /// Show in simplified editor
        const SIMPLE_EDITOR_MASK = 1 << 11;

        /// When the options of a serializer tells you to serialize debug properties kSerializeDebugProperties
        /// All debug properties have to be marked kDebugPropertyMask
        /// Debug properties are shown in expert mode in the inspector but are not serialized normally
        const DEBUG_PROPERTY_MASK = 1 << 12;

        const ALIGN_BYTES_FLAG = 1 << 14;
        const ANY_CHILD_USES_ALIGN_BYTES_FLAG = 1 << 15;
        const IGNORE_WITH_INSPECTOR_UNDO_MASK = 1 << 16;

        // unused = 1 << 18,

        // Ignore this property when reading or writing .meta files
        const IGNORE_IN_META_FILES = 1 << 19;

        // When reading meta files and this property is not present, read array entry name instead (for backwards compatibility).
        const TRANSFER_AS_ARRAY_ENTRY_NAME_IN_META_FILES = 1 << 20;

        // When writing YAML Files, uses the flow mapping style (all properties in one line, with "{}").
        const TRANSFER_USING_FLOW_MAPPING_STYLE = 1 << 21;

        // Tells SerializedProperty to generate bitwise difference information for this field.
        const GENERATE_BITWISE_DIFFERENCES = 1 << 22;

        const DONT_ANIMATE = 1 << 23;
    }
}

macro_rules! generate_read_as {
    ($format: expr, $result: ty, $conv_i8: expr, $conv_u8: expr, $conv_i16: expr, $conv_u16: expr, $conv_i32: expr, $conv_u32: expr, $conv_i64: expr, $conv_u64: expr, $conv_f32: expr, $conv_f64: expr, $conv_bool: expr, $conv_str: expr, $conv_bytes: expr, $conv_map: expr, $conv_array: expr, $conv_cls: expr) => {
        paste::item! {
        #[doc = "Parses the data as of the object into the " $format "."]
        pub fn [< read_as_ $format >]<R: Read + Seek, B: ByteOrder> (&self, reader: &mut R,) -> Result<$result, std::io::Error>{
            // pub fn read_as_json2<R: Read + Seek, B: ByteOrder>(
            //     &self,
            //     reader: &mut R,
            // ) -> Result<serde_json::Value, std::io::Error> {
                let mut align = self.requires_align();
                let value: $result = match self.m_Type.as_str() {
                    "SInt8" => {
                        $conv_i8(reader.read_i8().unwrap())
                    }
                    "UInt8" => {
                        $conv_u8(reader.read_u8().unwrap())
                    }
                    "char" => {
                        $conv_str(reader.read_string::<B>().unwrap())
                    }
                    "SInt16" | "short" => {
                        $conv_i16(reader.read_i16::<B>().unwrap())
                    }
                    "UInt16" | "unsigned short" => {
                        $conv_u16(reader.read_u16::<B>().unwrap())
                    }
                    "SInt32" | "int" => {
                        $conv_i32(reader.read_i32::<B>().unwrap())
                    }
                    "UInt32" | "unsigned int" | "Type*" => {
                        $conv_u32(reader.read_u32::<B>().unwrap())
                    }
                    "SInt64" | "long long" => {
                        $conv_i64(reader.read_i64::<B>().unwrap())
                    }
                    "UInt64" | "unsigned long long" | "FileSize" => {
                        $conv_u64(reader.read_u64::<B>().unwrap())
                    }
                    "float" => {
                        $conv_f32(reader.read_f32::<B>().unwrap())
                    }
                    "double" => {
                        $conv_f64(reader.read_f64::<B>().unwrap())
                    }
                    "bool" => {
                        $conv_bool(reader.read_bool().unwrap())
                    }
                    "string" => {
                        align |= &self.children[0].requires_align();
                        $conv_str(reader.read_string::<B>().unwrap())
                    }
                    "TypelessData" => $conv_bytes(&reader.read_bytes::<B>().unwrap()),
                    "map" => {
                        // map m_Container
                        //  Array Array
                        //      int size
                        //      pair data
                        //          TYPE first
                        //          TYPE second
                        //assert_eq!(self.children.len(), 1);
                        let size = reader.read_array_len::<B>().unwrap();
                        //assert_eq!(self.children[0].children.len(), 2);
                        let pair = &self.children[0].children[1];
                        align |= pair.requires_align();
                        //assert_eq!(pair.children.len(), 2);
                        let first = &pair.children[0];
                        let second = &pair.children[1];
                        $conv_map(reader, size, first, second)
                    }
                    default => {
                        // array
                        //vector m_Component // ByteSize{ffffffff}, Index{1}, Version{1}, IsArray{0}, MetaFlag{8041}
                        //  Array Array // ByteSize{ffffffff}, Index{2}, Version{1}, IsArray{1}, MetaFlag{4041}
                        //      int size // ByteSize{4}, Index{3}, Version{1}, IsArray{0}, MetaFlag{41}
                        //      ComponentPair data // ByteSize{c}, Index{4}, Version{1}, IsArray{0}, MetaFlag{41}
                        if self.children.len() == 1 && self.children[0].m_Type == "Array" {
                            let array = &self.children[0];
                            align |= array.requires_align();

                            let size = reader.read_array_len::<B>().unwrap();
                            let array_node = &array.children[1];
                            $conv_array(reader, size, array_node)
                        } else {
                            // class
                            $conv_cls(reader, &self.children)
                        }
                    }
                };
                if align {
                    reader.align4()?;
                }
                Ok(value)
            }
        }
    };
}

#[derive(Debug, Clone)]
pub struct TypeTreeNode {
    m_Version: i32,
    m_Level: u8,
    m_TypeFlags: i32,
    m_ByteSize: i32,
    m_Index: Option<i32>,
    m_MetaFlag: Option<i32>,
    m_Type: String,
    m_Name: String,
    //unsigned short children_count,
    //struct TypeTreeNodeObject **children,
    // UnityFS
    // unsigned int m_TypeStrOffset,
    // unsigned int m_NameStrOffset,
    // UnityFS - version >= 19
    m_RefTypeHash: Option<u64>,
    // UnityRaw - versin = 2
    m_VariableCount: Option<i32>,
    // helper fields
    //typehash: u32,
    children: Vec<TypeTreeNode>,
}
impl TypeTreeNode {
    pub fn from_reader<R: std::io::Read + std::io::Seek, B: ByteOrder>(
        reader: &mut R,
        version: u32,
    ) -> Result<TypeTreeNode, std::io::Error> {
        fn read_node_base<R: std::io::Read + std::io::Seek, B: ByteOrder>(
            reader: &mut R,
            version: u32,
            level: u8,
        ) -> Result<TypeTreeNode, std::io::Error> {
            let mut node = TypeTreeNode {
                m_Level: level,
                m_Type: reader.read_cstr().unwrap(),
                m_Name: reader.read_cstr().unwrap(),
                m_ByteSize: reader.read_i32::<B>().unwrap(),
                m_VariableCount: if version == 2 {
                    Some(reader.read_i32::<B>().unwrap())
                } else {
                    None
                },
                m_Index: if version != 3 {
                    Some(reader.read_i32::<B>().unwrap())
                } else {
                    None
                },
                // in version 4, m_TypeFlags are m_IsArray
                m_TypeFlags: reader.read_i32::<B>().unwrap(),
                m_Version: reader.read_i32::<B>().unwrap(),
                m_MetaFlag: if version != 3 {
                    Some(reader.read_i32::<B>().unwrap())
                } else {
                    None
                },
                m_RefTypeHash: None,
                children: Vec::new(),
            };
            let children_count = reader.read_i32::<B>().unwrap();
            node.children = (0..children_count)
                .map(|_| read_node_base::<R, B>(reader, version, node.m_Level + 1).unwrap())
                .collect();
            Ok(node)
        }
        Ok(read_node_base::<R, B>(reader, version, 0).unwrap())
    }

    pub fn blob_from_reader<R: std::io::Read + std::io::Seek, B: ByteOrder>(
        reader: &mut R,
        version: u32,
    ) -> Result<TypeTreeNode, std::io::Error> {
        // originally a list with level slicing
        // reordered here to fit the newer tree structure
        let node_size = if version >= 19 { 32 } else { 24 };
        let node_count = reader.read_i32::<B>()?;
        let string_buffer_size = reader.read_i32::<B>()?;

        let mut node_reader = std::io::Cursor::new(
            reader.read_bytes_sized(node_size as usize * node_count as usize)?,
        );
        let mut string_buffer_reader =
            std::io::Cursor::new(reader.read_bytes_sized(string_buffer_size as usize)?);

        fn read_string<R: std::io::Read + std::io::Seek, B: ByteOrder>(
            string_buffer_reader: &mut R,
            value: u32,
        ) -> Result<String, std::io::Error> {
            // TODO - cache strings
            let isOffset = (value & 0x80000000) == 0;
            if isOffset {
                string_buffer_reader
                    .seek(std::io::SeekFrom::Start(value as u64))
                    .unwrap();
                return string_buffer_reader.read_cstr();
            }
            let offset = value & 0x7FFFFFFF;

            let ret = COMMONSTRING.get(&offset);

            if let Some(ret) = ret {
                Ok(ret.to_string())
            } else {
                Ok(offset.to_string())
            }
        }

        let nodes: Vec<TypeTreeNode> = (0..node_count)
            .map(|_| TypeTreeNode {
                m_Version: node_reader.read_u16::<B>().unwrap() as i32,
                m_Level: node_reader.read_u8().unwrap(),
                m_TypeFlags: node_reader.read_u8().unwrap() as i32,
                m_Type: read_string::<std::io::Cursor<Vec<u8>>, B>(
                    &mut string_buffer_reader,
                    node_reader.read_u32::<B>().unwrap(),
                )
                .unwrap(),
                m_Name: read_string::<std::io::Cursor<Vec<u8>>, B>(
                    &mut string_buffer_reader,
                    node_reader.read_u32::<B>().unwrap(),
                )
                .unwrap(),
                m_ByteSize: node_reader.read_i32::<B>().unwrap(),
                m_Index: Some(node_reader.read_i32::<B>().unwrap()),
                m_MetaFlag: Some(node_reader.read_i32::<B>().unwrap()),
                m_RefTypeHash: if version >= 19 {
                    Some(node_reader.read_u64::<B>().unwrap())
                } else {
                    None
                },
                children: Vec::new(),
                m_VariableCount: None,
            })
            .collect();

        fn add_children(parent: &mut TypeTreeNode, nodes: &[TypeTreeNode], offset: usize) -> i32 {
            let mut added: i32 = 0;
            for i in (offset + 1)..nodes.len() {
                let mut node = nodes[i].clone();
                if node.m_Level == parent.m_Level + 1 {
                    added += add_children(&mut node, nodes, i) + 1;
                    parent.children.push(node.clone());
                } else if node.m_Level <= parent.m_Level {
                    break;
                }
            }
            added
        }

        let mut root_node = nodes[0].clone();
        let added = add_children(&mut root_node, &nodes, 0);
        if added != node_count - 1 {
            println!("Warning: not all nodes were added to the tree");
        }
        Ok(root_node)
    }

    fn requires_align(&self) -> bool {
        (self.m_MetaFlag.unwrap_or(0) & TransferMetaFlags::ALIGN_BYTES_FLAG.bits()) != 0
    }

    generate_read_as!(
        json,
        serde_json::Value,
        |x: i8| serde_json::Value::Number(serde_json::Number::from(x)),
        |x: u8| serde_json::Value::Number(serde_json::Number::from(x)),
        |x: i16| serde_json::Value::Number(serde_json::Number::from(x)),
        |x: u16| serde_json::Value::Number(serde_json::Number::from(x)),
        |x: i32| serde_json::Value::Number(serde_json::Number::from(x)),
        |x: u32| serde_json::Value::Number(serde_json::Number::from(x)),
        |x: i64| serde_json::Value::Number(serde_json::Number::from(x)),
        |x: u64| serde_json::Value::Number(serde_json::Number::from(x)),
        |x: f32| serde_json::Value::Number(serde_json::Number::from_f64(x as f64).unwrap()),
        |x: f64| serde_json::Value::Number(serde_json::Number::from_f64(x).unwrap()),
        serde_json::Value::Bool,
        serde_json::Value::String,
        (|x: &[u8]| serde_json::Value::Array(
            x.iter()
                .map(|y| serde_json::Value::Number(serde_json::Number::from(*y)))
                .collect()
        )),
        // map
        (|reader: &mut R, size: usize, first: &TypeTreeNode, second: &TypeTreeNode| {
            serde_json::Value::Array(
                (0..size)
                    .map(|_| {
                        serde_json::Value::Array(vec![
                            first.read_as_json::<R, B>(reader).unwrap(),
                            second.read_as_json::<R, B>(reader).unwrap(),
                        ])
                    })
                    .collect(),
            )
        }),
        // array
        (|reader: &mut R, size: usize, array_node: &TypeTreeNode| {
            serde_json::Value::Array(
                (0..size)
                    .map(|_| array_node.read_as_json::<R, B>(reader).unwrap())
                    .collect(),
            )
        }),
        // class
        |reader: &mut R, children: &[TypeTreeNode]| {
            let mut map = serde_json::Map::new();
            for child in children {
                map.insert(
                    child.m_Name.clone(),
                    child.read_as_json::<R, B>(reader).unwrap(),
                );
            }
            serde_json::Value::from(map)
        }
    );

    generate_read_as!(
        yaml,
        Result<serde_yaml::Value, serde_yaml::Error>,
        serde_yaml::to_value,
        serde_yaml::to_value,
        serde_yaml::to_value,
        serde_yaml::to_value,
        serde_yaml::to_value,
        serde_yaml::to_value,
        serde_yaml::to_value,
        serde_yaml::to_value,
        serde_yaml::to_value,
        serde_yaml::to_value,
        serde_yaml::to_value,
        serde_yaml::to_value,
        serde_yaml::to_value,
        // map
        (|reader: &mut R, size: usize, first: &TypeTreeNode, second: &TypeTreeNode| {
            serde_yaml::to_value(
                (0..size)
                    .map(|_| {
                        serde_yaml::to_value(vec![
                            first.read_as_yaml::<R, B>(reader).unwrap().unwrap(),
                            second.read_as_yaml::<R, B>(reader).unwrap().unwrap(),
                        ])
                    })
                    .collect::<Result<Vec<serde_yaml::Value>, _>>()?,
            )
        }),
        // array
        (|reader: &mut R, size: usize, array_node: &TypeTreeNode| {
            serde_yaml::to_value(
                (0..size)
                    .map(|_| array_node.read_as_yaml::<R, B>(reader).unwrap())
                    .collect::<Result<Vec<serde_yaml::Value>, _>>()?,
            )
        }),
        // class
        |reader: &mut R, children: &[TypeTreeNode]| {
            let mut map = serde_yaml::Mapping::new();
            for child in children {
                map.insert(
                    serde_yaml::Value::String(child.m_Name.clone()),
                    child.read_as_yaml::<R, B>(reader).unwrap().unwrap()
                );
            }
            serde_yaml::to_value(map)
        }
    );
}
