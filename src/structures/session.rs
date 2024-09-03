use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{config::SECRET, prelude::*};

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Session {
    pub id: String,
    pub exp: u64,
}

impl Session {
    pub fn new(user_id: String) -> Self {
        Self {
            id: user_id,
            exp: 10000000000, // TODO
        }
    }
    pub fn gen_token(&self) -> Result<String, Error> {
        let token = encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret((*SECRET).as_ref()),
        )?;
        Ok(token)
    }
    pub fn decode(token: String) -> Result<Session, Error> {
        let token = decode::<Session>(
            &token,
            &DecodingKey::from_secret((*SECRET).as_ref()),
            &Validation::default(),
        )?;

        Ok(token.claims)
    }
}
