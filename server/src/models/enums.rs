// use diesel::sql_types::*;
// use diesel::expression::{AsExpression, Expression};
// use diesel::pg::Pg;
// use diesel::serialize::{self, ToSql, Output};
// use diesel::deserialize::{self, FromSql};
// use std::io::Write;
// use diesel::backend::Backend;
// use std::fmt::{Debug, Display, Result, Formatter};
// use crate::models::enums::serialize::IsNull;
// use serde::{Serialize, Deserialize};
// use strum_macros::EnumString;
// // // use diesel::Queryable;
// // // use diesel::types::{self, HasSqlType, IsNull, ToSqlOutput};

// // * ProposalStatus II

// // // #[derive(Debug, Clone, Copy, PartialEq, FromSqlRow, Serialize, Deserialize, EnumString)]
// // // #[sql_type = "ProposalStatusSql"]
// // // pub enum ProposalStatus {
// // //     UnderReview,
// // //     Rejected,
// // //     Active,
// // //     Completed,
// // // }

// // #[derive(Debug, Clone, Copy, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize, EnumString)]
// // #[sql_type = "ProposalStatusSql"]
// // pub enum ProposalStatus {
// //     UnderReview,
// //     Rejected,
// //     Active,
// //     Completed,
// // }

// // #[derive(SqlType)]
// // #[postgres(type_name = "proposal_status")]
// // pub struct ProposalStatusSql;

// // // impl AsExpression<Text> for ProposalStatus {
// // //     type Expression = diesel::expression::bound::Bound<Text, Self>;

// // //     fn as_expression(self) -> Self::Expression {
// // //         diesel::expression::bound::Bound::new(self)
// // //     }
// // // }

// // impl Display for ProposalStatus {
// //     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
// //         match self {
// //             ProposalStatus::UnderReview => write!(f, "Under_Review"),
// //             ProposalStatus::Rejected => write!(f, "Rejected"),
// //             ProposalStatus::Active => write!(f, "Active"),
// //             ProposalStatus::Completed => write!(f, "Completed"),
// //         }
// //     }
// // }

// // impl Default for ProposalStatus {
// //     fn default() -> Self {
// //         ProposalStatus::UnderReview
// //     }
// // }

// // // impl ToSql<Text, Pg> for ProposalStatus {
// // //     fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
// // //         match *self {
// // //             ProposalStatus::UnderReview => out.write_all(b"Under_Review")?,
// // //             ProposalStatus::Rejected => out.write_all(b"Rejected")?,
// // //             ProposalStatus::Active => out.write_all(b"Active")?,
// // //             ProposalStatus::Completed => out.write_all(b"Completed")?,
// // //         }
// // //         Ok(IsNull::No)
// // //     }
// // // }

// impl<DB> ToSql<Text, DB> for ProposalStatus
// where
//     DB: Backend,
//     String: ToSql<Text, DB>,
// {
//     fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
//         let text = match *self {
//             ProposalStatus::UnderReview => "Under_Review",
//             ProposalStatus::Rejected => "Rejected",
//             ProposalStatus::Active => "Active",
//             ProposalStatus::Completed => "Completed",
//         };
//         let text_str = text.to_string();
//         <String as ToSql<Text, DB>>::to_sql(&text_str, out)
//     }
// }
// // // match *self {
// // //     ProposalStatus::UnderReview => "Under_Review".to_sql(out),
// // //     ProposalStatus::Rejected => "Rejected".to_sql(out),
// // //     ProposalStatus::Active => "Active".to_sql(out),
// // //     ProposalStatus::Completed => "Completed".to_sql(out),
// // // }

