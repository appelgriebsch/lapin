/// Ask the server to redeliver all unacknowledged messages (fire-and-forget).
///
/// This is the asynchronous variant of [`Channel::basic_recover`]: it sends
/// the request but does not wait for a broker confirmation. If
/// [`BasicRecoverAsyncOptions::requeue`] is `true` the messages may be
/// delivered to a different consumer; if `false` they are redelivered to the
/// original consumer.
///
/// Prefer [`Channel::basic_recover`] unless you specifically need
/// fire-and-forget semantics.
