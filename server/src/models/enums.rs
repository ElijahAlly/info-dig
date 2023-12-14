use diesel::sql_types::Text;
use diesel::pg::Pg;
use diesel::serialize::{self, ToSql, Output};
// use diesel::types::NotNull;
use diesel::deserialize::{self, FromSql};
use std::io::Write;
use diesel::backend::Backend;
use std::fmt::{Debug, Display, Result, Formatter};
// use crate::utils::camel_to_snake_case_capitalized;
use crate::models::enums::serialize::IsNull;
// use log::{info, error};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProposalStatus {
    UnderReview,
    Rejected,
    Active,
    Completed,
}

impl<DB: Backend> ToSql<Text, DB> for ProposalStatus
where
    String: ToSql<Text, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        let text = match *self {
            ProposalStatus::UnderReview => "Under_Review",
            ProposalStatus::Rejected => "Rejected",
            ProposalStatus::Active => "Active",
            ProposalStatus::Completed => "Completed",
        };
        let text_str = text.to_string();
        <String as ToSql<Text, DB>>::to_sql(&text_str, out)
    }
}

impl<DB: Backend> FromSql<Text, DB> for ProposalStatus
where
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        match <String as FromSql<Text, DB>>::from_sql(bytes)? {
            ref v if v == "Under_Review" => Ok(ProposalStatus::UnderReview),
            ref v if v == "Rejected" => Ok(ProposalStatus::Rejected),
            ref v if v == "Active" => Ok(ProposalStatus::Active),
            ref v if v == "Completed" => Ok(ProposalStatus::Completed),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CampaignStatus {
    UnderReview,
    Rejected,
    Active,
    Completed,
}

impl<DB: Backend> ToSql<Text, DB> for CampaignStatus
where
    String: ToSql<Text, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        let text = match *self {
            CampaignStatus::UnderReview => "Under_Review",
            CampaignStatus::Rejected => "Rejected",
            CampaignStatus::Active => "Active",
            CampaignStatus::Completed => "Completed",
        };
        let text_str = text.to_string();
        <String as ToSql<Text, DB>>::to_sql(&text_str, out)
    }
}

impl<DB: Backend> FromSql<Text, DB> for CampaignStatus
where
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        match <String as FromSql<Text, DB>>::from_sql(bytes)? {
            ref v if v == "Under_Review" => Ok(CampaignStatus::UnderReview),
            ref v if v == "Rejected" => Ok(CampaignStatus::Rejected),
            ref v if v == "Active" => Ok(CampaignStatus::Active),
            ref v if v == "Completed" => Ok(CampaignStatus::Completed),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[sql_type = "Text"]
pub enum RatingType {
    ProvenTruth,
    InQuestion,
    NotTrue,
}

impl Display for RatingType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            RatingType::ProvenTruth => write!(f, "Proven_Truth"),
            RatingType::InQuestion => write!(f, "In_Question"),
            RatingType::NotTrue => write!(f, "Not_True"),
        }
    }
}

impl Default for RatingType {
    fn default() -> Self {
        RatingType::InQuestion
    }
}

impl ToSql<Text, Pg> for RatingType {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            RatingType::ProvenTruth => out.write_all(b"Proven_Truth")?,
            RatingType::InQuestion => out.write_all(b"In_Question")?,
            RatingType::NotTrue => out.write_all(b"Not_True")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for RatingType {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"Proven_Truth" => Ok(RatingType::ProvenTruth),
            b"In_Question" => Ok(RatingType::InQuestion),
            b"Not_True" => Ok(RatingType::NotTrue),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

// impl ToSql<RatingType, Pg> for RatingType {
//     fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
//         info!("Converting RatingType");
//         println!("Converting RatingType to SQL: {:?}", self);
//         let text = match *self {
//             RatingType::ProvenTruth => "Proven_Truth",
//             RatingType::InQuestion => "In_Question",
//             RatingType::NotTrue => "Not_True",
//         };
//         println!("Converting RatingType to SQL: {}", text);
//         out.write_all(text.as_bytes())?;
//         Ok(IsNull::No)
//     }
// }

// impl FromSql<RatingType, Pg> for RatingType {
//     fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
//         match not_none!(bytes) {
//             b"Proven_Truth" => Ok(RatingType::ProvenTruth),
//             b"In_Question" => Ok(RatingType::InQuestion),
//             b"Not_True" => Ok(RatingType::NotTrue),
//             _ => Err("Unrecognized enum variant".into()),
//         }
//     }
// }

// impl RatingType {
//     fn to_snake_case(&self) -> String {
//         camel_to_snake_case_capitalized(&format!("{:?}", self))
//     }
// }

// impl<DB: Backend> ToSql<Text, DB> for RatingType
// where
//     String: ToSql<Text, DB>,
// {
//     fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
//         let text = self.to_snake_case();
//         <String as ToSql<Text, DB>>::to_sql(&text, out)
//     }
// }

// impl<DB: Backend> FromSql<Text, DB> for RatingType
// where
//     String: FromSql<Text, DB>,
// {
//     fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
//         let string = <String as FromSql<Text, DB>>::from_sql(bytes)?;
//         match string.as_str() {
//             "Proven_Truth" => Ok(RatingType::ProvenTruth),
//             "In_Question" => Ok(RatingType::InQuestion),
//             "Not_True" => Ok(RatingType::NotTrue),
//             _ => Err("Unrecognized enum variant".into()),
//         }
//     }
// }
