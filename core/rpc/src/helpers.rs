// Copyright 2018-2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

use futures::{prelude::*, sync::oneshot};

/// Wraps around `oneshot::Receiver` and adjusts the error type to produce an internal error if the
/// sender gets dropped.
pub struct Receiver<T>(pub oneshot::Receiver<T>);

impl<T> Future for Receiver<T> {
	type Item = T;
	type Error = jsonrpc_core::Error;

	fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
		self.0.poll().map_err(|_| jsonrpc_core::Error::internal_error())
	}
}

/// Unwraps the trailing parameter or falls back with the closure result.
pub fn unwrap_or_else<F, H, E>(or_else: F, optional: Option<H>) -> Result<H, E> where
	F: FnOnce() -> Result<H, E>,
{
	match optional.into() {
		None => or_else(),
		Some(x) => Ok(x),
	}
}
