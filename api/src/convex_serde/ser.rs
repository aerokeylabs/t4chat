use std::collections::BTreeMap;
use std::fmt::Display;

use convex::Value;
use serde::ser::{
  Impossible, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
  SerializeTupleStruct, SerializeTupleVariant,
};
use serde::{Serialize, Serializer};

pub fn to_value<T: Serialize>(value: &T) -> Result<Value, Error> {
  value.serialize(ValueSerializer)
}

pub fn to_map<T: Serialize>(value: &T) -> Result<BTreeMap<String, Value>, Error> {
  match value.serialize(ValueSerializer)? {
    Value::Object(map) => Ok(map),
    _ => Err(Error::Custom("Expected an object".to_string())),
  }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("cannot serialize u64 values outside of i64 range: {0}")]
  IntOutOfRange(u64),
  #[error("cannot serialize key that is not a string: {0}")]
  KeyNotString(String),
  #[error("cannot serialize map that is not an object: {0}")]
  MapNotObject(String),
  #[error("{0}")]
  Custom(String),
}

impl Error {
  fn key_not_string<T: Display, R>(value: T) -> Result<R, Self> {
    Err(Self::KeyNotString(value.to_string()))
  }

  fn map_not_object<T: Display, R>(value: T) -> Result<R, Self> {
    Err(Self::MapNotObject(value.to_string()))
  }
}

impl serde::ser::Error for Error {
  fn custom<T: std::fmt::Display>(msg: T) -> Self {
    Error::Custom(msg.to_string())
  }
}

pub struct ValueSerializer;

impl Serializer for ValueSerializer {
  type Error = Error;
  type Ok = Value;
  type SerializeMap = SerializeValueMap;
  type SerializeSeq = SerializeValueArray;
  type SerializeStruct = SerializeValueMap;
  type SerializeStructVariant = SerializeValueMap;
  type SerializeTuple = SerializeValueArray;
  type SerializeTupleStruct = SerializeValueArray;
  type SerializeTupleVariant = SerializeValueArray;

  fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
    Ok(Value::Boolean(v))
  }

  fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
    self.serialize_i64(v as i64)
  }

  fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
    self.serialize_i64(v as i64)
  }

  fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
    self.serialize_i64(v as i64)
  }

  fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
    Ok(Value::Int64(v))
  }

  fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
    self.serialize_i64(v as i64)
  }

  fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
    self.serialize_i64(v as i64)
  }

  fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
    self.serialize_i64(v as i64)
  }

  fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
    if v <= i64::MAX as u64 {
      Ok(Value::Int64(v as i64))
    } else {
      Err(Error::IntOutOfRange(v))
    }
  }

  fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
    self.serialize_f64(v as f64)
  }

  fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
    Ok(Value::Float64(v))
  }

  fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
    Ok(Value::String(v.to_string()))
  }

  fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
    Ok(Value::String(v.to_string()))
  }

  fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
    Ok(Value::Bytes(v.to_vec()))
  }

  fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
    Ok(Value::Null)
  }

  fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
    value.serialize(self)
  }

  fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
    Ok(Value::Null)
  }

  fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
    Ok(Value::Null)
  }

  fn serialize_unit_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
  ) -> Result<Self::Ok, Self::Error> {
    Ok(Value::String(variant.to_string()))
  }

  fn serialize_newtype_struct<T: ?Sized + Serialize>(
    self,
    _name: &'static str,
    value: &T,
  ) -> Result<Self::Ok, Self::Error> {
    value.serialize(self)
  }

  fn serialize_newtype_variant<T: ?Sized + Serialize>(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    value: &T,
  ) -> Result<Self::Ok, Self::Error> {
    let mut map = BTreeMap::new();
    map.insert(variant.to_string(), value.serialize(self)?);
    Ok(Value::Object(map))
  }

  fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
    Ok(SerializeValueArray {
      values: Vec::with_capacity(len.unwrap_or(0)),
    })
  }

  fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
    self.serialize_seq(Some(len))
  }

  fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
    self.serialize_tuple(len)
  }

  fn serialize_tuple_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    _variant: &'static str,
    len: usize,
  ) -> Result<Self::SerializeTupleVariant, Self::Error> {
    self.serialize_tuple(len)
  }

  fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
    Ok(SerializeValueMap::new())
  }

  fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
    self.serialize_map(None)
  }

  fn serialize_struct_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    _variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeStructVariant, Self::Error> {
    self.serialize_map(None)
  }
}

pub struct SerializeValueArray {
  values: Vec<Value>,
}

impl SerializeSeq for SerializeValueArray {
  type Error = Error;
  type Ok = Value;

  fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
    self.values.push(value.serialize(ValueSerializer)?);

    Ok(())
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
    Ok(Value::Array(self.values))
  }
}

impl SerializeTuple for SerializeValueArray {
  type Error = Error;
  type Ok = Value;

  fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
    SerializeSeq::serialize_element(self, value)
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
    SerializeSeq::end(self)
  }
}

impl SerializeTupleStruct for SerializeValueArray {
  type Error = Error;
  type Ok = Value;

  fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
    SerializeSeq::serialize_element(self, value)
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
    SerializeSeq::end(self)
  }
}

impl SerializeTupleVariant for SerializeValueArray {
  type Error = Error;
  type Ok = Value;

  fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
    SerializeSeq::serialize_element(self, value)
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
    SerializeSeq::end(self)
  }
}

pub struct KeySerializer;

impl Serializer for KeySerializer {
  type Error = Error;
  type Ok = String;
  type SerializeMap = Impossible<Self::Ok, Self::Error>;
  type SerializeSeq = Impossible<Self::Ok, Self::Error>;
  type SerializeStruct = Impossible<Self::Ok, Self::Error>;
  type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;
  type SerializeTuple = Impossible<Self::Ok, Self::Error>;
  type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
  type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;

  fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
    Error::key_not_string(v)
  }

  fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
    Error::key_not_string(v)
  }

  fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
    Error::key_not_string(v)
  }

  fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
    Error::key_not_string(v)
  }

  fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
    Error::key_not_string(v)
  }

  fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
    Error::key_not_string(v)
  }

  fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
    Error::key_not_string(v)
  }

  fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
    Error::key_not_string(v)
  }

  fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
    Error::key_not_string(v)
  }

  fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
    Error::key_not_string(v)
  }

  fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
    Error::key_not_string(v)
  }

  fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
    Ok(v.to_string())
  }

  fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
    Ok(v.to_string())
  }

  fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
    Error::key_not_string(format!("{:?}", v))
  }

  fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
    Error::key_not_string("None")
  }

  fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
    value.serialize(self)
  }

  fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
    Error::key_not_string("()")
  }

  fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
    Error::key_not_string(name)
  }

  fn serialize_unit_variant(
    self,
    name: &'static str,
    _variant_index: u32,
    variant: &'static str,
  ) -> Result<Self::Ok, Self::Error> {
    Error::key_not_string(format!("{name}::{variant}"))
  }

  fn serialize_newtype_struct<T: ?Sized + Serialize>(
    self,
    _name: &'static str,
    value: &T,
  ) -> Result<Self::Ok, Self::Error> {
    value.serialize(self)
  }

  fn serialize_newtype_variant<T: ?Sized + Serialize>(
    self,
    name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    _value: &T,
  ) -> Result<Self::Ok, Self::Error> {
    Error::key_not_string(format!("{name}::{variant}()"))
  }

  fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
    Error::key_not_string("[]")
  }

  fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
    Error::key_not_string("()")
  }

  fn serialize_tuple_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
    Error::key_not_string(format!("{name}()"))
  }

  fn serialize_tuple_variant(
    self,
    name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeTupleVariant, Self::Error> {
    Error::key_not_string(format!("{name}::{variant}()"))
  }

  fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
    Error::key_not_string("{}")
  }

  fn serialize_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
    Error::key_not_string(format!("{name} {{}}"))
  }

  fn serialize_struct_variant(
    self,
    name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeStructVariant, Self::Error> {
    Error::key_not_string(format!("{name}::{variant} {{}}"))
  }
}

pub struct SerializeBTreeMap {
  map: BTreeMap<String, Value>,
  key: Option<String>,
}

impl SerializeBTreeMap {
  pub fn new() -> Self {
    SerializeBTreeMap {
      key: None,
      map: BTreeMap::new(),
    }
  }
}

impl SerializeMap for SerializeBTreeMap {
  type Error = Error;
  type Ok = BTreeMap<String, Value>;

  fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
    self.key = Some(key.serialize(KeySerializer)?);
    Ok(())
  }

  fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
    let key = self
      .key
      .take()
      .ok_or_else(|| Error::Custom("Key not set before value serialization".to_string()))?;
    self.map.insert(key, value.serialize(ValueSerializer)?);
    Ok(())
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
    if self.map.is_empty() {
      Ok(BTreeMap::new())
    } else {
      Ok(self.map)
    }
  }
}

impl SerializeStruct for SerializeBTreeMap {
  type Error = Error;
  type Ok = BTreeMap<String, Value>;

  fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
    self.key = Some(key.to_string());
    SerializeMap::serialize_value(self, value)
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
    SerializeMap::end(self)
  }
}

impl SerializeStructVariant for SerializeBTreeMap {
  type Error = Error;
  type Ok = BTreeMap<String, Value>;

  fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
    self.key = Some(key.to_string());
    SerializeMap::serialize_value(self, value)
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
    SerializeMap::end(self)
  }
}

pub struct SerializeValueMap {
  inner: SerializeBTreeMap,
}

impl SerializeValueMap {
  pub fn new() -> Self {
    SerializeValueMap {
      inner: SerializeBTreeMap::new(),
    }
  }
}

impl SerializeMap for SerializeValueMap {
  type Error = Error;
  type Ok = Value;

  fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
    self.inner.serialize_key(key)
  }

  fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
    self.inner.serialize_value(value)
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
    Ok(Value::Object(SerializeMap::end(self.inner)?))
  }
}

