use super::lib::RedirectWithCookie;

#[get("/")]
pub fn handler() -> RedirectWithCookie {
  RedirectWithCookie::new("id=0; HttpOnly=true; Max-Age=0; Path=/; SameSite=Strict; Secure=true;".into())
}
