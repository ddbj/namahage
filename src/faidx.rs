use std::ffi;
use std::path::Path;

use rust_htslib::htslib;

use crate::errors::{Error, Result};

pub fn build<P: AsRef<Path>>(input: P) -> Result<()> {
    match input.as_ref().to_str() {
        Some(path) => {
            let p = ffi::CString::new(path).unwrap();

            // pub fn fai_build(fn_: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
            let ret: i32 = unsafe { htslib::fai_build(p.as_ptr()) };

            if ret == 0 {
                Ok(())
            } else {
                Err(Error::FaidxBuildError(path.to_string()))?
            }
        }
        None => Err(Error::FilePathError(
            input.as_ref().to_string_lossy().to_string(),
        ))?,
    }
}
