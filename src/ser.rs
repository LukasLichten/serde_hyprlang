use serde::{ser, Serialize};

use crate::{Error, Result};

const SPACES_PER_TAB:usize = 4;

pub fn to_string<T>(value: &T) -> Result<String> where T: Serialize {
    let mut serial = Serializer {
        output: String::new(),
        fieldname: vec![],
        field_prefix_needed: true,
        within_tupple: 0
    };
    value.serialize(&mut serial)?;
    Ok(serial.output)
}


pub struct Serializer {
    output: String,
    fieldname: Vec<&'static str>,
    field_prefix_needed: bool,
    within_tupple: usize
}

impl Serializer {
    fn write_field_prefix(&mut self) {
        if !self.field_prefix_needed {
            self.field_prefix_needed = true;
            return;
        }

        self.indent();
        self.output += match self.fieldname.last() {
            Some(current) => current,
            None => ""
        };
        self.output += " = "
    }

    fn add_fieldname(&mut self, name: &'static str) {
        self.fieldname.push(name);
        self.field_prefix_needed = true;
        
        self.write_field_prefix();

        self.field_prefix_needed = false;
    }

    fn pop_fieldname(&mut self) {
        self.fieldname.pop();
    }
    
    fn indent(&mut self) {
        for _ in 1..self.fieldname.len() {
            // Yes, we relly on the compiler to hopefully optimize the inner for loop away
            for _ in 0..SPACES_PER_TAB {
                self.output += " ";
            }
        }
    }

    fn is_in_tupple(&self) -> bool {
        self.within_tupple > 0
    }

    fn tupple_start(&mut self) -> Result<()> {
        self.within_tupple += 1;

        Ok(())
    }

    fn tupple_element<T>(&mut self, value: &T) -> Result<()>
        where T: ?Sized + Serialize {

        value.serialize(&mut *self)?;
        self.output += ", ";

        Ok(())
    }

    fn tupple_end(&mut self) -> Result<()> {
        if let Some(trimmed) = self.output.strip_suffix(", ") {
            self.output = trimmed.to_string();
        } else {
            return Err(Error::UnexpectedSequence(self.output.clone()))
        }
        self.within_tupple -= 1;
        
        Ok(())
    }

    fn struct_start(&mut self) -> Result<()> {
        if self.fieldname.is_empty() {
            // This is the root object, so no closure and indentation
        } else {
            if self.is_in_tupple() {
                // the struct is within a tupple
            } else {
                // standard serialization as a category
                if let Some(trimmed) = self.output.strip_suffix("= ") {
                    self.output = trimmed.to_string();
                } else {
                    return Err(Error::UnexpectedSequence(self.output.clone()))
                }

                self.output += "{\n";
            }
        }

        Ok(())
    }

    fn struct_element<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize {

        if self.is_in_tupple() {
            self.output += key;
            self.output += ": ";
            
            value.serialize(&mut *self)?;

            self.output += ", ";


        } else {
            self.add_fieldname(key);
            
            value.serialize(&mut *self)?;
            self.output += "\n";

            self.pop_fieldname();   
        }
        Ok(())
    }

    fn struct_end(&mut self) -> Result<()> {
        if self.is_in_tupple() {
            // We would otherwise produce an extra ,
            // as the tupple containing this struct will also append one
            if let Some(trimmed) = self.output.strip_suffix(", ") {
                self.output = trimmed.to_string();
            } else {
                return Err(Error::UnexpectedSequence(self.output.clone()))
            }
        } else if !self.fieldname.is_empty() {
            // We have to close this category
            self.indent();
            self.output += "}";
            self.output += "\n";
        }

        Ok(())
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_i8(self, v: i8) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i16(self, v: i16) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(self, v: i32) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(self, v: i64) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u16(self, v: u16) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u32(self, v: u32) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u64(self, v: u64) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(self, v: f64) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += &v.to_string();
        Ok(())
    }

    fn serialize_bool(self, v: bool) -> std::result::Result<Self::Ok, Self::Error> {
        self.output += if v { "true" } else { "false" };
        Ok(())
    }

