
trait IntoKeyLocation {
    fn location(&self) -> (u8, u8);
}

enum Key {
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Print,
    Pause,
    ScreenMode,
    A1,
    A2,
    A3,
    Mode,
    Tilda,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    Minus,
    Equals,
    Backspace,
    Insert,
    Home,
    PageUp,
    
    Tab,
    
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    LeftBracket,
    RightBracket,
    BackSlash,
    Delete,
    End,
    PageDown,

    CapsLock,

    LeftShift,
    RightShift,

    ISOLeft,
    ISORight,

    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    NumDelete,
    NumReturn,
    NumLock,
    NumDivide,
    NumMultiply,
    NumMinus,

    LeftControl,
    RightControl,

    LeftOS,
    RightOS,

    LeftAlt,
    RightAlt,

    Space,

    Function,

    Semicolon,
    ForwardSlash,
    Comma,
    Backtick,
    Period,
}

impl IntoKeyLocation for Key {
    fn location(&self) -> (u8, u8) {
        match *self {
            MyEnum::A => 123,
            MyEnum::B => 456,
        }
    }
}