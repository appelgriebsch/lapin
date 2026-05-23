/// Request the server to update the authentication secret (e.g. rotate an OAuth2 token).
///
/// Use [`Connection::update_secret`] or [`auth::TokenAuthProvider`] for automatic rotation.
///
/// [`Connection::update_secret`]: crate::Connection::update_secret
/// [`auth::TokenAuthProvider`]: crate::auth::TokenAuthProvider
