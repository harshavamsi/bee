// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![cfg(feature = "standalone")]

use tokio::time::{self, Duration};

use std::future::Future;

pub fn shutdown(secs: u64) -> Box<dyn Future<Output = ()> + Send + Unpin> {
    Box::new(Box::pin(time::sleep(Duration::from_secs(secs))))
}