    fn serialize_char(self, v: char) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> std::result::Result<Self::Ok, Self::Error> {
        if v.contains(',') {
            self.output += "\"";            
            self.output += v;            
            self.output += "\"";            
        } else {
            self.output += v;
        }

        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::Message("Not supported yet".to_string()))
    }

    fn serialize_some<T>(self, value: &T) -> std::result::Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize {
        value.serialize(self)
    }

    fn serialize_none(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit(self) -> std::result::Result<Self::Ok, Self::Error> {
        // self.output += "";
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> std::result::Result<Self::Ok, Self::Error> {
        // self.output += "";
        Ok(())
    }

    fn serialize_unit_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
        ) -> std::result::Result<Self::Ok, Self::Error> {
        // Unit Variant is a "Classic" Enum, aka without data

        self.output += variant;
        Ok(())
    }

    // Newtype are single field types, aka wrappers
    fn serialize_newtype_struct<T>(
            self,
            _name: &'static str,
            value: &T,
        ) -> std::result::Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
            value: &T,
        ) -> std::result::Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize {
        
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> std::result::Result<Self::SerializeSeq, Self::Error> {
        // We serialize sequences as multiple fields of the same name
        
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> std::result::Result<Self::SerializeTuple, Self::Error> {
        
        self.tupple_start()?;
        Ok(self)
    }

    fn serialize_tuple_struct(
            self,
            _name: &'static str,
            _len: usize,
        ) -> std::result::Result<Self::SerializeTupleStruct, Self::Error> {

        self.tupple_start()?;
        Ok(self)
    }

    fn serialize_tuple_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
            _len: usize,
        ) -> std::result::Result<Self::SerializeTupleVariant, Self::Error> {
        
        self.tupple_start()?;
        Ok(self)
    }

    fn serialize_struct(
            self,
            _name: &'static str,
            _len: usize,
        ) -> std::result::Result<Self::SerializeStruct, Self::Error> {
        
        self.struct_start()?;
        Ok(self)
    }

    // Enum Varient with multiple values, all named
    fn serialize_struct_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
            _len: usize,
        ) -> std::result::Result<Self::SerializeStructVariant, Self::Error> {

        self.struct_start()?;
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> std::result::Result<Self::SerializeMap, Self::Error> {
        // We use this to serialize things such as class, title etc in windowrules
        
        Err(Error::NotSupported("Maps"))
    }

    fn collect_str<T>(self, value: &T) -> std::result::Result<Self::Ok, Self::Error>
        where
            T: ?Sized + std::fmt::Display {
        self.serialize_str(value.to_string().as_str())
        
    }
}

// We serialize sequences as the same fieldname/key multiple times
impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
        where
            T: ?Sized + Serialize {
        if self.is_in_tupple() {
            return Err(Error::NotSupported("A sequence (Array/List/etc) contained inside a Tupple"));

        } else {
            self.write_field_prefix();
            value.serialize(&mut **self)?;
            self.output += "\n";
        }
        
        Ok(())
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        // We are producing an additional new line, but oh well

        if self.is_in_tupple() {
            return Err(Error::NotSupported("A sequence (Array/List/etc) contained inside a Tupple"));

        } else {}

        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> std::result::Result<(), Self::Error>
        where
            T: ?Sized + Serialize {
        
        Err(Error::NotSupported("Maps"))
    }

    fn serialize_value<T>(&mut self, _value: &T) -> std::result::Result<(), Self::Error>
        where
            T: ?Sized + Serialize {
        
        Err(Error::NotSupported("Maps"))
    }

    fn serialize_entry<K, V>(&mut self, _key: &K, _value: &V) -> std::result::Result<(), Self::Error>
        where
            K: ?Sized + Serialize,
            V: ?Sized + Serialize, {
        
        Err(Error::NotSupported("Maps"))
    }


    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::NotSupported("Maps"))
    }
}

// Structs are serialized as categories
// Except if they are within tupples
impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> std::result::Result<(), Self::Error>
        where
            T: ?Sized + Serialize {
        self.struct_element(key, value)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.struct_end()
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> std::result::Result<(), Self::Error>
        where
            T: ?Sized + Serialize {
        self.struct_element(key, value)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.struct_end()
    }

}

// Tupples are always sequences behind a single key/fieldname
impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
        where
            T: ?Sized + Serialize {
        
        self.tupple_element(value)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.tupple_end()
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
        where
            T: ?Sized + Serialize {
        
        self.tupple_element(value)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.tupple_end()
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
        where
            T: ?Sized + Serialize {
        
        self.tupple_element(value)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.tupple_end()
    }
}

