use anyhow::{ensure, Result};
use zerocopy::{
    little_endian::{F32, F64, I32, I64, U32, U64},
    FromBytes, Immutable, KnownLayout,
};

// All formats are encoded in little endian

#[derive(KnownLayout, Immutable, FromBytes, Clone)]
pub struct StringRef<'a> {
    length: I32,
    /// Includes 1-2 termination characters
    chars: &'a [u8],
}

impl std::fmt::Debug for StringRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StringRef")
            .field("len", &self.length.get())
            .field("chars", &self.as_str())
            .finish()
    }
}

impl StringRef<'_> {
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
pub struct Vec4 {
    pub w: F32,
    pub x: F32,
    pub y: F32,
    pub z: F32,
}

#[derive(Debug)]
pub struct Vec3 {
    pub x: F32,
    pub y: F32,
    pub z: F32,
}

#[derive(Debug)]
pub struct Savegame<'a> {
    pub header: SaveFileHeader<'a>,
    pub compressed_chunks: Vec<CompressedBodyChunk<'a>>,
}

impl<'a> Savegame<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self> {
        let mut parser = Parser::new(data);

        let header = SaveFileHeader::parse(&mut parser)?;

        let mut compressed_chunks = Vec::new();

        while parser.has_remaining() {
            let chunk = CompressedBodyChunk::parse(&mut parser)?;
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
            .map(|c| c.uncompressed_size.get() as usize)
            .sum()
    }

    pub fn decompress(&self) -> Result<Vec<u8>> {
        use flate2::read::ZlibDecoder;
        use std::io::Read;

        let mut out = Vec::with_capacity(self.uncompressed_len());
        let mut decoder = ZlibDecoder::new(&[][..]);
        for chunk in &self.compressed_chunks {
            decoder.reset(&chunk.bytes[..]);
            let out_size = decoder.read_to_end(&mut out)?;
            ensure!(
                out_size == chunk.uncompressed_size.get() as usize,
                "inconsistent chunk data"
            );
        }

        Ok(out)
    }
}

#[derive(Debug)]
pub struct SaveFileBody<'a> {
    pub grid: Grid<'a>,
    pub levels: Vec<Level<'a>>,
}

#[derive(Debug)]
pub struct Level<'a> {
    pub name: StringRef<'a>,
    pub object_headers: Vec<ObjectHeader<'a>>,
    pub collectables: Vec<ObjectReference<'a>>,
    pub objects: Vec<Object<'a>>,
    pub collectables_2: Vec<ObjectReference<'a>>,
}

#[derive(Debug)]
pub struct Grid<'a> {
    pub u1: StringRef<'a>,
    pub u2: I64,
    pub u3: I32,
    pub u4: StringRef<'a>,
    pub u5: I32,
    pub data: Vec<GridData<'a>>,
}

#[derive(Debug)]
pub struct GridData<'a> {
    pub u1: StringRef<'a>,
    pub u2: I32,
    pub u3: I32,
    pub levels: Vec<GridLevel<'a>>,
}

#[derive(Debug, KnownLayout, Immutable, FromBytes)]
pub struct GridLevel<'a> {
    pub u1: StringRef<'a>,
    pub u2: I32,
}

#[derive(Debug)]
pub struct ObjectReference<'a> {
    pub level_name: StringRef<'a>,
    pub path_name: StringRef<'a>,
}

impl<'a> ObjectReference<'a> {
    pub fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let level_name = parser.read_string()?;
        dbg!(&level_name);
        let path_name = parser.read_string()?;
        dbg!(&path_name);

        Ok(Self {
            level_name,
            path_name,
        })
    }
}

#[derive(Debug)]
pub enum Object<'a> {
    Component {
        entity_save_version: I32,
        properties: Properties<'a>,
    },
    Actor {
        entity_save_version: I32,
        parent_object_root: StringRef<'a>,
        parent_object_name: StringRef<'a>,
        components: Vec<ObjectReference<'a>>,
        properties: Properties<'a>,
        extra_data: Option<ExtraData<'a>>,
    },
}

