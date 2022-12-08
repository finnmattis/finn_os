use core::{
    pin::Pin,
    task::{Context, Poll},
};
use futures_util::stream::{Stream, StreamExt};
use futures_util::task::AtomicWaker;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref TICKS: Mutex<TickCount> = Mutex::new(TickCount::new());
}
static WAKER: AtomicWaker = AtomicWaker::new();

pub struct TickCount {
    ticks: usize,
    new_tick: bool,
}

impl TickCount {
    pub fn new() -> Self {
        Self {
            ticks: 0,
            new_tick: false,
        }
    }

    pub fn increment(&mut self) {
        self.ticks += 1;
        self.new_tick = true;
        WAKER.wake();
    }

    pub fn handle(&mut self) -> Option<usize> {
        if !self.new_tick {
            return None;
        }
        self.new_tick = false;
        Some(self.ticks)
    }
}

pub struct TickStream {
    _private: (), //field to prevent construction of the struct from outside of the module
}

impl TickStream {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Stream for TickStream {
    type Item = usize;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<usize>> {
        if let Some(i) = TICKS.lock().handle() {
            Poll::Ready(Some(i))
        } else {
            WAKER.register(cx.waker());
            Poll::Pending
        }
    }
}

pub fn tick() {
    TICKS.lock().increment();
}

pub async fn sleep(ticks: usize) {
    let mut tick_stream = TickStream::new();
    for _ in 0..ticks {
        tick_stream.next().await;
    }
}
