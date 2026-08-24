use std::io::{Read, Write, Result};

pub const TAG_END: u8 = 0;
pub const TAG_BYTE: u8 = 1;
pub const TAG_SHORT: u8 = 2;
pub const TAG_INT: u8 = 3;
pub const TAG_LONG: u8 = 4;
pub const TAG_FLOAT: u8 = 5;
pub const TAG_DOUBLE: u8 = 6;
pub const TAG_BYTE_ARRAY: u8 = 7;
pub const TAG_STRING: u8 = 8;
pub const TAG_LIST: u8 = 9;
pub const TAG_COMPOUND: u8 = 10;
pub const TAG_INT_ARRAY: u8 = 11;

#[derive(Debug, Clone, PartialEq)]
pub enum TagValue {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List { element_type: u8, items: Vec<TagValue> },
    Compound(Vec<NamedTag>),
    IntArray(Vec<i32>),
}

impl TagValue {
    pub fn id(&self) -> u8 {
        match self {
            TagValue::End => TAG_END,
            TagValue::Byte(_) => TAG_BYTE,
            TagValue::Short(_) => TAG_SHORT,
            TagValue::Int(_) => TAG_INT,
            TagValue::Long(_) => TAG_LONG,
            TagValue::Float(_) => TAG_FLOAT,
            TagValue::Double(_) => TAG_DOUBLE,
            TagValue::ByteArray(_) => TAG_BYTE_ARRAY,
            TagValue::String(_) => TAG_STRING,
            TagValue::List { .. } => TAG_LIST,
            TagValue::Compound(_) => TAG_COMPOUND,
            TagValue::IntArray(_) => TAG_INT_ARRAY,
        }
    }

    pub fn tag_name(id: u8) -> &'static str {
        match id {
            TAG_END => "TAG_End",
            TAG_BYTE => "TAG_Byte",
            TAG_SHORT => "TAG_Short",
            TAG_INT => "TAG_Int",
            TAG_LONG => "TAG_Long",
            TAG_FLOAT => "TAG_Float",
            TAG_DOUBLE => "TAG_Double",
            TAG_BYTE_ARRAY => "TAG_Byte_Array",
            TAG_STRING => "TAG_String",
            TAG_LIST => "TAG_List",
            TAG_COMPOUND => "TAG_Compound",
            TAG_INT_ARRAY => "TAG_Int_Array",
            _ => "UNKNOWN",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedTag {
    pub name: String,
    pub value: TagValue,
}

impl NamedTag {
    pub fn new(name: impl Into<String>, value: TagValue) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

pub fn read_utf<R: Read>(reader: &mut R) -> Result<String> {
    let mut len_bytes = [0u8; 2];
    reader.read_exact(&mut len_bytes)?;
    let len = u16::from_be_bytes(len_bytes) as usize;
    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf)?;
    Ok(String::from_utf8_lossy(&buf).into_owned())
}

pub fn write_utf<W: Write>(writer: &mut W, s: &str) -> Result<()> {
    let bytes = s.as_bytes();
    let len = bytes.len() as u16;
    writer.write_all(&len.to_be_bytes())?;
    writer.write_all(bytes)?;
    Ok(())
}

pub fn read_tag_value<R: Read>(reader: &mut R, id: u8, depth: usize) -> Result<TagValue> {
    match id {
        TAG_END => Ok(TagValue::End),
        TAG_BYTE => {
            let mut buf = [0u8; 1];
            reader.read_exact(&mut buf)?;
            Ok(TagValue::Byte(buf[0] as i8))
        }
        TAG_SHORT => {
            let mut buf = [0u8; 2];
            reader.read_exact(&mut buf)?;
            Ok(TagValue::Short(i16::from_be_bytes(buf)))
        }
        TAG_INT => {
            let mut buf = [0u8; 4];
            reader.read_exact(&mut buf)?;
            Ok(TagValue::Int(i32::from_be_bytes(buf)))
        }
        TAG_LONG => {
            let mut buf = [0u8; 8];
            reader.read_exact(&mut buf)?;
            Ok(TagValue::Long(i64::from_be_bytes(buf)))
        }
        TAG_FLOAT => {
            let mut buf = [0u8; 4];
            reader.read_exact(&mut buf)?;
            Ok(TagValue::Float(f32::from_be_bytes(buf)))
        }
        TAG_DOUBLE => {
            let mut buf = [0u8; 8];
            reader.read_exact(&mut buf)?;
            Ok(TagValue::Double(f64::from_be_bytes(buf)))
        }
        TAG_BYTE_ARRAY => {
            let mut len_buf = [0u8; 4];
            reader.read_exact(&mut len_buf)?;
            let len = i32::from_be_bytes(len_buf) as usize;
            let mut buf = vec![0u8; len];
            reader.read_exact(&mut buf)?;
            Ok(TagValue::ByteArray(buf))
        }
        TAG_STRING => {
            let s = read_utf(reader)?;
            Ok(TagValue::String(s))
        }
        TAG_LIST => {
            let mut type_buf = [0u8; 1];
            reader.read_exact(&mut type_buf)?;
            let elem_type = type_buf[0];

            let mut len_buf = [0u8; 4];
            reader.read_exact(&mut len_buf)?;
            let len = i32::from_be_bytes(len_buf) as usize;

            let mut items = Vec::with_capacity(len);
            for _ in 0..len {
                items.push(read_tag_value(reader, elem_type, depth + 1)?);
            }
            Ok(TagValue::List { element_type: elem_type, items })
        }
        TAG_COMPOUND => {
            let mut entries = Vec::new();
            loop {
                let named = read_named_tag(reader, depth + 1)?;
                if named.value == TagValue::End {
                    break;
                }
                entries.push(named);
            }
            Ok(TagValue::Compound(entries))
        }
        TAG_INT_ARRAY => {
            let mut len_buf = [0u8; 4];
            reader.read_exact(&mut len_buf)?;
            let len = i32::from_be_bytes(len_buf) as usize;
            let mut items = Vec::with_capacity(len);
            for _ in 0..len {
                let mut buf = [0u8; 4];
                reader.read_exact(&mut buf)?;
                items.push(i32::from_be_bytes(buf));
            }
            Ok(TagValue::IntArray(items))
        }
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Invalid NBT tag ID {}", id),
        )),
    }
}

