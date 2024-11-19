
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

const SESSION_COOKIE_NAME: &str = "session_id";

pub fn create_session(cookies: &mut Cookies) -> String {
    let session_id = Uuid::new_v4().to_string();
    let mut cookie = Cookie::new(SESSION_COOKIE_NAME, session_id.clone());
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookies.add(cookie);
    session_id
}

pub fn get_session(cookies: &Cookies) -> Option<String> {
    cookies.get(SESSION_COOKIE_NAME).map(|c| c.value().to_string())
}

pub fn remove_session(cookies: &mut Cookies) {
    cookies.remove(Cookie::build(SESSION_COOKIE_NAME).finish());
}

pub fn update_session(cookies: &mut Cookies, new_session_id: String) {
    let mut cookie = Cookie::new(SESSION_COOKIE_NAME, new_session_id);
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookies.add(cookie);
}
