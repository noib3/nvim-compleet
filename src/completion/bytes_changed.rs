use mlua::{prelude::LuaResult, Lua};
use neovim::{Api, Neovim};
use std::cmp;

use crate::state::State;
use crate::ui::menu;

/// Executed every time some bytes in the current buffer are changed.
pub fn bytes_changed(
    lua: &Lua,
    state: &mut State,
    start_row: u32,
    start_col: u32,
    rows_deleted: u32,
    bytes_deleted: u32,
    rows_added: u32,
    bytes_added: u32,
) -> LuaResult<()> {
    let api = Neovim::new(lua)?.api;

    // TODO: detach buffer on every `InsertLeave` and re-attach it on every
    // `InsertEnter`?
    //
    // We only care about insert mode events.
    if api.get_mode()?.0 != "i" {
        return Ok(());
    }

    // If we've added a new line, deleted one or stayed in the same line but
    // deleted characters we don't do anything. We only do something when new
    // characters are added to the current line.
    if rows_added != 0 || rows_deleted != 0 || bytes_deleted != 0 {
        return Ok(());
    }

    let cursor = &mut state.cursor;

    cursor.row = start_row;
    cursor.line = get_current_line(&api, cursor.row)?;
    cursor.bytes = start_col + bytes_added;

    // // Used for debugging.
    // let nvim = Neovim::new(lua)?;
    // nvim.print(format!("Start row: {start_row}"))?;
    // nvim.print(format!("Start col: {start_col}"))?;
    // nvim.print(format!("Rows deleted: {rows_deleted}"))?;
    // nvim.print(format!("Bytes deleted: {bytes_deleted}"))?;
    // nvim.print(format!("Rows added: {rows_added}"))?;
    // nvim.print(format!("Bytes added: {bytes_added}"))?;

    // // Used for debugging.
    // let nvim = Neovim::new(lua)?;
    // let mut current_line = buffer.line.clone();
    // current_line.insert(buffer.bytes_before_cursor, '|');
    // nvim.print(format!("Current row: {}", buffer.row))?;
    // nvim.print(format!("Current line (`|` is cursor): '{current_line}'"))?;

    let completions = &mut state.completions;
    *completions = super::complete(&cursor.line, cursor.bytes as usize);

    if completions.is_empty() {
        return Ok(());
    }

    let settings = &state.settings;
    let ui = &mut state.ui;
    let menu = &mut ui.completion_menu;

    // Queue an update for the completion menu.
    if settings.autoshow_menu {
        ui.queued_updates.menu_position = menu::positioning::get_position(
            &api,
            &completions,
            settings.max_menu_height,
        )?;

        // Update the selected completion.
        menu.selected_index = menu
            .selected_index
            .map(|old| cmp::min(old, completions.len() - 1));
    }

    // If hints are enabled and the cursor is at the end of the line, queue an
    // update for the completion hint.
    if settings.show_hints && cursor.is_at_eol() {
        ui.queued_updates.hinted_index =
            Some(ui.completion_menu.selected_index.unwrap_or(0));
    }

    Ok(())
}

fn get_current_line(api: &Api, current_row: u32) -> LuaResult<String> {
    let current_line = api
        .buf_get_lines(
            0,
            current_row,
            (current_row + 1).try_into().unwrap(),
            false,
        )?
        .into_iter()
        .nth(0)
        .expect("There's always at least 1 line in this range");

    Ok(current_line)
}
