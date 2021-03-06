use crate::{
    bar::item_section::ItemSection, keybindings::keybinding::Keybinding, popup::Popup,
    system::DisplayId, win_event_handler::win_event::WinEvent,
};
use crossbeam_channel::unbounded;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use interpreter::RuntimeError;

#[derive(Debug, Clone)]
pub enum Event {
    Keybinding(Keybinding),
    WinEvent(WinEvent),
    NewPopup(Popup),
    ConfigError(RuntimeError),
    CallCallback {
        idx: usize,
        /// This is required, because the callbacks run in a seperate thread and mode callbacks
        /// have to notify the keybindings manager that they finished executing so it can register
        /// all of the mode specific bindings
        is_mode_callback: bool,
    },
    ToggleAppbar(DisplayId),
    UpdateBarSections(DisplayId, ItemSection, ItemSection, ItemSection),
    ChangeWorkspace(i32, bool),
    RedrawAppBar,
    ReloadConfig,
    Exit,
}

pub type EventSender = Sender<Event>;
pub type EventReceiver = Receiver<Event>;

#[derive(Debug, Clone)]
pub struct EventChannel {
    pub sender: EventSender,
    pub receiver: EventReceiver,
}

impl Default for EventChannel {
    fn default() -> Self {
        let (sender, receiver) = unbounded();

        Self { sender, receiver }
    }
}