impl<'a> Object<'a> {
    pub fn parse(header: &ObjectHeader<'a>, parser: &mut Parser<'a>) -> Result<Self> {
        let entity_save_version = parser.read_i32()?;
        let _unknown = parser.read_i32()?;
        dbg!(_unknown);
        let binary_size = parser.read_i32()?.get() as usize;
        dbg!(entity_save_version, binary_size);
        let mut sub_parser = parser.sub_parser(binary_size)?;

        match header {
            ObjectHeader::Component { .. } => {
                let properties = Properties::parse(&mut sub_parser)?;
                // TODO: can there be extra data here?

                let remaining = sub_parser.remaining();
                dbg!(&remaining);
                if remaining == 4 {
                    let _ = sub_parser.read_bytes(4);
                }

                ensure!(!sub_parser.has_remaining(), "failed to parse full actor");
                Ok(Self::Component {
                    entity_save_version,
                    properties,
                })
            }
            ObjectHeader::Actor { .. } => {
                let parent_object_root = sub_parser.read_string()?;
                let parent_object_name = sub_parser.read_string()?;
                let components_count = sub_parser.read_i32()?.get() as usize;
                let mut components = Vec::with_capacity(components_count);

                dbg!(&parent_object_root, &parent_object_name, components_count);
                for _j in 0..components_count {
                    println!("parsing object ref {}/{}", _j, components_count);
                    let component = ObjectReference::parse(parser)?;
                    dbg!(&component);
                    components.push(component);
                }

                let (properties, extra_data) = if sub_parser.has_remaining() {
                    let properties = Properties::parse(&mut sub_parser)?;
                    let extra_data = ExtraData::parse(header.type_path(), &mut sub_parser)?;
                    (properties, Some(extra_data))
                } else {
                    (Properties::default(), None)
                };

                ensure!(!sub_parser.has_remaining(), "failed to parse full actor");

                Ok(Self::Actor {
                    entity_save_version,
                    parent_object_root,
                    parent_object_name,
                    components,
                    properties,
                    extra_data,
                })
            }
        }
    }
}

#[derive(Debug)]
pub enum ExtraData<'a> {
    Unknown {
        type_path: StringRef<'a>,
        data: &'a [u8],
    },
}

impl<'a> ExtraData<'a> {
    pub fn parse(type_path: &StringRef<'a>, parser: &mut Parser<'a>) -> Result<Self> {
        let remaining = parser.remaining();
        dbg!(&type_path, remaining);

        // Unknown
        let data = if remaining > 4 {
            if type_path.as_str().starts_with("/Script/FactoryGame.FG") {
                parser.read_bytes(8)?
            } else {
                parser.read_bytes(remaining)?
            }
        } else {
            let _ = parser.read_bytes(4)?;
            &[][..]
        };

        dbg!(std::string::String::from_utf8_lossy(data));

        Ok(Self::Unknown {
            type_path: type_path.clone(),
            data,
        })
    }
}

#[derive(Debug, Default)]
pub struct Properties<'a> {
    pub props: Vec<Property<'a>>,
}

impl<'a> Properties<'a> {
    pub fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let mut props = Vec::new();
        while parser.has_remaining() {
            let prop = Property::parse(parser)?;
            dbg!(&prop);
            if prop.is_none() {
                break;
            }
            props.push(prop)
        }

        Ok(Self { props })
    }
}

#[derive(Debug)]
pub enum Property<'a> {
    Array(ArrayProperty<'a>),
    Object {
        index: I32,
        value: ObjectReference<'a>,
    },
    Bool {
        index: I32,
        value: u8,
    },
    Int8 {
        index: I32,
        value: u8,
    },
    Int {
        index: I32,
        value: I32,
    },
    UInt32 {
        index: I32,
        value: U32,
    },
    Int64 {
        index: I32,
        value: I64,
    },
    UInt64 {
        index: I32,
        value: U64,
    },
    Float {
        index: I32,
        value: F32,
    },
    Double {
        index: I32,
        value: F64,
    },
    String {
        index: I32,
        value: StringRef<'a>,
    },
    Enum {
        index: I32,
        r#type: StringRef<'a>,
        value: StringRef<'a>,
    },
    Byte {
        index: I32,
        name: StringRef<'a>,
        value: StringRef<'a>,
    },
    Text {
        index: I32,
        value: TextProperty<'a>,
    },
    Struct {
        index: I32,
        value: TypedData<'a>,
    },
    /// Signals the end of a property list
    None,
}

