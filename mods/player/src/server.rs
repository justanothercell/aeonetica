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
        let eid = engine.new_entity();
        let handler = engine.mut_entity(&eid).unwrap();
        handler.add_module(PlayerHandler { players: Default::default() });
        handler.add_module(ConnectionListener::new(
            |id, engine, client| {
                let pid = engine.new_entity();
                let ph: &mut PlayerHandler = engine.mut_module_of(id).unwrap();
                // adding player to list of players
                ph.players.insert(*client, pid);
                let players = ph.players.keys().cloned().collect::<Vec<_>>();

                // creating self player
                let player = engine.mut_entity(&pid).unwrap();
                player.add_module(Messenger::new::<PlayerHandle>());
                let messenger: &mut Messenger = player.mut_module().unwrap();
                messenger.register_receiver(Player::client_position_update);
                // register this player for all players
                for pid in &players {
                    messenger.add_client(*pid);
                }
                // tell this player that they may control themselves
                messenger.call_client_fn_for(PlayerHandle::set_controlling, client, true, SendMode::Safe);
                // register all other players for this player
                for pid in &players {
                    if pid == client { continue }
                    engine.mut_module_of::<Messenger>(pid).unwrap().add_client(*client);
                }
            },
            |_id, _engine, client| {

        }));
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
        log!("ping ponging position of client {client_id}");
        let player = engine.mut_entity(id).unwrap();
        player.mut_module::<Player>().unwrap().position = position;
        player.mut_module::<Messenger>().unwrap().call_client_fn(PlayerHandle::receive_position, position,SendMode::Quick);
    }
}

impl Module for Player {
    fn start(id: &EntityId, engine: &mut Engine) where Self: Sized {

    }
}