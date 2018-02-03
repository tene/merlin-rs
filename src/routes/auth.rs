extern crate rocket_simpleauth as auth;

pub use self::auth::userpass::UserPass;
use self::auth::status::{LoginStatus,LoginRedirect};
use self::auth::authenticator::Authenticator;
use rocket::request::Form;
use rocket::response::content::Html;
use rocket::http::Cookies;

pub struct MerlinAuthenticator{
    username: String,
}

impl Authenticator for MerlinAuthenticator{
    type User = String;

    fn user(&self) -> String{
        self.username.to_owned()
    }   

    fn check_credentials(_username: String, _password: String) -> Result<Self,Self>{
        Ok(MerlinAuthenticator{username: _username})
    }
}

#[get("/login")]
fn admin(info: UserPass<String>) -> String{
	// we use request guards to fall down to the login page if UserPass couldn't find a valid cookie
	format!("Restricted administration area, user logged in: {}", info.user)
}


#[get("/login", rank = 2)]
fn login() -> Html<&'static str>{
    Html(
    "<h1>Login Page</h1>
    <form action=\"/login\" method=\"POST\">
        <input type=\"text\" name=\"username\" placeholder=\"Username\"/>
        <input type=\"password\" name=\"password\" placeholder=\"Password\"/>
        <input type=\"submit\" value=\"Login\" />
    </form>"
    )
}

#[post("/login", data = "<form>")]
fn login_post(form: Form<LoginStatus<MerlinAuthenticator>>, cookies: Cookies) -> LoginRedirect{
	// creates a response with either a cookie set (in case of a succesfull login)
	// or not (in case of a failure). In both cases a "Location" header is send.
	// the first parameter indicates the redirect URL when successful login,
	// the second a URL for a failed login
	form.into_inner().redirect("/login", "/login", cookies)
}