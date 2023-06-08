use std::cmp::Ordering;
use std::collections::{BinaryHeap};
use std::collections::hash_map::Entry;
use std::marker::PhantomData;
use std::ops::{Generator, GeneratorState};
use aeonetica_engine::TypeId;
use aeonetica_engine::util::id_map::IdMap;
use aeonetica_engine::util::type_to_id;
use crate::ecs::Engine;

pub trait TaskFunc = for<'a> Generator<&'a mut Engine, Yield = Yielder<'a>, Return = ()>;

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
    fn eq(&self, other: &Self) -> bool { self.timestamp == other.timestamp }
}

pub trait Event {}

impl Eq for Task {}

#[derive(Default)]
pub(crate) struct TaskQueue {
    pub(crate) heap: BinaryHeap<Task>,
    pub(crate) event_queue: IdMap<Vec<Box<dyn TaskFunc>>>
}

pub type EventId = TypeId;

pub struct PrivateWaiter;

pub enum WaitFor {
    Ticks(usize, PrivateWaiter),
    Event(EventId, PrivateWaiter)
}

impl WaitFor {
    pub fn ticks(ticks: usize) -> Self {
        WaitFor::Ticks(ticks, PrivateWaiter)
    }
    
    pub fn event<T: Event>() -> Self {
        WaitFor::Event(type_to_id::<T>(), PrivateWaiter)
    }
}

pub struct Yielder<'a>(PrivateYielder, PhantomData<&'a ()>, WaitFor);

struct PrivateYielder;

#[macro_export]
macro_rules! yield_task {
    ($e: ident, $wait: expr) => {
        $e = yield $e.yield_fn($wait)
    };
}

impl Engine {
    pub fn yield_fn(&mut self, waiter: WaitFor) -> Yielder<'_> {
        Yielder(PrivateYielder, PhantomData, waiter)
    }

    pub fn queue_task<'a>(&mut self, task: impl Generator<&'a mut Engine, Yield = Yielder<'a>, Return = ()> + 'static) {
        let taskfn: Box<dyn Generator<&'a mut Engine, Yield = Yielder<'a>, Return = ()>> = Box::new(*Box::new(task));
        self.tasks.heap.push(Task {
            timestamp: self.tick,
            func: unsafe { std::mem::transmute::<_, _>(taskfn) }
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
        if let Some(q) = self.tasks.event_queue.remove(id) {
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

    pub(crate) fn run_task(&mut self, f: Box<dyn TaskFunc>) {
        let mut fnpin = Box::into_pin(f);
        match fnpin.as_mut().resume(self) {
            GeneratorState::Yielded(yielder) => match yielder.2 {
                WaitFor::Ticks(t, _) => self.tasks.heap.push(Task {
                    timestamp: { self.tick + t },
                    func: Box::from(fnpin),
                }),
                WaitFor::Event(event, _) => {
                    if let Entry::Vacant(e) = self.tasks.event_queue.entry(event) {
                        e.insert(vec![Box::from(fnpin)]);
                    } else {
                        self.tasks.event_queue.get_mut(&event).unwrap().push(Box::from(fnpin));
                    }
                }
            }
            GeneratorState::Complete(_) => (),
        }
    }
}
