use validator::ValidationError;

pub fn validate_gender(gender: &str) -> Result<(), ValidationError> {
    let validated_gender = ["male", "female", "other", "prefer_not_to_say"];

    if validated_gender.contains(&gender.to_lowercase().as_str()) {
        Ok(())
    } else {
        let mut error: ValidationError = ValidationError::new("Invalid_gender");

        error.message = Some("Just male, female, other or prefer_not_to_say".into());

        Err(error)
    }
}
