//! The parts of the installer that are not the installer.
//!
//! Three layers live here, and they are stacked in one direction only:
//!
//! - [`ui`] is the reusable toolkit. It knows about buttons and colours.
//! - [`dialogue`] is the engine. It knows about steps and answers, and nothing
//!   about widgets.
//! - [`script`] is the glue, and the only layer that knows both.
//!
//! They are in the library rather than the binary on purpose. A library module
//! cannot see the binary's modules, so the compiler now enforces what used to be
//! only a convention: none of these can reach back into the installer's own
//! content, such as `Answers` or the consent script.

pub mod dialogue;
pub mod script;
pub mod ui;

use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Welcome,
    TermsOfService,
    Sign,
    Test,
    Map,
}
