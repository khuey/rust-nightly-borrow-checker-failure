use std::collections::HashMap;
use std::pin::Pin;
use std::sync::{Arc, Mutex, MutexGuard};
use std::ptr;

use futures::channel::mpsc::UnboundedSender;

enum Continuation {
    RegisterRead(u64),
}

pub struct ps_prochandle {
    global_state: Arc<GlobalState>,
    global_lock: Option<MutexGuard<'static, HashMap<u64, UnboundedSender<Continuation>>>>,
}

impl ps_prochandle {
    fn lock_to_run(mut self: Pin<&mut Self>) {
        assert!(self.global_lock.is_none());

        let global_lock = ptr::NonNull::from(&self.global_state.global_lock);

        unsafe {
            let locked = global_lock.as_ref().lock().unwrap();
            ptr::write(&mut self.global_lock, Some(locked));
        }
    }
}

struct GlobalState {
    global_lock: Mutex<HashMap<u64, UnboundedSender<Continuation>>>,
}
