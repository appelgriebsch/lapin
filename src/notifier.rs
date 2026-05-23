use crate::listener::Listener;
use std::{
    fmt,
    future::Future,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    task::{Context, Poll},
};

/// One-shot notifier: cloneable future that resolves once `notify_all` is called.
#[derive(Clone, Default)]
pub(crate) struct Notifier {
    done: Arc<AtomicBool>,
    listener: Listener,
}

impl Notifier {
    pub(crate) fn notify_all(&self) {
        self.done.store(true, Ordering::Release);
        self.listener.notify();
    }

    fn ready(&self) -> bool {
        self.done.load(Ordering::Acquire)
    }
}

impl Future for Notifier {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.ready() {
            return Poll::Ready(());
        }
        self.listener.arm();
        // Re-check after registering listener to close the lost-wakeup window.
        if self.ready() {
            return Poll::Ready(());
        }
        self.listener.poll(cx)
    }
}

impl fmt::Debug for Notifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Notifier").finish()
    }
}
