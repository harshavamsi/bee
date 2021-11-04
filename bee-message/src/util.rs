// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Module that provides utilities.

use crate::error::ValidationError;

use alloc::borrow::ToOwned;

/// Tries to decode a hexadecimal encoded string to an array of bytes.
pub fn hex_decode<const N: usize>(hex: &str) -> Result<[u8; N], ValidationError> {
    hex::decode(hex)
        .map_err(|_| ValidationError::InvalidHexadecimalChar(hex.to_owned()))?
        .try_into()
        .map_err(|_| ValidationError::InvalidHexadecimalLength {
            expected: N * 2,
            actual: hex.len(),
        })
}
