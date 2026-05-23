/// Request to close the channel, providing a reply code and text.
///
/// The peer must respond with `channel.close-ok`. Use [`Channel::close`] rather
/// than calling this directly.
///
/// [`Channel::close`]: crate::Channel::close
