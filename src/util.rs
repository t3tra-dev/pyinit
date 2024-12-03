pub fn is_available_name_on_pypi(name: &str) -> crate::Result<bool> {
    let status: reqwest::StatusCode = reqwest::blocking::get(format!("https://pypi.org/pypi/{name}/json"))?.status();
    Ok(status == reqwest::StatusCode::NOT_FOUND)
}

pub fn current_year() -> u16 {
    use chrono::Datelike;

    let now: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
    now.year() as u16
}

#[derive(Clone)]
pub struct AlphaNumeric(String);
const _: () = {
    impl AlphaNumeric {
        pub fn new(mut input: &str) -> crate::Result<Self> {
            input = input.trim();

            if input.is_empty() {
                return Err(crate::Error::ArgValidation("it can't be empty"));
            }

            if input.starts_with('-') || input.ends_with('-') {
                return Err(crate::Error::ArgValidation("the first and last characters can't be hyphens"));
            }

            if !input.chars().all(|c: char| c.is_alphanumeric() || c == '-') {
                return Err(crate::Error::ArgValidation("it can only have alphanumeric characters or hyphens"));
            }

            Ok(AlphaNumeric(input.into()))
        }
    }

    /// to use for field of `derive(Parser)` struct
    impl std::str::FromStr for AlphaNumeric {
        type Err = crate::Error;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            AlphaNumeric::new(s)
        }
    }

    impl std::fmt::Display for AlphaNumeric {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(&self.0)
        }
    }

    impl std::ops::Deref for AlphaNumeric {
        type Target = str;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
};
