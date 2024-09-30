use anyhow::{ensure, Result};
use bytes::{Buf, Bytes, BytesMut};

// All formats are encoded in little endian

type Byte = u8;
type Int = i32;
type Long = i64;
type Float = f32;

#[derive(Debug)]
pub struct String {
    length: Int,
    /// Includes 1-2 termination characters
    chars: Bytes,
}

#[derive(Debug)]
pub struct Vec4 {
    pub w: Float,
    pub x: Float,
    pub y: Float,
    pub z: Float,
}
impl Vec4 {
    pub fn parse<B: Buf>(b: &mut B) -> Result<Self> {
        let w = b.get_f32_le();
        let x = b.get_f32_le();
        let y = b.get_f32_le();
        let z = b.get_f32_le();
        Ok(Self { w, x, y, z })
    }
}

#[derive(Debug)]
pub struct Vec3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}
impl Vec3 {
    pub fn parse<B: Buf>(b: &mut B) -> Result<Self> {
        let x = b.get_f32_le();
        let y = b.get_f32_le();
        let z = b.get_f32_le();
        Ok(Self { x, y, z })
    }
}

impl String {
    pub fn parse<B: Buf>(data: &mut B) -> Result<Self> {
        let length = data.get_i32_le();

        if length == 0 {
            Ok(Self {
                length,
                chars: Bytes::default(),
            })
        } else if length > 0 {
            // UTF 8
            let chars = data.copy_to_bytes(length as _);
            Ok(Self { length, chars })
        } else {
            // Utf 16
            let actual_length = length * -2;
            let chars = data.copy_to_bytes(actual_length as _);
            Ok(Self { length, chars })
        }
    }

    /// Return the UTF-8 encoded version of this string.
    pub fn as_str(&self) -> &str {
        if self.length == 0 {
            return "";
        }

        if self.length > 0 {
            // UTF8
            std::str::from_utf8(&self.chars[..self.chars.len() - 1]).expect("valid utf8")
        } else {
            // UTF16
            todo!("utf16")
        }
    }
}

#[derive(Debug)]
pub struct Savegame {
    pub header: SaveFileHeader,
    pub compressed_chunks: Vec<CompressedBodyChunk>,
}

impl Savegame {
    pub fn parse<B: Buf>(data: &mut B) -> Result<Self> {
        let header = SaveFileHeader::parse(data)?;

        let mut compressed_chunks = Vec::new();

        while data.has_remaining() {
            let chunk = CompressedBodyChunk::parse(data)?;
            compressed_chunks.push(chunk);
        }

        Ok(Savegame {
            header,
            compressed_chunks,
        })
    }

    pub fn uncompressed_len(&self) -> usize {
        self.compressed_chunks
            .iter()
            .map(|c| c.uncompressed_size as usize)
            .sum()
    }

    pub fn decompress(&self) -> Result<Vec<u8>> {
        use flate2::read::ZlibDecoder;
        use std::io::Read;

        let mut out = Vec::with_capacity(self.uncompressed_len());
        let mut decoder = ZlibDecoder::new(&[][..]);
        for chunk in &self.compressed_chunks {
            decoder.reset(&chunk.bytes[..]);
            decoder.read_to_end(&mut out)?;
        }

        Ok(out)
    }
}

#[derive(Debug)]
pub struct SaveFileBody {
    pub grid: Grid,
    pub levels: Vec<Level>,
}
#[derive(Debug)]
pub struct Level {
    pub name: String,
    pub object_headers: Vec<ObjectHeader>,
    pub collectables: Vec<ObjectReference>,
    pub objects: Vec<Object>,
    pub collectables_2: Vec<ObjectReference>,
}

#[derive(Debug)]
pub struct Grid {
    pub u1: String,
    pub u2: Long,
    pub u3: Int,
    pub u4: String,
    pub u5: Int,
    pub data: Vec<GridData>,
}

#[derive(Debug)]
pub struct GridData {
    pub u1: String,
    pub u2: Int,
    pub u3: Int,
    pub levels: Vec<GridLevel>,
}

#[derive(Debug)]
pub struct GridLevel {
    pub u1: String,
    pub u2: Int,
}

#[derive(Debug)]
pub struct ObjectReference {
    pub level_name: String,
    pub path_name: String,
}

impl ObjectReference {
    pub fn parse<B: Buf>(b: &mut B) -> Result<Self> {
        let level_name = String::parse(b)?;
        let path_name = String::parse(b)?;

        Ok(Self {
            level_name,
            path_name,
        })
    }
}

