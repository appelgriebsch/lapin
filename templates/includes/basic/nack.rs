/// Negatively acknowledge one or more messages (RabbitMQ extension).
///
/// Similar to [`Channel::basic_reject`] but supports bulk rejection via
/// [`BasicNackOptions::multiple`]. If `multiple` is `true`, all
/// unacknowledged messages up to and including `delivery_tag` are rejected.
/// If [`BasicNackOptions::requeue`] is `true` the messages are re-queued;
/// otherwise they are discarded or dead-lettered.
///
/// Prefer using [`crate::Acker::nack`] on the delivery directly; this low-level
/// method is exposed for advanced use-cases.
