use chrono::Utc;
use fitness_backend::{
    application::security::{password_service::PasswordService, token_service::TokenService},
    infrastructure::{auth::jwt::JwtTokenService, security::password::Argon2PasswordService},
};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde_json::json;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
struct Claims {
    subject: String,
    exp: usize,
    token_type: String,
}

const SECRET: &str = "unit-only-secret";

#[test]
fn token_service_preserves_identity_format_and_lifetimes() {
    let user = Uuid::new_v4();
    let service = JwtTokenService::new(SECRET);
    for (token, lifetime, token_type) in [
        (service.create_access_token(&user).unwrap(), 3600, "access"),
        (
            service.create_refresh_token(&user).unwrap(),
            604800,
            "refresh",
        ),
    ] {
        let identity = if token_type == "access" {
            service.validate_access_token(&token)
        } else {
            service.validate_refresh_token(&token)
        }
        .unwrap();
        assert_eq!(identity.user_id, user);
        let claims = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(SECRET.as_bytes()),
            &Validation::default(),
        )
        .unwrap()
        .claims;
        assert_eq!(claims.subject, user.to_string());
        assert_eq!(claims.token_type, token_type);
        assert!((claims.exp as i64 - Utc::now().timestamp() - lifetime).abs() <= 2);
    }
}
#[test]
fn invalid_signature_expiration_and_subject_are_rejected() {
    let service = JwtTokenService::new(SECRET);
    let wrong = JwtTokenService::new("another-secret");
    for token in [
        wrong.create_access_token(&Uuid::new_v4()).unwrap(),
        wrong.create_refresh_token(&Uuid::new_v4()).unwrap(),
    ] {
        assert!(service.validate_access_token(&token).is_err());
        assert!(service.validate_refresh_token(&token).is_err());
    }
    for token_type in ["access", "refresh"] {
        for claims in [
            Claims {
                subject: Uuid::new_v4().to_string(),
                exp: (Utc::now().timestamp() - 3600) as usize,
                token_type: token_type.into(),
            },
            Claims {
                subject: "not-a-uuid".into(),
                exp: (Utc::now().timestamp() + 3600) as usize,
                token_type: token_type.into(),
            },
        ] {
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(SECRET.as_bytes()),
            )
            .unwrap();
            assert!(service.validate_access_token(&token).is_err());
            assert!(service.validate_refresh_token(&token).is_err());
        }
    }
    assert!(service.validate_access_token("malformed").is_err());
    assert!(service.validate_refresh_token("malformed").is_err());
}

#[test]
fn access_and_refresh_tokens_cannot_be_interchanged() {
    let service = JwtTokenService::new(SECRET);
    let user = Uuid::new_v4();
    let access = service.create_access_token(&user).unwrap();
    let refresh = service.create_refresh_token(&user).unwrap();
    assert_eq!(
        service.validate_access_token(&access).unwrap().user_id,
        user
    );
    assert_eq!(
        service.validate_refresh_token(&refresh).unwrap().user_id,
        user
    );
    assert!(service.validate_access_token(&refresh).is_err());
    assert!(service.validate_refresh_token(&access).is_err());
}

#[test]
fn missing_unknown_types_and_unexpected_algorithms_are_rejected() {
    let service = JwtTokenService::new(SECRET);
    let valid = json!({
        "subject": Uuid::new_v4().to_string(),
        "exp": Utc::now().timestamp() + 3600,
        "token_type": "access",
    });
    for token_type in [
        None,
        Some(json!("unknown")),
        Some(json!(null)),
        Some(json!(1)),
    ] {
        let mut claims = valid.clone();
        if let Some(token_type) = token_type {
            claims["token_type"] = token_type;
        } else {
            claims.as_object_mut().unwrap().remove("token_type");
        }
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(SECRET.as_bytes()),
        )
        .unwrap();
        assert!(service.validate_access_token(&token).is_err());
        assert!(service.validate_refresh_token(&token).is_err());
    }
    for field in ["exp", "subject"] {
        let mut claims = valid.clone();
        claims.as_object_mut().unwrap().remove(field);
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(SECRET.as_bytes()),
        )
        .unwrap();
        assert!(service.validate_access_token(&token).is_err());
        assert!(service.validate_refresh_token(&token).is_err());
    }
    for token_type in ["access", "refresh"] {
        let mut claims = valid.clone();
        claims["token_type"] = json!(token_type);
        let token = encode(
            &Header::new(Algorithm::HS512),
            &claims,
            &EncodingKey::from_secret(SECRET.as_bytes()),
        )
        .unwrap();
        assert!(service.validate_access_token(&token).is_err());
        assert!(service.validate_refresh_token(&token).is_err());
    }
}
#[tokio::test]
async fn password_service_handles_valid_wrong_and_corrupt_hashes() {
    let passwords = Argon2PasswordService;
    let hash = passwords.hash("correct-password").await.unwrap();
    assert!(hash.starts_with("$argon2id$"));
    assert!(passwords.verify("correct-password", &hash).await.unwrap());
    assert!(!passwords.verify("incorrect-password", &hash).await.unwrap());
    assert!(passwords.verify("password", "corrupt-hash").await.is_err());
}
