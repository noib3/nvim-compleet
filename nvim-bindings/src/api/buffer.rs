use mlua::{
    prelude::{LuaFunction, LuaResult},
    FromLua,
    Lua,
    Table,
    ToLua,
};

/// Binding to `vim.api.nvim_buf_attach`.
pub fn buf_attach(
    lua: &Lua,
    bufnr: u32,
    send_buffer: bool,
    opts: Table,
) -> LuaResult<bool> {
    super::api(lua)?.get::<_, LuaFunction>("nvim_buf_attach")?.call((
        bufnr,
        send_buffer,
        opts,
    ))
}

#[allow(dead_code)]
/// Binding to `vim.api.nvim_buf_call`.
pub fn buf_call(lua: &Lua, bufnr: u32, fun: LuaFunction) -> LuaResult<()> {
    super::api(lua)?.get::<_, LuaFunction>("nvim_buf_call")?.call((bufnr, fun))
}

/// Binding to `vim.api.nvim_buf_get_lines`.
pub fn buf_get_lines(
    lua: &Lua,
    bufnr: u32,
    start: i32,
    end: i32,
    strict_indexing: bool,
) -> LuaResult<Vec<String>> {
    super::api(lua)?.get::<_, LuaFunction>("nvim_buf_get_lines")?.call((
        bufnr,
        start,
        end,
        strict_indexing,
    ))
}

/// Binding to `vim.api.nvim_buf_get_name`.
pub fn buf_get_name(lua: &Lua, bufnr: u32) -> LuaResult<String> {
    super::api(lua)?.get::<_, LuaFunction>("nvim_buf_get_name")?.call(bufnr)
}

/// Binding to `vim.api.nvim_buf_get_option`.
pub fn buf_get_option<'lua, V: FromLua<'lua>>(
    lua: &'lua Lua,
    bufnr: u32,
    name: &str,
) -> LuaResult<V> {
    super::api(lua)?
        .get::<_, LuaFunction>("nvim_buf_get_option")?
        .call((bufnr, name))
}

/// Binding to `vim.api.nvim_buf_set_lines`.
pub fn buf_set_lines(
    lua: &Lua,
    bufnr: u32,
    start: i32,
    end: i32,
    strict_indexing: bool,
    replacement: Vec<String>,
) -> LuaResult<()> {
    super::api(lua)?.get::<_, LuaFunction>("nvim_buf_set_lines")?.call((
        bufnr,
        start,
        end,
        strict_indexing,
        replacement,
    ))
}

/// Binding to `vim.api.nvim_win_set_option`
pub fn buf_set_option<'lua, V: ToLua<'lua>>(
    lua: &'lua Lua,
    bufnr: u32,
    name: &str,
    value: V,
) -> LuaResult<()> {
    super::api(lua)?
        .get::<_, LuaFunction>("nvim_buf_set_option")?
        .call((bufnr, name, value))
}

/// Binding to `vim.api.nvim_buf_set_text`.
pub fn buf_set_text(
    lua: &Lua,
    bufnr: u32,
    start_row: u16,
    start_col: u16,
    end_row: u16,
    end_col: u16,
    replacement: Vec<String>,
) -> LuaResult<()> {
    super::api(lua)?.get::<_, LuaFunction>("nvim_buf_set_text")?.call((
        bufnr,
        start_row,
        start_col,
        end_row,
        end_col,
        replacement,
    ))
}
