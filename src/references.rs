//! Reimplementations of [`std::ptr::null()`] and [`std::ptr::null_mut()`], with safe code only.
//! Relies on [`crate::transmute`] under the hood.

/// Equivalent to [`std::ptr::null()`], but returns a null reference instead.
pub fn null<'a, T: 'static>() -> &'a T {
	crate::transmute(0usize)
}
/// Equivalent to [`std::ptr::null_mut()`], but returns a mutable null reference instead.
pub fn null_mut<'a, T: 'static>() -> &'a mut T {
	crate::transmute(0usize)
}

/// Not allocate an object. The returned reference is always invalid.
///
/// **Note:** It turns out that `null` is a valid memory address in WASM.
/// So here we use the maximum address instead.
pub fn not_alloc<'a, T: 'static>() -> &'a mut T {
	crate::transmute(usize::MAX)
}