#[derive(Debug)]
pub struct TextProperty<'a> {
    pub flags: I32,
    pub history_type: TextPropertyHistoryType<'a>,
}

impl<'a> TextProperty<'a> {
    pub fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let flags = parser.read_i32()?;
        let history = parser.read_u8()?;
        let history_type = match history {
            0 => {
                let namespace = parser.read_string()?;
                let key = parser.read_string()?;
                let value = parser.read_string()?;

                TextPropertyHistoryType::A {
                    namespace,
                    key,
                    value
                }
            }
            1 => TextPropertyHistoryType::B,
            3 => {
                let source_fmt = TextProperty::parse(parser)?;
                let count = parser.read_i32()?.get() as usize;
                let mut arguments = Vec::with_capacity(count);
                for _i in 0..count {
                    let arg = Argument::parse(parser)?;
                    arguments.push(arg);
                }

                TextPropertyHistoryType::C {
                    source_fmt: Box::new(source_fmt),
                    arguments,
                }
            }
            _ => anyhow::bail!("Unknown History Type {}", history),
        };

        Ok(Self {
            flags,
            history_type,
        })
    }
}

#[derive(Debug)]
pub enum TextPropertyHistoryType<'a> {
    A {
        namespace: StringRef<'a>,
        key: StringRef<'a>,
        value: StringRef<'a>,
    },
    B,
    C {
        source_fmt: Box<TextProperty<'a>>,
        arguments: Vec<Argument<'a>>,
    },
    D {
        source_text: Box<TextProperty<'a>>,
        transform_type: u8,
    },
    E {
        table_id: StringRef<'a>,
        text_key: StringRef<'a>,
    },
    F {
        has_culture_invariant_string: I32,
        value: Option<StringRef<'a>>,
    }
}

#[derive(Debug)]
pub struct Argument<'a> {
    pub name: StringRef<'a>,
    pub value: ArgumentValue<'a>,
}

impl<'a> Argument<'a> {
    pub fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let name = parser.read_string()?;
        let value_type = parser.read_u8()?;
        let value = match value_type {
            0 => {
                let value = parser.read_i32()?;
                let unknown = parser.read_i32()?;
                ArgumentValue::A {
                    value,
                    unknown
                }
            }
            4 => {
                let value = TextProperty::parse(parser)?;
                ArgumentValue::B {
                    value,
                }
            }
            _ => anyhow::bail!("unknown argument value type {}", value_type),
        };

        Ok(Self {
            name,
            value,
        })
    }
}

#[derive(Debug)]
pub enum ArgumentValue<'a> {
    A {
        value: I32,
        unknown: I32,
    },
    B {
        value: TextProperty<'a>,
    }
}

#[derive(Debug)]
pub struct ArrayProperty<'a> {
    pub r#type: StringRef<'a>,
    pub data: ArrayPropertyElements<'a>,
}

impl<'a> ArrayProperty<'a> {
    pub fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let r#type = parser.read_string()?;
        let _ = parser.read_u8()?;

        let length = parser.read_i32()?.get() as usize;
        let typ = r#type.as_str();

