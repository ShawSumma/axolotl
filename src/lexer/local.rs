use serde::{
    de::{self, Deserialize, Deserializer, Visitor},
    ser::{Serialize, Serializer},
};

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub file: String,
    pub line: u32,
    pub col: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Coordinate {
    pub line: u32,
    pub col: u32,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.col)
    }
}

impl Serialize for Location {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct LocationVisitor;
impl Visitor<'_> for LocationVisitor {
    type Value = Location;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "something in the form of filename:line:col")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let parts: Vec<&str> = s.split(':').collect();
        if let Some(file) = parts.get(0) {
            if let Some(line) = parts.get(1) {
                if let Some(col) = parts.get(2) {
                    Ok(Location {
                        file: file.to_string(),
                        line: match line.parse::<u32>() {
                            Ok(val) => val,
                            Err(e) => return Err(de::Error::custom(e)),
                        },
                        col: match col.parse::<u32>() {
                            Ok(val) => val,
                            Err(e) => return Err(de::Error::custom(e)),
                        },
                    })
                } else {
                    Err(de::Error::custom("missing column number"))
                }
            } else {
                Err(de::Error::custom("missing line number"))
            }
        } else {
            Err(de::Error::custom("missing file name"))
        }
    }
}

impl<'de> Deserialize<'de> for Location {
    fn deserialize<D>(deserializer: D) -> Result<Location, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(LocationVisitor)
    }
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

impl Serialize for Coordinate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct CoordinateVisitor;
impl Visitor<'_> for CoordinateVisitor {
    type Value = Coordinate;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "something in the form of filename:line:col")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let parts: Vec<&str> = s.split(':').collect();
        if let Some(line) = parts.get(0) {
            if let Some(col) = parts.get(1) {
                Ok(Coordinate {
                    line: match line.parse::<u32>() {
                        Ok(val) => val,
                        Err(e) => return Err(de::Error::custom(e)),
                    },
                    col: match col.parse::<u32>() {
                        Ok(val) => val,
                        Err(e) => return Err(de::Error::custom(e)),
                    },
                })
            } else {
                Err(de::Error::custom("missing column number"))
            }
        } else {
            Err(de::Error::custom("missing line number"))
        }
    }
}

impl<'de> Deserialize<'de> for Coordinate {
    fn deserialize<D>(deserializer: D) -> Result<Coordinate, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(CoordinateVisitor)
    }
}

#[test]
fn local_serialize_test() {
    let location = Location {
        file: String::from("main.axol"),
        line: 12,
        col: 32,
    };
    println!(
        "{}",
        ron::from_str::<Location>(&ron::to_string(&location).unwrap()).unwrap()
    );
}
