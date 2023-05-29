use aeonetica_engine::{math::vector::Vector2, EntityId, log, networking::SendMode};
use aeonetica_server::{ServerMod, ecs::{module::Module, Engine, messaging::Messenger}};
use player_mod::server::{PLAYER_HANDLER, PlayerHandler, Player};

use crate::client::WormHandle;


pub struct WormsModServer {

}

impl WormsModServer {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl ServerMod for WormsModServer {
    fn start(&mut self, engine: &mut Engine) {
        Worm::create(engine);
    }
}

pub(crate) struct Worm {
    ppos: Vector2<f32>,
    segments: Vec<Vector2<f32>>
}

impl Worm {
    fn create(engine: &mut Engine) -> EntityId {
        let eid = engine.new_entity();
        
        let mut entity = engine.mut_entity(&eid);
        entity.add_module(Worm::new(Vector2::new(-10.0, 0.0), Vector2::new(0.0, 1.0), 10));
        entity.add_module(Messenger::new::<WormHandle>());
        eid
    }

    fn new(pos: Vector2<f32>, dir: Vector2<f32>, segs: usize) -> Self {
        Self {
            ppos: Default::default(),
            segments: {
                let dir = dir.normalized();
                let mut segments = vec![];
                for i in 0..segs {
                    segments.push(pos + dir * 0.8 * i as f32);
                }
                segments
            }
        }
    }
}

impl Module for Worm {
    fn tick(id: &EntityId, engine: &mut Engine) {
        let prcrc = engine.mut_module_by_tag::<PlayerHandler>(PLAYER_HANDLER).players.clone();
        let players = prcrc.borrow_mut();
        let worm = engine.get_module_of::<Worm>(id);
        let self_pos = worm.segments[0];
        let ppos = worm.ppos;
        for (pid, epid) in players.iter() {
            let pos = engine.get_module_of::<Player>(epid).position;
            if (pos - self_pos).mag_sq() > 24.0*24.0 {
                engine.mut_module_of::<Messenger>(id).remove_client(pid);

            } else {
                if engine.mut_module_of::<Messenger>(id).add_client(*pid) {
                    println!("ADDDD");
                    let (mut messenger, worm) = engine.two_mut_modules_of::<Messenger, Worm>(id);
                    messenger.call_client_fn(WormHandle::receive_position, (worm.segments.clone(), true), SendMode::Safe);
                }
            }
        }
        if (ppos - self_pos).mag_sq() > -0.02 {
            let (mut messenger, mut worm) = engine.two_mut_modules_of::<Messenger, Worm>(id);
            worm.ppos = self_pos;
            messenger.call_client_fn(WormHandle::receive_position, (worm.segments.clone(), false), SendMode::Quick);
            //println!("sent!");
        }
    }
}