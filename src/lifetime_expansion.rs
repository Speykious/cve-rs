//! Domain expansion: `'static` lifetime
//!
//! This is the cursed witchery behind all the bugs we have implemented so far.
//!
//! # How it works
//!
//! There is a soundness hole in the Rust compiler that allows our domain expansion to work.
//!
//! In the [`expand`] function, we use [`lifetime_translator`] with [`STATIC_UNIT`],
//! which has a `'static` lifetime, allowing us to translate an arbitrary lifetime
//! into any other lifetime.
//!
//! `rustc` *should* infer that one of the lifetimes does not outlive `'static`, so
//! that we can't use [`lifetime_translator`]; however, for whatever reason, it doesn't,
//! so this exploit works.
//!
//! See <https://github.com/rust-lang/rust/issues/25860> for this bug's bug report.
//! It's been open for multiple years!

/// Converts lifetime `'b` to lifetime `'a`.
///
/// This function, on its own, is sound:
/// - `_val_a`'s lifetime is `&'a &'b`. This means that `'b` must outlive `'a`, so
/// that the `'a` reference is never dangling. If `'a` outlived `'b` then it could
/// borrow data that's already been dropped.
/// - Therefore, `val_b`, which has a lifetime of `'b`, is valid for `'a`.
#[inline(never)]
pub const fn lifetime_translator<'a, 'b, T: ?Sized>(_val_a: &'a &'b (), val_b: &'b T) -> &'a T {
	val_b
}

/// This does the same thing as [`lifetime_translator`], just for mutable refs.
#[inline(never)]
pub fn lifetime_translator_mut<'a, 'b, T: ?Sized>(
	_val_a: &'a &'b (),
	val_b: &'b mut T,
) -> &'a mut T {
	val_b
}

/// Expands the domain of `'a` to `'b`.
///
/// # Safety
///
/// Safety? What's that?
pub fn expand<'a, 'b, T: ?Sized>(x: &'a T) -> &'b T {
	let f: fn(_, &'a T) -> &'b T = lifetime_translator;
	f(STATIC_UNIT, x)
}

/// This does the same thing as [`expand`] for mutable references.
///
/// # Safety
///
/// Safety? What's that?
pub fn expand_mut<'a, 'b, T: ?Sized>(x: &'a mut T) -> &'b mut T {
	let f: fn(_, &'a mut T) -> &'b mut T = lifetime_translator_mut;
	f(STATIC_UNIT, x)
}

/// A unit with a static lifetime.
///
/// Thanks to the soundness hole, this lets us cast any value all the way up to
/// a `'static` lifetime, meaning any lifetime we want.
pub const STATIC_UNIT: &&() = &&();
