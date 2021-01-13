use actix_web::{post, get, web, HttpResponse};
use csml_engine::{create_bot_version, get_bot_by_version_id, get_bot_versions, get_last_bot_version};
use csml_interpreter::data::csml_bot::CsmlBot;
use serde::{Deserialize, Serialize};
use std::thread;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRequest {
  bot: CsmlBot,
}


/*
 * create bot version
 *
 * {"statusCode": 200,"body": {"version_id": String} }
 *
 */
#[post("/bots")]
pub async fn add_bot_version(body: web::Json<CsmlBot>) -> HttpResponse {
  let bot = body.to_owned();

  let res = thread::spawn(move || {
    create_bot_version(bot)
  }).join().unwrap();

  match res {
    Ok(data) => HttpResponse::Created().json(serde_json::json!({"version_id": data})),
    Err(err) => {
      eprintln!("EngineError: {:?}", err);
      HttpResponse::InternalServerError().finish()
    }
  }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GetBotPath {
  bot_id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBotVersionsQuery {
  limit: Option<i64>,
  last_key: Option<String>,
}

/*
 * get the latest version of a given bot
 *
 * {"statusCode": 200,"body": Bot}
 *
 * BOT = {
 *  "version_id": String,
 *  "id": String,
 *  "name": String,
 *  "custom_components": Option<String>,
 *  "default_flow": String
 *  "engine_version": String
 *  "created_at": String
 * }
 */
#[get("/bots/{bot_id}")]
pub async fn get_bot_latest_version(path: web::Path<GetBotPath>) -> HttpResponse {
  let bot_id = path.bot_id.to_owned();

  let res = thread::spawn(move || {
    get_last_bot_version(&bot_id)
  }).join().unwrap();

  match res {
    Ok(Some(bot_version)) => HttpResponse::Ok().json(bot_version.flatten()),
    Ok(None) => HttpResponse::NotFound().finish(),
    Err(err) => {
      eprintln!("EngineError: {:?}", err);
      HttpResponse::InternalServerError().finish()
    }
  }
}

/*
 * Get the last versions of the bot. This does not return the flows!
 * Limited to 20 versions if not specified
 *
 * {"statusCode": 200,"body": Vec<Bot>}
 *
 * BOT = {
 *  "version_id": String,
 *  "id": String,
 *  "name": String,
 *  "custom_components": Option<String>,
 *  "default_flow": String
 *  "engine_version": String
 *  "created_at": String
 * }
 */
#[get("/bots/{bot_id}/versions")]
pub async fn get_bot_latest_versions(path: web::Path<GetBotPath>, query: web::Query<GetBotVersionsQuery>) -> HttpResponse {
  let bot_id = path.bot_id.to_owned();
  let limit = query.limit.to_owned();
  let last_key = query.last_key.to_owned();

  let res = thread::spawn(move || {
    get_bot_versions(&bot_id, limit, last_key)
  }).join().unwrap();

  match res {
    Ok(data) => HttpResponse::Ok().json(data),
    Err(err) => {
      eprintln!("EngineError: {:?}", err);
      HttpResponse::InternalServerError().finish()
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BotVersionPath {
  bot_id: String,
  version_id: String,
}

/*
 * Retrieve a specific version of a bot
 *
 * {"statusCode": 200,"body": Bot}
 *
 * BOT = {
 *  "version_id": String,
 *  "id": String,
 *  "name": String,
 *  "custom_components": Option<String>,
 *  "default_flow": String
 *  "engine_version": String
 *  "created_at": String
 * }
 */
#[get("/bots/{bot_id}/versions/{version_id}")]
pub async fn get_bot_version(
  path: web::Path<BotVersionPath>) -> HttpResponse {
  let bot_id = path.bot_id.to_owned();
  let version_id = path.version_id.to_owned();

  let res = thread::spawn(move || {
    get_bot_by_version_id(&version_id, &bot_id)
  }).join().unwrap();

  match res {
    Ok(Some(bot_version)) => HttpResponse::Ok().json(bot_version.flatten()),
    Ok(None) => HttpResponse::NotFound().finish(),
    Err(err) => {
      eprintln!("EngineError: {:?}", err);
      HttpResponse::InternalServerError().finish()
    }
  }
}