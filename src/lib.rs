#[macro_export]
macro_rules! custom_string {
    (
        $(#[$meta:meta])*,
        $owned_struct_name:ident,
        $ref_struct_name:ident,
        $with_trait_name:ident,
        $field_name:ident,
        $validate_fn:expr
    ) => {

        $(#[$meta])*
        #[derive(Clone, Ord, PartialOrd, Eq, Hash, Debug)]
        pub struct $owned_struct_name {
            value: String,
        }

        $(#[$meta])*
        #[derive(Copy, Clone, Ord, PartialOrd, Eq, Hash, Debug)]
        pub struct $ref_struct_name<'a> {
            value: &'a str,
        }

        impl<S: AsRef<str>> PartialEq<S> for $owned_struct_name {
            fn eq(&self, other: &S) -> bool {
                self.value() == other.as_ref()
            }
        }

        impl<'a, S: AsRef<str>> PartialEq<S> for $ref_struct_name<'a> {
            fn eq(&self, other: &S) -> bool {
                self.value() == other.as_ref()
            }
        }

        impl $owned_struct_name {
            //! Validation

            /// Validates the `value`.
            ///
            /// Returns `Ok(value)`.
            /// Returns `Err(error_message)` if the `value` is invalid.
            pub fn validate(value: &str) -> Result<&str, &'static str> {
                match $validate_fn(value) {
                    Ok(()) => Ok(value),
                    Err(e) => Err(e),
                }
            }

            /// Checks if the `value` is valid.
            pub fn is_valid(value: &str) -> bool {
                Self::validate(value).is_ok()
            }
        }

        impl<'a> $ref_struct_name<'a> {
            //! Validation

            /// Validates the `value`.
            ///
            /// Returns `Ok(value)`.
            /// Returns `Err(error_message)` if the `value` is invalid.
            pub fn validate(value: &str) -> Result<&str, &'static str> {
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
            pub fn new<S>(value: S) -> Result<Self, &'static str>
            where
                S: AsRef<str> + Into<String>,
            {
                Ok(unsafe { Self::new_unchecked(Self::validate(value.as_ref())?) })
            }
        }

        impl<'a> $ref_struct_name<'a> {
            //! Construction

            #[doc = concat!("Creates a new `", stringify!($ref_struct_name), "` from the `value`.")]
            #[doc = ""]
            #[doc = "# Safety"]
            #[doc = "The `value` must be valid."]
            pub unsafe fn new_unchecked(value: &'a str) -> Self {
                debug_assert!(Self::is_valid(value));

                Self { value }
            }

            #[doc = concat!("Creates a new `", stringify!($ref_struct_name), "` from the `value`.")]
            pub fn new(value: &'a str) -> Result<Self, &'static str> {
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

        impl<'a> $ref_struct_name<'a> {
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
            pub fn to_ref<'a>(&'a self) -> $ref_struct_name<'a> {
                unsafe { $ref_struct_name::new_unchecked(self.value.as_str()) }
            }
        }

        impl<'a> $ref_struct_name<'a> {
            //! Conversions

            /// Converts the reference type to an owned type.
            pub fn to_owned(self) -> $owned_struct_name {
                unsafe { $owned_struct_name::new_unchecked(self.value.to_string()) }
            }
        }

        impl<'a> From<$ref_struct_name<'a>> for $owned_struct_name {
            fn from(reference: $ref_struct_name<'a>) -> Self {
                reference.to_owned()
            }
        }

        impl From<$owned_struct_name> for String {
            fn from(value: $owned_struct_name) -> Self {
                value.value
            }
        }

        impl<'a> From<$ref_struct_name<'a>> for String {
            fn from(value: $ref_struct_name<'a>) -> Self {
                value.to_string()
            }
        }

        impl AsRef<str> for $owned_struct_name {
            fn as_ref(&self) -> &str {
                self.value.as_ref()
            }
        }

        impl<'a> AsRef<str> for $ref_struct_name<'a> {
            fn as_ref(&self) -> &str {
                self.value
            }
        }

        impl std::borrow::Borrow<str> for $owned_struct_name {
            fn borrow(&self) -> &str {
                self.value.borrow()
            }
        }

        impl<'a> std::borrow::Borrow<str> for $ref_struct_name<'a> {
            fn borrow(&self) -> &str {
                self.value
            }
        }

        impl std::fmt::Display for $owned_struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        impl<'a> std::fmt::Display for $ref_struct_name<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        #[doc = concat!("An element with a `", stringify!($owned_struct_name), "`.")]
        pub trait $with_trait_name {
            #[doc = concat!("Gets the `", stringify!($field_name), "`.")]
            fn $field_name(&self) -> $ref_struct_name<'_>;
        }

        impl<'a> $with_trait_name for $ref_struct_name<'a> {
            fn $field_name(&self) -> $ref_struct_name<'_> {
                *self
            }
        }

        impl $with_trait_name for $owned_struct_name {
            fn $field_name(&self) -> $ref_struct_name<'_> {
                self.to_ref()
            }
        }
    };
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    custom_string!(#[doc = "A lowercase string."], Lower, LowerRef, WithLower, lower, |s: &str| if !s
        .as_bytes()
        .iter()
        .all(|c| c.is_ascii_lowercase())
    {
        Err("not lowercase")
    } else {
        Ok(())
    });

    #[test]
    fn equals() {
        let one: Lower = Lower::new("one").unwrap();
        let two: Lower = Lower::new("two").unwrap();

        assert_eq!(one, "one");
        assert_eq!(one, one);
        assert_ne!(one, "two");
        assert_ne!(one, two);
    }

    #[test]
    fn validation() {
        assert!(Lower::is_valid("one"));
        assert!(!Lower::is_valid("ONE"));

        assert!(Lower::validate("one").is_ok());
        assert_eq!(Lower::validate("ONE").err().unwrap(), "not lowercase");
    }
}
