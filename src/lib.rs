//! This crate is a rust port of a Andre Eisenbach's excellent
//! program phpSyntaxTree. It also follows Yoichihiro Hasebe's
//! implementation in ruby, RSyntaxTree. Check both out!
//!
//! Author: benjirewis
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate typed_builder;
extern crate image;
extern crate regex;

mod element;
mod error;
mod graph;
mod parser;

use std::path::{Path, PathBuf};

lazy_static! {
    static ref FONT_DIR: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("fonts");
    static ref SUBSCRIPT_CONST: f32 = option_env!("SUBSCRIPT_CONST")
        .unwrap_or("0.7")
        .parse::<f32>()
        .expect("invalid SUBSCRIPT_CONST");
}

#[derive(TypedBuilder)]
pub struct TreeGenerator {
    /// Data to be formatted
    #[builder(default)]
    data: Option<String>,

    /// Path to find data to be formatted
    #[builder(default)]
    data_path: Option<PathBuf>,

    /// Auto subscripting of phrasal levels:w
    #[builder(default=Some(false))]
    auto_sub: Option<bool>,

    /// Display with color
    #[builder(default=Some(true))]
    color: Option<bool>,

    /// Font to display
    #[builder(default)]
    font: Option<Font>,

    /// Font size to display
    #[builder(default=Some(18))]
    font_size: Option<usize>,

    /// Vertical height to display
    #[builder(default=Some(1.0))]
    height: Option<f32>,

    /// Margins of display
    #[builder(default=Some(0))]
    margin: Option<usize>,

    /// Symmetry of display
    #[builder(default=Some(true))]
    symmetrize: Option<bool>,
}

impl TreeGenerator {
    pub fn draw_tree() {
        unimplemented!();
    }
}

pub enum Font {
    NotoSans,
    NotoSerif,
}

impl Default for Font {
    fn default() -> Self {
        Font::NotoSans
    }
}
