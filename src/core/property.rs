//
// Copyright (c) 2026-present, SkillerRaptor
//
// SPDX-License-Identifier: MIT
//

use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use relm4::tokio::sync::{
    Notify,
    watch::{self, Receiver, Sender},
};

#[derive(Clone)]
pub struct Property<T: Clone + Send + Sync + 'static> {
    tx: Sender<T>,
    rx: Receiver<T>,
    subscriber_count: Arc<AtomicUsize>,
    notify: Arc<Notify>,
}

impl<T: Clone + Send + Sync + 'static> Property<T> {
    pub fn new(initial: T) -> Self {
        let (tx, rx) = watch::channel(initial);

        Self {
            tx,
            rx,
            subscriber_count: Arc::new(AtomicUsize::default()),
            notify: Arc::new(Notify::new()),
        }
    }

    pub fn write(&self, value: T)
    where
        T: PartialEq,
    {
        self.tx.send_if_modified(|current| {
            if *current != value {
                *current = value;
                true
            } else {
                false
            }
        });
    }

    pub fn write_unconditional(&self, value: T) {
        self.tx.send_modify(|current| *current = value);
    }

    pub fn read(&self) -> T {
        self.rx.borrow().clone()
    }

    pub fn subscribe(&self) -> Subscription<T> {
        self.subscriber_count.fetch_add(1, Ordering::SeqCst);
        self.notify.notify_waiters();

        Subscription {
            rx: self.rx.clone(),
            subscriber_count: self.subscriber_count.clone(),
            initial: false,
        }
    }
}

pub struct Subscription<T> {
    rx: Receiver<T>,
    subscriber_count: Arc<AtomicUsize>,
    initial: bool,
}

impl<T: Clone> Subscription<T> {
    pub async fn next(&mut self) -> Option<T> {
        if !self.initial {
            self.initial = true;
            return Some(self.rx.borrow_and_update().clone());
        }

        if self.rx.changed().await.is_ok() {
            Some(self.rx.borrow_and_update().clone())
        } else {
            None
        }
    }
}

impl<T> Drop for Subscription<T> {
    fn drop(&mut self) {
        self.subscriber_count.fetch_sub(1, Ordering::Release);
    }
}
