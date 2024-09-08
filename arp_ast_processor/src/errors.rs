use thiserror::Error;

use crate::{chumsky_folder::ChumskyFoldError, type_resolver::{managed_dll_info::DllImporterError, TypeResolverError}, validations::ValidationError};

#[derive(Error, Debug)]
pub enum ProcessingError {

    #[error("ChumskyFold {0}")]
    ChumskyFold(#[from] ChumskyFoldError),

    #[error("TypeResolver {0}")]
    TypeResolver(#[from] TypeResolverError),

    #[error("Validation {0}")]
    Validation(#[from] ValidationError),

    #[error("Dll import {0}")]
    DllImport(#[from] DllImporterError),

    #[error("No Sources")]
    NotSourcesProvided,

}