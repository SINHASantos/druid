// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Platform specific implementations.

#[cfg_attr(target_os = "windows", path = "windows/mod.rs")]
#[cfg_attr(target_os = "macos", path = "mac/mod.rs")]
#[cfg_attr(all(target_os = "linux", feature = "x11"), path = "x11/mod.rs")]
#[cfg_attr(all(target_os = "linux", not(feature = "x11")), path = "gtk/mod.rs")]
#[cfg_attr(target_arch = "wasm32", path = "wasm32/mod.rs")]
mod platform_impl;
pub use platform_impl::*;

pub(crate) mod shared;
