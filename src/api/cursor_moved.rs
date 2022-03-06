use mlua::{Lua, Result};

use crate::state::UIState;
use crate::Nvim;

/// Executed on every `CursorMovedI` event.
pub fn cursor_moved(lua: &Lua, ui_state: &mut UIState) -> Result<()> {
    let nvim = Nvim::new(lua)?;

    ui_state.completion_menu.hide(&nvim)?;
    ui_state.completion_hint.erase(&nvim)?;
    ui_state.details_pane.hide(&nvim)?;
    // TODO: reset selected index?

    // TODO: if the cursor is at the end of line, hints are enabled and there's
    // at least one completion maybe we could show the hint?
    //
    // TODO: same on `InsertEnter`.

    Ok(())
}
