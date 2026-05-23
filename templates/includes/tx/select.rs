/// Enable standard AMQP transactions on this channel.
///
/// Once selected, publishes and acknowledgements are grouped into atomic
/// transactions that are committed with [`Channel::tx_commit`] or rolled back
/// with [`Channel::tx_rollback`]. Transactions significantly reduce throughput;
/// prefer publisher confirms ([`Channel::confirm_select`]) when possible.
