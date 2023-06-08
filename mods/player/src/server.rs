use std::cell::RefCell;
use std::rc::Rc;
use aeonetica_engine::{ClientId, EntityId, log};
use aeonetica_engine::networking::SendMode;
use aeonetica_engine::util::id_map::IdMap;
use aeonetica_engine::math::vector::Vector2;
use aeonetica_server::ecs::Engine;
use aeonetica_server::ecs::events::ConnectionListener;
use aeonetica_server::ecs::messaging::Messenger;
use aeonetica_server::ecs::module::Module;
use aeonetica_server::ServerMod;
use crate::client::PlayerHandle;

pub const PLAYER_HANDLER: &str = "PLAYER_HANDLER";

pub struct PlayerModServer {

}

impl ServerMod for PlayerModServer {
    fn start(&mut self, engine: &mut Engine) {
        log!("starting player mod server...");
        let eid = engine.new_entity();
        engine.tag_entity(eid, PLAYER_HANDLER);
        let handler = engine.mut_entity(&eid).unwrap();
        handler.add_module(PlayerHandler { players: Default::default() });
        handler.add_module(ConnectionListener::new(
            |id, engine, client| {
                let pid = engine.new_entity();
                {
                    // creating self player
                    let mut player = engine.mut_entity(&pid);
                    player.add_module(Messenger::new::<PlayerHandle>());
                    player.add_module(Player { position: Vector2::new(3.0, 0.0) });
                }
                let prcrc = engine.get_module_of::<PlayerHandler>(id).players.clone();
                let mut players = prcrc.borrow_mut();
                // adding player to list of players
                players.insert(*client, pid);
                let players_positions = players.iter().map(|(k, v)| {
                    (k, v, engine.mut_module_of::<Player>(v).position)
                }).collect::<Vec<_>>();
                {
                    let mut player = engine.mut_entity(&pid);
                    let position = player.mut_module::<Player>().position;

                    let mut messenger = player.mut_module::<Messenger>();
                    messenger.register_receiver(Player::client_position_update);

                    // register this player for all players
                    for (pid, ..) in &players_positions {
                        messenger.add_client(**pid);
                    }
                    // tell this player that they may control themselves
                    messenger.call_client_fn_for(PlayerHandle::set_controlling, client, true, SendMode::Safe);
                    messenger.call_client_fn(PlayerHandle::receive_position, (position, true), SendMode::Safe);
                }
                // register all other players for this player
                for (pid, eid, position) in &players_positions {
                    if *pid == client { continue }
                    let mut messenger = engine.mut_module_of::<Messenger>(eid);
                    messenger.add_client(*client);
                    messenger.call_client_fn_for(PlayerHandle::receive_position, client, (*position, true), SendMode::Safe);
                }
                log!("set up client ons server side");
            }, |id, engine, client| {
                // get list of players and entity id of client player
                let prcrc = engine.get_module_of::<PlayerHandler>(id).players.clone();
                let mut players = prcrc.borrow_mut();
                let eid = players.remove(client).unwrap();
				// unregister this player for all other players
                let messenger: &mut Messenger = &mut engine.mut_module_of(&eid);
				for (pid, _eid) in players.iter() {
                    messenger.remove_client(pid);
                }
                // unregister all other players from this player
                for (pid, eid) in players.iter() {
                    if pid == client { continue }
                    engine.mut_module_of::<Messenger>(eid).remove_client(client);
                }
                log!("removed client ons server side");
        }));
        log!("player handler all set up");
    }
}

pub struct PlayerHandler {
    /// key: client_id, value: entity_id
    pub players: Rc<RefCell<IdMap<EntityId>>>
}

impl Module for PlayerHandler {

}

pub struct Player {
    pub position: Vector2<f32>
}

impl Player {
    pub(crate) fn client_position_update(id: &EntityId, engine: &mut Engine, _client_id: &ClientId, (position, teleporting): (Vector2<f32>, bool)) {
        let mut player = engine.mut_entity(id);
        player.mut_module::<Player>().position = position;
        player.mut_module::<Messenger>().call_client_fn(PlayerHandle::receive_position, (position, teleporting), SendMode::Safe);
    }
}

impl Module for Player {
    fn start(_id: &EntityId, _engine: &mut Engine) where Self: Sized {

    }
}