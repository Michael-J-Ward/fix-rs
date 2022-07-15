// Copyright 2016 James Bendig. See the COPYRIGHT file at the top-level
// directory of this distribution.
//
// Licensed under:
//   the MIT license
//     <LICENSE-MIT or https://opensource.org/licenses/MIT>
//   or the Apache License, Version 2.0
//     <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0>,
// at your option. This file may not be copied, modified, or distributed
// except according to those terms.

//! fix-rs is a [FIX][1] (Financial Information Exchange) engine written in Rust.
//!
//! # Supported FIX Versions
//! - FIX 4.0
//! - FIX 4.1
//! - FIX 4.2
//! - FIX 4.3
//! - FIX 4.4
//! - FIX 5.0
//! - FIX 5.0 SP 1
//! - FIX 5.0 SP 2
//!
//! [1]: http://www.fixtradingcommunity.org/

#![allow(clippy::suspicious_else_formatting)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::manual_map)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::redundant_pattern_matching)]
#![allow(clippy::toplevel_ref_arg)]
#![allow(clippy::from_over_into)]
#![allow(clippy::vec_box)]
#![allow(clippy::match_ref_pats)]
#![allow(clippy::mem_replace_with_default)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::ptr_offset_with_cast)]
#![allow(clippy::into_iter_on_ref)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::needless_return)]

pub mod byte_buffer;
#[macro_use]
pub mod fixt;
pub mod constant;
#[macro_use]
pub mod field;
pub mod field_tag;
pub mod field_type;
pub mod fix;
pub mod fix_version;
pub mod hash;
#[macro_use]
pub mod message;
pub mod message_version;
mod network_read_retry;
pub mod rule;
mod token_generator;

//Dictionary is put last because it needs the above macros.
#[macro_use]
pub mod dictionary;

pub use fix_rs_macros::{BuildField, BuildMessage};

mod generate;
pub use generate::generate_dictionary;
