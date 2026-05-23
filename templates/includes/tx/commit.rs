/// Commit the current transaction.
///
/// All publishes and acknowledgements issued since the last
/// [`Channel::tx_select`], [`Channel::tx_commit`], or
/// [`Channel::tx_rollback`] are made permanent. Requires transaction mode to
/// be enabled first via [`Channel::tx_select`].
