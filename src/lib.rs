//! TODO_DOCS_DESCRIPTION
//!
//! [![Zulip Chat](https://img.shields.io/endpoint?label=chat&url=https%3A%2F%2Fiteration-square-automation.schichler.dev%2F.netlify%2Ffunctions%2Fstream_subscribers_shield%3Fstream%3Dproject%252Fasteracea-native-windows-gui)](https://iteration-square.schichler.dev/#narrow/stream/project.2Fasteracea-native-windows-gui)

#![doc(html_root_url = "https://docs.rs/asteracea-native-windows-gui/0.0.1")]
#![warn(clippy::pedantic, missing_docs)]
#![allow(clippy::semicolon_if_nothing_returned, clippy::type_complexity)]

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

#[cfg(feature = "TextInput")]
mod text_input;

#[cfg(feature = "TextInput")]
pub use text_input::TextInput;
