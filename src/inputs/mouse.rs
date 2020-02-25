pub enum MouseEvent {
    Click(u32, u32),
    RightClick(u32, u32),
    DoubleClick(u32, u32),
    Move(u32, u32),
    Enter(u32, u32),
    Leave(u32, u32),
    Up(u32, u32),
    Down(u32, u32)
}
