use crate::ai::msg::{get_text_content, user_msg};
use crate::ai::OaClient;
use crate::utils::cli::{
	ico_check, ico_deleted_ok, ico_err, ico_uploaded, ico_uploading,
};
use crate::utils::files::XFile;
use crate::Result;
use async_openai::types::{
	AssistantObject, AssistantToolsRetrieval, CreateAssistantFileRequest,
	CreateAssistantRequest, CreateFileRequest, CreateRunRequest,
	CreateThreadRequest, ModifyAssistantRequest, RunStatus, ThreadObject,
};
use console::Term;
use derive_more::{Deref, Display, From};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;
