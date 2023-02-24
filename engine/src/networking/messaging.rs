pub trait ClientHandle {
    fn send_data(&mut self) -> Vec<u8> { vec![] }
    #[allow(unused_variables)]
    fn receive_data(&mut self, data: &Vec<u8>) {}
}