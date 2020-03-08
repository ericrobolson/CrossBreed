use std::path::Path;

extern crate collada;
use collada::*;

pub struct ColladaMesh {
    document: collada::document::ColladaDocument,
    id: usize,
}

impl ColladaMesh {
    pub fn from_collada(id: usize, path: &Path) -> Self {
        let doc = collada::document::ColladaDocument::from_path(path).unwrap();

        let mut c = Self {
            id: id,
            document: doc,
        };
        return c;
    }
}
