/// Reject a single message.
///
/// Signals to the server that the consumer could not process the message
/// identified by `delivery_tag`. If [`BasicRejectOptions::requeue`] is `true`
/// the message is re-queued; otherwise it is discarded or sent to a dead-letter
/// exchange.
///
/// To reject multiple messages in one call, use [`Channel::basic_nack`] with
/// [`BasicNackOptions::multiple`] set to `true`.
///
/// Prefer using [`crate::Acker::reject`] on the delivery directly; this low-level
/// method is exposed for advanced use-cases.
