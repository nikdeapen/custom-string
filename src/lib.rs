#[macro_export]
macro_rules! custom_string {
    ($owned_struct_name:ident, $ref_struct_name:ident, $validate_fn:expr) => {
        #[derive(Clone, Ord, PartialOrd, Eq, Hash, Debug)]
        pub struct $owned_struct_name {
            value: String,
        }

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
            pub fn validate(value: &str) -> Result<&str, &'static str> {
                match $validate_fn(value) {
                    Ok(()) => Ok(value),
                    Err(e) => Err(e),
                }
            }

            pub fn is_valid(value: &str) -> bool {
                Self::validate(value).is_ok()
            }
        }

        impl<'a> $ref_struct_name<'a> {
            pub fn validate(value: &str) -> Result<&str, &'static str> {
                match $validate_fn(value) {
                    Ok(()) => Ok(value),
                    Err(e) => Err(e),
                }
            }

            pub fn is_valid(value: &str) -> bool {
                Self::validate(value).is_ok()
            }
        }

        impl $owned_struct_name {
            pub unsafe fn new_unchecked<S>(value: S) -> Self
            where
                S: Into<String>,
            {
                let value: String = value.into();

                debug_assert!(Self::is_valid(value.as_str()));

                Self { value }
            }

            pub fn new<S>(value: S) -> Result<Self, &'static str>
            where
                S: AsRef<str> + Into<String>,
            {
                Ok(unsafe { Self::new_unchecked(Self::validate(value.as_ref())?) })
            }
        }

        impl<'a> $ref_struct_name<'a> {
            pub unsafe fn new_unchecked(value: &'a str) -> Self {
                debug_assert!(Self::is_valid(value));

                Self { value }
            }

            pub fn new(value: &'a str) -> Result<Self, &'static str> {
                Ok(unsafe { Self::new_unchecked(Self::validate(value)?) })
            }
        }

        impl $owned_struct_name {
            pub fn value(&self) -> &str {
                self.value.as_str()
            }

            pub fn len(&self) -> usize {
                self.value.len()
            }

            pub fn is_empty(&self) -> bool {
                self.value.is_empty()
            }
        }

        impl<'a> $ref_struct_name<'a> {
            pub fn value(&self) -> &'a str {
                self.value
            }

            pub fn len(&self) -> usize {
                self.value.len()
            }

            pub fn is_empty(&self) -> bool {
                self.value.is_empty()
            }
        }

        impl $owned_struct_name {
            pub fn to_ref<'a>(&'a self) -> $ref_struct_name<'a> {
                unsafe { $ref_struct_name::new_unchecked(self.value.as_str()) }
            }
        }

        impl<'a> $ref_struct_name<'a> {
            pub fn to_owned(&self) -> $owned_struct_name {
                unsafe { $owned_struct_name::new_unchecked(self.value.to_string()) }
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

        impl Into<String> for $owned_struct_name {
            fn into(self) -> String {
                self.value
            }
        }

        impl<'a> Into<String> for $ref_struct_name<'a> {
            fn into(self) -> String {
                self.value.into()
            }
        }

        impl<'a> Into<$owned_struct_name> for $ref_struct_name<'a> {
            fn into(self) -> $owned_struct_name {
                self.to_owned()
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
    };
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    custom_string!(Lower, LowerRef, |s: &str| if !s
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
