use serde::{Deserialize, Deserializer};

/// Deserializes a field from a JSON value that might be `null`.
///
/// This function is intended to be used with the `#[serde(deserialize_with = "")]`
/// attribute. It deserializes a value of a generic type `T`. If the JSON value
/// is `null`, it returns the default value for `T`. Otherwise, it attempts to
/// deserialize the value normally.
///
/// # Type Parameters
///
/// * `T`: The target type to deserialize to. This type must implement
///   `serde::Deserialize` and `std::default::Default`.
pub fn from_null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    let value = Option::<T>::deserialize(deserializer)?;

    Ok(value.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_json::{self, json};

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestData<T>
    where
        T: for<'a> Deserialize<'a> + PartialEq + Default,
    {
        #[serde(deserialize_with = "from_null_to_default")]
        value: T,
    }

    #[derive(Deserialize, Debug, PartialEq, Default)]
    enum TestEnum {
        One,
        #[default]
        Two,
        Tree,
    }

    #[test]
    fn test_i64_from_null() {
        let data = json!({ "value": null });
        let result: TestData<i64> = serde_json::from_value(data).unwrap();
        assert_eq!(result.value, 0);
    }

    #[test]
    fn test_i64_from_number() {
        let data = json!({ "value": 123 });
        let result: TestData<i64> = serde_json::from_value(data).unwrap();
        assert_eq!(result.value, 123);
    }

    #[test]
    fn test_f64_from_null() {
        let data = json!({ "value": null });
        let result: TestData<f64> = serde_json::from_value(data).unwrap();
        assert_eq!(result.value, 0.0);
    }

    #[test]
    fn test_f64_from_number() {
        let data = json!({ "value": 45.67 });
        let result: TestData<f64> = serde_json::from_value(data).unwrap();
        assert_eq!(result.value, 45.67);
    }

    #[test]
    fn test_bool_from_null() {
        let data = json!({ "value": null });
        let result: TestData<bool> = serde_json::from_value(data).unwrap();
        assert!(!result.value);
    }

    #[test]
    fn test_string_from_string() {
        let data = json!({ "value": "String" });
        let result: TestData<String> = serde_json::from_value(data).unwrap();
        assert_eq!(result.value, "String");
    }

    #[test]
    fn test_string_from_null() {
        let data = json!({ "value": null });
        let result: TestData<String> = serde_json::from_value(data).unwrap();
        assert_eq!(result.value, "");
    }

    #[test]
    fn test_enum_from_value() {
        let data = json!({ "value": "One" });
        let result: TestData<TestEnum> = serde_json::from_value(data).unwrap();
        assert_eq!(result.value, TestEnum::One);
    }

    #[test]
    fn test_enum_from_null() {
        let data = json!({ "value": null });
        let result: TestData<TestEnum> = serde_json::from_value(data).unwrap();
        assert_eq!(result.value, TestEnum::Two);
    }
}