pub fn read_named_tag<R: Read>(reader: &mut R, depth: usize) -> Result<NamedTag> {
    let mut type_buf = [0u8; 1];
    reader.read_exact(&mut type_buf)?;
    let tag_type = type_buf[0];

    if tag_type == TAG_END || tag_type == 255 {
        return Ok(NamedTag::new("", TagValue::End));
    }

    let name = read_utf(reader)?;
    let value = read_tag_value(reader, tag_type, depth)?;
    Ok(NamedTag::new(name, value))
}

pub fn write_tag_value<W: Write>(writer: &mut W, value: &TagValue) -> Result<()> {
    match value {
        TagValue::End => Ok(()),
        TagValue::Byte(b) => writer.write_all(&[*b as u8]),
        TagValue::Short(s) => writer.write_all(&s.to_be_bytes()),
        TagValue::Int(i) => writer.write_all(&i.to_be_bytes()),
        TagValue::Long(l) => writer.write_all(&l.to_be_bytes()),
        TagValue::Float(f) => writer.write_all(&f.to_be_bytes()),
        TagValue::Double(d) => writer.write_all(&d.to_be_bytes()),
        TagValue::ByteArray(arr) => {
            writer.write_all(&(arr.len() as i32).to_be_bytes())?;
            writer.write_all(arr)
        }
        TagValue::String(s) => write_utf(writer, s),
        TagValue::List { element_type, items } => {
            writer.write_all(&[*element_type])?;
            writer.write_all(&(items.len() as i32).to_be_bytes())?;
            for item in items {
                write_tag_value(writer, item)?;
            }
            Ok(())
        }
        TagValue::Compound(entries) => {
            for entry in entries {
                write_named_tag(writer, entry)?;
            }
            writer.write_all(&[TAG_END])
        }
        TagValue::IntArray(arr) => {
            writer.write_all(&(arr.len() as i32).to_be_bytes())?;
            for &item in arr {
                writer.write_all(&item.to_be_bytes())?;
            }
            Ok(())
        }
    }
}

pub fn write_named_tag<W: Write>(writer: &mut W, tag: &NamedTag) -> Result<()> {
    writer.write_all(&[tag.value.id()])?;
    if tag.value == TagValue::End {
        return Ok(());
    }
    write_utf(writer, &tag.name)?;
    write_tag_value(writer, &tag.value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nbt_roundtrip() {
        let root = NamedTag::new(
            "Level",
            TagValue::Compound(vec![
                NamedTag::new("gameType", TagValue::Int(1)),
                NamedTag::new("generatorName", TagValue::String("default".into())),
                NamedTag::new("spawnX", TagValue::Int(120)),
                NamedTag::new("spawnY", TagValue::Int(64)),
                NamedTag::new("spawnZ", TagValue::Int(-250)),
                NamedTag::new("inventory", TagValue::List {
                    element_type: TAG_COMPOUND,
                    items: vec![
                        TagValue::Compound(vec![
                            NamedTag::new("id", TagValue::Short(276)),
                            NamedTag::new("count", TagValue::Byte(1)),
                        ])
                    ],
                }),
            ]),
        );

        let mut buf = Vec::new();
        write_named_tag(&mut buf, &root).unwrap();

        let mut cursor = std::io::Cursor::new(buf);
        let read_back = read_named_tag(&mut cursor, 0).unwrap();

        assert_eq!(root, read_back);
    }
}