#[derive(Debug)]
pub enum Object {
    Component {
        entity_save_version: Int,
        properties: Properties,
    },
    Actor {
        entity_save_version: Int,
        parent_object_root: String,
        parent_object_name: String,
        components: Vec<ObjectReference>,
        properties: Properties,
    },
}

impl Object {
    pub fn parse<B: Buf>(header: &ObjectHeader, b: &mut B) -> Result<Self> {
        let entity_save_version = b.get_i32_le();
        let _unknown = b.get_i32_le();
        dbg!(_unknown);
        let binary_size = b.get_i32_le() as usize;
        let mut data = b.take(binary_size);

        match header {
            ObjectHeader::Component { .. } => {
                let properties = Properties::parse(&mut data)?;

                let remaining = data.copy_to_bytes(data.remaining());
                dbg!(&remaining);

                Ok(Self::Component {
                    entity_save_version,
                    properties
                })
            }
            ObjectHeader::Actor { .. } => {
                let parent_object_root = String::parse(&mut data)?;
                let parent_object_name = String::parse(&mut data)?;
                let components_count = data.get_i32_le() as usize;
                let mut components = Vec::with_capacity(components_count);

                for _j in 0..components_count {
                    let component = ObjectReference::parse(&mut data)?;
                    components.push(component);
                }

                let properties = if data.has_remaining() {
                    Properties::parse(&mut data)?
                } else {
                    Properties::default()
                };

                let remaining = data.copy_to_bytes(data.remaining());
                dbg!(remaining);


                Ok(Self::Actor {
                    entity_save_version,
                    parent_object_root,
                    parent_object_name,
                    components,
                    properties,
                })
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Properties {
    pub props: Vec<Property>,
}

impl Properties {
    pub fn parse<B: Buf>(b: &mut B) -> Result<Self> {
        let mut props = Vec::new();
        while b.has_remaining() {
            let prop = Property::parse(b)?;
            if prop.is_none() {
                break;
            }
            props.push(prop)
        }

        Ok(Self { props })
    }
}

#[derive(Debug)]
pub enum Property {
    Array(ArrayProperty),
    /// Signals the end of a property list
    None,
}

#[derive(Debug)]
pub struct ArrayProperty {
    pub r#type: String,
    pub index: Int,
    pub data: Vec<ArrayPropertyElement>,
}

impl ArrayProperty {
    pub fn parse<B: Buf>(b: &mut B) -> Result<Self> {
        let binary_size = b.get_i32_le();
        let index = b.get_i32_le();
        let r#type = String::parse(b)?;
        let padding = b.get_u8();
        ensure!(padding == 0, "corrupted array property: invalid padding");
        let length = b.get_i32_le();

        let typ = r#type.as_str();
        dbg!(typ, length);

        let mut data = Vec::new();

        Ok(Self {
            index,
            r#type,
            data,
        })
    }
}

#[derive(Debug)]
pub enum ArrayPropertyElement {
    Byte(u8),
    Enum(String),
    Str(String),
    Interface {
        level_name: String,
        path_name: String,
    },
    Object {
        level_name: String,
        path_name: String,
    },
    Int(Int),
    Int64(Long),
    Struct {
        name: String,
        r#type: String,
        values: Vec<TypedData>,
    }
}

#[derive(Debug)]
pub enum TypedData {

}
impl TypedData {
    pub fn parse<B: Buf>(typ: &str, b: &mut B) -> Result<Self> {
        todo!("{}", typ)
    }
}
impl Property {
    pub fn parse<B: Buf>(b: &mut B) -> Result<Self> {
        let property_name = String::parse(b)?;

        let name = property_name.as_str();
        if name == "None" || name == "" {
            return Ok(Self::None);
        }
        let type_name = String::parse(b)?;
        match dbg!(type_name.as_str()) {
            "ArrayProperty" => {
                let prop = ArrayProperty::parse(b)?;
                Ok(Self::Array(prop))
            }
            name => todo!("{}", name),
        }
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

#[derive(Debug)]
pub enum ObjectHeader {
    Component {
        type_path: String,
        root_object: String,
        instance_name: String,
        parent_actor_name: String,
    },
    Actor {
        type_path: String,
        root_object: String,
        instance_name: String,
        need_transform: Int,
        rotation: Vec4,
        position: Vec3,
        scale: Vec3,
        was_placed_in_level: Int,
    },
}

impl ObjectHeader {
    pub fn parse<B: Buf>(b: &mut B) -> Result<Self> {
        let typ = b.get_i32_le();
        match typ {
            0 => {
                // Component
                let type_path = String::parse(b)?;
                let root_object = String::parse(b)?;
                let instance_name = String::parse(b)?;
                let parent_actor_name = String::parse(b)?;

                Ok(Self::Component {
                    type_path,
                    root_object,
                    instance_name,
                    parent_actor_name,
                })
            }
            1 => {
                // Actor
                let type_path = String::parse(b)?;
                let root_object = String::parse(b)?;
                let instance_name = String::parse(b)?;
                let need_transform = b.get_i32_le();
                let rotation = Vec4::parse(b)?;
                let position = Vec3::parse(b)?;
                let scale = Vec3::parse(b)?;
                let was_placed_in_level = b.get_i32_le();
                Ok(Self::Actor {
                    type_path,
                    root_object,
                    instance_name,
                    need_transform,
                    rotation,
                    position,
                    scale,
                    was_placed_in_level,
                })
            }
            _ => anyhow::bail!("unknown object header typ: {}", typ),
        }
    }
}

impl SaveFileBody {
    pub fn parse(data_owned: Vec<u8>) -> Result<Self> {
        let mut data = BytesMut::from(&data_owned[..]);
        // Check that the size is correct
        let total_len = data.get_i64_le();
        ensure!(
            data.len() as u64 == total_len as u64,
            "invalid length: {} != {}",
            data.len(),
            total_len
        );

        let grid_count = data.get_i32_le() as usize;
        let unknown_1 = String::parse(&mut data)?;
        let unknown_2 = data.get_i64_le();
        let unknown_3 = data.get_i32_le();
        let unknown_4 = String::parse(&mut data)?;
        let unknown_5 = data.get_i32_le();

        let mut grid = Grid {
            u1: unknown_1,
            u2: unknown_2,
            u3: unknown_3,
            u4: unknown_4,
            u5: unknown_5,
            data: Vec::with_capacity(grid_count - 1),
        };

        for x in 0..grid_count - 1 {
            let unknown_6 = String::parse(&mut data)?;
            let unknown_7 = data.get_i32_le();
            let unknown_8 = data.get_i32_le();
            let level_count = data.get_i32_le() as usize;

            let mut levels = Vec::with_capacity(level_count);

            println!("Grid {}: Level Count {}", x, level_count);
            for _y in 0..level_count {
                let unknown_9 = String::parse(&mut data)?;
                let unknown_10 = data.get_i32_le();

                levels.push(GridLevel {
                    u1: unknown_9,
                    u2: unknown_10,
                });
            }

            grid.data.push(GridData {
                u1: unknown_6,
                u2: unknown_7,
                u3: unknown_8,
                levels,
            });
        }

        let level_count = data.get_i32_le() as usize;
        let mut levels = Vec::with_capacity(level_count + 1);
        for i in 0..=level_count {
            let name = String::parse(&mut data)?;
            let binary_length = data.get_i64_le();
            let object_header_count = data.get_i32_le() as usize;
            let mut object_headers = Vec::with_capacity(object_header_count);

            println!(
                "Level {}: parsing {} object headers",
                i, object_header_count
            );
            for _j in 0..object_header_count {
                let object_header = ObjectHeader::parse(&mut data)?;
                object_headers.push(object_header);
            }

            let collectables_count = data.get_i32_le() as usize;
            let mut collectables = Vec::with_capacity(collectables_count);
            println!("Level {}: parsing {} collectable", i, collectables_count);
            for _j in 0..collectables_count {
                let collectable = ObjectReference::parse(&mut data)?;
                collectables.push(collectable);
            }

            let binary_length_objects = data.get_i64_le() as usize;
            let objects_count = data.get_i32_le() as usize;
            ensure!(object_header_count == objects_count, "corrupted level");

            let mut objects = Vec::with_capacity(objects_count);
            println!("Level {}: parsing {} objects", i, objects_count);
            for j in 0..objects_count {
                let object = Object::parse(&object_headers[j], &mut data)?;
                objects.push(object);
            }

            let collectables_count_2 = data.get_i32_le() as usize;
            let mut collectables_2 = Vec::with_capacity(collectables_count_2);
            println!(
                "Level {}: parsing {} collectables 2",
                i, collectables_count_2
            );
            for _j in 0..collectables_count_2 {
                let collectable = ObjectReference::parse(&mut data)?;
                collectables_2.push(collectable);
            }

            levels.push(Level {
                name,
                object_headers,
                collectables,
                objects,
                collectables_2,
            })
        }

        Ok(Self { grid, levels })
    }
}

#[derive(derive_more::Debug)]
pub struct CompressedBodyChunk {
    pub compressed_size: Int,
    pub uncompressed_size: Int,
    #[debug("{} bytes", bytes.len())]
    pub bytes: Bytes,
}

fn read_chunk<B: Buf>(data: &mut B) -> (Int, Int, Int, Int) {
    let a = data.get_i32_le();
    let b = data.get_i32_le();
    let c = data.get_i32_le();
    let d = data.get_i32_le();
    (a, b, c, d)
}

const MAGIC_SIGNATURE: Int = -1641380927;
const MAX_CHUNK_SIZE: Int = 128 * 1024;

impl CompressedBodyChunk {
    pub fn parse<B: Buf>(data: &mut B) -> Result<Self> {
        let (signature, _, maximum_chunk_size, _) = read_chunk(data);
        ensure!(signature == MAGIC_SIGNATURE, "corrupted chunk: magic");
        ensure!(
            maximum_chunk_size == MAX_CHUNK_SIZE,
            "corrupted chunk: maximum chunk size"
        );

        let _ = data.get_u8();
        let summary = read_chunk(data);
        let sub_chunk = read_chunk(data);

        let uncompressed_size = summary.2;
        ensure!(
            uncompressed_size == sub_chunk.2,
            "corrupted chunk: uncompressed size"
        );

        let compressed_size = summary.0;
        let bytes = data.copy_to_bytes(compressed_size as _);

        Ok(Self {
            compressed_size,
            uncompressed_size,
            bytes,
        })
    }
}

#[derive(Debug)]
pub struct SaveFileHeader {
    pub header_version: Int,
    pub save_version: Int,
    pub build_version: Int,
    pub map_name: String,
    pub map_options: String,
    pub session_name: String,
    pub session_play_time_secs: Int,
    pub save_timestamp_ticks: Long,
    pub session_visibility: Byte,
    pub editor_object_version: Int,
    pub mod_metadata: String,
    pub mod_flags: Int,
    pub save_identifier: String,
    pub is_partitioned_new_world: Int,
    pub save_data_hash: [u8; 20],
    pub is_creative_mode_enabled: Int,
}

impl SaveFileHeader {
    pub fn parse<B: Buf>(data: &mut B) -> Result<Self> {
        let header_version = data.get_i32_le();
        let save_version = data.get_i32_le();
        let build_version = data.get_i32_le();
        let map_name = String::parse(data)?;
        let map_options = String::parse(data)?;
        let session_name = String::parse(data)?;
        let session_play_time_secs = data.get_i32_le();
        let save_timestamp_ticks = data.get_i64_le();
        let session_visibility = data.get_u8();
        let editor_object_version = data.get_i32_le();
        let mod_metadata = String::parse(data)?;
        let mod_flags = data.get_i32_le();
        let save_identifier = String::parse(data)?;

        let is_partitioned_new_world = data.get_i32_le();
        let mut save_data_hash = [0u8; 20];
        data.copy_to_slice(&mut save_data_hash);
        let is_creative_mode_enabled = data.get_i32_le();

        Ok(Self {
            header_version,
            save_version,
            build_version,
            map_name,
            map_options,
            session_name,
            session_play_time_secs,
            save_timestamp_ticks,
            session_visibility,
            editor_object_version,
            mod_metadata,
            mod_flags,
            save_identifier,
            is_partitioned_new_world,
            save_data_hash,
            is_creative_mode_enabled,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        let file = std::fs::read("./tests/simple_save_1_0_0.sav").unwrap();

        let mut buf = &file[..];
        let save_game = Savegame::parse(&mut buf).unwrap();
        dbg!(&save_game);
        assert_eq!(save_game.header.header_version, 13);

        let file_data = save_game.decompress().unwrap();
        assert_eq!(file_data.len(), save_game.uncompressed_len());
        dbg!(file_data.len(), save_game.uncompressed_len());

        let body = SaveFileBody::parse(file_data).unwrap();

        assert!(false);
    }
}
