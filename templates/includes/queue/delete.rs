/// Delete a queue.
///
/// Returns the number of messages that were in the queue. The queue and all
/// its bindings are removed. If [`QueueDeleteOptions::if_unused`] is set, the
/// delete only succeeds when there are no consumers; if
/// [`QueueDeleteOptions::if_empty`] is set, it only succeeds when the queue
/// has no messages.
