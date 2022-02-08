pub mod config;
pub mod app;
pub mod response;

use self::response::APIResponse;

type APIResult<T> = std::result::Result<T, response::APIError>;


use super::models::app::Entity as Application;
use super::models::app::ActiveModel as ApplicationActive;
use super::models::app::Model as ApplicationModel;