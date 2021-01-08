/**
 * This module defines the interactions between the CSML Manager and the underlying
 * database engines.
 *
 * There are several engines to choose from (see module features). To use one
 * of the database options, the ENGINE_DB_TYPE env var must be set
 * to one of the accepted values:
 *
 * - `mongodb`: requires a MongoDB-compatible database and additional variables:
 *   - MONGODB_HOST
 *   - MONGODB_PORT
 *   - MONGODB_DATABASE
 *   - MONGODB_USERNAME
 *   - MONGODB_PASSWORD
 *
 * - `dynamodb`: requires a DynamoDB-compatible database (on AWS, or dynamodb-local
 * for dev purposes). The following env vars are required (alternatively if deploying on AWS,
 * use IAM roles)
 *   - AWS_REGION
 *   - AWS_ACCESS_KEY_ID
 *   - AWS_SECRET_ACCESS_KEY
 *   - AWS_DYNAMODB_TABLE
 *   - AWS_DYNAMODB_ENDPOINT optional, defaults to the default dynamodb endpoint for the given region.
 * Both AWS_REGION AND AWS_DYNAMODB_ENDPOINT must be set to use a custom dynamodb-compatible DB.
 *
 * If the ENGINE_DB_TYPE env var is not set, mongodb is used by default.
 *
 * To add a new DB type, please use one of the existing templates implementations.
 * Each method of each module must be fully reimplemented in order to extend the "generic"
 * implementation at the root of db_connectors directory.
 */
use crate::data::{Database, EngineError};
use crate::error_messages::ERROR_DB_SETUP;
use serde::{Deserialize, Serialize};
use csml_interpreter::data::csml_bot::CsmlBot;

#[cfg(feature = "dynamo")]
use self::dynamodb as dynamodb_connector;
#[cfg(feature = "mongo")]
use self::mongodb as mongodb_connector;

pub mod bot;
pub mod conversations;
pub mod interactions;
pub mod memories;
pub mod messages;
pub mod nodes;
pub mod state;

use crate::Client;

#[cfg(feature = "dynamo")]
mod dynamodb;
#[cfg(feature = "mongo")]
mod mongodb;

#[derive(Serialize, Deserialize, Debug)]
pub struct DbConversation {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: String,
    pub client: Client,
    pub flow_id: String,
    pub step_id: String,
    pub metadata: serde_json::Value,
    pub status: String,
    pub last_interaction_at: String,
    pub updated_at: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DbInteraction {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: String,
    pub client: Client,
    pub success: bool,
    pub event: serde_json::Value,
    pub updated_at: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DbMemory {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: String,
    pub client: Client,
    pub interaction_id: String,
    pub conversation_id: String,
    pub flow_id: String,
    pub step_id: String,
    pub memory_order: i32,
    pub interaction_order: i32,
    pub key: String,
    pub value: serde_json::Value,
    pub expires_at: Option<String>,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DbMessage {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: String,
    pub client: Client,
    pub interaction_id: String,
    pub conversation_id: String,
    pub flow_id: String,
    pub step_id: String,
    pub message_order: i32,
    pub interaction_order: i32,
    pub direction: String,
    pub payload: serde_json::Value,
    pub content_type: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DbNode {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: String,
    pub client: Client,
    pub interaction_id: String,
    pub conversation_id: String,
    pub flow_id: String,
    pub step_id: String,
    pub next_step: Option<String>,
    pub next_flow: Option<String>,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DbState {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: String,
    pub client: Client,
    #[serde(rename = "type")]
    pub _type: String,
    pub value: serde_json::Value,
    pub expires_at: Option<String>,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DbBot {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: String,
    pub bot_id: String,
    pub bot: String,
    pub engine_version: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BotVersion {
    pub bot: CsmlBot,
    pub version_id: String,
}

impl BotVersion {
    pub fn flatten(&self) -> serde_json::Value {
        serde_json::json!(
            {
                "version_id": self.version_id,
                "id": self.bot.id,
                "name": self.bot.name,
                "fn_endpoint": self.bot.fn_endpoint,
                "flows": self.bot.flows,
                "custom_components": self.bot.custom_components,
                "default_flow": self.bot.default_flow,
            }
        )
    }
}

#[cfg(feature = "mongo")]
pub fn is_mongodb() -> bool {
    // If the env var is not set at all, use mongodb by default
    match std::env::var("ENGINE_DB_TYPE") {
        Ok(val) => val == "mongodb".to_owned(),
        Err(_) => true,
    }
}

#[cfg(feature = "dynamo")]
pub fn is_dynamodb() -> bool {
    match std::env::var("ENGINE_DB_TYPE") {
        Ok(val) => val == "dynamodb".to_owned(),
        Err(_) => false,
    }
}

pub fn init_db() -> Result<Database, EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        return mongodb_connector::init();
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        return dynamodb_connector::init();
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}
