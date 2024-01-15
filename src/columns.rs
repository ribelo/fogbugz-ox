use serde::Serialize;
use strum::{AsRefStr, Display};

#[derive(Debug, AsRefStr, Display)]
pub enum Column {
    #[strum(serialize = "ixBug")]
    TicketNumber,
    #[strum(serialize = "sTitle")]
    Title,
    #[strum(serialize = "sHtmlBody")]
    Body,
    #[strum(serialize = "events")]
    Events,
}
