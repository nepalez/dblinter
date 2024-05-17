use regex::Regex;
use serde::{ser, Serialize};
use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// Sealed trait to deserialize struct into SQL WHERE condition.
pub(crate) trait ToSql: Serialize {
    fn to_sql(&self) -> crate::error::Result<String> {
        let mut serializer = WhereSerializer::new();
        self.serialize(&mut serializer)?;
        Ok(serializer.output)
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub enum Error {
    Name(String),
    Value(String),
    Filter(String),
    Other(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Name(e) => write!(f, "The {} cannot be used for a column name", e),
            Self::Value(e) => write!(f, "The {} cannot be used for a column value", e),
            Self::Filter(e) => write!(f, "The {} cannot be used for a filter", e),
            Self::Other(e) => write!(f, "{}", e),
        }
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Other(msg.to_string())
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Name(_) => None,
            Self::Value(_) => None,
            Self::Filter(_) => None,
            Self::Other(_) => None,
        }
    }
}

// Serialize a string value to double-quoted string representing a column name.
struct NameSerializer {
    output: String,
}

impl NameSerializer {
    fn new() -> Self {
        Self {
            output: String::new(),
        }
    }
}

impl<'a> ser::Serializer for &'a mut NameSerializer {
    type Ok = String;
    type Error = Error;

    type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::Name(format!("bool {:?}", v)))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::Name(format!("number {:?}", v)))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::Name(format!("number {:?}", v)))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::Name(format!("number {:?}", v)))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::Name(format!("number {:?}", v)))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::Name(format!("number {:?}", v)))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::Name(format!("number {:?}", v)))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::Name(format!("number {:?}", v)))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::Name(format!("number {:?}", v)))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::Name(format!("number {:?}", v)))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::Name(format!("number {:?}", v)))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        if v.is_empty() {
            return Err(Error::Name("empty string".into()));
        } else if v.contains('"') {
            return Err(Error::Name(format!(
                "string containing quotation mark {:?}",
                v
            )));
        }

        let re: Regex = Regex::new(r#"^[_a-zA-Z0-9]+$"#).unwrap();
        if re.is_match(v) {
            self.output += v;
        } else {
            self.output += "\"";
            self.output += v;
            self.output += "\"";
        }

        Ok(self.output.to_string())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Error> {
        Err(Error::Name(format!("byte array {:?}", v)))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::Name("none".into()))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::Name(format!("unit struct {}", name)))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::Name(format!("unit variant {}::{}", name, variant)))
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::Name(format!(
            "newtype variant {}::{}",
            name, variant
        )))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::Name("sequence".into()))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::Name("tuple".into()))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::Name(format!("tuple struct {}", name)))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::Name(format!("tuple variant {}::{}", name, variant)))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::Name("map".into()))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::Name(format!("struct {}", name)))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::Name(format!("struct variant {}::{}", name, variant)))
    }
}

// Serialize value to a string representing a column value.
// Supported values: bool, numbers, char, &str, nested arrays, optional values.
// Empty tuples and Nones are ignored (serialized into the empty string).
struct ValueSerializer {
    output: String,
}

impl ValueSerializer {
    fn new() -> Self {
        Self {
            output: String::new(),
        }
    }
}

impl<'a> ser::Serializer for &'a mut ValueSerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.output += if v { "TRUE" } else { "FALSE" };
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let quotation_mark = if !v.contains('\'') {
            String::from("'")
        } else if !v.contains("$$") {
            String::from("$$")
        } else {
            let mut i = 0;
            loop {
                let quotation_mark = format!("${}$", &i);
                if !v.contains(&quotation_mark) {
                    break quotation_mark;
                }
                i += 1;
            }
        };
        self.output += &quotation_mark;
        self.output += v;
        self.output += &quotation_mark;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::Value(format!("bytes array {:?}", v)))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::Value(format!("unit struct {}", name)))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        variant.serialize(self)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::Value(format!("{}::{}", name, variant)))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.output += "ARRAY[";
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::Value("tuple".into()))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::Value(format!("tuple struct {}", name)))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::Value(format!("{}::{}", name, variant)))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::Value("map".into()))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::Value(format!("struct {}", name)))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::Value(format!(
            "struct variant {}::{}",
            name, variant
        )))
    }
}

