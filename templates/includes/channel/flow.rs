/// Enable or disable message flow on this channel.
///
/// When [`ChannelFlowOptions::active`] is `false`, the server stops sending
/// content frames. This is a flow-control mechanism: pause delivery when your
/// consumer is overwhelmed and resume it when ready. Returns the actual flow
/// state confirmed by the server.
///
/// Note: RabbitMQ supports this method but does not apply back-pressure on
/// the publisher side; prefer [`Channel::basic_qos`] for consumer-side
/// throttling.
