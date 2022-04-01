use mlua::{Function, Result, Table};

use super::Api;

impl<'a> Api<'a> {
    /// Binding to `vim.api.nvim_create_augroup`.
    ///
    /// Creates or gets an augroup. Returns the id to be used in
    /// `Nvim::del_augroup_by_id`.
    ///
    /// # Arguments
    ///
    /// * `name`  The name of the augroup to create.
    /// * `opts`  Optional parameters. See `:h nvim_create_augroup` for
    ///   details.
    pub fn create_augroup(&self, name: &str, opts: Table) -> Result<u32> {
        self.0
            .get::<&str, Function>("nvim_create_augroup")?
            .call((name, opts))
    }

    /// Binding to `vim.api.nvim_create_autocmd`.
    ///
    /// Creates an autocmd. Returns an id to be used in `Nvim::del_autocmd`.
    ///
    /// # Arguments
    ///
    /// * `events`  A slice of strings reprenting event names.
    /// * `opts`    Optional parameters. See `:h nvim_create_autocmd` for
    ///   details.
    pub fn create_autocmd(&self, events: &[&str], opts: Table) -> Result<u32> {
        self.0
            .get::<&str, Function>("nvim_create_autocmd")?
            .call((events, opts))
    }

    /// Binding to `vim.api.nvim_del_augroup_by_id`.
    ///
    /// Delete an augroup by id. The `id` can only be returned when
    /// augroup was created with `Api::create_augroup.`
    ///
    /// # Arguments
    ///
    /// * `id`  The id of the augroup to delete.
    pub fn del_augroup_by_id(&self, id: u32) -> Result<()> {
        self.0
            .get::<&str, Function>("nvim_del_augroup_by_id")?
            .call(id)
    }

    /// Binding to `vim.api.nvim_del_augroup_by_name`.
    ///
    /// Delete an augroup by name. The `name` can only be returned when
    /// augroup was created with `Api::create_augroup.`
    ///
    /// # Arguments
    ///
    /// * `name`  The name of the augroup to delete.
    pub fn del_augroup_by_name(&self, name: &str) -> Result<()> {
        self.0
            .get::<&str, Function>("nvim_del_augroup_by_name")?
            .call::<_, ()>(name)
    }

    /// Binding to `vim.api.nvim_del_autocmd`.
    ///
    /// Delete an autocmd by `id`.
    ///
    /// # Arguments
    ///
    /// * `id`  The id of the autocmd to delete.
    pub fn del_autocmd(&self, id: u32) -> Result<()> {
        self.0.get::<&str, Function>("nvim_del_autocmd")?.call(id)
    }

    /// Binding to `vim.api.nvim_do_autocmd`.
    ///
    /// Do one autocmd.
    ///
    /// # Arguments
    ///
    /// * `event`  The event or events to execute.
    /// * `opts`  Optional parameters. See `:h nvim_do_autocmd` for  details.
    pub fn do_autocmd(&self, events: &[&str], opts: Table) -> Result<()> {
        self.0
            .get::<&str, Function>("nvim_do_autocmd")?
            .call((events, opts))
    }

    /// Binding to `vim.api.nvim_get_autocmds`.
    ///
    /// Get autocmds that match the requirements passed to `opts`.
    ///
    /// # Arguments
    ///
    /// * `opts`  Optional parameters. See `:h nvim_get_autocmds` for  details.
    pub fn get_autocmds(&self, opts: Table<'a>) -> Result<Table> {
        self.0
            .get::<&str, Function>("nvim_get_autocmds")?
            .call(opts)
    }
}