// impl FromSql<Text, Pg> for ProposalStatus {
//     fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
//         match <String as FromSql<Text, Pg>>::from_sql(bytes)?.as_str() {
//             "Under_Review" => Ok(ProposalStatus::UnderReview),
//             "Rejected" => Ok(ProposalStatus::Rejected),
//             "Active" => Ok(ProposalStatus::Active),
//             "Completed" => Ok(ProposalStatus::Completed),
//             _ => Err("Unrecognized status".into()),
//         }
//     }
// }
// // //     match bytes {
// // //         Some(b"Under_Review") => Ok(ProposalStatus::UnderReview),
// // //         Some(b"Rejected") => Ok(ProposalStatus::Rejected),
// // //         Some(b"Active") => Ok(ProposalStatus::Active),
// // //         Some(b"Completed") => Ok(ProposalStatus::Completed),
// // //         Some(_) => Err("Unrecognized value for ProposalStatus enum".into()),
// // //         None => Err("Unexpected null for non-nullable enum ProposalStatus".into()),
// // //     }

// // * CampaignStatus

// // #[derive(Debug, Clone, Copy, PartialEq)]
// // pub enum CampaignStatus {
// //     UnderReview,
// //     Rejected,
// //     Active,
// //     Completed,
// // }

// impl<DB: Backend> ToSql<Text, DB> for CampaignStatus
// where
//     String: ToSql<Text, DB>,
// {
//     fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
//         let text = match *self {
//             CampaignStatus::UnderReview => "Under_Review",
//             CampaignStatus::Rejected => "Rejected",
//             CampaignStatus::Active => "Active",
//             CampaignStatus::Completed => "Completed",
//         };
//         let text_str = text.to_string();
//         <String as ToSql<Text, DB>>::to_sql(&text_str, out)
//     }
// }

// impl<DB: Backend> FromSql<Text, DB> for CampaignStatus
// where
//     String: FromSql<Text, DB>,
// {
//     fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
//         match <String as FromSql<Text, DB>>::from_sql(bytes)? {
//             ref v if v` == "Under_Review" => Ok(CampaignStatus::UnderReview),
//             ref v if v == "Rejected" => Ok(CampaignStatus::Rejected),
//             ref v if v == "Active" => Ok(CampaignStatus::Active),
//             ref v if v == "Completed" => Ok(CampaignStatus::Completed),
//             _ => Err("Unrecognized enum variant".into()),
//         }
//     }
// }

// // * RatingType

// // #[derive(SqlType, Insertable, AsChangeset)]
// // #[postgres(type_name = "rating_type")]
// // pub struct RatingTypeSql;

// // #[derive(Debug, Clone, Copy, PartialEq, AsExpression, FromSqlRow, Serialize, Deserialize)]
// // #[sql_type = "RatingTypeSql"]
// // pub enum RatingType {
// //     ProvenTruth,
// //     InQuestion,
// //     NotTrue,
// // }

// // impl Display for RatingType {
// //     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
// //         match self {
// //             RatingType::ProvenTruth => write!(f, "Proven_Truth"),
// //             RatingType::InQuestion => write!(f, "In_Question"),
// //             RatingType::NotTrue => write!(f, "Not_True"),
// //         }
// //     }
// // }

// // impl Default for RatingType {
// //     fn default() -> Self {
// //         RatingType::InQuestion
// //     }
// // }

// impl ToSql<Text, Pg> for RatingType {
//     fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
//         match *self {
//             RatingType::ProvenTruth => out.write_all(b"Proven_Truth")?,
//             RatingType::InQuestion => out.write_all(b"In_Question")?,
//             RatingType::NotTrue => out.write_all(b"Not_True")?,
//         }
//         Ok(IsNull::No)
//     }
// }

// impl FromSql<Text, Pg> for RatingType {
//     fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
//         match not_none!(bytes) {
//             b"Proven_Truth" => Ok(RatingType::ProvenTruth),
//             b"In_Question" => Ok(RatingType::InQuestion),
//             b"Not_True" => Ok(RatingType::NotTrue),
//             _ => Err("Unrecognized enum variant".into()),
//         }
//     }
// }
