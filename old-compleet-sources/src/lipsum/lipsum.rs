use async_trait::async_trait;
use bindings::opinionated::{Buffer, Neovim};
use mlua::Lua;

use super::lorems::{LOREMS, LOREM_IPSUM};
use super::LipsumConfig;
use crate::completion_context::Cursor;
use crate::completion_item::{CompletionItemBuilder, Completions};
use crate::completion_source::{CompletionSource, ShouldAttach};

#[derive(Debug, Default)]
pub struct Lipsum {
    pub _config: LipsumConfig,
}

#[async_trait]
impl CompletionSource for Lipsum {
    fn on_buf_enter(
        &mut self,
        _: &Lua,
        _: &Buffer,
    ) -> crate::Result<ShouldAttach> {
        Ok(true)
    }

    async fn complete(
        &mut self,
        _: &Neovim,
        _: &Cursor,
        _: &Buffer,
    ) -> crate::Result<Completions> {
        // // Simulate a slow source, this shouldn't block.
        // tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        Ok(LOREMS
            .iter()
            .map(|&lorem| {
                CompletionItemBuilder::new(lorem)
                    .details_text(LOREM_IPSUM)
                    .build()
            })
            .collect())
    }
}
