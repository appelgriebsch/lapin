/// Acknowledge one or more messages.
///
/// Tells the server that the consumer has successfully processed the message
/// identified by `delivery_tag`. If [`BasicAckOptions::multiple`] is `true`,
/// all unacknowledged messages up to and including `delivery_tag` are
/// acknowledged in one shot.
///
/// Prefer using [`crate::Acker::ack`] on the delivery directly; this low-level
/// method is exposed for advanced use-cases.
