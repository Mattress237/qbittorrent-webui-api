// pub fn f64_to_string<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     serializer.serialize_str(&value.to_string())
// }

// pub fn u64_to_string<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     serializer.serialize_str(&value.to_string())
// }

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

pub fn vec_to_string_comma_separated<S>(
    value: &Vec<String>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.join(","))
}

pub fn option_vec_to_string_pipe_separated<S>(
    value: &Option<Vec<String>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(v) => serializer.serialize_str(&v.join("|")),
        None => serializer.serialize_none(),
    }
}

pub fn option_vec_to_string_newline_separated<S>(
    value: &Option<Vec<String>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(v) => serializer.serialize_str(&v.join("\n")),
        None => serializer.serialize_none(),
    }
}

pub fn option_vec_to_string_semicolon_separated<S>(
    value: &Option<Vec<String>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(v) => serializer.serialize_str(&v.join(";")),
        None => serializer.serialize_none(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::Serialize;

    mod vec_to_string_newline_separated {
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
    mod vec_to_string_semicolon_separated {
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
        fn from_multiple() {
            let test_data = TestDataVecString {
                value: vec!["String".to_string(), "String2".to_string()],
            };

            let result = serde_json::to_string(&test_data).unwrap();
            assert_eq!(result, "{\"value\":\"String;String2\"}");
        }
    }

    mod vec_to_string_comma_separated {
        use super::*;

        #[derive(Debug, PartialEq, Serialize)]
        struct TestDataVecString {
            #[serde(serialize_with = "vec_to_string_comma_separated")]
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
                value: vec!["String".to_string(), "String2".to_string()],
            };

            let result = serde_json::to_string(&test_data).unwrap();
            assert_eq!(result, "{\"value\":\"String,String2\"}");
        }
    }

    #[cfg(test)]
    mod vec_to_string_pipe_separated {
        use super::*;

        #[derive(Debug, PartialEq, Serialize)]
        struct TestDataVecString {
            #[serde(serialize_with = "option_vec_to_string_pipe_separated")]
            value: Option<Vec<String>>,
        }

        #[test]
        fn from_none() {
            let test_data = TestDataVecString { value: None };

            let result = serde_json::to_string(&test_data).unwrap();
            assert_eq!(result, "{\"value\":null}");
        }

        #[test]
        fn from_empty() {
            let test_data = TestDataVecString {
                value: Some(vec![]),
            };

            let result = serde_json::to_string(&test_data).unwrap();
            assert_eq!(result, "{\"value\":\"\"}");
        }

        #[test]
        fn from_single() {
            let test_data = TestDataVecString {
                value: Some(vec!["String".to_string()]),
            };

            let result = serde_json::to_string(&test_data).unwrap();
            assert_eq!(result, "{\"value\":\"String\"}");
        }

        #[test]
        fn from_multiple() {
            let test_data = TestDataVecString {
                value: Some(vec!["String".to_string(), "String2".to_string()]),
            };

            let result = serde_json::to_string(&test_data).unwrap();
            assert_eq!(result, "{\"value\":\"String|String2\"}");
        }
    }

    mod option_vec_to_string_newline_separated {
        use super::*;

        #[derive(Debug, PartialEq, Serialize)]
        struct TestDataOptionVecString {
            #[serde(serialize_with = "option_vec_to_string_newline_separated")]
            value: Option<Vec<String>>,
        }

        #[test]
        fn from_none() {
            let test_data = TestDataOptionVecString { value: None };

            let result = serde_json::to_string(&test_data).unwrap();
            assert_eq!(result, "{\"value\":null}");
        }

        #[test]
        fn from_empty() {
            let test_data = TestDataOptionVecString {
                value: Some(vec![]),
            };

            let result = serde_json::to_string(&test_data).unwrap();
            assert_eq!(result, "{\"value\":\"\"}");
        }

        #[test]
        fn from_single() {
            let test_data = TestDataOptionVecString {
                value: Some(vec!["String".to_string()]),
            };

            let result = serde_json::to_string(&test_data).unwrap();
            assert_eq!(result, "{\"value\":\"String\"}");
        }

        #[test]
        fn from_multiple() {
            let test_data = TestDataOptionVecString {
                value: Some(vec!["String".to_string(), "String2".to_string()]),
            };

            let result = serde_json::to_string(&test_data).unwrap();
            assert_eq!(result, "{\"value\":\"String\\nString2\"}");
        }
    }

    mod option_vec_to_string_semicolon_separated {
        use super::*;

        #[derive(Debug, PartialEq, Serialize)]
        struct TestDataOptionVecString {
            #[serde(serialize_with = "option_vec_to_string_semicolon_separated")]
            value: Option<Vec<String>>,
        }

        #[test]
        fn from_none() {
            let test_data = TestDataOptionVecString { value: None };

            let result = serde_json::to_string(&test_data).unwrap();
            assert_eq!(result, "{\"value\":null}");
        }

        #[test]
        fn from_empty() {
            let test_data = TestDataOptionVecString {
                value: Some(vec![]),
            };

            let result = serde_json::to_string(&test_data).unwrap();
            assert_eq!(result, "{\"value\":\"\"}");
        }

        #[test]
        fn from_single() {
            let test_data = TestDataOptionVecString {
                value: Some(vec!["String".to_string()]),
            };

            let result = serde_json::to_string(&test_data).unwrap();
            assert_eq!(result, "{\"value\":\"String\"}");
        }

        #[test]
        fn from_multiple() {
            let test_data = TestDataOptionVecString {
                value: Some(vec!["String".to_string(), "String2".to_string()]),
            };

            let result = serde_json::to_string(&test_data).unwrap();
            assert_eq!(result, "{\"value\":\"String;String2\"}");
        }
    }
}
