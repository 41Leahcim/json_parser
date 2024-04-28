use core::fmt::{self, Display, Formatter};

use alloc::{string::String, vec::Vec};

use crate::Json;

#[derive(Debug, PartialEq, Default)]
pub struct Object {
    values: Vec<(String, Json)>,
}

impl From<Vec<(String, Json)>> for Object {
    #[inline]
    fn from(values: Vec<(String, Json)>) -> Self {
        Self { values }
    }
}

impl Display for Object {
    #[inline]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{{")?;
        let mut values = self.values.iter();
        if let Some((key, value)) = values.next() {
            write!(formatter, "{key:?}:{value}")?;
            for (key, value) in values {
                write!(formatter, ",{key:?}:{value}")?;
            }
        }
        write!(formatter, "}}")
    }
}

impl Object {
    #[allow(clippy::must_use_candidate)]
    #[inline]
    pub fn get(&self, requested_key: &str) -> Option<&Json> {
        self.values
            .iter()
            .find(|(key, _)| key == requested_key)
            .map(|(_, value)| value)
    }

    #[allow(clippy::must_use_candidate)]
    #[inline]
    pub fn get_mut(&mut self, requested_key: &str) -> Option<&mut Json> {
        self.values
            .iter_mut()
            .find(|(key, _)| key == requested_key)
            .map(|(_, value)| value)
    }

    #[inline]
    pub fn remove(&mut self, requested_key: &str) -> Option<Json> {
        let (index, _) = self
            .values
            .iter()
            .enumerate()
            .find(|(_, (key, _))| key == requested_key)?;
        Some(self.values.remove(index).1)
    }

    #[inline]
    pub fn add(&mut self, key: String, value: Json) {
        if let Some(target) = self.get_mut(&key) {
            *target = value;
        } else {
            self.values.push((key, value));
        }
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.values.shrink_to_fit();
    }
}
