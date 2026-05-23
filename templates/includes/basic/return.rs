/// Server notification that a mandatory or immediate message could not be routed.
///
/// Unroutable messages are returned to the publisher as [`BasicReturnMessage`] values
/// accessible via [`Channel::wait_for_confirms`] or from the [`Confirmation`] resolved
/// by the matching [`PublisherConfirm`].
///
/// [`BasicReturnMessage`]: crate::message::BasicReturnMessage
/// [`Channel::wait_for_confirms`]: crate::Channel::wait_for_confirms
/// [`Confirmation`]: crate::Confirmation
/// [`PublisherConfirm`]: crate::PublisherConfirm
