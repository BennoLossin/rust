//! Field Reflection

/// Type representing a field of a `struct`.
///
/// # Safety
///
/// Given a valid value of type `Self::Base`, there exists a valid value of type `Self::Type` at
/// byte offset `OFFSET`.
#[lang = "UnalignedField"]
#[unstable(feature = "field_projections", issue = "145383")]
pub unsafe trait UnalignedField: Sized {
    /// The type of the base where this field exists in.
    type Base: ?Sized;

    /// The type of the field.
    type Type: ?Sized;

    /// The offset of the field in bytes.
    const OFFSET: usize;
}

/// Type representing an aligned field of a `struct`.
///
/// # Safety
///
/// Given a well-aligned value of type `Self::Base`, the field at `Self::OFFSET` of type
/// `Self::Type` is well-aligned.
#[lang = "Field"]
#[unstable(feature = "field_projections", issue = "145383")]
pub unsafe trait Field: UnalignedField {}

/// Type representing a field with structural pinning information.
///
/// # Safety
///
/// `Self::Projected<'a>` either is `Pin<&'a mut Self::Type>` or `&'a mut Self::Type`. In the first
/// case the field is structurally pinned.
#[lang = "PinnableField"]
#[unstable(feature = "field_projections", issue = "145383")]
pub unsafe trait PinnableField: UnalignedField {
    /// The pin-projection of this field.
    ///
    /// If this field is structurally pinned, this is `Pin<P>` otherwise it is `P`.
    type Projected<P>;

    /// Sets the correct value for a pin projection.
    ///
    /// # Safety
    ///
    /// The supplied reference must be derived from a `Pin<&mut Self::Base>`.
    unsafe fn from_pinned_ref(r: &mut Self::Type) -> Self::Projected<&mut Self::Type>;
}

/// TODO
#[unstable(feature = "field_projections", issue = "145383")]
#[allow_internal_unstable(builtin_syntax)]
pub macro field_of($Container:ty, $($fields:expr)+ $(,)?) {
    builtin # field_of($Container, $($fields)+)
}
