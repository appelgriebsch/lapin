/// Cancel a consumer subscription.
///
/// Tells the server to stop delivering messages for the consumer identified by
/// `consumer_tag`. This is the counterpart to [`Channel::basic_consume`]. After
/// this call the consumer's stream will end with `None`.
///
/// Prefer calling this over simply dropping the [`Consumer`]: an explicit cancel
/// sends the cancellation through the server so that no further messages are
/// delivered, whereas a drop only discards already-delivered ones.
