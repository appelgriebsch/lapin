/// Roll back the current transaction.
///
/// Discards all publishes and acknowledgements issued since the last
/// [`Channel::tx_select`], [`Channel::tx_commit`], or
/// [`Channel::tx_rollback`]. Requires transaction mode to be enabled first
/// via [`Channel::tx_select`].
