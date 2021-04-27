use std::{
    any::Any,
    cell::{RefCell, RefMut},
    cmp::Ordering,
    collections::HashMap,
};

const STREAM_SIZE_BLOCK: usize = 10;

#[derive(Debug)]
pub struct EventStream<E: Any> {
    buffer: Vec<E>,
    head: Option<usize>,
    subscriptions: HashMap<SubscriptionToken, RefCell<Subscription>>,
    token_serial: u64,
}

impl<E: Any> Default for EventStream<E> {
    fn default() -> Self {
        EventStream {
            buffer: Vec::with_capacity(STREAM_SIZE_BLOCK),
            head: None,
            subscriptions: HashMap::default(),
            token_serial: 0,
        }
    }
}

impl<E: Any> EventStream<E> {
    pub fn subscribe(&mut self) -> SubscriptionToken {
        let token = self.generate_token();
        self.subscriptions.insert(
            token,
            RefCell::new(Subscription {
                position: self.head,
            }),
        );

        token
    }

    pub fn unsubscribe(&mut self, token: SubscriptionToken) {
        self.subscriptions.remove(&token);
    }

    pub fn publish(&mut self, event: E) {
        if let Some(mut head) = self.head {
            head += 1;
            if head == self.buffer.capacity() {
                head = 0;
            }
            let tail = self.tail();
            if let Some(tail) = tail {
                if head == tail && self.buffer.capacity() == self.buffer.len() {
                    head = self.increase_capacity(tail);
                }
            }

            if self.buffer.len() == self.buffer.capacity() {
                self.buffer[head] = event;
            } else {
                self.buffer.push(event);
            }
        } else {
            self.buffer.push(event);
            self.head = Some(0);
        }
    }

    pub fn read(&self, token: &SubscriptionToken) -> impl Iterator<Item = &E> {
        let head = self.head.unwrap_or(0);
        let mut subscription = self.get_subscription(token);
        let start = match subscription.position {
            Some(pos) => pos + 1,
            None => 0,
        };
        subscription.position = self.head;

        self.buffer
            .iter()
            .cycle()
            .skip(start)
            .take(if start >= self.buffer.len() {
                0
            } else if head >= start {
                head - start + 1
            } else {
                self.buffer.len() - (start - head) + 1
            })
    }

    pub fn read_last(&self) -> Option<&E> {
        match self.head {
            Some(head) => self.buffer.get(head),
            _ => None,
        }
    }

    fn get_subscription(&self, token: &SubscriptionToken) -> RefMut<Subscription> {
        self.subscriptions
            .get(token)
            .expect("Invalid token supplied")
            .borrow_mut()
    }

    fn generate_token(&mut self) -> SubscriptionToken {
        let new_token = self.token_serial;
        self.token_serial += 1;

        SubscriptionToken(new_token)
    }

    fn tail(&self) -> Option<usize> {
        self.subscriptions
            .values()
            .min()
            .map(|sub| sub.borrow().position.unwrap_or(0))
    }

    fn increase_capacity(&mut self, tail: usize) -> usize {
        let mut new_buffer: Vec<E> = Vec::with_capacity(self.buffer.capacity() + STREAM_SIZE_BLOCK);

        for e in self.buffer.drain(tail..) {
            new_buffer.push(e);
        }

        for e in self.buffer.drain(0..tail) {
            new_buffer.push(e);
        }

        for s in self.subscriptions.values_mut() {
            let mut sub = s.borrow_mut();

            if let Some(pos) = sub.position {
                if sub.position > self.head {
                    sub.position = Some(pos - tail);
                } else {
                    sub.position = Some(pos + tail);
                }
            }
        }

        let new_head = new_buffer.len();
        self.head = Some(new_head);
        self.buffer = new_buffer;

        new_head
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct SubscriptionToken(u64);

#[derive(Debug, Eq, PartialEq)]
pub struct Subscription {
    pub position: Option<usize>,
}

impl Ord for Subscription {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position.cmp(&other.position)
    }
}

impl PartialOrd for Subscription {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
