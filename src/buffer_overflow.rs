//! A memory-safe buffer overflow.
//!
//! We allocate a slice on the stack, then transmute it into a String with a large capacity.
//! Then, we read input from stdin into that String. This overwrites another stack-allocated
//! slice, and then we can check if it's successfully overwritten.

use std::io::{stdin, stdout, Write};
use std::time::Duration;
use std::{io, mem, thread};

use crate::construct_fake_string;

/// Perform a buffer overflow.
///
/// This is implemented in the form of a little password cracking game in the terminal.
#[inline(never)]
pub fn buffer_overflow() -> io::Result<()> {
	use std::hint::black_box;

	#[repr(C)]
	#[derive(Default)]
	struct Authentication {
		name_buf: [u8; 16],
		password: [u8; 16],
	}

	let mut auth = black_box(Authentication::default());

	// Noone will ever have the time to type more than 1024 characters... ;v
	let mut name = construct_fake_string(auth.name_buf.as_mut_ptr(), 1024usize, 0usize);

	print!("Hello! What's your name? > ");
	stdout().flush()?;
	stdin().read_line(&mut name)?;

	// If we don't forget our fake String, Rust will try to deallocate it as if it was a heap pointer.
	mem::forget(name);

	let password = &auth.password[0..8];

	if password.iter().all(|&x| x == 0) {
		println!("You didn't even modify the password...");
	} else if &password != b"letmein!" {
		println!(
			"Wrong password! You entered: {:?}",
			std::str::from_utf8(password).unwrap()
		);
	} else {
		#[cfg(unix)]
		println!("Correct password, running sudo rm -rf /* ...");
		#[cfg(windows)]
		println!("Correct password, deleting C:\\Windows\\System32 ...");

		thread::sleep(Duration::from_secs(2));
	}

	black_box(auth);

	Ok(())
}
