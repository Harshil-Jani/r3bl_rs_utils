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

use crossterm::event::*;
use r3bl_rs_utils::*;

#[test]
fn test_convert_keyevent_into_twinputevent() {
  // Crossterm KeyEvents.
  let x = KeyEvent {
    code: KeyCode::Char('x'),
    modifiers: KeyModifiers::NONE,
  };
  let caps_x = KeyEvent {
    code: KeyCode::Char('X'),
    modifiers: KeyModifiers::SHIFT,
  };
  let ctrl_x = KeyEvent {
    code: KeyCode::Char('x'),
    modifiers: KeyModifiers::CONTROL,
  };

  // TWInputEvents.
  let x_tw = TWInputEvent::from(x);
  let caps_x_tw = TWInputEvent::from(caps_x);
  let ctrl_x_tw = TWInputEvent::from(ctrl_x);

  // Check that the conversion is correct.
  assert_eq2!(x_tw, TWInputEvent::DisplayableKeypress('x'));
  assert_eq2!(caps_x_tw, TWInputEvent::DisplayableKeypress('X'));
  assert_eq2!(ctrl_x_tw, TWInputEvent::NonDisplayableKeypress(ctrl_x));
}