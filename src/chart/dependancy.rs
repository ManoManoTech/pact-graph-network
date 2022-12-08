// Copyright 2022 ManoMano Colibri SAS.
// SPDX-License-Identifier: MIT

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Dependency {
    source: usize,
    target: usize,
}

impl Dependency {
    /// Creates a new [`Dependency`].
    pub fn new(source: usize, target: usize) -> Self {
        Self { source, target }
    }
}

#[cfg(test)]
mod test {
    use super::Dependency;

    #[test]
    fn create_dependency() {
        let deps = Dependency::new(4, 6);
        assert_eq!(4, deps.source);
        assert_eq!(6, deps.target);
    }
}