        let data = match typ {
            "ByteProperty" => {
                // TODO: if e.name == "mFogOfWarRawData"

                let mut data = Vec::with_capacity(length);
                for _i in 0..length {
                    let el = parser.read_u8()?;
                    data.push(el);
                }
                ArrayPropertyElements::Byte(data)
            }
            "BoolProperty" => {
                let mut data = Vec::with_capacity(length);
                for _i in 0..length {
                    let el = parser.read_u8()?;
                    data.push(el);
                }
                ArrayPropertyElements::Bool(data)
            }
            "EnumProperty" => {
                let mut data = Vec::with_capacity(length);
                for _i in 0..length {
                    let el = parser.read_string()?;
                    data.push(el);
                }
                ArrayPropertyElements::Enum(data)
            }
            "StrProperty" => {
                let mut data = Vec::with_capacity(length);
                for _i in 0..length {
                    let el = parser.read_string()?;
                    data.push(el);
                }
                ArrayPropertyElements::Str(data)
            }
            "InterfaceProperty" => {
                let mut data = Vec::with_capacity(length);
                for _i in 0..length {
                    let el = ObjectReference::parse(parser)?;
                    data.push(el);
                }
                ArrayPropertyElements::Interface(data)
            }
            "ObjectProperty" => {
                let mut data = Vec::with_capacity(length);
                for _i in 0..length {
                    let el = ObjectReference::parse(parser)?;
                    data.push(el);
                }
                ArrayPropertyElements::Object(data)
            }
            "IntProperty" => {
                let mut data = Vec::with_capacity(length);
                for _i in 0..length {
                    let el = parser.read_i32()?;
                    data.push(el);
                }
                ArrayPropertyElements::Int(data)
            }
            "Int64Property" => {
                let mut data = Vec::with_capacity(length);
                for _i in 0..length {
                    let el = parser.read_i64()?;
                    data.push(el);
                }
                ArrayPropertyElements::Int64(data)
            }
            "FloatProperty" => {
                let mut data = Vec::with_capacity(length);
                for _i in 0..length {
                    let el = parser.read_f32()?;
                    data.push(el);
                }
                ArrayPropertyElements::Float(data)
            }
            "DoubleProperty" => {
                let mut data = Vec::with_capacity(length);
                for _i in 0..length {
                    let el = parser.read_f64()?;
                    data.push(el);
                }
                ArrayPropertyElements::Double(data)
            }
            "SoftObjectProperty" => {
                let mut data = Vec::with_capacity(length);
                for _i in 0..length {
                    let path_name = parser.read_string()?;
                    let sub_path = parser.read_string()?;
                    let el = SoftObject {
                        path_name,
                        sub_path,
                    };
                    data.push(el);
                }
                ArrayPropertyElements::SoftObject(data)
            }
            "TextProperty" => {
                let mut data = Vec::with_capacity(length);
                for _i in 0..length {
                    let el = TextProperty::parse(parser)?;
                    data.push(el);
                }
                ArrayPropertyElements::Text(data)
            }
            "StructProperty" => {
                let property_name = parser.read_string()?;
                let property_type = parser.read_string()?;

                let binary_size = parser.read_i32()?.get() as usize;
                let padding = parser.read_i32()?;
                ensure!(padding == 0, "invalid padding");

                let element_type = parser.read_string()?;
                let uuid = [
                    parser.read_i32()?,
                    parser.read_i32()?,
                    parser.read_i32()?,
                    parser.read_i32()?,
                ];

                let mut values = Vec::with_capacity(length);

                if length > 0 {
                    let padding = parser.read_u8()?;
                    ensure!(padding == 0, "invalid padding");
                    println!(
                        "reading {} typed values {} {}",
                        length,
                        element_type.as_str(),
                        binary_size
                    );

                    let mut value_space = parser.sub_parser(binary_size)?;

                    for _i in 0..length {
                        let val = TypedData::parse(element_type.as_str(), &mut value_space)?;
                        values.push(val);
                    }
                    ensure!(!value_space.has_remaining(), "failed to parse full array");
                }

                ArrayPropertyElements::Struct {
                    name: property_name,
                    r#type: property_type,
                    uuid,
                    values,
                }
            }
            _ => anyhow::bail!("unknown ArrayProperty: {}", typ),
        };

        ensure!(!parser.has_remaining(), "failed to fully parse");

        Ok(Self {
            r#type,
            data,
        })
    }
}

