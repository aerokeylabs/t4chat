use std::collections::BTreeMap;
use std::collections::btree_map::Iter;
use std::fmt::Display;

use convex::Value;
use serde::de::{
  Deserialize, DeserializeOwned, DeserializeSeed, Deserializer, EnumAccess, IntoDeserializer, MapAccess, SeqAccess,
  VariantAccess, Visitor,
};

pub fn from_value_ref<'de, T: Deserialize<'de>>(value: &'de Value) -> Result<T, Error> {
  let mut deserializer = ValueDeserializer::from_value(value);
  T::deserialize(&mut deserializer)
}

pub fn from_value<T: DeserializeOwned>(value: Value) -> Result<T, Error> {
  let mut deserializer = ValueDeserializer::from_value(&value);
  T::deserialize(&mut deserializer)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("expected bool value, got {0:?}")]
  ExpectedBool(Value),
  #[error("expected i64 value, got {0:?}")]
  ExpectedInt(Value),
  #[error("expected f64 value, got {0:?}")]
  ExpectedFloat(Value),
  #[error("expected string value, got {0:?}")]
  ExpectedString(Value),
  #[error("expected bytes value, got {0:?}")]
  ExpectedBytes(Value),
  #[error("expected sequence value, got {0:?}")]
  ExpectedSeq(Value),
  #[error("expected object value, got {0:?}")]
  ExpectedObject(Value),
  #[error("expected value for map access")]
  ExpectedValue,
  #[error("expected enum value, got {0:?}")]
  ExpectedEnum(Value),
  #[error("{0}")]
  Custom(String),
}

impl serde::de::Error for Error {
  fn custom<T: Display>(msg: T) -> Self {
    Error::Custom(msg.to_string())
  }
}

pub struct ValueDeserializer<'de> {
  input: &'de Value,
}

impl<'de> ValueDeserializer<'de> {
  pub fn from_value(input: &'de Value) -> Self {
    Self { input }
  }
}

impl<'de: 'a, 'a> Deserializer<'de> for &'a mut ValueDeserializer<'de> {
  type Error = Error;

  serde::forward_to_deserialize_any! {
    i8 i16 i32 u8 u16 u32 u64 f32 char
    tuple tuple_struct ignored_any byte_buf
    map
  }

  fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
    match self.input {
      Value::Null => self.deserialize_option(visitor),
      Value::Int64(_) => self.deserialize_i64(visitor),
      Value::Float64(_) => self.deserialize_f64(visitor),
      Value::Boolean(_) => self.deserialize_bool(visitor),
      Value::String(_) => self.deserialize_str(visitor),
      Value::Bytes(_) => self.deserialize_bytes(visitor),
      Value::Array(_) => self.deserialize_seq(visitor),
      Value::Object(map) => ObjectDeserializer::new(map).deserialize_any(visitor),
    }
  }

  fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
    match self.input {
      Value::Boolean(b) => visitor.visit_bool(*b),
      value => Err(Self::Error::ExpectedBool(value.clone())),
    }
  }

  fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
    match self.input {
      Value::Int64(i) => visitor.visit_i64(*i),
      value => Err(Self::Error::ExpectedInt(value.clone())),
    }
  }

  fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
    match self.input {
      Value::Float64(f) => visitor.visit_f64(*f),
      value => Err(Self::Error::ExpectedFloat(value.clone())),
    }
  }

  fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
    match self.input {
      Value::String(s) => visitor.visit_borrowed_str(s),
      value => Err(Self::Error::ExpectedString(value.clone())),
    }
  }

  fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
    match self.input {
      Value::String(s) => visitor.visit_string(s.clone()),
      value => Err(Self::Error::ExpectedString(value.clone())),
    }
  }

  fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
    match self.input {
      Value::Bytes(b) => visitor.visit_borrowed_bytes(b),
      value => Err(Self::Error::ExpectedBytes(value.clone())),
    }
  }

  fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
    match self.input {
      Value::Null => visitor.visit_none(),
      _ => visitor.visit_some(self),
    }
  }

  fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
    match self.input {
      Value::Array(values) => visitor.visit_seq(SeqValues::new(values)),
      value => Err(Self::Error::ExpectedSeq(value.clone())),
    }
  }

  fn deserialize_newtype_struct<V: Visitor<'de>>(
    self,
    _name: &'static str,
    visitor: V,
  ) -> Result<V::Value, Self::Error> {
    visitor.visit_newtype_struct(self)
  }

  fn deserialize_struct<V: Visitor<'de>>(
    self,
    _name: &'static str,
    _fields: &'static [&'static str],
    visitor: V,
  ) -> Result<V::Value, Self::Error> {
    self.deserialize_any(visitor)
  }

  fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
    visitor.visit_unit()
  }

  fn deserialize_unit_struct<V: Visitor<'de>>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error> {
    visitor.visit_unit()
  }

  fn deserialize_enum<V: Visitor<'de>>(
    self,
    _name: &'static str,
    _variants: &'static [&'static str],
    visitor: V,
  ) -> Result<V::Value, Self::Error> {
    match self.input {
      Value::String(s) => visitor.visit_enum(s.clone().into_deserializer()),
      Value::Object(_) => visitor.visit_enum(EnumValue::new(self)),
      value => Err(Self::Error::ExpectedEnum(value.clone())),
    }
  }

  fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
    self.deserialize_str(visitor)
  }
}

