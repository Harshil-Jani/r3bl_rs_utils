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

use super::*;
use crate::{DEBUG, *};

pub async fn run_app() -> CommonResult<()> {
    throws!({
        // Ignore errors: https://doc.rust-lang.org/std/result/enum.Result.html#method.ok
        if DEBUG {
            try_to_set_log_level(log::LevelFilter::Trace).ok();
        } else {
            try_to_set_log_level(log::LevelFilter::Off).ok();
        }

        // Create store.
        let store = create_store().await;

        // Create an App (renders & responds to user input).
        let shared_app = AppNoLayout::new_shared();

        // Exit if these keys are pressed.
        let exit_keys: Vec<InputEvent> =
            vec![InputEvent::Keyboard(keypress! { @char 'x' })];

        // Create a window.
        TerminalWindow::main_event_loop(shared_app, store, exit_keys).await?
    });
}
