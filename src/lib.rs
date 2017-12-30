//! # Cursive
//!
//! [Cursive](https://github.com/gyscos/Cursive) is a TUI library built on top
//! of ncurses-rs.
//! It allows to easily build layouts for text-based applications.
//!
//! ## Getting started
//!
//! * Every application should start with a [`Cursive`](struct.Cursive.html)
//!   object. It is the main entry-point to the library.
//! * A declarative phase then describes the structure of the UI by adding
//!   views and configuring their behaviours.
//! * Finally, the event loop is started by calling
//!   [`Cursive::run(&mut self)`](struct.Cursive.html#method.run).
//!
//! ## Views
//!
//! Views are the main components of a cursive interface.
//! The [`views`](./views/index.html) module contains many views to use in your
//! application; if you don't find what you need, you may also implement the
//! [`View`](view/trait.View.html) trait and build your own.
//!
//! ## Callbacks
//!
//! Cursive is a *reactive* UI: it *reacts* to events generated by user input.
//!
//! During the declarative phase, callbacks are set to trigger on specific
//! events. These functions usually take a `&mut Cursive` argument, allowing
//! them to modify the view tree at will.
//!
//! ## Examples
//!
//! ```no_run
//! extern crate cursive;
//!
//! use cursive::Cursive;
//! use cursive::views::TextView;
//!
//! fn main() {
//!     let mut siv = Cursive::new();
//!
//!     siv.add_layer(TextView::new("Hello World!\nPress q to quit."));
//!
//!     siv.add_global_callback('q', |s| s.quit());
//!
//!     siv.run();
//! }
//! ```
//!
//! ## Debugging
//!
//! The `Cursive` root initializes the terminal on creation, and do cleanups
//! on drop. While it is alive, printing to the terminal will not work
//! as expected, making debugging a bit harder.
//!
//! One solution is to redirect stderr to a file when running the application,
//! and log to it instead of stdout.
//!
//! Or you can use gdb as usual.
#![deny(missing_docs)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate maplit;
extern crate num;
extern crate owning_ref;
extern crate toml;
extern crate unicode_segmentation;
extern crate unicode_width;

#[cfg(feature = "termion")]
#[macro_use]
extern crate chan;

macro_rules! new_default(
    ($c:ty) => {
        impl Default for $c {
            fn default() -> Self {
                Self::new()
            }
        }
    }
);

pub mod traits;

pub mod event;
#[macro_use]
pub mod view;

pub mod views;
pub mod vec;
pub mod theme;
pub mod align;
pub mod menu;
pub mod direction;
pub mod utils;

// This probably doesn't need to be public?
mod cursive;
mod printer;
mod xy;
mod with;

mod div;
mod utf8;

#[doc(hidden)]
pub mod backend;

pub use cursive::{Cursive, ScreenId};
pub use printer::Printer;
pub use with::With;
pub use xy::XY;
