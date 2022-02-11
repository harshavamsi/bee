// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{payload::fpc::Opinion, MessageId, MessageUnpackError};

use packable::Packable;

/// Describes a vote in a given round for a message timestamp.
#[derive(Clone, Debug, Eq, PartialEq, Packable)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[packable(unpack_error = MessageUnpackError)]
pub struct Timestamp {
    /// Identifier of the message that contains the timestamp.
    message_id: MessageId,
    /// The node's opinion value in a given round.
    opinion: Opinion,
    /// Voting round number.
    round: u8,
}

impl Timestamp {
    /// Creates a new [`Timestamp`].
    pub fn new(message_id: MessageId, opinion: Opinion, round: u8) -> Self {
        Self {
            message_id,
            opinion,
            round,
        }
    }

    /// Returns the identifier of the message that contains the timestamp.
    pub fn message_id(&self) -> &MessageId {
        &self.message_id
    }

    /// Returns the node's opinion value in a given round.
    pub fn opinion(&self) -> Opinion {
        self.opinion
    }

    /// Returns the voting round number.
    pub fn round(&self) -> u8 {
        self.round
    }
}