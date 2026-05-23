use event_listener::{Event, EventListener};
use std::{
    fmt,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

/// Shared wakeup primitive used by [`crate::notifier::Notifier`] and
/// [`crate::consumer::Consumer`].
///
/// All clones share the same [`Event`] so any [`notify`] call wakes every
/// concurrent poller.  Each clone owns its own pending [`EventListener`] slot
/// so individual tasks can register independently.
///
/// Usage pattern:
/// 1. Call [`arm`] *before* checking the guarded condition.
/// 2. If the condition is met, call [`disarm`] and proceed.
/// 3. If not, call [`poll`]: `Ready` means a notification arrived while
///    checking (loop and retry); `Pending` means the waker is registered.
///
/// [`arm`]: Self::arm
/// [`disarm`]: Self::disarm
/// [`poll`]: Self::poll
/// [`notify`]: Self::notify
#[derive(Default)]
pub(crate) struct Listener {
    event: Arc<Event>,
    pending: Option<Pin<Box<EventListener>>>,
}

impl Listener {
    /// Register a listener if one is not already pending.
    pub(crate) fn arm(&mut self) {
        if self.pending.is_none() {
            self.pending = Some(Box::pin(self.event.listen()));
        }
    }

    /// Drop the pending listener without polling it.
    pub(crate) fn disarm(&mut self) {
        self.pending = None;
    }

    /// Poll the pending listener.
    ///
    /// Returns `Ready` if the event fired (listener is cleared); `Pending` if
    /// not yet notified (waker registered, listener kept for the next poll).
    ///
    /// # Panics
    ///
    /// Panics if called before [`arm`].
    ///
    /// [`arm`]: Self::arm
    pub(crate) fn poll(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        match self
            .pending
            .as_mut()
            .expect("poll called before arm")
            .as_mut()
            .poll(cx)
        {
            Poll::Ready(()) => {
                self.pending = None;
                Poll::Ready(())
            }
            Poll::Pending => Poll::Pending,
        }
    }

    /// Wake all clones that have a pending listener registered.
    pub(crate) fn notify(&self) {
        self.event.notify(usize::MAX);
    }
}

impl Clone for Listener {
    /// Produce a clone that shares the same [`Event`] but starts with no
    /// pending listener so each task registers its own waker independently.
    fn clone(&self) -> Self {
        Self {
            event: self.event.clone(),
            pending: None,
        }
    }
}

impl fmt::Debug for Listener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Listener")
            .field("pending", &self.pending.is_some())
            .finish()
    }
}
