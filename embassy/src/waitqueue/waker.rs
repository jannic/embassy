use core::ptr::{self, NonNull};
use core::task::Waker;

use atomic_polyfill::{compiler_fence, AtomicPtr, Ordering};

use crate::executor::raw::{task_from_waker, wake_task, TaskHeader};

/// Utility struct to register and wake a waker.
///
/// # Safety
///
/// This type is optimized for (and only works with) embassy tasks.
#[derive(Debug)]
pub struct WakerRegistration {
    waker: Option<NonNull<TaskHeader>>,
}

impl WakerRegistration {
    /// Create a new `WakerRegistration`.
    pub const fn new() -> Self {
        Self { waker: None }
    }

    /// Register a waker. Overwrites the previous waker, if any.
    pub fn register(&mut self, w: &Waker) {
        let w = task_from_waker(w);
        match self.waker {
            // Optimization: If both the old and new Wakers wake the same task, do nothing.
            Some(w2) if w == w2 => {}
            Some(w2) => {
                // We had a waker registered for another task. Wake it, so the other task can
                // reregister itself if it's still interested.
                //
                // If two tasks are waiting on the same thing concurrently, this will cause them
                // to wake each other in a loop fighting over this WakerRegistration. This wastes
                // CPU but things will still work.
                //
                // If the user wants to have two tasks waiting on the same thing they should use
                // a more appropriate primitive that can store multiple wakers.

                unsafe { wake_task(w2) }
                self.waker = Some(w);
            }
            None => self.waker = Some(w),
        }
    }

    /// Wake the registered waker, if any.
    pub fn wake(&mut self) {
        if let Some(w) = self.waker.take() {
            unsafe { wake_task(w) }
        }
    }

    /// Returns true if a waker is currently registered
    pub fn occupied(&self) -> bool {
        self.waker.is_some()
    }
}

// SAFETY: `WakerRegistration` effectively contains an `Option<Waker>`,
// which is `Send` and `Sync`.
unsafe impl Send for WakerRegistration {}
unsafe impl Sync for WakerRegistration {}

/// Utility struct to atomically register and wake a waker.
///
/// # Safety
///
/// This type is optimized for (and only works with) embassy tasks.
pub struct AtomicWaker {
    waker: AtomicPtr<TaskHeader>,
}

impl AtomicWaker {
    /// Create a new `AtomicWaker`.
    pub const fn new() -> Self {
        Self {
            waker: AtomicPtr::new(ptr::null_mut()),
        }
    }

    /// Register a waker. Overwrites the previous waker, if any.
    pub fn register(&self, w: &Waker) {
        let w = task_from_waker(w);
        self.waker.store(w.as_ptr(), Ordering::Relaxed);
        compiler_fence(Ordering::SeqCst);
    }

    /// Wake the registered waker, if any.
    pub fn wake(&self) {
        let w2 = self.waker.load(Ordering::Relaxed);
        if let Some(w2) = NonNull::new(w2) {
            unsafe { wake_task(w2) };
        }
    }
}
