use std::ops::Deref;

use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

use lib::settings::Settings;

pub struct Config(pub Settings);

impl Deref for Config {
    type Target = Settings;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Config {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Config, ()> {
        let state = request.guard::<State<Config>>()?;
        Outcome::Success(Config(state.0.clone()))
    }
}
