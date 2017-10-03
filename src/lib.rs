//! Inspect and do work with RiSCAN Pro projects.
//!
//! [RiSCAN PRO](http://www.riegl.com/products/software-packages/riscan-pro/) is [Terrestrial Laser
//! Scanning (TLS)](https://en.wikipedia.org/wiki/Lidar#Terrestrial_Lidar) software made by
//! [Riegl](http://riegl.com/). This crate provides a library and an executable for inspecting and
//! doing work with RiSCAN Pro projects.
//!
//! **Riegl did not make this software, and cannot provide any support for it.**
//! **Please do not contact Riegl with questions about this software.**
//!
//! # Examples
//!
//! Projects can be opened by providing the project's directory to `Project::from_path`:
//!
//! ```
//! use riscan_pro::Project;
//! let project = Project::from_path("data/project.RiSCAN").unwrap();
//! ```
//!
//! or the `project.rsp` file:
//!
//! ```
//! use riscan_pro::Project;
//! let project = Project::from_path("data/project.RiSCAN/project.rsp").unwrap();
//! ```

#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
        trivial_numeric_casts, unsafe_code, unstable_features, unused_qualifications)]

extern crate alga;
#[cfg(test)]
#[macro_use]
extern crate approx;
extern crate nalgebra;
extern crate sxd_document;
extern crate sxd_xpath;

#[macro_use]
mod macros;

mod image;
mod project;
mod utils;

pub use image::Image;
pub use project::{Project, rsp_path};

/// Our custom error enum.
#[derive(Debug)]
pub enum Error {
    /// Wrapper around `std::io::Error`.
    Io(std::io::Error),
    /// Wrapper around `std::num::ParseFloatError`.
    ParseFloat(std::num::ParseFloatError),
    /// Invalid project path.
    ProjectPath(std::path::PathBuf),
    /// An error that occurs while parsing an xml file.
    XmlParse(usize, Vec<sxd_document::parser::Error>),
    /// Wrapper around `sxd_xpath::Error`.
    Xpath(sxd_xpath::Error),
    /// The provided xpath was not found.
    XpathNotFound(String),
}

/// Our custom result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Our type of Transform3.
pub type Transform3 = nalgebra::Transform3<f64>;

/// Our type of Point3.
pub type Point3 = nalgebra::Point3<f64>;

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Error {
        Error::ParseFloat(err)
    }
}

impl From<(usize, Vec<sxd_document::parser::Error>)> for Error {
    fn from((n, v): (usize, Vec<sxd_document::parser::Error>)) -> Error {
        Error::XmlParse(n, v)
    }
}

impl From<sxd_xpath::Error> for Error {
    fn from(err: sxd_xpath::Error) -> Error {
        Error::Xpath(err)
    }
}
