//! Tokio runtime integration for GPUI.
//!
//! This module provides a bridge between GPUI's async executor and Tokio's
//! runtime, allowing Tokio-based async operations (like network I/O) to work
//! seamlessly within GPUI applications.
//!
//! TODO: REMOVE after bumping gpui to latest version that is compatible with
//! `gpui_tokio` in upstream
//!
//! Based on: https://github.com/zed-industries/zed/blob/main/crates/gpui_tokio/src/gpui_tokio.rs

use std::future::Future;
use std::mem::ManuallyDrop;

use gpui::{App, AppContext, Global, ReadGlobal, Task};
use tokio::runtime::{Builder, Handle, Runtime};
use tokio::task::JoinError;

/// Initializes the Tokio wrapper using a new Tokio runtime with 2 worker
/// threads.
///
/// If you need more threads (or access to the runtime outside of GPUI), you can
/// create the runtime yourself and pass a Handle to [`init_from_handle`].
pub fn init(cx: &mut App) {
    let runtime = Builder::new_multi_thread()
        // Since we now have two executors, let's try to keep our footprint small
        .worker_threads(2)
        .enable_all()
        .build()
        .expect("Failed to initialize Tokio runtime");

    cx.set_global(GlobalTokio::new(RuntimeHolder::Owned(runtime)));
}

/// Initializes the Tokio wrapper using a Tokio runtime handle.
#[allow(dead_code)]
pub fn init_from_handle(cx: &mut App, handle: Handle) {
    cx.set_global(GlobalTokio::new(RuntimeHolder::Shared(handle)));
}

enum RuntimeHolder {
    Owned(Runtime),
    Shared(Handle),
}

impl RuntimeHolder {
    pub fn handle(&self) -> &Handle {
        match self {
            RuntimeHolder::Owned(runtime) => runtime.handle(),
            RuntimeHolder::Shared(handle) => handle,
        }
    }
}

struct GlobalTokio {
    runtime: RuntimeHolder,
}

impl Global for GlobalTokio {}

impl GlobalTokio {
    fn new(runtime: RuntimeHolder) -> Self {
        Self { runtime }
    }
}

/// A helper struct for spawning Tokio tasks within GPUI.
pub struct Tokio;

impl Tokio {
    /// Spawns the given future on Tokio's thread pool, and returns it via a
    /// GPUI task.
    ///
    /// Note that the Tokio task will be cancelled if the GPUI task is dropped.
    pub fn spawn<C, Fut, R>(cx: &C, f: Fut) -> <C as AppContext>::Result<Task<Result<R, JoinError>>>
    where
        C: AppContext,
        Fut: Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        cx.read_global(|tokio: &GlobalTokio, cx| {
            let join_handle = tokio.runtime.handle().spawn(f);
            let abort_handle = join_handle.abort_handle();
            let cancel = Defer::new(move || {
                abort_handle.abort();
            });

            cx.background_spawn(async move {
                let result = join_handle.await;
                drop(cancel);

                result
            })
        })
    }

    /// Spawns the given future on Tokio's thread pool, and returns it via a
    /// GPUI task.
    ///
    /// This variant unwraps the `JoinError` into an `anyhow::Result`.
    /// Note that the Tokio task will be cancelled if the GPUI task is dropped.
    #[allow(dead_code)]
    pub fn spawn_result<C, Fut, R>(
        cx: &C,
        f: Fut,
    ) -> <C as AppContext>::Result<Task<anyhow::Result<R>>>
    where
        C: AppContext,
        Fut: Future<Output = anyhow::Result<R>> + Send + 'static,
        R: Send + 'static,
    {
        cx.read_global(|tokio: &GlobalTokio, cx| {
            let join_handle = tokio.runtime.handle().spawn(f);
            let abort_handle = join_handle.abort_handle();
            let cancel = Defer::new(move || {
                abort_handle.abort();
            });

            cx.background_spawn(async move {
                let result = join_handle.await?;
                drop(cancel);

                result
            })
        })
    }

    /// Returns a clone of the Tokio runtime handle.
    #[allow(dead_code)]
    pub fn handle(cx: &App) -> Handle {
        GlobalTokio::global(cx).runtime.handle().clone()
    }
}

/// A simple RAII guard that runs a closure when dropped.
struct Defer<F: FnOnce()> {
    f: ManuallyDrop<F>,
}

impl<F: FnOnce()> Defer<F> {
    fn new(f: F) -> Self {
        Self {
            f: ManuallyDrop::new(f),
        }
    }
}

impl<F: FnOnce()> Drop for Defer<F> {
    fn drop(&mut self) {
        // SAFETY: We only take the value once, in drop
        let f = unsafe { ManuallyDrop::take(&mut self.f) };
        f();
    }
}
