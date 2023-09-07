use serde::Deserialize;

// use crate::error::Error;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum InternalResponseUnion<T> {
    Error { message: String, code: Option<u32> },
    Result(T),
    SystemError(Vec<String>),
}

// TODO: Used for REST API
// impl<T> InternalResponseUnion<T> {
//     pub fn simplify_result(self) -> Result<T, Error> {
//         match self {
//             InternalResponseUnion::Result(result) => Ok(result),
//             InternalResponseUnion::Error { message, code } => {
//                 Err(Error::ShikimoriError { message, code })?
//             }
//             InternalResponseUnion::SystemError(mut errors) => Err(Error::ShikimoriError {
//                 message: errors.remove(0),
//                 code: None,
//             })?,
//         }
//     }
// }
