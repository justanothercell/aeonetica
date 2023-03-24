use std::collections::{HashMap};
use aeonetica_engine::{ClientId, EntityId, Id};
use aeonetica_engine::networking::SendMode;
use aeonetica_server::ecs::Engine;
use aeonetica_server::ecs::events::ConnectionListener;
use aeonetica_server::ecs::messaging::Messenger;
use aeonetica_server::ecs::module::Module;
use aeonetica_server::ServerMod;
use crate::client::PlayerHandle;

pub(crate) struct PlayerModServer {

}

impl ServerMod for PlayerModServer {
    fn start(&mut self, engine: &mut Engine) {
        let pm = engine.new_entity();
        engine.tag_entity(pm, PLAYER_MANAGER.to_string());
        engine.mut_entity(&pm).unwrap().add_module(PlayerManager {
            players: Default::default()
        });
    }
}

const PLAYER_MANAGER: &str = "PLAYER_MANAGER";

struct PlayerManager {
    players: HashMap<ClientId, EntityId>
}

impl Module for PlayerManager {
    fn start(id: &Id, engine: &mut Engine) where Self: Sized {
        engine.mut_entity(id).unwrap().add_module(ConnectionListener::new(|id, engine, client| {
            let pm = engine.mut_module_of::<Self>(id).unwrap();
            let player = world.new_entity();
            pm.players.insert(*client, player);
            engine.mut_entity(&player).unwrap().add_module(Player {
                client_id: *client,
                pos: (0.0, 0.0)
            });
        }, |id, engine, client| {
            let pm = engine.mut_module_of::<Self>(id).unwrap();
            let eid = pm.players.remove(client).unwrap();
            engine.remove_entity(&eid);
        }));
    }
}

struct Player {
    client_id: ClientId,
    pos: (f32, f32)
}

impl Player {
    fn position_update(id: &EntityId, engine: &mut Engine, pos: (f32, f32)) {
        let player = engine.mut_module_of::<Self>(id).unwrap();
        player.pos = pos;
        let messenger = engine.mut_module_of::<Messenger>(id).unwrap();
        messenger.call_client_fn(PlayerHandle::server_position_update, pose, SendMode::Quick);
    }
}

impl Module for Player {
    fn start(id: &Id, engine: &mut Engine) where Self: Sized {
        let mut messenger = Messenger::new();
        messenger.register_receiver(Player::position_update);

        // spawn for every player that already exists
        for player in &engine.get_module_by_tag::<PlayerManager>(PLAYER_MANAGER).unwrap().players {
            messenger.add_client(*player.0);
        }

        // make this the owning player's controller
        messenger.call_client_fn_for(PlayerHandle::set_owning, &engine.get_module_of::<Self>(id).unwrap().client_id, true, SendMode::Safe);

        engine.get_entity(id).unwrap().add_module(ConnectionListener::new(|id, engine, client| {
            let m = engine.mut_module_of::<Messenger>(id).unwrap();
            m.add_client(*client);
        }, |id, engine, client| {
            let m = engine.mut_module_of::<Messenger>(id).unwrap();
            m.remove_client(client);
        }));
    }
}