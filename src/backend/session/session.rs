// session.rs

use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

// Definizione di una chiave costante per il nome del cookie di sessione
const SESSION_COOKIE_NAME: &str = "session_id";

pub fn create_session(cookies: &mut Cookies) -> String {
    // Genera un nuovo UUID per la sessione
    let session_id = Uuid::new_v4().to_string();
    // Crea un nuovo cookie con il session_id
    let mut cookie = Cookie::new(SESSION_COOKIE_NAME, session_id.clone());
    cookie.set_http_only(true); // Imposta il cookie come HTTP-only per sicurezza
    cookie.set_path("/"); // Imposta il percorso del cookie per tutte le route

    cookies.add(cookie);
    session_id
}

pub fn get_session(cookies: &Cookies) -> Option<String> {
    // Ottiene il valore del cookie di sessione se esiste
    cookies.get(SESSION_COOKIE_NAME).map(|c| c.value().to_string())
}

pub fn remove_session(cookies: &mut Cookies) {
    // Rimuove il cookie di sessione
    cookies.remove(Cookie::build(SESSION_COOKIE_NAME).finish());
}

pub fn update_session(cookies: &mut Cookies, new_session_id: String) {
        // Aggiorna il valore del cookie di sessione
    let mut cookie = Cookie::new(SESSION_COOKIE_NAME, new_session_id);
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookies.add(cookie);
}