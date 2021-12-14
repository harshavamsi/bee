// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::rand::bytes::rand_bytes_array;

use bee_message::address::Ed25519Address;

/// Generates a random [`Ed25519Address`].
pub fn rand_ed25519_address() -> Ed25519Address {
    Ed25519Address::new(rand_bytes_array())
}