/// Remove a binding between a queue and an exchange.
///
/// Removes the binding created by [`Channel::queue_bind`]. `queue`,
/// `exchange`, `routing_key`, and `arguments` must exactly match the
/// parameters used when the binding was created.
