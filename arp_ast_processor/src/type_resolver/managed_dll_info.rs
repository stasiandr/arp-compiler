use std::{path::Path, process::Command};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DllImporterError {
    #[error("Std error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Utf8 error")]
    Utf8Convert(#[from] std::string::FromUtf8Error),

    #[error("Not Success. stderr:\n{0}")]
    NotSuccess(String),

    #[error("Json error")]
    JsonError(#[from] serde_json::Error),

    #[error("Type not found")]
    TypeNotFound,
}

pub fn get_info<P: AsRef<Path>>(path: P) -> Result<SharpTypeCollection, DllImporterError> {
    let _ = path;
    let cmd_result = Command::new("dotnet") // TODO Temporary path to exporter
        .args([
            "run",
            "--project",
            "/Users/stas/learn/csharp-assembly-info-exporter",
            "/Users/stas/learn/csharp-assembly-info-exporter/System.Console.dll", // &path.as_ref().to_string_lossy()
        ])
        .output()?;

    if !cmd_result.status.success() {
        return Err(DllImporterError::NotSuccess(String::from_utf8(
            cmd_result.stderr,
        )?));
    }

    let output = String::from_utf8(cmd_result.stdout)?;
    let sharp_collection: SharpTypeCollection = serde_json::from_str(&output)?;

    Ok(sharp_collection)
}

pub fn resolve_type<P: AsRef<Path>, N: AsRef<str>>(
    path: &P,
    name: N,
) -> Result<SharpTypeInfo, DllImporterError> {
    let info = get_info(path)?;
    info.collection
        .iter()
        .find(|ty| ty.full_name == name.as_ref())
        .cloned()
        .ok_or(DllImporterError::TypeNotFound)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SharpTypeCollection {
    pub collection: Vec<SharpTypeInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SharpTypeInfo {
    pub full_name: String,
    pub short_name: Option<String>,
    pub fields: Vec<SharpTypedIdent>,
    pub methods: Vec<SharpMethodInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SharpTypedIdent {
    pub ident: String,
    pub ty_full_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SharpMethodInfo {
    pub ident: String,
    pub return_ty_full_name: String,
    pub args: Vec<SharpTypedIdent>,
}
