use super::*;

use std::sync::Arc;

use client::{AnyProtoClient, Client};
use gpui::{App, Context};

use super::Project;
use crate::{BufferStore, ContextServerStore, LspStore, SettingsObserver, TaskStore, WorktreeStore};
use crate::debugger::breakpoint_store::BreakpointStore;
use crate::debugger::dap_store::DapStore;
use crate::git_store::GitStore;
// 上游这两个 init 与本函数同处 project.rs，可直接按模块名调用；拆分后
// project/mod.rs 只转发了 ContextServerStore 类型，没转发模块名，需显式引入。
use crate::{connection_manager, context_server_store};

impl Project {
    pub fn init(client: &Arc<Client>, cx: &mut App) {
         connection_manager::init(client.clone(), cx);

         let client: AnyProtoClient = client.clone().into();
         client.add_entity_message_handler(Self::handle_add_collaborator);
         client.add_entity_message_handler(Self::handle_update_project_collaborator);
         client.add_entity_message_handler(Self::handle_remove_collaborator);
         client.add_entity_message_handler(Self::handle_update_project);
         client.add_entity_message_handler(Self::handle_unshare_project);
         client.add_entity_request_handler(Self::handle_update_buffer);
         client.add_entity_message_handler(Self::handle_update_worktree);
         client.add_entity_request_handler(Self::handle_synchronize_buffers);

         client.add_entity_request_handler(Self::handle_search_candidate_buffers);
         client.add_entity_request_handler(Self::handle_open_buffer_by_id);
         client.add_entity_request_handler(Self::handle_open_buffer_by_path);
         client.add_entity_request_handler(Self::handle_open_new_buffer);
         client.add_entity_message_handler(Self::handle_create_buffer_for_peer);
         client.add_entity_message_handler(Self::handle_toggle_lsp_logs);
         client.add_entity_message_handler(Self::handle_create_image_for_peer);
         client.add_entity_request_handler(Self::handle_find_search_candidates_chunk);
         client.add_entity_message_handler(Self::handle_find_search_candidates_cancel);
         client.add_entity_message_handler(Self::handle_create_file_for_peer);

         WorktreeStore::init(&client);
         BufferStore::init(&client);
         LspStore::init(&client);
         GitStore::init(&client);
         SettingsObserver::init(&client);
         TaskStore::init(Some(&client));
         ToolchainStore::init(&client);
         DapStore::init(&client, cx);
         BreakpointStore::init(&client);
         context_server_store::init(cx);
     }
}
