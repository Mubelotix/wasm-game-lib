#[derive(Debug)]
pub enum JoystickEvent {
    JoystickConnected(),
    JoystickDisonnected(),
    JoystickMove(),
    JoystickButton()
}