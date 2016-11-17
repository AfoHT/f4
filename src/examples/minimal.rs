// Auto-generated. Do not modify this file! Instead modify examples/minimal.rs
//! Minimal binary size
//!
//! ``` rust,no_run
//! #![feature(lang_items)]
//! #![no_main]
//! #![no_std]
//!
//! extern crate f3;
//!
//! #[no_mangle]
//! pub fn main() -> ! {
//!     loop {}
//! }
//!
//! #[lang = "panic_fmt"]
//! extern "C" fn panic_fmt() -> ! {
//!     loop {}
//! }
//!
//! #[export_name = "_default_exception_handler"]
//! pub extern "C" fn handler() {
//!     loop {}
//! }
//!
//! #[export_name = "_init"]
//! pub fn init() {}
//! ```
