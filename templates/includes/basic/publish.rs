/// Publish a message to an exchange.
///
/// Routes `payload` through `exchange` using `routing_key`. The empty string
/// `""` selects the default exchange, which routes messages directly to the
/// queue whose name matches `routing_key`.
///
/// `properties` carries AMQP message metadata (content-type, headers,
/// delivery-mode, priority, …). Use [`BasicProperties::default()`] when you
/// do not need to set any.
///
/// Returns a [`crate::PublisherConfirm`] future. If publisher confirms are **not**
/// enabled (via [`Channel::confirm_select`]) the future resolves immediately
/// to [`crate::Confirmation::NotRequested`]. If they are enabled, it resolves once
/// the broker acknowledges or negatively-acknowledges the message.
///
/// Use [`Channel::wait_for_confirms`] to drain all outstanding confirms at
/// once.
