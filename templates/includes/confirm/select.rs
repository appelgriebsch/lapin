/// Enable publisher confirms on this channel.
///
/// After calling this method, every [`Channel::basic_publish`] call returns a
/// [`crate::PublisherConfirm`] future that resolves once the broker has either
/// acknowledged ([`crate::Confirmation::Ack`]) or negatively acknowledged
/// ([`crate::Confirmation::Nack`]) the message.
///
/// Publisher confirms and AMQP transactions ([`Channel::tx_select`]) are
/// mutually exclusive. Use [`Channel::wait_for_confirms`] to drain all
/// outstanding confirms at once.
