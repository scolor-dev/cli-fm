#[derive(Clone, Copy, Debug)]
pub enum Action {
    Up,
    Down,
    Enter,
    Back,
    Quit,
    EnterCommandMode,
    EnterSearchMode,
    EnterPathMode,
    EnterNormalMode,
    InputChar(char),
    InputBackspace,
    InputSubmit,
    TabComplete,
    None,
}
