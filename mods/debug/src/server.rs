use aeonetica_server::ServerMod;


pub struct DebugModServer {

}

impl DebugModServer {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl ServerMod for DebugModServer {
	
}