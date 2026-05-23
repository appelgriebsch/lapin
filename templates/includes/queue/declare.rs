/// Declare a queue, creating it if it does not already exist.
///
/// Returns a [`Queue`] value carrying the queue name, message count, and
/// consumer count as reported by the server.
///
/// Common option presets are available as constructor methods on
/// [`QueueDeclareOptions`]: [`QueueDeclareOptions::durable`],
/// [`QueueDeclareOptions::exclusive`].
///
/// When [`QueueDeclareOptions::passive`] is `true` the server only checks
/// whether the queue exists without modifying it; an error is returned if it
/// does not.
