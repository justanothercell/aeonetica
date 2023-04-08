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

###### As with the previous tutorial, replace ExampleMod with your actual Mod name

## 1. Create a Module
Create a Module which we will later attach to an entity
```rs
use aeonetica_server::ecs::module::Module;

pub struct MyModule {
    counter: usize
}

impl Module for MyModule {
    
}
```

## 2. Create an entity with the example Module
Implement the `start` method of ExampleModServer. 

This differs from `init`, 
since you now have access to `Engine`. Use `init` to load and setup resources and use `start`
to initiate all `Engine` based stuff.

First of all we create a new entity. Then we use its id to obtain a mutable 
reference to it and add our custom Module.

```rs
use aeonetica_server::ecs::Engine;

impl ServerMod for TestModServer {
    
    ...
    
    fn start(&mut self, engine: &mut Engine) {
        log!("server test start");
        let id = engine.new_entity();
        engine.mut_entity(&id).unwrap().add_module(MyModule { counter: 0 });
    }
}
```

## 3. Module Functionality
```rs
use aeonetica_server::ecs::Engine;
use aeonetica_engine::{EntityId, log};

impl Module for MyModule {
    
    ...
    
    fn tick(id: &EntityId, engine: &mut Engine) where Self: Sized {
        let m: &mut MyModule = engine.mut_module_of(id).unwrap();
        m.counter += 1;
        if m.counter % 20 == 0 {
            log!("one second has passed");
        }
    }
}
```

## 4. Run
Recompile the mod with the `--deploy` argument (see last steps in [hello world](hello_world.md))
amd start the first server and then the client. The log messages should appear in the server console