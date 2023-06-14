use crate::commonstring::COMMONSTRING;
use crate::read_ext::ReadUrexExt;
use byteorder::{ByteOrder, ReadBytesExt};

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
}
