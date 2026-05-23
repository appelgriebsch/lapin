/// The routing algorithm used by an AMQP exchange.
///
/// Passed to [`Channel::exchange_declare`].
///
/// [`Channel::exchange_declare`]: crate::Channel::exchange_declare
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ExchangeKind {
    /// A custom exchange type provided by a broker plugin (e.g. `"x-delayed-message"`).
    Custom(String),
    /// Routes each message to the queue whose binding key exactly matches the routing key.
    #[default]
    Direct,
    /// Routes each message to every bound queue regardless of the routing key.
    Fanout,
    /// Routes messages based on header attributes rather than the routing key.
    Headers,
    /// Routes messages using routing key patterns with `*` (one word) and `#` (zero or more words).
    Topic,
}

impl ExchangeKind {
    pub(crate) fn kind(&self) -> &str {
        match self {
            Self::Custom(c) => c.as_str(),
            Self::Direct => "direct",
            Self::Fanout => "fanout",
            Self::Headers => "headers",
            Self::Topic => "topic",
        }
    }
}
