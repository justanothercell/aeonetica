# Server Side Entity Module System (EMS)

###### [Back to Home](README.md)

The EMS consists of Entities, which may have one of each type of Module.
The modules must be unique per entity, but may be generically differentiated.

Each Module has event functions, such as `start` or `update`, in which the core game
logic takes place.

To satisfy the borrow checker, this is implemented in a way that these event functions
do not immediately have access to `&mut self`. Rather, they receive a mutable reference
to the `Engine` and current `EntityId`, with which a (mut) borrow to the current `Module`
can be obtained.

The following example implementation takes the [hello world](hello_world.md) as a starting point.

## 1. 