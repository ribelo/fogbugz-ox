use std::fmt;

use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
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
    #[strum(serialize = "ixProject")]
    ProjectId,
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
#[serde(rename_all = "lowercase")]
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

#[derive(Debug, strum::Display)]
pub enum Status {
    Active,
    Resolved,
    Approved,
    Rejected,
    WontReview,
    AbandonedNoConsensus,
}
impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Status, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let status = i32::deserialize(deserializer)?;
        match status {
            1 | 17 | 20 | 23 | 26 | 33 | 36 | 37 | 40 => Ok(Status::Active),
            2..=16 | 18 | 19 | 21 | 22 | 24 | 25 | 31 | 32 | 34 | 35 | 38 | 39 => {
                Ok(Status::Resolved)
            }
            27 => Ok(Status::Approved),
            28 => Ok(Status::Rejected),
            29 => Ok(Status::WontReview),
            30 => Ok(Status::AbandonedNoConsensus),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown status type: {}",
                status
            ))),
        }
    }
}
// //       {
// //         "ixStatus": 26,
// //         "sStatus": "Active",
// //         "ixCategory": 5,
// //         "fWorkDone": false,
// //         "fResolved": false,
// //         "fDuplicate": false,
// //         "fDeleted": false,
// //         "fReactivate": false,
// //         "iOrder": 0
// //       },
// //       {
// //         "ixStatus": 27,
// //         "sStatus": "Approved",
// //         "ixCategory": 5,
// //         "fWorkDone": false,
// //         "fResolved": true,
// //         "fDuplicate": false,
// //         "fDeleted": false,
// //         "fReactivate": false,
// //         "iOrder": 1
// //       },
// //       {
// //         "ixStatus": 28,
// //         "sStatus": "Rejected",
// //         "ixCategory": 5,
// //         "fWorkDone": false,
// //         "fResolved": true,
// //         "fDuplicate": false,
// //         "fDeleted": false,
// //         "fReactivate": false,
// //         "iOrder": 2
// //       },
// //       {
// //         "ixStatus": 29,
// //         "sStatus": "Won't Review",
// //         "ixCategory": 5,
// //         "fWorkDone": false,
// //         "fResolved": true,
// //         "fDuplicate": false,
// //         "fDeleted": true,
// //         "fReactivate": false,
// //         "iOrder": 3
// //       },
// //       {
// //         "ixStatus": 30,
// //         "sStatus": "Abandoned - No Consensus",
// //         "ixCategory": 5,
// //         "fWorkDone": false,
// //         "fResolved": true,
// //         "fDuplicate": false,
// //         "fDeleted": false,
// //         "fReactivate": false,
// //         "iOrder": 3
// //       },
// //       {
// //         "ixStatus": 31,
// //         "sStatus": "Resolved (Postponed)",
// //         "ixCategory": 4,
// //         "fWorkDone": false,
// //         "fResolved": true,
// //         "fDuplicate": false,
// //         "fDeleted": false,
// //         "fReactivate": true,
// //         "iOrder": 0
// //       },
// //       {
// //         "ixStatus": 32,
// //         "sStatus": "Resolved (Postponed)",
// //         "ixCategory": 2,
// //         "fWorkDone": false,
// //         "fResolved": true,
// //         "fDuplicate": false,
// //         "fDeleted": false,
// //         "fReactivate": true,
// //         "iOrder": 0
// //       },
// //       {
// //         "ixStatus": 33,
// //         "sStatus": "Active",
// //         "ixCategory": 6,
// //         "fWorkDone": false,
// //         "fResolved": false,
// //         "fDuplicate": false,
// //         "fDeleted": false,
// //         "fReactivate": false,
// //         "iOrder": 2
// //       },
// //       {
// //         "ixStatus": 34,
// //         "sStatus": "Resolved (Completed)",
// //         "ixCategory": 6,
// //         "fWorkDone": true,
// //         "fResolved": true,
// //         "fDuplicate": false,
// //         "fDeleted": false,
// //         "fReactivate": false,
// //         "iOrder": 0
// //       },
// //       {
// //         "ixStatus": 35,
// //         "sStatus": "Resolved (Duplicate)",
// //         "ixCategory": 6,
// //         "fWorkDone": false,
// //         "fResolved": true,
// //         "fDuplicate": true,
// //         "fDeleted": false,
// //         "fReactivate": false,
// //         "iOrder": 1
// //       },
// //       {
// //         "ixStatus": 36,
// //         "sStatus": "Active new",
// //         "ixCategory": 6,
// //         "fWorkDone": false,
// //         "fResolved": false,
// //         "fDuplicate": false,
// //         "fDeleted": true,
// //         "fReactivate": false,
// //         "iOrder": 1
// //       },
// //       {
// //         "ixStatus": 37,
// //         "sStatus": "Active",
// //         "ixCategory": 7,
// //         "fWorkDone": false,
// //         "fResolved": false,
// //         "fDuplicate": false,
// //         "fDeleted": false,
// //         "fReactivate": false,
// //         "iOrder": 0
// //       },
// //       {
// //         "ixStatus": 38,
// //         "sStatus": "Resolved (Completed)",
// //         "ixCategory": 7,
// //         "fWorkDone": true,
// //         "fResolved": true,
// //         "fDuplicate": false,
// //         "fDeleted": false,
// //         "fReactivate": false,
// //         "iOrder": 0
// //       },
// //       {
// //         "ixStatus": 39,
// //         "sStatus": "Resolved (Duplicate)",
// //         "ixCategory": 7,
// //         "fWorkDone": false,
// //         "fResolved": true,
// //         "fDuplicate": true,
// //         "fDeleted": false,
// //         "fReactivate": false,
// //         "iOrder": 1
// //       },
// //       {
// //         "ixStatus": 40,
// //         "sStatus": "Active (waiting for pricing)",
// //         "ixCategory": 3,
// //         "fWorkDone": false,
// //         "fResolved": false,
// //         "fDuplicate": false,
// //         "fDeleted": false,
// //         "fReactivate": false,
// //         "iOrder": 1
// //       }
