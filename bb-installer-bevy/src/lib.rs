use bevy::prelude::*;

pub mod cursor;
pub mod network;
pub mod pages;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Welcome,
    TermsOfService,
    SecondaryTos,
    CursorCrowd,
    Sign,
    Test,
    Map,
}
