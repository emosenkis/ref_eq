// Copyright 2012-2016 The Rust Project Developers. See the COPYRIGHT
// at http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The `req_eq` crate makes it easy to check reference equality.
//!
//! # Examples
//!
//! ```
//! use ref_eq::ref_eq;
//!
//! let x = 1;
//! let y = 1;

//! assert!(ref_eq(&x, &x));
//! assert!(!ref_eq(&x, &y));
//! ```

/// Determine if two borrowed pointers point to the same thing.
#[inline]
pub fn ref_eq<'a, 'b, T>(thing: &'a T, other: &'b T) -> bool {
    (thing as *const T) == (other as *const T)
}
