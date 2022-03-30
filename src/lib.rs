use std::panic;
use std::sync::Arc;

use mlua::{prelude::LuaResult, Lua, Table};
use neovim::Neovim;
use parking_lot::Mutex;

mod api;
mod autocmds;
mod commands;
mod completion;
mod hlgroups;
mod mappings;
mod settings;
mod state;
mod ui;

use state::State;

/*
BUGs:
1. (ui) menu's position doesn't update when the signcolumn changes. Not really
   sure how to solve this, there's no `SignColumnChanged` autocmd to listen to;

TODOs: On Hold

1. Show scroll indicator if number of completions is bigger than the completion
   menu's max height. This needs floating windows to support scrollbars. See
   `:h api-floatwin`.

2. Add option `ui.details.add_menu_spacing` to add 1 column of horizontal
   spacing between the completion menu and the details window.
*/

#[mlua::lua_module]
fn compleet(lua: &Lua) -> LuaResult<Table> {
    // The plugin runs in the main thread, so panics will take down the whole
    // Neovim process. We can't do a lot except relaying the panic infos.
    panic::set_hook(Box::new(|infos| {
        eprintln!(
            "[nvim-compleet] {infos}. \
             Please open a new issue at \
             'https://github.com/noib3/nvim-compleet/issues'."
        );
        std::process::exit(1);
    }));

    let api = Neovim::new(lua)?.api;
    let state = Arc::new(Mutex::new(State::new(&api)?));

    let _state = state.clone();
    let has_completions = lua.create_function(move |lua, ()| {
        api::has_completions(lua, &mut _state.lock())
    })?;

    let _state = state.clone();
    let is_completion_selected = lua.create_function(move |_, ()| {
        Ok(_state.lock().ui.completion_menu.is_item_selected())
    })?;

    let _state = state.clone();
    let is_hint_visible = lua.create_function(move |_, ()| {
        Ok(_state.lock().ui.completion_hint.is_visible())
    })?;

    let _state = state.clone();
    let is_menu_visible = lua.create_function(move |_, ()| {
        Ok(_state.lock().ui.completion_menu.is_visible())
    })?;

    let setup = lua.create_function(move |lua, preferences| {
        api::setup(lua, &state, preferences)
    })?;

    Ok(lua.create_table_from([
        ("has_completions", has_completions),
        ("is_completion_selected", is_completion_selected),
        ("is_hint_visible", is_hint_visible),
        ("is_menu_visible", is_menu_visible),
        ("setup", setup),
    ])?)
}