impl<'a> ser::SerializeSeq for &'a mut ValueSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output += "]";
        Ok(())
    }
}

// Serialize a plain structure into a condition.
// Only structures, their optionals and newtypes are supported here.
struct FilterItemSerializer {
    output: String,
}

impl FilterItemSerializer {
    fn new() -> Self {
        Self {
            output: String::new(),
        }
    }
}

impl<'a> ser::Serializer for &'a mut FilterItemSerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = ser::Impossible<Self::Ok, Error>;
    type SerializeTuple = ser::Impossible<Self::Ok, Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Error>;
    type SerializeMap = ser::Impossible<Self::Ok, Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("bool {:?}", v)))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("char {:?}", v)))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("string {:?}", v)))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("bytes array {:?}", v)))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter("none".into()))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("unit variant {}::{}", name, variant)))
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::Filter(format!(
            "newtype variant {}::{}",
            name, variant
        )))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::Filter("sequence".into()))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::Filter("tuple".into()))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::Filter(format!("tuple struct {}", name)))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::Filter(format!(
            "tuple variant {}::{}",
            name, variant
        )))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::Filter("map".into()))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::Filter(format!(
            "struct variant {}::{}",
            name, variant
        )))
    }
}

impl<'a> ser::SerializeStruct for &'a mut FilterItemSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut value_serializer = ValueSerializer::new();
        value.serialize(&mut value_serializer)?;
        let value = &value_serializer.output;
        // skip if value is not provided (empty tuple or None is given)
        if value.is_empty() {
            return Ok(());
        }

        if !self.output.is_empty() {
            self.output += " AND ";
        }

        let mut name_serializer = NameSerializer::new();
        key.serialize(&mut name_serializer)?;
        self.output += &name_serializer.output;
        self.output += " = ";
        self.output += value;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

// Serialize a list of conditions into a single condition with OR operator.
struct FilterListSerializer {
    output: String,
}

impl FilterListSerializer {
    fn new() -> Self {
        Self {
            output: String::new(),
        }
    }
}

impl<'a> ser::Serializer for &'a mut FilterListSerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = ser::Impossible<Self::Ok, Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Error>;
    type SerializeMap = ser::Impossible<Self::Ok, Error>;
    type SerializeStruct = ser::Impossible<Self::Ok, Error>;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("bool {:?}", v)))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("char {:?}", v)))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("string {:?}", v)))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("bytes array {:?}", v)))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::Name(format!("unit struct {}", name)))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::Name(format!("unit variant {}::{}", name, variant)))
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::Value(format!("{}::{}", name, variant)))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::Value("tuple".into()))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::Value(format!("tuple struct {}", name)))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::Value(format!("{}::{}", name, variant)))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::Value("map".into()))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::Value(format!("struct {}", name)))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::Value(format!(
            "struct variant {}::{}",
            name, variant
        )))
    }
}

impl<'a> ser::SerializeSeq for &'a mut FilterListSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut filter_item_serializer = FilterItemSerializer::new();
        value.serialize(&mut filter_item_serializer)?;
        let filter_item = &filter_item_serializer.output;
        if !filter_item.is_empty() && !self.output.is_empty() {
            if !self.output.starts_with('(') {
                self.output = format!("({}", self.output);
            }
            self.output += " OR ";
        }
        self.output += filter_item;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.output.starts_with('(') {
            self.output += ")";
        }
        Ok(())
    }
}

// Serialize a structure into WHERE clause
struct WhereSerializer {
    output: String,
}

