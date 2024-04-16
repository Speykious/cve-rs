//! An 100% memory-safe implementation of [`std::mem::transmute`].

/// Interprets a value of type `A` as a value of type `B`. Equivalent of [`std::mem::transmute`], but implemented in safe code.
///
/// # Explanation
///
/// This transmute implementation relies on what I call "Enum abuse" — exploiting the memory representation of enums (+ other UB) in Rust to achieve silly things.
///
/// The core of the exploit is this enum:
///
/// ```ignore
/// enum DummyEnum<A, B> {
///     A(Option<Box<A>>),
///     B(Option<Box<B>>),
/// }
/// ```
///
/// First, we initialize a `DummyEnum::B(None)` and we get a dangling `&mut Option<Box<B>>` from it.
/// Then, we overwrite the dummy variable with a `DummyEnum::A(Some(<...>))` with the object we want to transmute.
/// Finally, we take the `Box<A>`, interpreted as a `Box<B>`, out of the dangling reference and we've transmuted our data!
///
/// # Safety
/// lol
///
pub fn transmute<A, B>(obj: A) -> B {
	use std::hint::black_box;

	// The layout of `DummyEnum` is approximately
	// DummyEnum {
	//     is_a_or_b: u8,
	//     data: usize,
	// }
	// Note that `data` is shared between `DummyEnum::A` and `DummyEnum::B`.
	// This should hopefully be more reliable than spamming the stack with a value and hoping the memory
	// is placed correctly by the compiler.
	#[allow(dead_code)]
	enum DummyEnum<A, B> {
		A(Option<Box<A>>),
		B(Option<Box<B>>),
	}

	#[inline(never)]
	fn transmute_inner<A, B>(dummy: &mut DummyEnum<A, B>, obj: A) -> B {
		let DummyEnum::B(ref_to_b) = dummy else {
			unreachable!()
		};
		let ref_to_b = crate::lifetime_expansion::expand_mut(ref_to_b);
		*dummy = DummyEnum::A(Some(Box::new(obj)));
		black_box(dummy);

		*ref_to_b.take().unwrap()
	}

	transmute_inner(black_box(&mut DummyEnum::B(None)), obj)
}

#[cfg(test)]
mod tests {
	// I'll allow it here.
	#![allow(unsafe_code)]

	#[test]
	#[allow(clippy::transmute_float_to_int, clippy::transmute_num_to_bytes)]
	fn test_transmute() {
		use crate::transmute;
		use std::mem;

		unsafe {
			assert_eq!(
				transmute::transmute::<f32, i32>(420.69),
				mem::transmute::<f32, i32>(420.69)
			);

			assert_eq!(
				transmute::transmute::<u32, i32>(0xf0000000),
				mem::transmute::<u32, i32>(0xf0000000)
			);

			assert_eq!(
				transmute::transmute::<f64, [u8; 8]>(123.456),
				mem::transmute::<f64, [u8; 8]>(123.456)
			);

			let my_ref = &42;
			assert_eq!(
				transmute::transmute::<&u8, isize>(my_ref),
				mem::transmute::<&u8, isize>(my_ref)
			);

			assert_eq!(
				transmute::transmute::<[i32; 5], [u8; 20]>([1, 2, 3, 4, 5]),
				mem::transmute::<[i32; 5], [u8; 20]>([1, 2, 3, 4, 5])
			);

			assert_eq!(
				transmute::transmute::<bool, u8>(true),
				mem::transmute::<bool, u8>(true)
			);
		}
	}
}
