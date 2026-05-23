/// Ask the server to redeliver all unacknowledged messages.
///
/// Waits for the broker to confirm that all outstanding unacknowledged
/// messages have been requeued or redelivered. If
/// [`BasicRecoverOptions::requeue`] is `true` the messages may be delivered
/// to a different consumer; if `false` they are redelivered to the original
/// consumer.
