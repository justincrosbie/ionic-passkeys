use rocket::fairing::AdHoc;

pub mod user;
pub mod passkeys;

#[options("/<_..>")]
fn all_options() {
    // No code here....
}

///
/// This is used to load all the API routes that we know about
/// 
pub(crate) fn stage() -> AdHoc {
    AdHoc::on_ignite("Api Stage", |rocket| async {
        rocket.mount("/api", routes![
            passkeys::hello,
            passkeys::start_registration,
            passkeys::complete_registration,
            passkeys::start_authentication,
            passkeys::complete_authentication,
            // user::get_info,
            // user::post_update,
            // user::get_users,
            // user::post_modify,
            // user::post_secret,
            // config::get_config,
            // config::post_config,
            self::all_options, 
        ])
    })
}