#[derive(Debug)]
pub enum ArrayPropertyElements<'a> {
    Byte(Vec<u8>),
    Bool(Vec<u8>),
    Enum(Vec<StringRef<'a>>),
    Str(Vec<StringRef<'a>>),
    Interface(Vec<ObjectReference<'a>>),
    Object(Vec<ObjectReference<'a>>),
    Int(Vec<I32>),
    Int64(Vec<I64>),
    Float(Vec<F32>),
    Double(Vec<F64>),
    SoftObject(Vec<SoftObject<'a>>),
    Text(Vec<TextProperty<'a>>),
    Struct {
        name: StringRef<'a>,
        r#type: StringRef<'a>,
        uuid: [I32; 4],
        values: Vec<TypedData<'a>>,
    },
}

#[derive(Debug)]
pub struct SoftObject<'a> {
    path_name: StringRef<'a>,
    sub_path: StringRef<'a>,
}

#[derive(Debug)]
pub enum TypedData<'a> {
    InventoryItem {
        item_name_level_name: StringRef<'a>,
        item_name_path_name: StringRef<'a>,
        item_state_level_name: StringRef<'a>,
        item_state_path_name: StringRef<'a>,
    },
    Guid(Guid),
}

impl<'a> TypedData<'a> {
    pub fn parse(typ: &str, parser: &mut Parser<'a>) -> Result<Self> {
        dbg!(typ);
        match typ {
            "InventoryItem" => {
                let item_name_level_name = parser.read_string()?;
                let item_name_path_name = parser.read_string()?;
                let item_state_level_name = parser.read_string()?;
                let item_state_path_name = parser.read_string()?;

                Ok(Self::InventoryItem {
                    item_name_level_name,
                    item_name_path_name,
                    item_state_level_name,
                    item_state_path_name,
                })
            }
            "Guid" => {
                let guid = parser.read_property_guid()?;
                Ok(Self::Guid(guid))
            }
            "FINNetworkTrace" => {
                todo!()
            }
            "Vector" => {
                todo!()
            }
            "LinearColor" => {
                todo!()
            }
            "FINGPUT1BufferPixel" => {
                todo!()
            }
            "FINDynamicStructHolder" => {
                todo!()
            }
            _ => anyhow::bail!("unknown typed data: {}", typ),
        }
    }
}

impl<'a> Property<'a> {
    pub fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let property_name = parser.read_string()?;
        dbg!(&property_name);

        let name = property_name.as_str();
        if name == "None" || name == "" {
            return Ok(Self::None);
        }
        let type_name = parser.read_string()?;
        let binary_size = parser.read_i32()?;
        let index = parser.read_i32()?;


        let mut parser = parser.sub_parser(binary_size.get() as _)?;
        let res = match dbg!(type_name.as_str()) {
            "BoolProperty" => {
                let _guid = parser.read_property_guid()?;
                let value = parser.read_u8()?;
                Ok(Self::Bool { index, value })
            }
            "Int8Property" => {
                let _guid = parser.read_property_guid()?;
                let value = parser.read_u8()?;
                Ok(Self::Int8 { index, value })
            }
            "IntProperty" => {
                let _guid = parser.read_property_guid()?;
                let value = parser.read_i32()?;
                Ok(Self::Int { index, value })
            }
            "UInt32Property" => {
                let _guid = parser.read_property_guid()?;
                let value = parser.read_u32()?;
                Ok(Self::UInt32 { index, value })
            }
            "Int64Property" => {
                let _guid = parser.read_property_guid()?;
                let value = parser.read_i64()?;
                Ok(Self::Int64 { index, value })
            }
            "UInt64Property" => {
                let _guid = parser.read_property_guid()?;
                let value = parser.read_u64()?;
                Ok(Self::UInt64 { index, value })
            }
            "FloatProperty" => {
                let _guid = parser.read_property_guid()?;
                let value = parser.read_f32()?;
                Ok(Self::Float { index, value })
            }
            "DoubleProperty" => {
                let _guid = parser.read_property_guid()?;
                let value = parser.read_f64()?;
                Ok(Self::Double { index, value })
            }
            "StrProperty" | "NameProperty"=> {
                let _guid = parser.read_property_guid()?;
                let value = parser.read_string()?;
                Ok(Self::String { index, value })
            }
            "ObjectProperty" | "InterfaceProperty" => {
                let _guid = parser.read_property_guid()?;
                let value = ObjectReference::parse(&mut parser)?;
                Ok(Self::Object { index, value })
            }
            "EnumProperty" => {
                let r#type = parser.read_string()?;
                let _guid = parser.read_property_guid()?;
                let value = parser.read_string()?;

                Ok(Self::Enum { index, r#type, value })
            }
            "ByteProperty" => {
                let name = parser.read_string()?;
                let _guid = parser.read_property_guid()?;
                let value = parser.read_string()?;
                Ok(Self::Byte {
                    index,
                    name,
                    value,
                })
            }
            "TextProperty" => {
                let _guid = parser.read_property_guid()?;
                let value = TextProperty::parse(&mut parser)?;
                Ok(Self::Text { index, value })
            }
            "ArrayProperty" => {
                let prop = ArrayProperty::parse(&mut parser)?;
                Ok(Self::Array(prop))
            }
            "StructProperty" => {
                let binary_size = parser.read_i32()?;
                let index = parser.read_i32()?;
                let type_name = parser.read_string()?;
                let padding = parser.read_i64()?;
                ensure!(padding == 0, "invalid padding");
                let padding = parser.read_i64()?;
                ensure!(padding == 0, "invalid padding");
                let padding = parser.read_u8()?;
                ensure!(padding == 0, "invalid padding");
                let mut rest = parser.sub_parser(binary_size.get() as _)?;
                todo!()
                // let value = TypedData::parse(type_name.as_str(), &mut rest)?;
                // ensure!(!rest.has_remaining(), "failed to read full property");
                // Ok(Self::Struct { index, value })
            }
            name => todo!("{}", name),
        };

        ensure!(!parser.has_remaining(), "failed to read full property");
        res
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

#[derive(Debug)]
pub enum ObjectHeader<'a> {
    Component {
        type_path: StringRef<'a>,
        root_object: StringRef<'a>,
        instance_name: StringRef<'a>,
        parent_actor_name: StringRef<'a>,
    },
    Actor {
        type_path: StringRef<'a>,
        root_object: StringRef<'a>,
        instance_name: StringRef<'a>,
        need_transform: I32,
        rotation: Vec4,
        position: Vec3,
        scale: Vec3,
        was_placed_in_level: I32,
    },
}

impl<'a> ObjectHeader<'a> {
    fn type_path(&self) -> &StringRef<'a> {
        match self {
            Self::Component { type_path, .. } => type_path,
            Self::Actor { type_path, .. } => type_path,
        }
    }

    pub fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let typ = parser.read_i32()?;
        match typ.get() {
            0 => {
                // Component
                let type_path = parser.read_string()?;
                let root_object = parser.read_string()?;
                let instance_name = parser.read_string()?;
                let parent_actor_name = parser.read_string()?;

                Ok(Self::Component {
                    type_path,
                    root_object,
                    instance_name,
                    parent_actor_name,
                })
            }
            1 => {
                // Actor
                let type_path = parser.read_string()?;
                let root_object = parser.read_string()?;
                let instance_name = parser.read_string()?;
                let need_transform = parser.read_i32()?;
                let rotation = parser.read_vec4()?;
                let position = parser.read_vec3()?;
                let scale = parser.read_vec3()?;
                let was_placed_in_level = parser.read_i32()?;
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

#[derive(Debug)]
pub struct ObjectProperty<'a> {
    pub level_name: StringRef<'a>,
    pub path_name: StringRef<'a>,
}

impl<'a> SaveFileBody<'a> {
    pub fn parse(header: &SaveFileHeader<'a>, data: &'a [u8]) -> Result<Self> {
        let mut parser = Parser::new(data);
        // Check that the size is correct
        let total_len = parser.read_i64()?;
        ensure!(
            parser.remaining() as u64 == total_len.get() as u64,
            "invalid length: {} != {}",
            parser.remaining(),
            total_len
        );

        let grid_count = parser.read_i32()?.get() as usize;
        let unknown_1 = parser.read_string()?;
        let unknown_2 = parser.read_i64()?;
        let unknown_3 = parser.read_i32()?;
        let unknown_4 = parser.read_string()?;
        let unknown_5 = parser.read_i32()?;

        let mut grid = Grid {
            u1: unknown_1,
            u2: unknown_2,
            u3: unknown_3,
            u4: unknown_4,
            u5: unknown_5,
            data: Vec::with_capacity(grid_count - 1),
        };

        for x in 0..grid_count - 1 {
            let unknown_6 = parser.read_string()?;
            let unknown_7 = parser.read_i32()?;
            let unknown_8 = parser.read_i32()?;
            let level_count = parser.read_i32()?.get() as usize;

            let mut levels = Vec::with_capacity(level_count);
            println!("Grid {}: Level Count {}", x, level_count);
            for _y in 0..level_count {
                let unknown_9 = parser.read_string()?;
                let unknown_10 = parser.read_i32()?;

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

        let level_count = parser.read_i32()?.get() as usize;
        let mut levels = Vec::with_capacity(level_count + 1);
        for i in 0..=level_count {
            let name = if i == level_count {
                // TODO: Add "Level " prefix
                header.map_name.clone()
            } else {
                parser.read_string()?
            };
            let binary_length = parser.read_i64()?;
            println!("level {:?}: {}bytes", name, binary_length);

            let mut level_parser = parser.sub_parser(binary_length.get() as usize)?;

            let object_header_count = level_parser.read_i32()?.get() as usize;
            dbg!(&object_header_count);
            let mut object_headers = Vec::with_capacity(object_header_count);

            println!(
                "Level {}: parsing {} object headers",
                i, object_header_count
            );
            for _j in 0..object_header_count {
                let object_header = ObjectHeader::parse(&mut level_parser)?;
                object_headers.push(object_header);
            }

            // TODO: could be missing

            let collectables_count = level_parser.read_i32()?.get() as usize;
            dbg!(collectables_count);
            let mut collectables = Vec::with_capacity(collectables_count);
            println!("Level {}: parsing {} collectable", i, collectables_count);
            for _j in 0..collectables_count {
                let collectable = ObjectReference::parse(&mut level_parser)?;
                collectables.push(collectable);
            }

            let binary_length_objects = parser.read_i64()?.get() as usize;
            ensure!(!level_parser.has_remaining(), "not fully read");

            println!("parsing objects {}bytes", binary_length_objects);

            let mut level_parser = parser.sub_parser(binary_length_objects)?;
            let objects_count = level_parser.read_i32()?.get() as usize;
            ensure!(object_header_count == objects_count, "corrupted level");

            let mut objects = Vec::with_capacity(objects_count);
            println!("Level {}: parsing {} objects", i, objects_count);
            for j in 0..objects_count {
                let object = Object::parse(&object_headers[j], &mut level_parser)?;
                objects.push(object);
            }

            ensure!(!level_parser.has_remaining(), "not fully read");

            let collectables_count_2 = parser.read_i32()?.get() as usize;
            let mut collectables_2 = Vec::with_capacity(collectables_count_2);
            println!(
                "Level {}: parsing {} collectables 2",
                i, collectables_count_2
            );
            for _j in 0..collectables_count_2 {
                let collectable = ObjectReference::parse(&mut parser)?;
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
pub struct CompressedBodyChunk<'a> {
    pub compressed_size: I32,
    pub uncompressed_size: I32,
    #[debug("{} bytes", bytes.len())]
    pub bytes: &'a [u8],
}

const MAGIC_SIGNATURE: i32 = -1641380927;
const MAX_CHUNK_SIZE: i32 = 128 * 1024;

impl<'a> CompressedBodyChunk<'a> {
    pub fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let (signature, _, maximum_chunk_size, _) = parser.read_chunk()?;
        ensure!(signature == MAGIC_SIGNATURE, "corrupted chunk: magic");
        ensure!(
            maximum_chunk_size == MAX_CHUNK_SIZE,
            "corrupted chunk: maximum chunk size"
        );

        let _ = parser.read_u8()?;
        let summary = parser.read_chunk()?;
        let sub_chunk = parser.read_chunk()?;

        let uncompressed_size = summary.2;
        ensure!(
            uncompressed_size == sub_chunk.2,
            "corrupted chunk: uncompressed size"
        );

        let compressed_size = summary.0;
        let bytes = parser.read_bytes(compressed_size.get() as _)?;

        Ok(Self {
            compressed_size,
            uncompressed_size,
            bytes,
        })
    }
}

#[derive(Debug)]
pub struct SaveFileHeader<'a> {
    pub header_version: I32,
    pub save_version: I32,
    pub build_version: I32,
    pub map_name: StringRef<'a>,
    pub map_options: StringRef<'a>,
    pub session_name: StringRef<'a>,
    pub session_play_time_secs: I32,
    pub save_timestamp_ticks: I64,
    pub session_visibility: u8,
    pub editor_object_version: I32,
    pub mod_metadata: StringRef<'a>,
    pub mod_flags: I32,
    pub save_identifier: StringRef<'a>,
    pub is_partitioned_new_world: I32,
    pub save_data_hash: &'a [u8; 20],
    pub is_creative_mode_enabled: I32,
}

struct Parser<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> Parser<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }

    fn remaining(&self) -> usize {
        self.data.len() - self.offset
    }

    fn has_remaining(&self) -> bool {
        self.remaining() > 0
    }

    fn read_u8(&mut self) -> Result<u8> {
        ensure!(self.remaining() > 0, "too short");
        let out = self.data[self.offset];
        self.offset += 1;
        Ok(out)
    }

    fn read_i32(&mut self) -> Result<I32> {
        let (out, _) = I32::read_from_prefix(&self.data[self.offset..])
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        self.offset += 4;
        Ok(out)
    }

    fn read_i64(&mut self) -> Result<I64> {
        let (out, _) = I64::read_from_prefix(&self.data[self.offset..])
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        self.offset += 8;
        Ok(out)
    }

    fn read_u32(&mut self) -> Result<U32> {
        let (out, _) = U32::read_from_prefix(&self.data[self.offset..])
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        self.offset += 4;
        Ok(out)
    }

    fn read_u64(&mut self) -> Result<U64> {
        let (out, _) = U64::read_from_prefix(&self.data[self.offset..])
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        self.offset += 8;
        Ok(out)
    }

    fn read_f32(&mut self) -> Result<F32> {
        let (out, _) = F32::read_from_prefix(&self.data[self.offset..])
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        self.offset += 4;
        Ok(out)
    }

    fn read_f64(&mut self) -> Result<F64> {
        let (out, _) = F64::read_from_prefix(&self.data[self.offset..])
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        self.offset += 8;
        Ok(out)
    }

    fn read_vec3(&mut self) -> Result<Vec3> {
        let x = self.read_f32()?;
        let y = self.read_f32()?;
        let z = self.read_f32()?;
        Ok(Vec3 { x, y, z })
    }

    fn read_vec4(&mut self) -> Result<Vec4> {
        let w = self.read_f32()?;
        let x = self.read_f32()?;
        let y = self.read_f32()?;
        let z = self.read_f32()?;
        Ok(Vec4 { w, x, y, z })
    }

    fn read_bytes(&mut self, len: usize) -> Result<&'a [u8]> {
        ensure!(
            self.remaining() >= len,
            "too short: remaining: {}, wanted: {} ({:?})",
            self.remaining(),
            len,
            std::str::from_utf8(&self.data[self.offset..]),
        );
        let res = &self.data[self.offset..self.offset + len];
        self.offset += len;
        Ok(res)
    }

    fn sub_parser(&mut self, len: usize) -> Result<Parser<'a>> {
        let bytes = self.read_bytes(len)?;
        Ok(Parser::new(bytes))
    }

    fn read_string(&mut self) -> Result<StringRef<'a>> {
        let length = self.read_i32()?;
        let actual_length = if length == 0 {
            length.get() as usize
        } else if length > 0 {
            // UTF8
            length.get() as usize
        } else {
            // UTF16
            (length.get() * -2) as usize
        };
        let chars = self.read_bytes(actual_length)?;

        Ok(StringRef { length, chars })
    }

    fn read_chunk(&mut self) -> Result<(I32, I32, I32, I32)> {
        let a = self.read_i32()?;
        let b = self.read_i32()?;
        let c = self.read_i32()?;
        let d = self.read_i32()?;

        Ok((a, b, c, d))
    }

    fn read_property_guid(&mut self) -> Result<Option<Guid>> {
        let b = self.read_u8()?;
        if b == 1 {
            let guid = self.read_bytes(16)?;
            return Ok(Some(Guid(guid.try_into()?)));
        }
        Ok(None)
    }
}

#[derive(Debug)]
pub struct Guid([u8; 16]);

impl<'a> SaveFileHeader<'a> {
    pub fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let header_version = parser.read_i32()?;
        let save_version = parser.read_i32()?;
        let build_version = parser.read_i32()?;
        let map_name = parser.read_string()?;
        let map_options = parser.read_string()?;
        let session_name = parser.read_string()?;
        let session_play_time_secs = parser.read_i32()?;
        let save_timestamp_ticks = parser.read_i64()?;
        let session_visibility = parser.read_u8()?;
        let editor_object_version = parser.read_i32()?;
        let mod_metadata = parser.read_string()?;
        let mod_flags = parser.read_i32()?;
        let save_identifier = parser.read_string()?;

        let is_partitioned_new_world = parser.read_i32()?;
        let save_data_hash = parser.read_bytes(20)?.try_into()?;
        let is_creative_mode_enabled = parser.read_i32()?;

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

        let body = SaveFileBody::parse(&save_game.header, &file_data).unwrap();
        dbg!(&body);

        assert!(false);
    }
}
