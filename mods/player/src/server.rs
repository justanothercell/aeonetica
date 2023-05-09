use aeonetica_client::networking::messaging::ClientHandle;
use aeonetica_engine::{ClientId, EntityId, Id, log};
use aeonetica_engine::networking::SendMode;
use aeonetica_engine::util::id_map::IdMap;
use aeonetica_engine::util::vector::Vector2;
use aeonetica_server::ecs::Engine;
use aeonetica_server::ecs::events::ConnectionListener;
use aeonetica_server::ecs::messaging::Messenger;
use aeonetica_server::ecs::module::Module;
use aeonetica_server::ServerMod;
use crate::client::PlayerHandle;

pub struct PlayerModServer {

}

impl ServerMod for PlayerModServer {
    fn start(&mut self, engine: &mut Engine) {
        log!("starting player mod server...");
        let eid = engine.new_entity();
        let handler = engine.mut_entity(&eid).unwrap();
        handler.add_module(PlayerHandler { players: Default::default() });
        handler.add_module(ConnectionListener::new(
            |id, engine, client| {
                let pid = engine.new_entity();
                let ph: &mut PlayerHandler = engine.mut_module_of(id).unwrap();
                // adding player to list of players
                ph.players.insert(*client, pid);
                let players = ph.players.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>();

                // creating self player
                let player = engine.mut_entity(&pid).unwrap();
                player.add_module(Messenger::new::<PlayerHandle>());
                player.add_module(Player { position: Default::default() });
                let messenger: &mut Messenger = player.mut_module().unwrap();
                messenger.register_receiver(Player::client_position_update);

                // register this player for all players
                for (pid, _eid) in &players {
                    messenger.add_client(*pid);
                }
                // tell this player that they may control themselves
                messenger.call_client_fn_for(PlayerHandle::set_controlling, client, true, SendMode::Safe);
                // register all other players for this player
                for (pid, eid) in &players {
                    if pid == client { continue }
                    engine.mut_module_of::<Messenger>(eid).unwrap().add_client(*client);
                }
                log!("set up client ons server side");
            },
            |id, engine, client| {
                // get list of players and entity id of client player
                let players = engine.get_module_of::<PlayerHandler>(id).unwrap().players.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>();
                let eid = *engine.get_module_of::<PlayerHandler>(id).unwrap().players.get(client).unwrap();
				// unregister this player for all other players
                let messenger: &mut Messenger = engine.mut_module_of(&eid).unwrap();
				for (pid, _eid) in &players {
                    messenger.remove_client(pid);
                }
                // unregister all other players from this player
                for (pid, eid) in &players {
                    if pid == client { continue }
                    engine.mut_module_of::<Messenger>(eid).unwrap().remove_client(client);
                }
                log!("removed client ons server side");
        }));
        log!("player handler all set up");
    }
}

struct PlayerHandler {
    /// key: client_id, value: entity_id
    players: IdMap<Id>
}

impl Module for PlayerHandler {

}

pub(crate) struct Player {
    position: Vector2<f32>
}

impl Player {
    pub(crate) fn client_position_update(id: &EntityId, engine: &mut Engine, client_id: &ClientId, position: Vector2<f32>) {
        let player = engine.mut_entity(id).unwrap();
        player.mut_module::<Player>().unwrap().position = position;
        player.mut_module::<Messenger>().unwrap().call_client_fn(PlayerHandle::receive_position, position,SendMode::Safe);
    }
}

impl Module for Player {
    fn start(id: &EntityId, engine: &mut Engine) where Self: Sized {

    }
}