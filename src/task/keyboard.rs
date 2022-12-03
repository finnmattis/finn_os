use crate::task::keyboard_util::{DecodedKey, Keyboard};
use crate::{serial_print, serial_println};
use alloc::vec::Vec;
use conquer_once::spin::OnceCell;
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use crossbeam_queue::{ArrayQueue, PopError};
use futures_util::stream::{Stream, StreamExt};
use futures_util::task::AtomicWaker;

//use OnceCell over lazy_static bc OnceCell type has the advantage that we can ensure that the initialization does not happen in the interrupt handler, thus preventing the interrupt handler from performing a heap allocation
pub static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

/// called by the keyboard interrupt handler - must not block or allocate.
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            serial_println!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        serial_println!("WARNING: scancode queue uninitialized");
    }
}

pub fn get_scancode() -> Result<u8, PopError> {
    let queue = SCANCODE_QUEUE.try_get();
    if let Ok(queue) = queue {
        queue.pop()
    } else {
        SCANCODE_QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");
        let queue = SCANCODE_QUEUE.try_get().unwrap();
        //Try to pop for race condition
        queue.pop()
    }
}

pub fn get_all_scancodes() -> Result<Vec<u8>, PopError> {
    let queue = SCANCODE_QUEUE.try_get();
    let mut codes = Vec::new();
    if let Ok(queue) = queue {
        while let Ok(code) = queue.pop() {
            codes.push(code);
        }
        if codes.len() == 0 {
            return Err(PopError);
        } else {
            Ok(codes)
        }
    } else {
        SCANCODE_QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");
        let queue = SCANCODE_QUEUE.try_get().unwrap();
        //Try to get codes for race condition
        while let Ok(code) = queue.pop() {
            codes.push(code);
        }
        if codes.len() == 0 {
            return Err(PopError);
        } else {
            Ok(codes)
        }
    }
}

static WAKER: AtomicWaker = AtomicWaker::new();
pub struct ScancodeStream {
    _private: (), //field to prevent construction of the struct from outside of the module
}

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");
        Self { _private: () }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        //don't want to register WAKER for nothing
        if let Ok(scancode) = get_scancode() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(&cx.waker());
        match get_scancode() {
            //IDT can fill this asynchronously fill queue right after - check again to avoid race condition
            Ok(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}

pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    // Scancode Set 1 = IMB XT
    let mut keyboard = Keyboard::new();
    while let Some(scancode) = scancodes.next().await {
        if let Ok(ev) = keyboard.get_key_ev(scancode) {
            if let Some(key) = keyboard.process_key_ev(ev) {
                match key {
                    DecodedKey::Unicode(character) => serial_print!("{}", character),
                    DecodedKey::RawKey(key) => serial_print!("{:?}", key),
                }
            }
        }
    }
}