pub struct EnumValue<'a, 'de: 'a> {
  de: &'a mut ValueDeserializer<'de>,
}

impl<'de: 'a, 'a> EnumValue<'a, 'de> {
  pub fn new(de: &'a mut ValueDeserializer<'de>) -> Self {
    Self { de }
  }
}

impl<'de, 'a> EnumAccess<'de> for EnumValue<'a, 'de> {
  type Error = Error;
  type Variant = Self;

  fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error> {
    Ok((seed.deserialize(&mut *self.de)?, self))
  }
}

impl<'de, 'a> VariantAccess<'de> for EnumValue<'a, 'de> {
  type Error = Error;

  fn unit_variant(self) -> Result<(), Self::Error> {
    Err(Error::ExpectedString(self.de.input.clone()))
  }

  fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value, Self::Error> {
    seed.deserialize(self.de)
  }

  fn tuple_variant<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error> {
    Deserializer::deserialize_seq(self.de, visitor)
  }

  fn struct_variant<V: Visitor<'de>>(
    self,
    _fields: &'static [&'static str],
    visitor: V,
  ) -> Result<V::Value, Self::Error> {
    Deserializer::deserialize_map(self.de, visitor)
  }
}

struct SeqValues<'de> {
  values: &'de [Value],
  index: usize,
}

impl<'de> SeqValues<'de> {
  fn new(values: &'de [Value]) -> Self {
    Self { values, index: 0 }
  }
}

impl<'de> SeqAccess<'de> for SeqValues<'de> {
  type Error = Error;

  fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> {
    if self.index < self.values.len() {
      let value = &self.values[self.index];
      self.index += 1;
      let mut deserializer = ValueDeserializer::from_value(value);
      seed.deserialize(&mut deserializer).map(Some)
    } else {
      Ok(None)
    }
  }

  fn size_hint(&self) -> Option<usize> {
    Some(self.values.len() - self.index)
  }
}

struct KeyDeserializer<'de> {
  input: &'de str,
}

impl<'de> KeyDeserializer<'de> {
  fn new(input: &'de str) -> Self {
    Self { input }
  }
}

impl<'de> Deserializer<'de> for &mut KeyDeserializer<'de> {
  type Error = Error;

  serde::forward_to_deserialize_any! {
    bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char
    bytes byte_buf option unit unit_struct newtype_struct
    seq tuple tuple_struct map struct enum ignored_any
  }

  fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
    visitor.visit_str(self.input)
  }

  fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
    visitor.visit_str(self.input)
  }

  fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
    visitor.visit_string(self.input.to_string())
  }

  fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
    visitor.visit_str(self.input)
  }
}

struct ObjectDeserializer<'de> {
  input: &'de BTreeMap<String, Value>,
}

impl<'de> ObjectDeserializer<'de> {
  fn new(input: &'de BTreeMap<String, Value>) -> Self {
    Self { input }
  }
}

impl<'de> Deserializer<'de> for ObjectDeserializer<'de> {
  type Error = Error;

  serde::forward_to_deserialize_any! {
    bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char
    bytes byte_buf option unit unit_struct newtype_struct
    seq tuple tuple_struct map struct enum ignored_any
    str string identifier
  }

  fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
    visitor.visit_map(ObjectMapAccess::new(self))
  }
}

struct ObjectMapAccess<'de> {
  iter: Iter<'de, String, Value>,
  value: Option<&'de Value>,
}

impl<'de> ObjectMapAccess<'de> {
  fn new(deserializer: ObjectDeserializer<'de>) -> Self {
    let iter = deserializer.input.iter();
    Self { iter, value: None }
  }
}

impl<'de> MapAccess<'de> for ObjectMapAccess<'de> {
  type Error = Error;

  fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
    if let Some((k, v)) = self.iter.next() {
      let mut deserializer = KeyDeserializer::new(k);
      let ret = seed.deserialize(&mut deserializer);
      self.value = Some(v);
      ret.map(Some)
    } else {
      Ok(None)
    }
  }

  fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
    if let Some(value) = self.value.take() {
      let mut deserializer = ValueDeserializer::from_value(value);
      seed.deserialize(&mut deserializer)
    } else {
      Err(Self::Error::ExpectedValue)
    }
  }
}

