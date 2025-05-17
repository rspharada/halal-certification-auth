use regex::Regex;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct SignupValidationErrors {
    pub email: Option<String>,
    pub password: Option<String>,
    pub confirm: Option<String>,
}

impl SignupValidationErrors {
    pub fn is_empty(&self) -> bool {
        self.email.is_none() && self.password.is_none() && self.confirm.is_none()
    }
}

pub fn validate_email(val: &str) -> Option<String> {
    if val.trim().is_empty() {
        Some("メールアドレスは必須です".into())
    } else if !Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$")
        .unwrap()
        .is_match(val)
    {
        Some("有効なメールアドレスを入力してください".into())
    } else {
        None
    }
}

pub fn validate_password(val: &str) -> Option<String> {
    if val.len() < 8 {
        return Some("パスワードは8文字以上で入力してください".into());
    }
    if !Regex::new(r"[0-9]").unwrap().is_match(val) {
        return Some("パスワードには少なくとも1つの数字が必要です".into());
    }
    if !Regex::new(r#"[!@#$%^&*(),.?\":{}|<>]"#)
        .unwrap()
        .is_match(val)
    {
        return Some("パスワードには少なくとも1つの特殊文字が必要です".into());
    }
    if !Regex::new(r"[A-Z]").unwrap().is_match(val) {
        return Some("パスワードには少なくとも1つの大文字が必要です".into());
    }
    if !Regex::new(r"[a-z]").unwrap().is_match(val) {
        return Some("パスワードには少なくとも1つの小文字が必要です".into());
    }
    None
}

pub fn validate_confirm(password: &str, confirm: &str) -> Option<String> {
    (password != confirm).then(|| "パスワードが一致しません".into())
}

pub fn validate_signup(
    email: &str,
    password: &str,
    confirm_password: &str,
) -> Result<(), SignupValidationErrors> {
    let errors = SignupValidationErrors {
        email: validate_email(email),
        password: validate_password(password),
        confirm: validate_confirm(password, confirm_password),
    };

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email_success() {
        let email = "test@example.com".to_string();
        assert_eq!(validate_email(&email), None);
    }

    #[test]
    fn test_validate_email_failure() {
        let email = "invalid".to_string();
        assert_eq!(
            validate_email(&email),
            Some("無効なメールアドレスです".to_string())
        );
    }

    #[test]
    fn test_validate_password_success() {
        let pwd = "Strong1!".to_string();
        assert_eq!(validate_password(&pwd), None);
    }

    #[test]
    fn test_validate_password_failure() {
        let pwd = "short".to_string();
        assert!(validate_password(&pwd).is_some());
    }

    #[test]
    fn test_validate_confirm_match() {
        let password = "Password123!";
        let confirm = "Password123!";
        assert_eq!(validate_confirm(password, confirm), None);
    }

    #[test]
    fn test_validate_confirm_mismatch() {
        let password = "Password123!";
        let confirm = "WrongPassword";
        assert_eq!(
            validate_confirm(password, confirm),
            Some("パスワードが一致しません".to_string())
        );
    }
}
