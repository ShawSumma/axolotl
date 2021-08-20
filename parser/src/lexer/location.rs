use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Error)]
pub enum LocParseErr {
    #[error(display = "failed to get column number")]
    FailedToGetCol,
    #[error(display = "failed to get line number")]
    FailedToGetLine,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub filename: String,
    pub coord: Coordinate,
}

impl Serialize for Location {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!(
            "{}:{}:{}",
            self.filename, self.coord.line, self.coord.col
        ))
    }
}

struct LocVisitor;

impl<'de> Visitor<'de> for LocVisitor {
    type Value = Location;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expected filename:line:col")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let parts = s.split(":").collect::<Vec<&str>>();

        let filename = if let Some(file) = parts.get(0) {
            file.to_string()
        } else {
            return Err(de::Error::custom("missing file name"));
        };
        let line = if let Some(line) = parts.get(1).map_or_else(
            || None,
            |v| v.parse::<usize>().map_or_else(|_| None, |v| Some(v)),
        ) {
            line
        } else {
            return Err(de::Error::custom("missing or unable to parse line number"));
        };
        let col = if let Some(col) = parts.get(2).map_or_else(
            || None,
            |v| v.parse::<usize>().map_or_else(|_| None, |v| Some(v)),
        ) {
            col
        } else {
            return Err(de::Error::custom(
                "missing or unable to parse column number",
            ));
        };
        Ok(Location {
            filename,
            coord: Coordinate { line, col },
        })
    }
}

#[derive(Clone, PartialEq, Eq, Copy)]
pub struct Coordinate {
    pub line: usize,
    pub col: usize,
}

impl Coordinate {
    pub fn newline(mut self, lines: usize) -> Coordinate {
        self.line += lines;
        self.col = 0;
        self
    }
}

impl std::ops::Add<usize> for Coordinate {
    type Output = Self;
    fn add(mut self, rhs: usize) -> Self {
        self.col += rhs;
        self
    }
}

impl std::ops::AddAssign<usize> for Coordinate {
    fn add_assign(&mut self, rhs: usize) {
        self.col += rhs;
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

impl Serialize for Coordinate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}:{}", self.line, self.col))
    }
}

impl std::str::FromStr for Coordinate {
    type Err = LocParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(":").collect::<Vec<&str>>();
        let line = if let Some(line) = parts.get(0).map_or_else(
            || None,
            |v| v.parse::<usize>().map_or_else(|_| None, |v| Some(v)),
        ) {
            line
        } else {
            return Err(LocParseErr::FailedToGetLine);
        };
        let col = if let Some(col) = parts.get(1).map_or_else(
            || None,
            |v| v.parse::<usize>().map_or_else(|_| None, |v| Some(v)),
        ) {
            col
        } else {
            return Err(LocParseErr::FailedToGetCol);
        };
        Ok(Coordinate { line, col })
    }
}

struct CoordVisitor;
impl<'de> Visitor<'de> for CoordVisitor {
    type Value = Coordinate;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expected line:col")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match Coordinate::from_str(s) {
            Ok(s) => Ok(s),
            Err(e) => Err(de::Error::custom(e)),
        }
    }
}

impl<'de> Deserialize<'de> for Coordinate {
    fn deserialize<D>(deserializer: D) -> Result<Coordinate, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(CoordVisitor)
    }
}
