/*
 *   Copyright (c) 2022 R3BL LLC
 *   All rights reserved.
 *
 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
 */

use std::sync::Arc;

use tokio::sync::RwLock;

use crate::*;

// GlobalData.
pub type SharedGlobalData = Arc<RwLock<GlobalData>>;

// App.
pub type SafeApp<S, A> = dyn App<S, A> + Send + Sync;
pub type BoxedSafeApp<S, A> = Box<SafeApp<S, A>>;
pub type SharedApp<S, A> = Arc<RwLock<SafeApp<S, A>>>;

// Component.
pub type SafeComponent<S, A> = dyn Component<S, A> + Send + Sync;
pub type BoxedSafeComponent<S, A> = Box<SafeComponent<S, A>>;
pub type SharedComponent<S, A> = Arc<RwLock<SafeComponent<S, A>>>;
