use std::{fs, io, path::{Path, PathBuf}};



pub struct Source {
    path: PathBuf,
    loaded_string: String,
}

impl std::fmt::Debug for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct("Source")
            .field("path", &self.path)
            // .field("loaded_string", &self.loaded_string)
            .finish()
    }
}

impl Source {
    pub fn new<P : Into<PathBuf> + AsRef<Path>>(path: P) -> io::Result<Self> {
        let loaded_string = fs::read_to_string(path.as_ref())?;

        Ok(Self { 
            path: path.as_ref().into(),
            loaded_string,
        })
    }

    pub fn new_inline<P : Into<PathBuf> + AsRef<Path>, S : Into<String>>(path: P, input: S) -> Self {
        Self {
            path: path.as_ref().into(),
            loaded_string: input.into(),
        }
    }

    pub fn content(&self) -> &str {
        &self.loaded_string
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }

    pub fn get_path_string(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    pub fn get_module_string(&self) -> String {
        self.path.to_string_lossy().to_string().trim_end_matches(".arp").replace("/", ".").replace("\\", ".")
    }


    pub fn len(&self) -> usize {
        self.loaded_string.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

}