#[cfg(test)]
mod tests {
  use anyhow::Context;
  use convex::Value;

  use super::*;

  #[derive(serde::Deserialize)]
  struct TestStruct {
    pub field1: i64,
    pub field2: String,
  }

  #[derive(serde::Deserialize)]
  struct TestStructWithArray {
    pub field1: i64,
    pub field2: String,
    pub field3: Vec<i64>,
  }

  #[test]
  fn test_deserialize_struct() {
    let value = Value::Object(
      vec![
        ("field1".to_string(), Value::Int64(42)),
        ("field2".to_string(), Value::String("hello".to_string())),
      ]
      .into_iter()
      .collect(),
    );

    let test_struct: TestStruct = from_value(value).context("deserialization failed").unwrap();

    assert_eq!(test_struct.field1, 42);
    assert_eq!(test_struct.field2, "hello");
  }

  #[test]
  fn test_deserialize_struct_with_array() {
    let value = Value::Object(
      vec![
        ("field1".to_string(), Value::Int64(42)),
        ("field2".to_string(), Value::String("hello".to_string())),
        (
          "field3".to_string(),
          Value::Array(vec![Value::Int64(1), Value::Int64(2), Value::Int64(3)]),
        ),
      ]
      .into_iter()
      .collect(),
    );

    let test_struct: TestStructWithArray = from_value(value).context("deserialization failed").unwrap();

    assert_eq!(test_struct.field1, 42);
    assert_eq!(test_struct.field2, "hello");
    assert_eq!(test_struct.field3, vec![1, 2, 3]);
  }

  #[test]
  fn test_deserialize_i64() {
    let value = Value::Int64(42);
    let result: Result<i64, Error> = from_value(value);
    assert_eq!(result.unwrap(), 42);
  }

  #[test]
  fn test_deserialize_string() {
    let value = Value::String("hello".to_string());
    let result: Result<String, Error> = from_value(value);
    assert_eq!(result.unwrap(), "hello");
  }

  #[test]
  fn test_deserialize_bool() {
    let value = Value::Boolean(true);
    let result: Result<bool, Error> = from_value(value);
    assert!(result.unwrap());
  }

  #[test]
  fn test_deserialize_option_struct() {
    let value = Value::Object(
      vec![
        ("field1".to_string(), Value::Int64(42)),
        ("field2".to_string(), Value::String("hello".to_string())),
      ]
      .into_iter()
      .collect(),
    );

    let test_struct: Option<TestStruct> = from_value(value).context("deserialization failed").ok();
    assert!(test_struct.is_some());
    let test_struct = test_struct.unwrap();
    assert_eq!(test_struct.field1, 42);
    assert_eq!(test_struct.field2, "hello");

    let none_value = Value::Null;
    let none_struct: Option<TestStruct> = from_value(none_value).context("deserialization failed").ok();
    assert!(none_struct.is_none());
  }

  #[test]
  fn test_deserialize_seq() {
    let value = Value::Array(vec![Value::Int64(1), Value::Int64(2), Value::Int64(3)]);
    let result: Result<Vec<i64>, Error> = from_value(value);
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
  }

  #[test]
  fn test_deserialize_map() {
    let value = Value::Object(
      vec![
        ("key1".to_string(), Value::String("value1".to_string())),
        ("key2".to_string(), Value::String("value2".to_string())),
      ]
      .into_iter()
      .collect(),
    );

    let result: Result<BTreeMap<String, String>, Error> = from_value(value);

    let expected: BTreeMap<String, String> = vec![
      ("key1".to_string(), "value1".to_string()),
      ("key2".to_string(), "value2".to_string()),
    ]
    .into_iter()
    .collect();

    assert_eq!(result.unwrap(), expected);
  }

  #[test]
  fn test_deserialize_error() {
    let value = Value::String("not an int".to_string());
    let result: Result<i64, Error> = from_value(value);
    assert!(result.is_err());
    assert_eq!(
      result.unwrap_err().to_string(),
      "expected i64 value, got String(\"not an int\")"
    );
  }

  #[test]
  fn test_deserialize_unit() {
    let value = Value::Null;
    from_value::<()>(value).unwrap()
  }

  #[derive(serde::Deserialize, PartialEq, Eq, Debug)]
  #[serde(rename_all = "camelCase")]
  enum TestEnum {
    Variant1,
    Variant2,
  }

  #[test]
  fn test_deserialize_enum() {
    let value = Value::String("variant1".to_string());
    let result = from_value::<TestEnum>(value).unwrap();
    assert_eq!(result, TestEnum::Variant1);

    let value = Value::String("variant2".to_string());
    let result = from_value::<TestEnum>(value).unwrap();
    assert_eq!(result, TestEnum::Variant2);

    let value = Value::String("invalid".to_string());
    let result: Result<TestEnum, Error> = from_value(value);
    assert!(result.is_err());
  }
}
