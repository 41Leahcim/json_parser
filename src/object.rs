use std::fmt::Display;

use crate::Json;

#[derive(Debug, PartialEq, Default)]
pub struct Object {
    values: Vec<(String, Json)>,
}

impl From<Vec<(String, Json)>> for Object {
    fn from(values: Vec<(String, Json)>) -> Self {
        Self { values }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        if !self.values.is_empty() {
            write!(f, "{:?}:{}", self.values[0].0, self.values[0].1)?;
            for (key, value) in self.values.iter().skip(1) {
                write!(f, ",{key:?}:{value}")?;
            }
        }
        write!(f, "}}")
    }
}

impl Object {
    #[allow(clippy::must_use_candidate)]
    pub fn get(&self, requested_key: &str) -> Option<&Json> {
        self.values
            .iter()
            .find(|(object_key, _)| object_key == requested_key)
            .map(|(_, value)| value)
    }

    #[allow(clippy::must_use_candidate)]
    pub fn get_mut(&mut self, requested_key: &str) -> Option<&mut Json> {
        self.values
            .iter_mut()
            .find(|(object_key, _)| object_key == requested_key)
            .map(|(_, value)| value)
    }

    pub fn remove(&mut self, requested_key: &str) -> Option<Json> {
        let (index, (_, _)) = self
            .values
            .iter()
            .enumerate()
            .find(|(_, (object_key, _))| object_key == requested_key)?;
        Some(self.values.remove(index).1)
    }

    pub fn add(&mut self, key: String, value: Json) {
        self.values.push((key, value));
    }

    pub fn shrink_to_fit(&mut self) {
        self.values.shrink_to_fit();
    }
}
