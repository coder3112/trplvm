/*
 * Copyright (c)  2023 Naitik Mundra.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 */

/// This module creates the AnyObject struct, important to manage state in a HashMap with
/// multiple types possible as a value. Taken from the sourcecode of the "state" crate.
/// (https://github.com/SergioBenitez/master/src/container.rs)

use std::any::Any;

/// So that we can use const and static with Any
#[repr(C)]
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct AnyObject {
	pub data: *mut (),
	pub vtable: *mut (),
}

impl AnyObject {
	/// "Set" value
	pub fn anonymise<T: 'static>(value: T) -> AnyObject {
		let any: Box<dyn Any> = Box::new(value) as Box<dyn Any>;
		let any: *mut dyn Any = Box::into_raw(any);
		unsafe { std::mem::transmute(any) }
	}

	/// "Get" Value
	/// SAFETY: Make sure that the object exists.
	pub fn deanonymise<T: 'static>(&self) -> Option<&T> {
		unsafe {
			let any: *const *const dyn Any = self as *const AnyObject as *const *const dyn Any;
			let any: &dyn Any = &*(*any as *const dyn Any);
			any.downcast_ref()
		}
	}
}

