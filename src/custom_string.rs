#[macro_export]
macro_rules! custom_string {
    (
        $(#[$meta:meta])*,
        $owned_struct_name:ident,
        $validate_fn:expr
    ) => {
        paste::paste! {

        $(#[$meta])*
        #[derive(Clone, Ord, PartialOrd, Eq, Hash, Debug)]
        pub struct $owned_struct_name {
            value: String,
        }

        $(#[$meta])*
        #[derive(Copy, Clone, Ord, PartialOrd, Eq, Hash, Debug)]
        pub struct [<$owned_struct_name Ref>]<'a> {
            value: &'a str,
        }

        impl<S: AsRef<str>> PartialEq<S> for $owned_struct_name {
            fn eq(&self, other: &S) -> bool {
                self.value() == other.as_ref()
            }
        }

        impl<'a, S: AsRef<str>> PartialEq<S> for [<$owned_struct_name Ref>]<'a> {
            fn eq(&self, other: &S) -> bool {
                self.value() == other.as_ref()
            }
        }

        impl $owned_struct_name {
            //! Validation

            /// Validates the `value`.
            ///
            /// Returns `Ok(value)`.
            /// Returns `Err(error)` if the `value` is invalid.
            pub fn validate(value: &str) -> Result<&str, $crate::ValidationError> {
                match $validate_fn(value) {
                    Ok(()) => Ok(value),
                    Err(e) => Err($crate::ValidationError::new(e)),
                }
            }

            /// Checks if the `value` is valid.
            pub fn is_valid(value: &str) -> bool {
                Self::validate(value).is_ok()
            }
        }

        impl<'a> [<$owned_struct_name Ref>]<'a> {
            //! Validation

            /// Validates the `value`.
            ///
            /// Returns `Ok(value)`.
            /// Returns `Err(error)` if the `value` is invalid.
            pub fn validate(value: &str) -> Result<&str, $crate::ValidationError> {
                $owned_struct_name::validate(value)
            }

            /// Checks if the `value` is valid.
            pub fn is_valid(value: &str) -> bool {
                Self::validate(value).is_ok()
            }
        }

        impl $owned_struct_name {
            //! Construction

            #[doc = concat!("Creates a new `", stringify!($owned_struct_name), "` from the `value`.")]
            #[doc = ""]
            #[doc = "# Safety"]
            #[doc = "The `value` must be valid."]
            pub unsafe fn new_unchecked<S>(value: S) -> Self
            where
                S: Into<String>,
            {
                let value: String = value.into();

                debug_assert!(Self::is_valid(value.as_str()));

                Self { value }
            }

            #[doc = concat!("Creates a new `", stringify!($owned_struct_name), "` from the `value`.")]
            pub fn new<S>(value: S) -> Result<Self, $crate::ValidationError>
            where
                S: AsRef<str> + Into<String>,
            {
                Self::validate(value.as_ref())?;
                Ok(unsafe { Self::new_unchecked(value) })
            }
        }

        impl<'a> [<$owned_struct_name Ref>]<'a> {
            //! Construction

            #[doc = concat!("Creates a new `", stringify!([<$owned_struct_name Ref>]), "` from the `value`.")]
            #[doc = ""]
            #[doc = "# Safety"]
            #[doc = "The `value` must be valid."]
            pub unsafe fn new_unchecked(value: &'a str) -> Self {
                debug_assert!(Self::is_valid(value));

                Self { value }
            }

            #[doc = concat!("Creates a new `", stringify!([<$owned_struct_name Ref>]), "` from the `value`.")]
            pub fn new(value: &'a str) -> Result<Self, $crate::ValidationError> {
                Ok(unsafe { Self::new_unchecked(Self::validate(value)?) })
            }
        }

        impl $owned_struct_name {
            //! Properties

            /// Gets the value.
            pub fn value(&self) -> &str {
                self.value.as_str()
            }

            /// Gets the length of the value. (in bytes)
            pub fn len(&self) -> usize {
                self.value.len()
            }

            /// Checks if the value is empty.
            pub fn is_empty(&self) -> bool {
                self.value.is_empty()
            }
        }

        impl<'a> [<$owned_struct_name Ref>]<'a> {
            //! Properties

            /// Gets the value.
            pub fn value(&self) -> &str {
                self.value
            }

            /// Gets the length of the value. (in bytes)
            pub fn len(&self) -> usize {
                self.value.len()
            }

            /// Checks if the value is empty.
            pub fn is_empty(&self) -> bool {
                self.value.is_empty()
            }
        }

        impl $owned_struct_name {
            //! Conversions

            /// Converts the owned type to a reference type.
            pub fn to_ref(&self) -> [<$owned_struct_name Ref>]<'_> {
                unsafe { [<$owned_struct_name Ref>]::new_unchecked(self.value.as_str()) }
            }
        }

        impl<'a> [<$owned_struct_name Ref>]<'a> {
            //! Conversions

            /// Converts the reference type to an owned type.
            pub fn to_owned(self) -> $owned_struct_name {
                unsafe { $owned_struct_name::new_unchecked(self.value.to_string()) }
            }
        }

        impl<'a> From<[<$owned_struct_name Ref>]<'a>> for $owned_struct_name {
            fn from(reference: [<$owned_struct_name Ref>]<'a>) -> Self {
                reference.to_owned()
            }
        }

        impl From<$owned_struct_name> for String {
            fn from(value: $owned_struct_name) -> Self {
                value.value
            }
        }

        impl<'a> From<[<$owned_struct_name Ref>]<'a>> for String {
            fn from(value: [<$owned_struct_name Ref>]<'a>) -> Self {
                value.to_string()
            }
        }

        impl TryFrom<String> for $owned_struct_name {
            type Error = $crate::ValidationError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl<'a> TryFrom<&'a str> for $owned_struct_name {
            type Error = $crate::ValidationError;

            fn try_from(value: &'a str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl<'a> TryFrom<&'a str> for [<$owned_struct_name Ref>]<'a> {
            type Error = $crate::ValidationError;

            fn try_from(value: &'a str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl AsRef<str> for $owned_struct_name {
            fn as_ref(&self) -> &str {
                self.value.as_str()
            }
        }

        impl<'a> AsRef<str> for [<$owned_struct_name Ref>]<'a> {
            fn as_ref(&self) -> &str {
                self.value
            }
        }

        impl std::borrow::Borrow<str> for $owned_struct_name {
            fn borrow(&self) -> &str {
                self.value.as_str()
            }
        }

        impl<'a> std::borrow::Borrow<str> for [<$owned_struct_name Ref>]<'a> {
            fn borrow(&self) -> &str {
                self.value
            }
        }

        impl std::ops::Deref for $owned_struct_name {
            type Target = str;

            fn deref(&self) -> &str {
                self.value.as_str()
            }
        }

        impl<'a> std::ops::Deref for [<$owned_struct_name Ref>]<'a> {
            type Target = str;

            fn deref(&self) -> &str {
                self.value
            }
        }

        impl std::fmt::Display for $owned_struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        impl<'a> std::fmt::Display for [<$owned_struct_name Ref>]<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        impl std::str::FromStr for $owned_struct_name {
            type Err = $crate::ValidationError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::new(s)
            }
        }

        impl<'a> PartialOrd<[<$owned_struct_name Ref>]<'a>> for $owned_struct_name {
            fn partial_cmp(&self, other: &[<$owned_struct_name Ref>]<'a>) -> Option<std::cmp::Ordering> {
                self.value().partial_cmp(other.value())
            }
        }

        impl<'a> PartialOrd<$owned_struct_name> for [<$owned_struct_name Ref>]<'a> {
            fn partial_cmp(&self, other: &$owned_struct_name) -> Option<std::cmp::Ordering> {
                self.value().partial_cmp(other.value())
            }
        }

        #[cfg(feature = "serde")]
        impl serde::Serialize for $owned_struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.value.serialize(serializer)
            }
        }

        #[cfg(feature = "serde")]
        impl<'a> serde::Serialize for [<$owned_struct_name Ref>]<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.value.serialize(serializer)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for $owned_struct_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value: String = String::deserialize(deserializer)?;
                Self::new(value).map_err(serde::de::Error::custom)
            }
        }

        }
    };
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use std::collections::HashMap;

    custom_string!(
        #[doc = "A lowercase string."],
        Lower,
        |s: &str| if !s.as_bytes().iter().all(|c| c.is_ascii_lowercase()) {
            Err("not lowercase")
        } else {
            Ok(())
        }
    );

    #[test]
    fn validation() {
        assert!(Lower::is_valid("abc"));
        assert!(!Lower::is_valid("ABC"));
        assert!(!Lower::is_valid("aBc"));

        let result: Result<&str, _> = Lower::validate("abc");
        assert!(result.is_ok());

        let error: crate::ValidationError = Lower::validate("ABC").unwrap_err();
        assert_eq!(error.message(), "not lowercase");
        assert_eq!(error.to_string(), "not lowercase");
    }

    #[test]
    fn validation_ref() {
        assert!(LowerRef::is_valid("abc"));
        assert!(!LowerRef::is_valid("ABC"));

        let error: crate::ValidationError = LowerRef::validate("ABC").unwrap_err();
        assert_eq!(error.message(), "not lowercase");
    }

    #[test]
    fn construction() {
        let owned: Lower = Lower::new("abc").unwrap();
        assert_eq!(owned.value(), "abc");

        let error: crate::ValidationError = Lower::new("ABC").unwrap_err();
        assert_eq!(error.message(), "not lowercase");

        let owned_from_string: Lower = Lower::new(String::from("abc")).unwrap();
        assert_eq!(owned_from_string.value(), "abc");
    }

    #[test]
    fn construction_ref() {
        let reference: LowerRef = LowerRef::new("abc").unwrap();
        assert_eq!(reference.value(), "abc");

        let error: crate::ValidationError = LowerRef::new("ABC").unwrap_err();
        assert_eq!(error.message(), "not lowercase");
    }

    #[test]
    fn properties() {
        let owned: Lower = Lower::new("abc").unwrap();
        assert_eq!(owned.value(), "abc");
        assert_eq!(owned.len(), 3);
        assert!(!owned.is_empty());

        let reference: LowerRef = LowerRef::new("abc").unwrap();
        assert_eq!(reference.value(), "abc");
        assert_eq!(reference.len(), 3);
        assert!(!reference.is_empty());
    }

    #[test]
    fn equality() {
        let one: Lower = Lower::new("one").unwrap();
        let two: Lower = Lower::new("two").unwrap();

        assert_eq!(one, "one");
        assert_eq!(one, one);
        assert_ne!(one, "two");
        assert_ne!(one, two);

        let one_ref: LowerRef = LowerRef::new("one").unwrap();
        let two_ref: LowerRef = LowerRef::new("two").unwrap();

        assert_eq!(one_ref, "one");
        assert_eq!(one_ref, one_ref);
        assert_ne!(one_ref, "two");
        assert_ne!(one_ref, two_ref);
    }

    #[test]
    fn ordering() {
        let a: Lower = Lower::new("a").unwrap();
        let b: Lower = Lower::new("b").unwrap();
        assert!(a < b);

        let a_ref: LowerRef = LowerRef::new("a").unwrap();
        let b_ref: LowerRef = LowerRef::new("b").unwrap();
        assert!(a_ref < b_ref);
    }

    #[test]
    fn display() {
        let owned: Lower = Lower::new("abc").unwrap();
        assert_eq!(format!("{}", owned), "abc");

        let reference: LowerRef = LowerRef::new("abc").unwrap();
        assert_eq!(format!("{}", reference), "abc");
    }

    #[test]
    fn deref() {
        let owned: Lower = Lower::new("abc").unwrap();
        let s: &str = &owned;
        assert_eq!(s, "abc");
        assert!(owned.starts_with("ab"));

        let reference: LowerRef = LowerRef::new("abc").unwrap();
        let s: &str = &reference;
        assert_eq!(s, "abc");
        assert!(reference.starts_with("ab"));
    }

    #[test]
    fn conversions_owned_ref() {
        let owned: Lower = Lower::new("abc").unwrap();
        let reference: LowerRef = owned.to_ref();
        assert_eq!(reference.value(), "abc");

        let back_to_owned: Lower = reference.to_owned();
        assert_eq!(back_to_owned.value(), "abc");

        let from_ref: Lower = Lower::from(reference);
        assert_eq!(from_ref.value(), "abc");
    }

    #[test]
    fn conversions_to_string() {
        let owned: Lower = Lower::new("abc").unwrap();
        let s: String = String::from(owned);
        assert_eq!(s, "abc");

        let reference: LowerRef = LowerRef::new("abc").unwrap();
        let s: String = String::from(reference);
        assert_eq!(s, "abc");
    }

    #[test]
    fn try_from() {
        let owned: Lower = Lower::try_from("abc").unwrap();
        assert_eq!(owned.value(), "abc");

        let owned_from_string: Lower = Lower::try_from(String::from("abc")).unwrap();
        assert_eq!(owned_from_string.value(), "abc");

        let reference: LowerRef = LowerRef::try_from("abc").unwrap();
        assert_eq!(reference.value(), "abc");

        let error: crate::ValidationError = Lower::try_from("ABC").unwrap_err();
        assert_eq!(error.message(), "not lowercase");
    }

    #[test]
    fn as_ref_and_borrow() {
        use std::borrow::Borrow;

        let owned: Lower = Lower::new("abc").unwrap();
        let s: &str = owned.as_ref();
        assert_eq!(s, "abc");
        let s: &str = owned.borrow();
        assert_eq!(s, "abc");

        let reference: LowerRef = LowerRef::new("abc").unwrap();
        let s: &str = reference.as_ref();
        assert_eq!(s, "abc");
        let s: &str = reference.borrow();
        assert_eq!(s, "abc");
    }

    #[test]
    fn hash_map_lookup() {
        let mut map: HashMap<Lower, i32> = HashMap::new();
        let key: Lower = Lower::new("abc").unwrap();
        map.insert(key, 42);

        assert_eq!(map.get("abc"), Some(&42));
        assert_eq!(map.get("xyz"), None);
    }

    #[test]
    fn clone() {
        let owned: Lower = Lower::new("abc").unwrap();
        let cloned: Lower = owned.clone();
        assert_eq!(owned, cloned);

        let reference: LowerRef = LowerRef::new("abc").unwrap();
        let cloned: LowerRef = reference.clone();
        assert_eq!(reference, cloned);

        let copied: LowerRef = reference;
        assert_eq!(reference, copied);
    }

    #[test]
    fn cross_type_equality() {
        let owned: Lower = Lower::new("abc").unwrap();
        let reference: LowerRef = LowerRef::new("abc").unwrap();

        assert_eq!(owned, reference);
        assert_eq!(reference, owned);

        let other_ref: LowerRef = LowerRef::new("xyz").unwrap();
        assert_ne!(owned, other_ref);
        assert_ne!(other_ref, owned);
    }

    #[test]
    fn cross_type_ordering() {
        let owned_a: Lower = Lower::new("a").unwrap();
        let ref_b: LowerRef = LowerRef::new("b").unwrap();

        assert!(owned_a < ref_b);
        assert!(ref_b > owned_a);

        let owned_b: Lower = Lower::new("b").unwrap();
        let ref_a: LowerRef = LowerRef::new("a").unwrap();

        assert!(owned_b > ref_a);
        assert!(ref_a < owned_b);
    }

    #[test]
    fn from_str() {
        let owned: Lower = "abc".parse().unwrap();
        assert_eq!(owned.value(), "abc");

        let error: crate::ValidationError = "ABC".parse::<Lower>().unwrap_err();
        assert_eq!(error.message(), "not lowercase");
    }

    #[cfg(feature = "serde")]
    mod serde_tests {
        use super::*;

        #[test]
        fn serialize_owned() {
            let owned: Lower = Lower::new("abc").unwrap();
            let json: String = serde_json::to_string(&owned).unwrap();
            assert_eq!(json, "\"abc\"");
        }

        #[test]
        fn serialize_ref() {
            let reference: LowerRef = LowerRef::new("abc").unwrap();
            let json: String = serde_json::to_string(&reference).unwrap();
            assert_eq!(json, "\"abc\"");
        }

        #[test]
        fn deserialize_valid() {
            let owned: Lower = serde_json::from_str("\"abc\"").unwrap();
            assert_eq!(owned.value(), "abc");
        }

        #[test]
        fn deserialize_invalid() {
            let result: Result<Lower, _> = serde_json::from_str("\"ABC\"");
            assert!(result.is_err());
        }

        #[test]
        fn round_trip() {
            let original: Lower = Lower::new("abc").unwrap();
            let json: String = serde_json::to_string(&original).unwrap();
            let deserialized: Lower = serde_json::from_str(&json).unwrap();
            assert_eq!(original, deserialized);
        }
    }

    #[test]
    fn validation_error() {
        let error: crate::ValidationError = crate::ValidationError::new("test error");
        assert_eq!(error.message(), "test error");
        assert_eq!(error.to_string(), "test error");
        assert_eq!(error, crate::ValidationError::new("test error"));
        assert_ne!(error, crate::ValidationError::new("other error"));
    }
}