impl WhereSerializer {
    fn new() -> Self {
        Self {
            output: String::new(),
        }
    }
}

impl<'a> ser::Serializer for &'a mut WhereSerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = ser::Impossible<Self::Ok, Error>;
    type SerializeTuple = ser::Impossible<Self::Ok, Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Error>;
    type SerializeMap = ser::Impossible<Self::Ok, Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("bool {:?}", v)))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("number {:?}", v)))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("char {:?}", v)))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("string {:?}", v)))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::Filter(format!("bytes array {:?}", v)))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::Name(format!("unit struct {}", name)))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::Name(format!("unit variant {}::{}", name, variant)))
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::Value(format!("{}::{}", name, variant)))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::Value("sequence".into()))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::Value("tuple".into()))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::Value(format!("tuple struct {}", name)))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::Value(format!("{}::{}", name, variant)))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::Value("map".into()))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::Value(format!(
            "struct variant {}::{}",
            name, variant
        )))
    }
}

impl<'a> ser::SerializeStruct for &'a mut WhereSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        match key {
            "only" => {
                let mut filter_list_serializer = FilterListSerializer::new();
                value.serialize(&mut filter_list_serializer)?;
                let filter_list = &filter_list_serializer.output;
                if !filter_list.is_empty() {
                    if !self.output.is_empty() {
                        self.output += " AND ";
                    }
                    self.output += filter_list;
                }
            }
            "except" => {
                let mut filter_list_serializer = FilterListSerializer::new();
                value.serialize(&mut filter_list_serializer)?;
                let filter_list = &filter_list_serializer.output;
                if !filter_list.is_empty() {
                    if !self.output.is_empty() {
                        self.output += " AND ";
                    }
                    self.output += "NOT ";
                    if filter_list.contains(" AND ") && !filter_list.starts_with('(') {
                        self.output += "(";
                        self.output += filter_list;
                        self.output += ")";
                    } else {
                        self.output += filter_list;
                    }
                }
            }
            _ => {
                let mut name_serializer = NameSerializer::new();
                key.serialize(&mut name_serializer)?;
                let name = name_serializer.output;

                let mut value_serializer = ValueSerializer::new();
                value.serialize(&mut value_serializer)?;
                let value = value_serializer.output;

                if !name.is_empty() && !value.is_empty() {
                    if !self.output.is_empty() {
                        self.output += " AND ";
                    }
                    self.output += &name;
                    self.output += " = ";
                    self.output += &value;
                }
            }
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if !self.output.is_empty() {
            self.output = format!(" WHERE {}", self.output);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde::Serialize;

    #[repr(C)]
    #[derive(Serialize)]
    struct MyFilterItem {
        namespace: Option<String>,
        table_name: Option<String>,
        column_names: Option<Vec<String>>,
    }

    #[repr(C)]
    #[derive(Serialize)]
    struct MyFilter {
        limit: i32,
        only: Option<Vec<MyFilterItem>>,
        except: Option<Vec<MyFilterItem>>,
    }

    impl ToSql for MyFilter {}

    #[test]
    fn config() {
        let f = MyFilter {
            limit: 10,
            only: Some(vec![
                MyFilterItem {
                    namespace: Some("public".to_string()),
                    table_name: None,
                    column_names: None,
                },
                MyFilterItem {
                    namespace: None,
                    table_name: Some("users".to_string()),
                    column_names: None,
                },
            ]),
            except: Some(vec![MyFilterItem {
                namespace: None,
                table_name: Some("messages".to_string()),
                column_names: Some(vec!["user_id".to_string()]),
            }]),
        };

        let sql = String::from(
            " WHERE limit = 10 \
              AND (namespace = 'public' OR table_name = 'users') \
              AND NOT (table_name = 'messages' AND column_names = ARRAY['user_id'])",
        );

        assert_eq!(sql, f.to_sql().unwrap());
    }
}
