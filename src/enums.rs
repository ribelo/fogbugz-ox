use serde::Serialize;
use serde_repr::Deserialize_repr;
use strum::{AsRefStr, Display};

#[derive(Debug, AsRefStr, Display)]
pub enum Column {
    #[strum(serialize = "ixBug")]
    CaseId,
    #[strum(serialize = "sTitle")]
    Title,
    #[strum(serialize = "sHtmlBody")]
    Body,
    #[strum(serialize = "events")]
    Events,
    #[strum(serialize = "sProject")]
    Project,
    #[strum(serialize = "sArea")]
    Area,
    #[strum(serialize = "ixPriority")]
    Priority,
    #[strum(serialize = "ixStatus")]
    Status,
    #[strum(serialize = "ixCategory")]
    Category,
    #[strum(serialize = "fOpen")]
    IsOpen,
}

#[derive(Debug, Deserialize_repr, strum::Display)]
#[repr(u8)]
pub enum Category {
    Bug = 1,
    Feature = 2,
    Inquiry = 3,
    Schedule = 4,
    Report = 5,
    Emergency = 6,
}

#[derive(Debug, Deserialize_repr, strum::Display)]
#[repr(u8)]
pub enum Priority {
    Blocker = 1,
    MuyImportante = 2,
    ShouldDo = 3,
    FixIfTime = 4,
    OhWell = 5,
    WhoCares = 6,
    DontFix = 7,
}

#[derive(Debug, Deserialize_repr, strum::Display)]
#[repr(u8)]
pub enum Status {
    Active = 1,
    Resolved = 2,
}
