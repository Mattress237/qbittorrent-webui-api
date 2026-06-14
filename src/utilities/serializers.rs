pub fn f64_to_string<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}

pub fn u64_to_string<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}

pub fn vec_to_string_newline_separated<S>(
    value: &Vec<String>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.join("\n"))
}

pub fn vec_to_string_semicolon_separated<S>(
    value: &Vec<String>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.join(";"))
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::Serialize;

    mod vec_to_string_newline_separated_tests {
        use super::*;

        #[derive(Debug, PartialEq, Serialize)]
        struct TestDataVecString {
            #[serde(serialize_with = "vec_to_string_newline_separated")]
            value: Vec<String>,
        }

        #[test]
        fn from_empty() {
            let test_data = TestDataVecString { value: vec![] };

            let result = serde_json::to_string(&test_data).unwrap();
            assert_eq!(result, "{\"value\":\"\"}");
        }

        #[test]
        fn from_single() {
            let test_data = TestDataVecString {
                value: vec!["String".to_string()],
            };

            let result = serde_json::to_string(&test_data).unwrap();
            assert_eq!(result, "{\"value\":\"String\"}");
        }

        #[test]
        fn from_multiple() {
            let test_data = TestDataVecString {
                value: vec![
                    "String1".to_string(),
                    "String2".to_string(),
                    "String3".to_string(),
                ],
            };

            let result = serde_json::to_string(&test_data).unwrap();
            assert_eq!(result, "{\"value\":\"String1\\nString2\\nString3\"}");
        }
    }

    #[cfg(test)]
    mod vec_to_string_semicolon_separated_tests {
        use super::*;

        #[derive(Debug, PartialEq, Serialize)]
        struct TestDataVecString {
            #[serde(serialize_with = "vec_to_string_semicolon_separated")]
            value: Vec<String>,
        }

        #[test]
        fn from_empty() {
            let test_data = TestDataVecString { value: vec![] };

            let result = serde_json::to_string(&test_data).unwrap();
            assert_eq!(result, "{\"value\":\"\"}");
        }

        #[test]
        fn from_single() {
            let test_data = TestDataVecString {
                value: vec!["String".to_string()],
            };

            let result = serde_json::to_string(&test_data).unwrap();
            assert_eq!(result, "{\"value\":\"String\"}");
        }

        #[test]
        fn from_vec() {
            let test_data = TestDataVecString {
                value: vec!["String".to_string(), "String2".to_string()],
            };

            let result = serde_json::to_string(&test_data).unwrap();
            assert_eq!(result, "{\"value\":\"String;String2\"}");
        }
    }
}
