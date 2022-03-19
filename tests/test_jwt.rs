#[macro_use]
#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use std::env;
    use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
    use chrono::Utc;
    use serde::{Deserialize, Serialize};

    const BEARER: &str = "Bearer ";

    #[derive(Debug, Clone, Deserialize, Serialize)]
    struct Claims {
        sub: String,
        role: String,
        exp: usize,
    }

    #[test]
    fn test_create_jwt() {
        dotenv().ok();
        let jwt_secret = env::var("JWT_SECRET").unwrap_or_default();

        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::seconds(3600))
            .expect("invalid timestamp")
            .timestamp();

        let claims = Claims {
            sub: "100000".to_owned(),
            role: "Admin".to_string(),
            exp: expiration as usize,
        };
        let header = Header::new(Algorithm::HS512);
        let jwt_str = encode(&header, &claims, &EncodingKey::from_secret(jwt_secret.as_bytes()))
            .map(|jwt_str| BEARER.to_string() + &jwt_str).unwrap();
        println!("jwt str: {}", jwt_str);
    }

    #[test]
    fn test_verify_jwt() {
        dotenv().ok();
        let jwt_secret = env::var("JWT_SECRET").unwrap_or_default();
        let jwt_str = String::from("Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxMDAwMDAiLCJyb2xlIjoiQWRtaW4iLCJleHAiOjE2NDc2OTk5MTB9.LLusFB2-rG1ht9i-F__83yPa9KOCY7lhGy2LvDbJodiDSxSUiGlT-9NGkPhtBtlOXwnsb2gph5xrOCnN9NVa1g");
        let jwt_str = jwt_str.trim_start_matches(BEARER).to_owned();

        let decoded = decode::<Claims>(jwt_str.as_ref(),
                                       &DecodingKey::from_secret(jwt_secret.as_bytes()),
                                       &Validation::new(Algorithm::HS512)).unwrap();
        println!("claims: {:?}", decoded.claims);
    }
}
