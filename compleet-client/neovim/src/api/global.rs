use mlua::{FromLua, Function, Result, Table};

use super::Api;

impl<'a> Api<'a> {
    // TODO: make `command` accept strings.
    /// Binding to `vim.api.nvim_add_user_command`.
    ///
    /// Creates a new user command.
    ///
    /// # Arguments
    ///
    /// * `name`     Name of the new user command. Must begin with an uppercase letter.
    /// * `command`  Replacement command to execute when this user command is executed.
    /// * `opts`     Optional parameters. See `:h nvim_add_user_command` for  details.
    pub fn add_user_command(
        &self,
        name: &str,
        command: Function,
        opts: Table,
    ) -> Result<()> {
        self.0
            .get::<&str, Function>("nvim_add_user_command")?
            .call((name, command, opts))
    }

    /// Binding to `vim.api.nvim_create_buf`.
    ///
    /// Creates a new, empty, unnamed buffer. Returns the new buffer handle, or
    /// 0 on error.
    ///
    /// # Arguments
    ///
    /// * `listed`   Whether to set `buflisted`.
    /// * `scratch`  Whether the new buffer is a "throwaway" (`:h scratch-buffer`) buffer used for temporary work.
    pub fn create_buf(&self, listed: bool, scratch: bool) -> Result<u32> {
        self.0
            .get::<&str, Function>("nvim_create_buf")?
            .call((listed, scratch))
    }

    /// Binding to `vim.api.nvim_echo`.
    ///
    /// Echoes a message.
    ///
    /// # Arguments
    ///
    /// * `chunks`   A slice of `(text, hlgroup)` tuples, each representing a
    /// text chunk with specified highlight. The `hl_group` element can be
    /// set to `None` for no highlight.
    /// * `history`  Whether to add the message to the message history.
    pub fn echo<S: AsRef<str>>(
        &self,
        chunks: &[(S, Option<&str>)],
        history: bool,
    ) -> Result<()> {
        let chunks = chunks
            .iter()
            .map(|c| match c.1 {
                Some(hl_group) => vec![c.0.as_ref(), hl_group],
                None => vec![c.0.as_ref()],
            })
            .collect::<Vec<Vec<&str>>>();

        self.0.get::<&str, Function>("nvim_echo")?.call((
            chunks,
            history,
            Vec::<u8>::new(),
        ))
    }

    /// Binding to `vim.api.nvim_get_current_buf`
    ///
    /// Returns the current buffer handle.
    pub fn get_current_buf(&self) -> Result<u32> {
        self.0
            .get::<&str, Function>("nvim_get_current_buf")?
            .call(())
    }

    /// Binding to `vim.api.nvim_get_current_line`
    ///
    /// Returns the current line.
    pub fn get_current_line(&self) -> Result<String> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_get_current_line")?
            .call(())?)
    }

    /// Binding to `vim.api.nvim_get_mode`
    ///
    /// Returns a `(mode, is_blocking)` tuple.
    pub fn get_mode(&self) -> Result<(String, bool)> {
        let t = self
            .0
            .get::<&str, Function>("nvim_get_mode")?
            .call::<_, Table>(())?;

        Ok((t.get("mode")?, t.get("blocking")?))
    }

    /// Binding to `vim.api.nvim_get_option`
    ///
    /// Returns the global value of an option.
    ///
    /// # Arguments
    ///
    /// * `name`         Option name.
    pub fn get_option<V: FromLua<'a>>(&self, name: &str) -> Result<V> {
        self.0.get::<&str, Function>("nvim_get_option")?.call(name)
    }

    /// Binding to `vim.api.nvim_notify`.
    ///
    /// Notify the user with a message.
    ///
    /// # Arguments
    ///
    /// * `msg`         Message to display to the user.
    /// * `log_level`   The log level.
    pub fn notify<S: AsRef<str>>(
        &self,
        msg: S,
        level: super::LogLevel,
    ) -> Result<()> {
        self.0.get::<&str, Function>("nvim_notify")?.call((
            msg.as_ref(),
            level as u8,
            Vec::<u8>::new(),
        ))
    }

    /// Binding to `vim.api.nvim_replace_termcodes`
    ///
    /// Replaces terminal codes and keycodes in a string with the internal
    /// representation.
    ///
    /// # Arguments
    ///
    /// * `str`         String to be converted.
    /// * `from_part`   Legacy vim parameter. Usually true.
    /// * `do_lt`       Also translate `<lt>`.
    /// * `special`     Replace keycodes, e.g. `<CR>` becomes a `"\r"` char.
    pub fn replace_termcodes(
        &self,
        str: &str,
        from_part: bool,
        do_lt: bool,
        special: bool,
    ) -> Result<std::ffi::CString> {
        self.0
            .get::<&str, Function>("nvim_replace_termcodes")?
            .call((str, from_part, do_lt, special))
    }

    /// Binding to `vim.api.nvim_set_hl`
    ///
    /// Sets a highlight group
    ///
    /// # Arguments
    ///
    /// * `ns_id`  Namespace to use, or 0 to set a highlight group in the global namespace.
    /// * `name`   Highlight group name.
    /// * `opts`   Optional parameters. See `:h nvim_set_hl` for  details.
    pub fn set_hl(&self, ns_id: u32, name: &str, opts: Table) -> Result<()> {
        self.0
            .get::<&str, Function>("nvim_set_hl")?
            .call((ns_id, name, opts))
    }

    /// Binding to `vim.api.nvim_set_keymap`
    ///
    /// Sets a global mapping for the given mode.
    ///
    /// # Arguments
    ///
    /// * `mode`    Mode short name.
    /// * `lhs`     Left-hand-side of the mapping.
    /// * `rhs`     Right-hand-side of the mapping.
    /// * `opts`    Optional parameters. See `:h nvim_set_keymap` for  details.
    pub fn set_keymap(
        &self,
        mode: &str,
        lhs: &str,
        rhs: &str,
        opts: Table,
    ) -> Result<()> {
        self.0
            .get::<&str, Function>("nvim_set_keymap")?
            .call((mode, lhs, rhs, opts))
    }
}
