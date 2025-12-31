use std::{
    cell::OnceCell,
    collections::HashMap,
    path::PathBuf,
    sync::{LazyLock, OnceLock},
};

use markdown::{ParseOptions, mdast, to_mdast};

/// A store is a repository of all of the state maintained by Nous.
pub struct Store {
    /// Mapping of each Markdown file to their contents.
    documents: HashMap<PathBuf, String>,
    /// Lazily initialised map of the AST of each document.
    /// Note that the actual keys themselves must be eagerly initialised in
    /// order to iterate through the documents.
    asts: HashMap<PathBuf, OnceLock<mdast::Node>>,
}

impl Store {
    pub fn new<I, E>(collection: I) -> Self
    where
        I: Iterator<Item = (E, String)>,
        E: Into<PathBuf>,
    {
        let documents = collection
            .map(|(entry, content)| (entry.into(), content))
            .collect::<HashMap<PathBuf, String>>();

        let asts = documents
            .keys()
            .cloned()
            .map(|k| (k, OnceLock::new()))
            .collect();

        Self { documents, asts }
    }

    pub fn get_ast(&self, key: &PathBuf) -> Option<&mdast::Node> {
        let cell = self.asts.get(key)?;

        let ast = cell.get_or_init(|| {
            let text = self.documents.get(key).expect(
                "If the key exists in `self.asts`, the text must exist in `self.documents`!",
            );
            to_mdast(text, &ParseOptions::default())
                .expect("Parsing should work")
                .clone()
        });

        Some(ast)
    }
}

impl<E> FromIterator<(E, String)> for Store
where
    E: Into<PathBuf>,
{
    fn from_iter<T: IntoIterator<Item = (E, String)>>(iter: T) -> Self {
        Store::new(iter.into_iter())
    }
}

impl<I> From<I> for Store
where
    I: IntoIterator<Item = (walkdir::DirEntry, String)>,
{
    fn from(value: I) -> Self {
        let store: Store = value.into_iter().map(|(k,v)| (k.path().to_path_buf(), v)).collect();
        store
    }
}
