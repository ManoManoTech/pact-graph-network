// Copyright 2022 ManoMano Colibri SAS.
// SPDX-License-Identifier: MIT

use serde::Serialize;

#[derive(Debug, Serialize, Clone, Default)]
pub struct Item {
    id: usize,
    name: String,
    depends_on: Vec<usize>,
}

impl Item {
    pub fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }

    /// Add a dependency on an other item an returns a new instance
    pub fn add_depends(&self, item_id: usize) -> Self {
        let mut this = self.clone();
        this.depends_on.push(item_id);
        this
    }

    /// Returns the id of this [`Item`].
    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns a reference to the name of this [`Item`].
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Returns a reference to the depends on of this [`Item`].
    pub fn depends_on(&self) -> &[usize] {
        self.depends_on.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn create_items() {
        let item = Item::new(32, String::from("foo"));
        assert_eq!(32, item.id);
        assert_eq!(String::from("foo"), item.name);
    }

    #[test]
    fn add_depends() {
        let item = Item::new(32, String::from("foo"));
        let new_item = item.add_depends(34);
        assert_eq!(0, item.depends_on.len());
        assert_eq!(1, new_item.depends_on.len());
    }
}