impl SerializeStruct for SerializeValueMap {
  type Error = Error;
  type Ok = Value;

  fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
    SerializeStruct::serialize_field(&mut self.inner, key, value)
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
    Ok(Value::Object(SerializeStruct::end(self.inner)?))
  }
}

impl SerializeStructVariant for SerializeValueMap {
  type Error = Error;
  type Ok = Value;

  fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
    SerializeStructVariant::serialize_field(&mut self.inner, key, value)
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
    Ok(Value::Object(SerializeStructVariant::end(self.inner)?))
  }
}

impl Serializer for SerializeBTreeMap {
  type Error = Error;
  type Ok = BTreeMap<String, Value>;
  type SerializeMap = SerializeBTreeMap;
  type SerializeSeq = Impossible<Self::Ok, Self::Error>;
  type SerializeStruct = SerializeBTreeMap;
  type SerializeStructVariant = SerializeBTreeMap;
  type SerializeTuple = Impossible<Self::Ok, Self::Error>;
  type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
  type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;

  fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
    Error::map_not_object(v)
  }

  fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
    Error::map_not_object(v)
  }

  fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
    Error::map_not_object(v)
  }

  fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
    Error::map_not_object(v)
  }

  fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
    Error::map_not_object(v)
  }

  fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
    Error::map_not_object(v)
  }

  fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
    Error::map_not_object(v)
  }

  fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
    Error::map_not_object(v)
  }

  fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
    Error::map_not_object(v)
  }

  fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
    Error::map_not_object(v)
  }

  fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
    Error::map_not_object(v)
  }

  fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
    Error::map_not_object(v)
  }

  fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
    Error::map_not_object(v)
  }

  fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
    Error::map_not_object(format!("{:?}", v))
  }

  fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
    Error::map_not_object("None")
  }

  fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
    value.serialize(self)
  }

  fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
    Error::map_not_object("()")
  }

  fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
    Error::map_not_object(name)
  }

  fn serialize_unit_variant(
    self,
    name: &'static str,
    _variant_index: u32,
    variant: &'static str,
  ) -> Result<Self::Ok, Self::Error> {
    Error::map_not_object(format!("{name}::{variant}"))
  }

  fn serialize_newtype_struct<T: ?Sized + Serialize>(
    self,
    name: &'static str,
    _value: &T,
  ) -> Result<Self::Ok, Self::Error> {
    Error::map_not_object(name)
  }

  fn serialize_newtype_variant<T: ?Sized + Serialize>(
    self,
    name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    _value: &T,
  ) -> Result<Self::Ok, Self::Error> {
    Error::map_not_object(format!("{name}::{variant}()"))
  }

  fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
    Error::map_not_object("[]")
  }

  fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
    Error::map_not_object("()")
  }

  fn serialize_tuple_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
    Error::map_not_object(format!("{name}()"))
  }

  fn serialize_tuple_variant(
    self,
    name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeTupleVariant, Self::Error> {
    Error::map_not_object(format!("{name}::{variant}()"))
  }

  fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
    Ok(SerializeBTreeMap::new())
  }

  fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
    self.serialize_map(None)
  }

  fn serialize_struct_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    _variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeStructVariant, Self::Error> {
    self.serialize_map(None)
  }
}

#[cfg(test)]
mod tests {
  use convex::Value;

  use super::*;

  #[test]
  fn test_serialize_bool() {
    let value = true.serialize(ValueSerializer).unwrap();
    assert_eq!(value, Value::Boolean(true));
  }

  #[test]
  fn test_serialize_i64() {
    let value = 42i64.serialize(ValueSerializer).unwrap();
    assert_eq!(value, Value::Int64(42));
  }

  #[test]
  fn test_serialize_string() {
    let value = "hello".serialize(ValueSerializer).unwrap();
    assert_eq!(value, Value::String("hello".to_string()));
  }

  #[derive(Serialize)]
  struct TestStruct {
    field1: i32,
    field2: String,
  }

  #[test]
  fn test_serialize_struct() {
    let test = TestStruct {
      field1: 123,
      field2: "test".to_string(),
    };

    let value = test.serialize(ValueSerializer).unwrap();

    let expected = Value::Object(BTreeMap::from([
      ("field1".to_string(), Value::Int64(123)),
      ("field2".to_string(), Value::String("test".to_string())),
    ]));

    assert_eq!(value, expected);
  }

  #[derive(Serialize)]
  struct TestStructWithArray {
    field1: i32,
    field2: Vec<String>,
  }

  #[test]
  fn test_serialize_struct_with_array() {
    let test = TestStructWithArray {
      field1: 123,
      field2: vec!["test1".to_string(), "test2".to_string()],
    };

    let value = test.serialize(ValueSerializer).unwrap();

    let expected = Value::Object(BTreeMap::from([
      ("field1".to_string(), Value::Int64(123)),
      (
        "field2".to_string(),
        Value::Array(vec![
          Value::String("test1".to_string()),
          Value::String("test2".to_string()),
        ]),
      ),
    ]));

    assert_eq!(value, expected);
  }
}
