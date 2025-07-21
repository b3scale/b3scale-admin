#!/bin/bash

# Script to fix ambiguous glob re-exports in b3scale_api models
# This script resolves naming conflicts by using explicit imports instead of glob imports

set -euo pipefail

MODELS_DIR="b3scale_api/src/models"
MOD_FILE="${MODELS_DIR}/mod.rs"

echo "🔧 Fixing ambiguous glob re-exports in ${MOD_FILE}..."

# Create backup
cp "${MOD_FILE}" "${MOD_FILE}.backup"

# Create the new mod.rs file with explicit imports to resolve conflicts
cat > "${MOD_FILE}" << 'EOF'
//! Auto-generated API models

pub mod attendee;
pub use attendee::*;
pub mod attendees_limit_settings;
pub use attendees_limit_settings::*;

// Backend module - contains AdminState and NodeState
pub mod backend;
pub use backend::{Backend, AdminState as BackendAdminState, NodeState as BackendNodeState};

pub mod backend_config;
pub use backend_config::*;

// Backend patch module - contains AdminState and NodeState (different from backend)
pub mod backend_patch;
pub use backend_patch::{BackendPatch, AdminState as BackendPatchAdminState, NodeState as BackendPatchNodeState};

// Backend request module - contains AdminState (different from backend and backend_patch)
pub mod backend_request;
pub use backend_request::{BackendRequest, AdminState as BackendRequestAdminState};

pub mod backend_settings;
pub use backend_settings::*;
pub mod breakout;
pub use breakout::*;

// Command module - contains Action
pub mod command;
pub use command::{Command, Action as CommandAction};

// Command request module - contains Action (different from command)
pub mod command_request;
pub use command_request::{CommandRequest, Action as CommandRequestAction};

pub mod default_presentation_settings;
pub use default_presentation_settings::*;
pub mod error;
pub use error::*;
pub mod format;
pub use format::*;
pub mod frontend;
pub use frontend::*;
pub mod frontend_config;
pub use frontend_config::*;
pub mod frontend_config_patch;
pub use frontend_config_patch::*;
pub mod frontend_patch;
pub use frontend_patch::*;
pub mod frontend_request;
pub use frontend_request::*;
pub mod frontend_settings;
pub use frontend_settings::*;
pub mod heartbeat;
pub use heartbeat::*;
pub mod image;
pub use image::*;
pub mod images;
pub use images::*;
pub mod meeting;
pub use meeting::*;
pub mod meeting_info;
pub use meeting_info::*;
pub mod meeting_info_patch;
pub use meeting_info_patch::*;
pub mod meeting_patch;
pub use meeting_patch::*;
pub mod migration_state;
pub use migration_state::*;
pub mod not_found_error;
pub use not_found_error::*;
pub mod preview;
pub use preview::*;
pub mod recording;
pub use recording::*;
pub mod recording_data;
pub use recording_data::*;
pub mod recording_visibility_update;
pub use recording_visibility_update::*;
pub mod recordings_settings;
pub use recordings_settings::*;
pub mod rpc_request;
pub use rpc_request::*;

// RPC Response module - contains Status (enum)
pub mod rpc_response;
pub use rpc_response::{RpcResponse, Status as RpcStatus};

pub mod schema_status;
pub use schema_status::*;
pub mod server_error;
pub use server_error::*;

// Status module - contains Status (struct, different from rpc_response)
pub mod status;
pub use status::Status;

pub mod validation_error;
pub use validation_error::*;
EOF

echo "✅ Fixed ambiguous glob re-exports!"
echo "📝 Original file backed up as ${MOD_FILE}.backup"

# Test that it compiles
echo "🧪 Testing compilation..."
cd b3scale_api
if cargo check --quiet; then
    echo "✅ Compilation successful!"
else
    echo "❌ Compilation failed. Restoring backup..."
    mv "${MOD_FILE}.backup" "${MOD_FILE}"
    exit 1
fi

echo "🎉 All done! Ambiguous glob re-exports have been resolved."