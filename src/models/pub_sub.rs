use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct PubSub<S: Clone + Eq + Hash, T: Clone + Eq + Hash> {
    subscribers: HashMap<S, HashSet<T>>,
    topics: HashMap<T, HashSet<S>>,
}

impl<S: Clone + Eq + Hash, T: Clone + Eq + Hash> PubSub<S, T> {
    pub fn new() -> Self {
        PubSub {
            subscribers: HashMap::new(),
            topics: HashMap::new(),
        }
    }

    pub fn get_subs(&self, topic: &T) -> Vec<&S> {
        if let Some(s) = self.topics.get(topic) {
            s.iter().collect()
        } else {
            Vec::new()
        }
    }

    pub fn sub(&mut self, subscriber: &S, topic: &T) {
        if let Some(t) = self.subscribers.get_mut(&subscriber) {
            t.insert(topic.clone());
        } else {
            let mut t = HashSet::new();
            t.insert(topic.clone());
            self.subscribers.insert(subscriber.clone(), t);
        }

        if let Some(s) = self.topics.get_mut(&topic) {
            s.insert(subscriber.clone());
        } else {
            let mut s = HashSet::new();
            s.insert(subscriber.clone());
            self.topics.insert(topic.clone(), s);
        }
    }

    pub fn unsub(&mut self, subscriber: &S, topic: &T) {
        if let Some(t) = self.subscribers.get_mut(subscriber) {
            t.remove(topic);
        }

        if let Some(s) = self.topics.get_mut(topic) {
            s.remove(subscriber);
        }
    }

    pub fn unsub_all(&mut self, subscriber: &S) {
        if let Some(t) = self.subscribers.get_mut(subscriber) {
            for topic in t.iter() {
                if let Some(s) = self.topics.get_mut(topic) {
                    s.remove(subscriber);
                }
            }
            self.subscribers.remove(subscriber);
        }
    }
}
