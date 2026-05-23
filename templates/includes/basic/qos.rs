/// Set the quality of service for this channel.
///
/// Limits the number of unacknowledged messages that the server will deliver.
/// `prefetch_count` controls the maximum number of unacknowledged messages;
/// 0 means no limit. The `global` flag (in [`BasicQosOptions`]) determines
/// whether the limit applies per-consumer (`false`) or across the whole channel
/// (`true`).
///
/// Call this before [`Channel::basic_consume`] to control back-pressure.
