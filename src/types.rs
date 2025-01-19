//! hyprlang specific types, such as color, vec2 and gradient

use serde::{de::Visitor, Deserialize, Serialize};

/// Serves to provide a 2 dimensional vector  
/// It is serialized as a string with the two numbers deliminated by a space
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Vec2(pub f64, pub f64);

impl Serialize for Vec2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(format!("{} {}", self.0, self.1).as_str())
    }
}

impl<'de> Deserialize<'de> for Vec2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        
        struct VecVisitor;
        
        impl<'de> Visitor<'de> for VecVisitor {
            type Value = Vec2;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string with two numbers seperated by a space")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {

                if let Some((first, second)) = v.split_once(" ") {
                    match (first.parse::<f64>(), second.parse::<f64>()) {
                        (Ok(f),Ok(s)) => Ok(Vec2(f, s)),
                        (Err(e), Err(e2)) => Err(E::custom(format!("Parse failed twice: {e}; {e2}"))),
                        (Ok(_), Err(e)) => Err(E::custom(format!("Parse failed on second value: {e}"))),
                        (Err(e), Ok(_)) => Err(E::custom(format!("Parse failed on first value: {e}"))),
                    }
                } else {
                    Err(E::custom(format!("Invalid formating")))
                }
            }
        }

        deserializer.deserialize_string(VecVisitor)
    }
}
