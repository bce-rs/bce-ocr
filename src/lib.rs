#![allow(dead_code)]
#![allow(non_camel_case_types)]

pub mod v1 {
    /// accurate text ocr without position
    pub mod accurate_basic;
}

/// 
mod access_token;
pub use access_token::acquire_access_token;
