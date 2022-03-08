pub mod api;
pub mod extract;
pub mod middleware;
pub mod route;
pub mod store;
pub mod user;

type APIResult<T> = std::result::Result<T, extract::response::APIError>;