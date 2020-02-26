#[derive(Debug)]
/// A joystick event.
/// Joysticks are not supported for now
pub enum JoystickEvent {
    /// A joystick has been connected
    JoystickConnected(),
    /// A joystick has been disconnected
    JoystickDisonnected(),
    /// A joystick moved
    JoystickMove(),
    /// A button has been pressed
    JoystickButton()
}