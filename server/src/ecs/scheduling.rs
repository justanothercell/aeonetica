use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::collections::hash_map::Entry;
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;
use aeonetica_engine::Id;
use aeonetica_engine::util::type_to_id;
use crate::ecs::Engine;

pub trait TaskFunc = Generator<Yield = WaitFor, Return = ()>;

pub(crate) struct Task {
    timestamp: usize,
    func: Box<dyn TaskFunc>,
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other).reverse()) }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering { self.timestamp.cmp(&other.timestamp) }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

pub trait Event {}

impl Eq for Task {}

#[derive(Default)]
pub(crate) struct TaskQueue {
    pub(crate) heap: BinaryHeap<Task>,
    pub(crate) event_queue: HashMap<EventId, Vec<Box<dyn TaskFunc>>>
}

pub type EventId = Id;

pub enum WaitFor {
    Ticks(usize),
    Event(EventId)
}

impl Engine {
    pub fn queue_task(&mut self, task: impl TaskFunc + 'static) {
        self.tasks.heap.push(Task {
            timestamp: self.tick,
            func: Box::new(task),
        });
    }

    pub fn fire_event<E: Event>(&mut self) {
        if let Some(q) = self.tasks.event_queue.remove(&type_to_id::<E>()) {
            for task in q {
                self.run_task(task);
            }
        }
    }

    pub fn fire_raw_event(&mut self, id: &EventId) {
        if let Some(q) = self.tasks.event_queue.remove(&id) {
            for task in q {
                self.run_task(task);
            }
        }
    }

    pub(crate) fn run_tasks(&mut self) {
        while self.tasks.heap.peek().map(|t| t.timestamp <= self.tick).unwrap_or(false) {
            let task = self.tasks.heap.pop().unwrap();
            self.run_task(task.func);
        }
    }

    pub(crate) fn run_task(&mut self, mut f: Box<dyn TaskFunc>) {
        let fnpin: Pin<&mut dyn TaskFunc> = unsafe { Pin::new_unchecked(&mut *f) };
        match fnpin.resume(()) {
            GeneratorState::Yielded(delay) => match delay {
                WaitFor::Ticks(t) => self.tasks.heap.push(Task {
                    timestamp: self.tick + t,
                    func: f,
                }),
                WaitFor::Event(event) => {
                    if let Entry::Vacant(e) = self.tasks.event_queue.entry(event) {
                        e.insert(vec![f]);
                    } else {
                        self.tasks.event_queue.get_mut(&event).unwrap().push(f);
                    }
                }
            }
            GeneratorState::Complete(_) => (),
        }
    }
}