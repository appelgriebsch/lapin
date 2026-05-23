/// Remove a binding between two exchanges.
///
/// Removes the exchange-to-exchange binding created by [`Channel::exchange_bind`].
/// `source`, `destination`, `routing_key`, and `arguments` must exactly match
/// the parameters used when the binding was created.
