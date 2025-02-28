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

use std::fmt::Debug;

use crossterm::style::Stylize;
use get_size::GetSize;
use r3bl_rs_utils_core::*;
use serde::{Deserialize, Serialize};

use crate::*;

/// Events that can be applied to the [EditorEngine] to modify an [EditorBuffer].
///
/// By providing a conversion from [InputEvent] to [EditorEvent] it becomes easier to write event
/// handlers that consume [InputEvent] and then execute [EditorEvent] on an [EditorBuffer].
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EditorEvent {
    InsertChar(char),
    InsertString(String),
    InsertNewLine,
    Delete,
    Backspace,
    Home,
    End,
    PageDown,
    PageUp,
    MoveCaret(CaretDirection),
    Resize(Size),
    Select(SelectionScope),
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionScope {
    OneCharLeft,
    OneCharRight,
    OneLineUp,
    OneLineDown,
    PageUp,
    PageDown,
    Home,
    End,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, GetSize)]
pub enum CaretDirection {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<&InputEvent> for EditorEvent {
    type Error = String;

    fn try_from(input_event: &InputEvent) -> Result<Self, Self::Error> {
        call_if_true!(DEBUG_TUI_COPY_PASTE, {
            log_debug(format!(
                "\n#️⃣#️⃣#️⃣  EditorEvent::try_from: {}",
                format!("{}", input_event).red().on_white()
            ));
        });

        match input_event {
            // Selection events.
            InputEvent::Keyboard(KeyPress::WithModifiers {
                key: Key::SpecialKey(SpecialKey::Right),
                mask: ModifierKeysMask::SHIFT,
            }) => Ok(EditorEvent::Select(SelectionScope::OneCharRight)),

            InputEvent::Keyboard(KeyPress::WithModifiers {
                key: Key::SpecialKey(SpecialKey::Left),
                mask: ModifierKeysMask::SHIFT,
            }) => Ok(EditorEvent::Select(SelectionScope::OneCharLeft)),

            InputEvent::Keyboard(KeyPress::WithModifiers {
                key: Key::SpecialKey(SpecialKey::Down),
                mask: ModifierKeysMask::SHIFT,
            }) => Ok(EditorEvent::Select(SelectionScope::OneLineDown)),

            InputEvent::Keyboard(KeyPress::WithModifiers {
                key: Key::SpecialKey(SpecialKey::Up),
                mask: ModifierKeysMask::SHIFT,
            }) => Ok(EditorEvent::Select(SelectionScope::OneLineUp)),

            InputEvent::Keyboard(KeyPress::WithModifiers {
                key: Key::SpecialKey(SpecialKey::PageUp),
                mask: ModifierKeysMask::SHIFT,
            }) => Ok(EditorEvent::Select(SelectionScope::PageUp)),

            InputEvent::Keyboard(KeyPress::WithModifiers {
                key: Key::SpecialKey(SpecialKey::PageDown),
                mask: ModifierKeysMask::SHIFT,
            }) => Ok(EditorEvent::Select(SelectionScope::PageDown)),

            InputEvent::Keyboard(KeyPress::WithModifiers {
                key: Key::SpecialKey(SpecialKey::Home),
                mask: ModifierKeysMask::SHIFT,
            }) => Ok(EditorEvent::Select(SelectionScope::Home)),

            InputEvent::Keyboard(KeyPress::WithModifiers {
                key: Key::SpecialKey(SpecialKey::End),
                mask: ModifierKeysMask::SHIFT,
            }) => Ok(EditorEvent::Select(SelectionScope::End)),

            // Other events.
            InputEvent::Keyboard(KeyPress::Plain {
                key: Key::SpecialKey(SpecialKey::PageDown),
            }) => Ok(EditorEvent::PageDown),

            InputEvent::Keyboard(KeyPress::Plain {
                key: Key::SpecialKey(SpecialKey::PageUp),
            }) => Ok(EditorEvent::PageUp),

            InputEvent::Keyboard(KeyPress::Plain {
                key: Key::SpecialKey(SpecialKey::Home),
            }) => Ok(EditorEvent::Home),

            InputEvent::Keyboard(KeyPress::Plain {
                key: Key::SpecialKey(SpecialKey::End),
            }) => Ok(EditorEvent::End),

            InputEvent::Resize(size) => Ok(EditorEvent::Resize(*size)),

            InputEvent::Keyboard(KeyPress::Plain {
                key: Key::Character(character),
            }) => Ok(Self::InsertChar(*character)),

            InputEvent::Keyboard(KeyPress::Plain {
                key: Key::SpecialKey(SpecialKey::Enter),
            }) => Ok(Self::InsertNewLine),

            InputEvent::Keyboard(KeyPress::Plain {
                key: Key::SpecialKey(SpecialKey::Delete),
            }) => Ok(Self::Delete),

            InputEvent::Keyboard(KeyPress::Plain {
                key: Key::SpecialKey(SpecialKey::Backspace),
            }) => Ok(Self::Backspace),

            InputEvent::Keyboard(KeyPress::Plain {
                key: Key::SpecialKey(SpecialKey::Up),
            }) => Ok(Self::MoveCaret(CaretDirection::Up)),

            InputEvent::Keyboard(KeyPress::Plain {
                key: Key::SpecialKey(SpecialKey::Down),
            }) => Ok(Self::MoveCaret(CaretDirection::Down)),

            InputEvent::Keyboard(KeyPress::Plain {
                key: Key::SpecialKey(SpecialKey::Left),
            }) => Ok(Self::MoveCaret(CaretDirection::Left)),

            InputEvent::Keyboard(KeyPress::Plain {
                key: Key::SpecialKey(SpecialKey::Right),
            }) => Ok(Self::MoveCaret(CaretDirection::Right)),
            _ => Err(format!("Invalid input event: {input_event:?}")),
        }
    }
}

impl EditorEvent {
    pub fn apply_editor_event<S, A>(
        editor_engine: &mut EditorEngine,
        editor_buffer: &mut EditorBuffer,
        editor_event: EditorEvent,
        _shared_global_data: &SharedGlobalData,
        _component_registry: &mut ComponentRegistry<S, A>,
        _self_id: FlexBoxId,
    ) where
        S: Debug + Default + Clone + PartialEq + Sync + Send,
        A: Debug + Default + Clone + Sync + Send,
    {
        match editor_event {
            EditorEvent::InsertChar(character) => {
                EditorEngineInternalApi::insert_str_at_caret(
                    EditorArgsMut {
                        editor_buffer,
                        editor_engine,
                    },
                    &String::from(character),
                )
            }
            EditorEvent::InsertNewLine => {
                EditorEngineInternalApi::insert_new_line_at_caret(EditorArgsMut {
                    editor_buffer,
                    editor_engine,
                });
            }
            EditorEvent::Delete => {
                EditorEngineInternalApi::delete_at_caret(editor_buffer, editor_engine);
            }
            EditorEvent::Backspace => {
                EditorEngineInternalApi::backspace_at_caret(editor_buffer, editor_engine);
            }
            EditorEvent::MoveCaret(direction) => {
                match direction {
                    CaretDirection::Left => EditorEngineInternalApi::left(
                        editor_buffer,
                        editor_engine,
                        SelectMode::Disabled,
                    ),
                    CaretDirection::Right => EditorEngineInternalApi::right(
                        editor_buffer,
                        editor_engine,
                        SelectMode::Disabled,
                    ),
                    CaretDirection::Up => EditorEngineInternalApi::up(
                        editor_buffer,
                        editor_engine,
                        SelectMode::Disabled,
                    ),
                    CaretDirection::Down => EditorEngineInternalApi::down(
                        editor_buffer,
                        editor_engine,
                        SelectMode::Disabled,
                    ),
                };
            }
            EditorEvent::InsertString(chunk) => {
                EditorEngineInternalApi::insert_str_at_caret(
                    EditorArgsMut {
                        editor_buffer,
                        editor_engine,
                    },
                    &chunk,
                )
            }
            EditorEvent::Resize(_) => {
                // Check to see whether scroll is valid.
                EditorEngineInternalApi::validate_scroll(EditorArgsMut {
                    editor_buffer,
                    editor_engine,
                });
            }
            EditorEvent::Home => {
                EditorEngineInternalApi::home(
                    editor_buffer,
                    editor_engine,
                    SelectMode::Disabled,
                );
            }
            EditorEvent::End => {
                EditorEngineInternalApi::end(
                    editor_buffer,
                    editor_engine,
                    SelectMode::Disabled,
                );
            }
            EditorEvent::PageDown => {
                EditorEngineInternalApi::page_down(
                    editor_buffer,
                    editor_engine,
                    SelectMode::Disabled,
                );
            }
            EditorEvent::PageUp => {
                EditorEngineInternalApi::page_up(
                    editor_buffer,
                    editor_engine,
                    SelectMode::Disabled,
                );
            }
            EditorEvent::Select(selection_scope) => match selection_scope {
                SelectionScope::OneCharRight => {
                    EditorEngineInternalApi::right(
                        editor_buffer,
                        editor_engine,
                        SelectMode::Enabled,
                    );
                }
                SelectionScope::OneCharLeft => {
                    EditorEngineInternalApi::left(
                        editor_buffer,
                        editor_engine,
                        SelectMode::Enabled,
                    );
                }
                SelectionScope::OneLineDown => {
                    EditorEngineInternalApi::down(
                        editor_buffer,
                        editor_engine,
                        SelectMode::Enabled,
                    );
                }
                SelectionScope::OneLineUp => {
                    EditorEngineInternalApi::up(
                        editor_buffer,
                        editor_engine,
                        SelectMode::Enabled,
                    );
                }
                SelectionScope::PageUp => {
                    EditorEngineInternalApi::page_up(
                        editor_buffer,
                        editor_engine,
                        SelectMode::Enabled,
                    );
                }
                SelectionScope::PageDown => {
                    EditorEngineInternalApi::page_down(
                        editor_buffer,
                        editor_engine,
                        SelectMode::Enabled,
                    );
                }
                SelectionScope::Home => {
                    EditorEngineInternalApi::home(
                        editor_buffer,
                        editor_engine,
                        SelectMode::Enabled,
                    );
                }
                SelectionScope::End => {
                    EditorEngineInternalApi::end(
                        editor_buffer,
                        editor_engine,
                        SelectMode::Enabled,
                    );
                }
            },
        };
    }

    pub fn apply_editor_events<S, A>(
        editor_engine: &mut EditorEngine,
        editor_buffer: &mut EditorBuffer,
        editor_event_vec: Vec<EditorEvent>,
        shared_global_data: &SharedGlobalData,
        component_registry: &mut ComponentRegistry<S, A>,
        self_id: FlexBoxId,
    ) where
        S: Debug + Default + Clone + PartialEq + Sync + Send,
        A: Debug + Default + Clone + Sync + Send,
    {
        for editor_event in editor_event_vec {
            EditorEvent::apply_editor_event(
                editor_engine,
                editor_buffer,
                editor_event,
                shared_global_data,
                component_registry,
                self_id,
            );
        }
    }
}
