use diesel::sql_types::Text;
use diesel::serialize::{self, ToSql, Output};
use diesel::deserialize::{self, FromSql};
use std::io::Write;
use diesel::backend::Backend;
use std::fmt::Debug;

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


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RatingType {
    ProvenTruth,
    InQuestion,
    NotTrue,
}

impl<DB: Backend> ToSql<Text, DB> for RatingType
where
    String: ToSql<Text, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        let text = match *self {
            RatingType::ProvenTruth => "Proven_Truth",
            RatingType::InQuestion => "In_Question",
            RatingType::NotTrue => "Not_True",
        };
        let text_str = text.to_string();
        <String as ToSql<Text, DB>>::to_sql(&text_str, out)
    }
}

impl<DB: Backend> FromSql<Text, DB> for RatingType
where
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        match <String as FromSql<Text, DB>>::from_sql(bytes)? {
            ref v if v == "Proven_Truth" => Ok(RatingType::ProvenTruth),
            ref v if v == "In_Question" => Ok(RatingType::InQuestion),
            ref v if v == "Not_True" => Ok(RatingType::NotTrue),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

