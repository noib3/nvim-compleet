use mlua::prelude::{Lua, LuaResult};
use neovim::Neovim;

use crate::state::State;
use crate::ui::menu;

/// Executed on `<Plug>(compleet-show-completions)`.
pub fn show_completions(lua: &Lua, state: &mut State) -> LuaResult<()> {
    let menu = &mut state.ui.completion_menu;
    let completions = &state.completions;

    if !menu.is_visible() && !completions.is_empty() {
        let api = Neovim::new(lua)?.api;

        let maybe_position = menu::positioning::get_position(
            &api,
            completions,
            state.settings.ui.menu.max_height,
        )?;

        if let Some(position) = maybe_position {
            menu.spawn(lua, &api, &position)?;
            menu.fill(lua, &api, completions)?;
        }
    }

    Ok(())
}
