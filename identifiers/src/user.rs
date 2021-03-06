use uuid::{
    Uuid,
    ParseError
};
use std::{
    fmt::{
        Display,
        Formatter,
        Result as FormatResult
    }
};


#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Default, Hash, Eq, PartialOrd, Ord)]
pub struct UserUuid(pub Uuid);

impl UserUuid {
    pub fn to_query_parameter(self) -> String {
        format!("{}={}", PARAM_NAME, self.0)
    }
    pub fn parse_str(input: &str) -> Result<Self, ParseError> {
        Uuid::parse_str(input).map(UserUuid)
    }
}

impl AsRef<Uuid> for UserUuid {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

const PARAM_NAME: &str = "user_uuid";

impl Display for UserUuid {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for UserUuid {
    fn from(uuid: Uuid) -> UserUuid {
        UserUuid(uuid)
    }
}

#[cfg(feature = "rocket_support")]
mod rocket {
    use super::*;
    use ::rocket::http::RawStr;
    use ::rocket::request::FromParam;
    use crate::uuid_from_param;
    use crate::uuid_from_form;
    use ::rocket::request::{FromForm, FormItems};

    impl<'a> FromParam<'a> for UserUuid {
        type Error = &'a RawStr;

        #[inline]
        fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
            uuid_from_param(param).map(UserUuid)
        }
    }


    impl<'f> FromForm<'f> for UserUuid {
        type Error = ();

        #[inline]
        fn from_form(items: &mut FormItems<'f>, strict: bool) -> Result<Self, ()> {
            uuid_from_form(items, strict, PARAM_NAME)
                .map(UserUuid)
        }
    }
}
