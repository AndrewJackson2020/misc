use ldap3::{LdapConnAsync, LdapConnSettings};
use futures::executor;

// Pass in username and password to authenticate against LDAP
async fn authenticate(username: &str, password: &str) -> bool {
    // Connection to the LDAP Server
    let (conn, mut ldap) =
        LdapConnAsync::with_settings(LdapConnSettings::new(),
                                     "ldap://ldap.forumsys.com:389").await.unwrap();
    ldap3::drive!(conn);

    // Takes the username provided and converts it into an email for validation
    // This is required because LDAP uses either the Distinguished name or Email in order to bind. Username alone will not work :/
    let email = format!("uid={},dc=example,dc=com", username);

    // Attempts a simple bind using the passed in values of username and Password
    let result = ldap.simple_bind(email.as_str(), password).await.unwrap().success();
    ldap.unbind().await.unwrap();

    // If the authentication is successful return true, else return false.
    if !result.is_err() {
        true
    } else { false }
}

#[tokio::main]
async fn main() {
    let x = executor::block_on(authenticate("tesla", "passwordfake"));
    dbg!(x);
    let x = executor::block_on(authenticate("tesla", "password"));
    dbg!(x);
}
