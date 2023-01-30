//! # My Crate
//! 
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.
//!
//! Art
//! 
//! A library for creating and manipulating art.

/// This is a doc comment for the `p1` module.
/// 
/// # Examples
/// 
/// ```
/// let five = 5;
/// 
/// assert_eq!(6, p1::add_one(five));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}


pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary color
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary color
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use super::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        // ANCHOR_END: here
        SecondaryColor::Orange
        // ANCHOR: here
    }
}