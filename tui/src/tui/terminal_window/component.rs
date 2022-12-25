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

use async_trait::async_trait;
use r3bl_rs_utils_core::*;

use crate::*;

/// See [App].
#[async_trait]
pub trait Component<S, A>
where
    S: Default + Clone + PartialEq + Debug + Sync + Send,
    A: Default + Clone + Sync + Send,
{
    fn get_id(&self) -> FlexBoxId;

    /// Render this component given the following.
    ///
    /// - Arguments:
    ///   - Get from `current_box`:
    ///     - box_origin_pos: Position
    ///     - box_bounding_size: Size
    ///     - maybe_box_style: `Option<Style>`
    ///   - Get from `state`:
    ///     - Content to render
    ///     - get_focus_id(): String to determine if this component has keyboard focus (might affect
    ///       the way it gets rendered)
    ///   - Maybe use `shared_store`:
    ///     - Dispatch an action if needed
    ///
    /// - Returns:
    ///   - [RenderPipeline] which must be rendered by the caller
    ///
    /// - Clipping, scrolling, overdrawing:
    ///   - Each implementation of this trait is solely responsible of taking care of these behaviors
    ///
    async fn render(
        &mut self,
        args: ComponentScopeArgs<'_, S, A>,
        current_box: &FlexBox,
    ) -> CommonResult<RenderPipeline>;

    /// If this component has focus [HasFocus] then this method will be called to handle input event
    /// that is meant for it.
    async fn handle_event(
        &mut self,
        args: ComponentScopeArgs<'_, S, A>,
        input_event: &InputEvent,
    ) -> CommonResult<EventPropagation>;
}

#[async_trait]
pub trait SurfaceRender<S, A>
where
    S: Default + Clone + PartialEq + Debug + Sync + Send,
    A: Default + Clone + Sync + Send,
{
    async fn render_in_surface(
        &mut self,
        args: GlobalScopeArgs<'_, S, A>,
        surface: &mut Surface,
    ) -> CommonResult<()>;
}
