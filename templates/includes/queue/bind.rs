/// Bind a queue to an exchange.
///
/// Messages published to `exchange` that match `routing_key` (and `arguments`
/// for header exchanges) will be routed to `queue`. Multiple bindings with
/// different routing keys can be created between the same queue and exchange.
///
/// The binding is removed with [`Channel::queue_unbind`].
