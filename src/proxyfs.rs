use url::Url;
use std::path::{Component, Path, PathBuf};
use std::collections::HashMap;

pub struct ProxyFS {
    entries: Vec<FSEntry>,
    urls: Vec<Url>,
    root: usize,
}

#[derive(Copy, Clone, Debug)]
struct FSDir {
    id: usize,
    parent: usize,

    destinaion: usize,
    children: HashMap<String, usize>,
}

#[derive(Copy, Clone, Debug)]
struct FSFile {
    id: usize,
    parent: usize,

    root: usize,
    branch: String,
}

enum FSEntry {
    Directory(FSDir),
    File(FSFile),
}

impl ProxyFS {
    pub fn new() -> Self {
        let entries = vec![FSDir {
            id: 0,
            parent: 0,
            destinaion: 0,
            children: HashMap::new(),
        }];
        // We put a default entry in index 0 so we can call it "invalid"
        let urls = vec![Url::parse("http://example.com").unwrap()];
        ProxyFS {
            entires,
            urls,
            root: 0,
        }
    }

    pub fn add_directory(&mut self, path: &Path, destination: &Url) -> bool {
        let mut 
        for component in path.iter() {
            
        }
    }

    pub fn add_mappings(&mut self, path: &Path, mappings: &HashMap<PathBuf, Url>) -> bool {

    }

    pub fn map_path(&self) -> Option<Url> {

    }

    fn resolve_path(&self, path: &Path) -> Option<usize> {
        
    }

    fn resolve_closest_path(&self, path: &Path) -> (usize, usize) {
        let mut id = self.root;
        let mut count = 0;
        for component in path.components() {
            if let FSEntry::Directory(d) = self.entries[id] {
                match component {
                    Component::Normal(os_str) => {
                        let s = os_str.to_str();
                        if s.is_none() {
                            return (id, count);
                        }
                        let new_id = d.children.get(s.unwrap().to_owned());
                        if new_id.is_none() {
                            return (id, count);
                        }
                        id = new_id.unwrap();
                    },
                    Component::ParentDir => {
                        // If id == parent, then it's the root object
                        if d.id == d.parent {
                            return (id, count);
                        }
                        id = d.parent;
                    },
                    Component::CurDir => {},
                    _ => {
                        return (id, count);
                    }
                }
                count += 1;
            } else {
                // entry is not a directory, fail
                return (id, count);
            }
        }
    }
}

impl FSEntry {
    fn id(&self) -> usize {
        match self {
            FSEntry::File(f) => f.id,
            FSEntry::Directory(d) => d.id,
        }
    }

    fn parent_id(&self) -> usize {
        match self {
            FSEntry::File(f) => f.parent,
            FSEntry::Directory(d) => d.parent,
        }
    }
}


