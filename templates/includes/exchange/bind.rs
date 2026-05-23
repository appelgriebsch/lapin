/// Bind a source exchange to a destination exchange.
///
/// Messages published to `source` that match `routing_key` (and `arguments`
/// for header exchanges) will be forwarded to `destination`. This is a
/// RabbitMQ extension that allows exchange-to-exchange routing.
///
/// The binding is removed with [`Channel::exchange_unbind`].
