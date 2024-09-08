pub mod config;
pub mod emit_ilproj;

use std::{fmt, fs::{self, File}, io::{self, Read}, path::{Path, PathBuf}};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use ariadne::{Cache, Source};
use config::Config;
use walkdir::WalkDir;

use crate::errors::CompilerError;


#[derive(Debug)]
pub struct Project {
    root_path: PathBuf,
    files: HashMap<PathBuf, Source>,
    pub config: Config,
    export_debug_symbols: bool,
}

impl Project {
    pub fn new(path: &Path, export_debug_symbols: bool) -> Result<Self, CompilerError> {
        let config_path = dunce::canonicalize(path)?.join(Self::config_file_rel_path());

        let mut file = File::open(config_path)?;

        let mut buf = "".to_string();
        file.read_to_string(&mut buf)?;

        let toml_config = toml::from_str(&buf)?;
        

        let project = Project {
            root_path: dunce::canonicalize(path)?,
            config: toml_config,
            files: HashMap::new(),
            export_debug_symbols
        };

        Ok(project)
    }

    pub fn config_file_rel_path() -> PathBuf {
        PathBuf::from("arpm.toml")
    }
    pub fn src_rel_path() -> PathBuf {
        PathBuf::from("src")
    }

    pub fn root_file_rel_path() -> PathBuf {
        PathBuf::from("main.arp")
    }

    pub fn export_debug_symbols(&self) -> bool {
        self.export_debug_symbols
    }

    pub fn debug_output_path(&self) -> PathBuf {
        self.root_path.join("build/debug")
    }

    pub fn il_output_path(&self) -> PathBuf {
        self.root_path.join("build/ilproj")
    }

    pub fn output_path(&self) -> PathBuf {
        self.root_path.join("build/output")
    }

    pub fn build_path(&self) -> PathBuf {
        self.root_path.join("build")
    }


    pub fn load_source(&mut self, path: &Path) -> Result<String, io::Error> {
        match self.fetch(&path) {
            Ok(source) => Ok(source.text().to_string()),
            Err(_) => {
                Err(io::Error::new(io::ErrorKind::NotFound, format!("Failed to load file {}", path.display())))
            },
        }
    }

    fn collect_sources(path: &Path) -> Vec<PathBuf> {
        WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
            .flat_map(|entry| {
                let absolute_path = entry.path();
                if let Ok(relative_path) = absolute_path.strip_prefix(path) {
                    Some(relative_path.to_path_buf())
                } else {
                    None
                }
            }).collect()
    }
    
    pub fn with_relative_path<T: AsRef<Path>>(&self, path: T) -> PathBuf {
        self.root_path.join(path)
    }

    pub fn debug_path_for_file<T: AsRef<Path>>(&self, path: T) -> PathBuf {
        self.debug_output_path().join(path)
    }

    pub fn build_path_for_file<T: AsRef<Path>>(&self, path: T) -> PathBuf {
        self.il_output_path().join(path)
    }
    
    pub(crate) fn project_files(&self) -> Vec<PathBuf> {
        let src_path = &self.root_path.join(Self::src_rel_path());
        
        let sources = Self::collect_sources(src_path).iter().map(|file_path| PathBuf::from("src").join(file_path)).collect::<Vec<_>>();

        sources
    }

    pub(crate) fn load_sources(&self) -> Result<Vec<arp_types::sources::Source>, CompilerError> {
        self.project_files()
            .iter()
            .map(|input_path| {
                let path = self.with_relative_path(input_path);
                let input = fs::read_to_string(path.clone())?;

                Ok(arp_types::sources::Source::new_inline(input_path.strip_prefix("src/").unwrap(), input))
            })
            .collect::<Result<Vec<_>, _>>()
    }
}


impl Cache<&Path> for Project {
    type Storage = String;

    fn fetch(&mut self, path: &&Path) -> Result<&Source, Box<dyn fmt::Debug + '_>> {
        let file = self.root_path.join(path);
        Ok(match self.files.entry(file.clone()) {
            // TODO: Don't allocate here
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(Source::from(
                fs::read_to_string(file).map_err(|e| Box::new(e) as _)?,
            )),
        })
    }
    fn display<'a>(&self, path: &&'a Path) -> Option<Box<dyn fmt::Display + 'a>> {
        Some(Box::new(path.display()))
    }
}