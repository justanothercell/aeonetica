#[derive(Copy, Clone, Debug)]
pub struct Time {
	/// The total time since startup in seconds
	pub time: f32,
	/// The time since last frame (client) or last tick (server) in seconds. 
	/// Capped to 0.05s (equivalent of 20fps) on client and 0.2s on server (equivalent of 5tps) 
	to avoid weird interpolation behavior when tabbed out of the game previously, when hitting a lag spike,
	/// or when the game was paused due to any other reasons.
	pub delta: f32,
	/// The uncapped delta time. only use if you know what you are doing and delta is not sufficient.
	pub raw_delta: f32
}