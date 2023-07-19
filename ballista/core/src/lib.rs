// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#![doc = include_str!("../README.md")]
pub const BALLISTA_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn print_version() {
    println!("Ballista version: {BALLISTA_VERSION}")
}

#[cfg(not(windows))]
pub mod cache_layer;
pub mod client;
pub mod config;
pub mod consistent_hash;
pub mod error;
pub mod event_loop;
pub mod execution_plans;
pub mod object_store_registry;
/// some plugins
pub mod plugin;
pub mod utils;

#[macro_use]
pub mod serde;
