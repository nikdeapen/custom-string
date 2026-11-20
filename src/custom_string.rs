/// Declares a string type with custom validation.
///
/// # Syntax
/// ```text
/// custom_string!(
///     #[doc = "The type docs."],
///     TypeName,
///     validate_fn
/// );
/// ```
/// - The attributes are applied to both of the generated types. The comma after the attributes
///   is required. (even when there are no attributes)
/// - The `validate_fn` is any expression that can be called as
///   `fn(&str) -> Result<(), &'static str>`.
///
/// # Generated Items
/// - `TypeName` — The owned type. (wraps a `String`)
/// - `TypeNameRef<'a>` — The `Copy` reference type. (wraps a `&'a str`)
/// - `WithTypeName` — The trait for elements with a `TypeName`. It has a single
///   `fn type_name(&self) -> TypeNameRef<'_>` function & is implemented for both types.
///
/// Both types can only be constructed with valid values. They deref to `str` & implement the
/// standard ordering, hashing, formatting, & conversion traits.
///
/// With the `serde` feature both types implement `Serialize` & the owned type implements
/// `Deserialize`. (deserialization validates the value)
///
/// # Equality
/// Both types can be compared with each other & with `str`, `&str`, & `String`. (in either
/// order) Different custom string types cannot be compared with each other:
/// ```compile_fail
/// use custom_string::custom_string;
///
/// custom_string!(#[doc = "A user id."], UserId, validate_id);
/// custom_string!(#[doc = "An order id."], OrderId, validate_id);
///
/// fn validate_id(_: &str) -> Result<(), &'static str> {
///     Ok(())
/// }
///
/// let user_id: UserId = UserId::new("42").unwrap();
/// let order_id: OrderId = OrderId::new("42").unwrap();
/// assert!(user_id == order_id); // error: can't compare `UserId` with `OrderId`
/// ```
///
/// # Note
/// - The `WithTypeName` function is named with the snake-case type name. If this is a keyword
///   the function is a raw identifier. (ex: `Type` has `fn r#type`) The type names `Crate` &
///   `Super` are not supported.
///
/// # Example
/// ```
/// use custom_string::custom_string;
///
/// custom_string!(
///     #[doc = "A non-empty string."],
///     NonEmpty,
///     validate_non_empty
/// );
///
/// fn validate_non_empty(s: &str) -> Result<(), &'static str> {
///     if s.is_empty() {
///         Err("the string is empty")
///     } else {
///         Ok(())
///     }
/// }
///
/// let owned: NonEmpty = NonEmpty::new("hello").unwrap();
/// let reference: NonEmptyRef = owned.to_ref();
/// assert_eq!(owned, reference);
/// assert_eq!(reference.value(), "hello");
/// assert_eq!(owned.non_empty(), "hello");
///
/// assert!(NonEmptyRef::new("hello").is_ok());
/// assert_eq!(NonEmpty::new("").unwrap_err().message(), "the string is empty");
/// ```
#[macro_export]
macro_rules! custom_string {
    (
        $(#[$meta:meta])*,
        $owned_struct_name:ident,
        $validate_fn:expr $(,)?
    ) => {
        $crate::__paste! {

            $(#[$meta])*
            #[derive(
                ::core::clone::Clone,
                ::core::cmp::Eq,
                ::core::cmp::PartialEq,
                ::core::cmp::Ord,
                ::core::cmp::PartialOrd,
                ::core::hash::Hash,
                ::core::fmt::Debug,
            )]
            pub struct $owned_struct_name {
                value: ::std::string::String,
            }

            $(#[$meta])*
            #[derive(
                ::core::marker::Copy,
                ::core::clone::Clone,
                ::core::cmp::Eq,
                ::core::cmp::PartialEq,
                ::core::cmp::Ord,
                ::core::cmp::PartialOrd,
                ::core::hash::Hash,
                ::core::fmt::Debug,
            )]
            pub struct [<$owned_struct_name Ref>]<'a> {
                value: &'a str,
            }

            $crate::__custom_string_eq_impl!([] $owned_struct_name, str);
            $crate::__custom_string_eq_impl!(['b] $owned_struct_name, &'b str);
            $crate::__custom_string_eq_impl!([] $owned_struct_name, ::std::string::String);
            $crate::__custom_string_eq_impl!(['a] $owned_struct_name, [<$owned_struct_name Ref>]<'a>);
            $crate::__custom_string_eq_impl!(['a] [<$owned_struct_name Ref>]<'a>, str);
            $crate::__custom_string_eq_impl!(['a, 'b] [<$owned_struct_name Ref>]<'a>, &'b str);
            $crate::__custom_string_eq_impl!(['a] [<$owned_struct_name Ref>]<'a>, ::std::string::String);

            impl<'a> ::core::cmp::PartialOrd<[<$owned_struct_name Ref>]<'a>> for $owned_struct_name {
                fn partial_cmp(
                    &self,
                    other: &[<$owned_struct_name Ref>]<'a>,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    ::core::cmp::PartialOrd::partial_cmp(self.value(), other.value())
                }
            }

            impl<'a> ::core::cmp::PartialOrd<$owned_struct_name> for [<$owned_struct_name Ref>]<'a> {
                fn partial_cmp(
                    &self,
                    other: &$owned_struct_name,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    ::core::cmp::PartialOrd::partial_cmp(self.value(), other.value())
                }
            }

            impl $owned_struct_name {
                //! Validation

                /// Validates the `value`.
                ///
                /// Returns `Ok(value)`.
                /// Returns `Err(error)` if the `value` is invalid.
                pub fn validate(value: &str) -> ::core::result::Result<&str, $crate::ValidationError> {
                    match $validate_fn(value) {
                        ::core::result::Result::Ok(()) => ::core::result::Result::Ok(value),
                        ::core::result::Result::Err(e) => {
                            ::core::result::Result::Err($crate::ValidationError::new(e))
                        }
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
                pub fn validate(value: &str) -> ::core::result::Result<&str, $crate::ValidationError> {
                    $owned_struct_name::validate(value)
                }

                /// Checks if the `value` is valid.
                pub fn is_valid(value: &str) -> bool {
                    Self::validate(value).is_ok()
                }
            }

            impl $owned_struct_name {
                //! Construction

                #[doc = concat!("Creates a new [", stringify!($owned_struct_name), "] from the `value`.")]
                #[doc = ""]
                #[doc = "# Safety"]
                #[doc = "The `value` must be valid."]
                pub unsafe fn new_unchecked<S>(value: S) -> Self
                where
                    S: ::core::convert::Into<::std::string::String>,
                {
                    let value: ::std::string::String = ::core::convert::Into::into(value);

                    ::core::debug_assert!(Self::is_valid(value.as_str()));

                    Self { value }
                }

                #[doc = concat!("Creates a new [", stringify!($owned_struct_name), "] from the `value`.")]
                pub fn new<S>(value: S) -> ::core::result::Result<Self, $crate::ValidationError>
                where
                    S: ::core::convert::Into<::std::string::String>,
                {
                    let value: ::std::string::String = ::core::convert::Into::into(value);
                    Self::validate(value.as_str())?;
                    ::core::result::Result::Ok(unsafe { Self::new_unchecked(value) })
                }
            }

            impl<'a> [<$owned_struct_name Ref>]<'a> {
                //! Construction

                #[doc = concat!("Creates a new [", stringify!([<$owned_struct_name Ref>]), "] from the `value`.")]
                #[doc = ""]
                #[doc = "# Safety"]
                #[doc = "The `value` must be valid."]
                pub unsafe fn new_unchecked(value: &'a str) -> Self {
                    ::core::debug_assert!(Self::is_valid(value));

                    Self { value }
                }

                #[doc = concat!("Creates a new [", stringify!([<$owned_struct_name Ref>]), "] from the `value`.")]
                pub fn new(value: &'a str) -> ::core::result::Result<Self, $crate::ValidationError> {
                    ::core::result::Result::Ok(unsafe { Self::new_unchecked(Self::validate(value)?) })
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
                pub fn value(&self) -> &'a str {
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
                #[must_use]
                pub fn to_ref(&self) -> [<$owned_struct_name Ref>]<'_> {
                    unsafe { [<$owned_struct_name Ref>]::new_unchecked(self.value.as_str()) }
                }
            }

            impl<'a> [<$owned_struct_name Ref>]<'a> {
                //! Conversions

                /// Converts the reference type to an owned type.
                #[must_use]
                pub fn into_owned(self) -> $owned_struct_name {
                    unsafe { $owned_struct_name::new_unchecked(self.value) }
                }
            }

            impl<'a> ::core::convert::From<[<$owned_struct_name Ref>]<'a>> for $owned_struct_name {
                fn from(reference: [<$owned_struct_name Ref>]<'a>) -> Self {
                    reference.into_owned()
                }
            }

            impl ::core::convert::From<$owned_struct_name> for ::std::string::String {
                fn from(value: $owned_struct_name) -> Self {
                    value.value
                }
            }

            impl<'a> ::core::convert::From<[<$owned_struct_name Ref>]<'a>> for ::std::string::String {
                fn from(value: [<$owned_struct_name Ref>]<'a>) -> Self {
                    value.into_owned().value
                }
            }

            impl ::core::convert::TryFrom<::std::string::String> for $owned_struct_name {
                type Error = $crate::ValidationError;

                fn try_from(value: ::std::string::String) -> ::core::result::Result<Self, Self::Error> {
                    Self::new(value)
                }
            }

            impl<'a> ::core::convert::TryFrom<&'a str> for $owned_struct_name {
                type Error = $crate::ValidationError;

                fn try_from(value: &'a str) -> ::core::result::Result<Self, Self::Error> {
                    Self::new(value)
                }
            }

            impl<'a> ::core::convert::TryFrom<&'a str> for [<$owned_struct_name Ref>]<'a> {
                type Error = $crate::ValidationError;

                fn try_from(value: &'a str) -> ::core::result::Result<Self, Self::Error> {
                    Self::new(value)
                }
            }

            impl ::core::convert::AsRef<str> for $owned_struct_name {
                fn as_ref(&self) -> &str {
                    self.value.as_str()
                }
            }

            impl<'a> ::core::convert::AsRef<str> for [<$owned_struct_name Ref>]<'a> {
                fn as_ref(&self) -> &str {
                    self.value
                }
            }

            impl ::core::borrow::Borrow<str> for $owned_struct_name {
                fn borrow(&self) -> &str {
                    self.value.as_str()
                }
            }

            impl<'a> ::core::borrow::Borrow<str> for [<$owned_struct_name Ref>]<'a> {
                fn borrow(&self) -> &str {
                    self.value
                }
            }

            impl ::core::ops::Deref for $owned_struct_name {
                type Target = str;

                fn deref(&self) -> &str {
                    self.value.as_str()
                }
            }

            impl<'a> ::core::ops::Deref for [<$owned_struct_name Ref>]<'a> {
                type Target = str;

                fn deref(&self) -> &str {
                    self.value
                }
            }

            impl ::core::fmt::Display for $owned_struct_name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.write_str(self.value())
                }
            }

            impl<'a> ::core::fmt::Display for [<$owned_struct_name Ref>]<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.write_str(self.value)
                }
            }

            impl ::core::str::FromStr for $owned_struct_name {
                type Err = $crate::ValidationError;

                fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                    Self::new(s)
                }
            }

            #[doc = concat!("An element with a [", stringify!($owned_struct_name), "].")]
            pub trait [<With $owned_struct_name>] {
                #[doc = concat!("Gets the [", stringify!($owned_struct_name), "].")]
                fn [<# $owned_struct_name:snake>](&self) -> [<$owned_struct_name Ref>]<'_>;
            }

            impl [<With $owned_struct_name>] for $owned_struct_name {
                fn [<# $owned_struct_name:snake>](&self) -> [<$owned_struct_name Ref>]<'_> {
                    self.to_ref()
                }
            }

            impl<'a> [<With $owned_struct_name>] for [<$owned_struct_name Ref>]<'a> {
                fn [<# $owned_struct_name:snake>](&self) -> [<$owned_struct_name Ref>]<'_> {
                    *self
                }
            }

            $crate::__custom_string_serde_impl!($owned_struct_name, [<$owned_struct_name Ref>]);
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __custom_string_eq_impl {
    ([$($generics:tt)*] $custom_type:ty, $other_type:ty) => {
        impl<$($generics)*> ::core::cmp::PartialEq<$other_type> for $custom_type {
            fn eq(&self, other: &$other_type) -> bool {
                self.value() == <$other_type as ::core::convert::AsRef<str>>::as_ref(other)
            }
        }

        impl<$($generics)*> ::core::cmp::PartialEq<$custom_type> for $other_type {
            fn eq(&self, other: &$custom_type) -> bool {
                <$other_type as ::core::convert::AsRef<str>>::as_ref(self) == other.value()
            }
        }
    };
}

#[cfg(feature = "serde")]
#[doc(hidden)]
#[macro_export]
macro_rules! __custom_string_serde_impl {
    ($owned_struct_name:ident, $ref_struct_name:ident) => {
        impl $crate::__serde::Serialize for $owned_struct_name {
            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: $crate::__serde::Serializer,
            {
                $crate::__serde::Serialize::serialize(&self.value, serializer)
            }
        }

        impl<'a> $crate::__serde::Serialize for $ref_struct_name<'a> {
            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: $crate::__serde::Serializer,
            {
                $crate::__serde::Serialize::serialize(&self.value, serializer)
            }
        }

        impl<'de> $crate::__serde::Deserialize<'de> for $owned_struct_name {
            fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
            where
                D: $crate::__serde::Deserializer<'de>,
            {
                let value: ::std::string::String =
                    <::std::string::String as $crate::__serde::Deserialize>::deserialize(
                        deserializer,
                    )?;
                Self::new(value).map_err(<D::Error as $crate::__serde::de::Error>::custom)
            }
        }
    };
}

#[cfg(not(feature = "serde"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __custom_string_serde_impl {
    ($owned_struct_name:ident, $ref_struct_name:ident) => {};
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
    fn construction_validates_converted_value() {
        /// A value where `AsRef<str>` & `Into<String>` disagree.
        struct Inconsistent;

        impl AsRef<str> for Inconsistent {
            fn as_ref(&self) -> &str {
                "abc"
            }
        }

        impl From<Inconsistent> for String {
            fn from(_: Inconsistent) -> Self {
                String::from("ABC")
            }
        }

        let error: crate::ValidationError = Lower::new(Inconsistent).unwrap_err();
        assert_eq!(error.message(), "not lowercase");
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
    fn value_ref_lifetime() {
        let owned: Lower = Lower::new("abc").unwrap();
        let value: &str = {
            let reference: LowerRef = owned.to_ref();
            reference.value()
        };
        assert_eq!(value, "abc");
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
    fn equality_strings() {
        let owned: Lower = Lower::new("abc").unwrap();
        let reference: LowerRef = owned.to_ref();
        let str_slice: &str = "abc";
        let string: String = String::from("abc");

        assert_eq!(owned, *str_slice);
        assert_eq!(owned, str_slice);
        assert_eq!(owned, string);
        assert_eq!(*str_slice, owned);
        assert_eq!(str_slice, owned);
        assert_eq!(string, owned);

        assert_eq!(reference, *str_slice);
        assert_eq!(reference, str_slice);
        assert_eq!(reference, string);
        assert_eq!(*str_slice, reference);
        assert_eq!(str_slice, reference);
        assert_eq!(string, reference);

        assert_ne!(owned, "xyz");
        assert_ne!("xyz", owned);
        assert_ne!(reference, "xyz");
        assert_ne!("xyz", reference);
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

        let back_to_owned: Lower = reference.into_owned();
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

    /// The macro must not depend on the names in scope at the call site.
    mod hygiene {
        struct Shadow;

        type Result<T> = std::result::Result<T, Shadow>;
        type Option = Shadow;
        type String = Shadow;

        trait AsRef {}
        trait Into {}
        trait From {}
        trait TryFrom {}
        trait PartialEq {}
        trait PartialOrd {}
        trait ToString {}
        trait ToOwned {}

        custom_string!(
            #[doc = "A non-empty string."],
            NonEmpty,
            validate_non_empty,
        );

        fn validate_non_empty(s: &str) -> std::result::Result<(), &'static str> {
            if s.is_empty() { Err("empty") } else { Ok(()) }
        }

        #[test]
        fn shadowed_names() {
            let owned: NonEmpty = NonEmpty::new("abc").unwrap();
            let reference: NonEmptyRef = owned.to_ref();
            assert_eq!(owned, reference);
            assert_eq!(reference.into_owned(), "abc");
            assert_eq!(std::string::String::from(reference), "abc");
            assert!(owned <= reference);
            assert!(NonEmpty::new("").is_err());
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
