use crate::system::NativeWindow;

#[derive(Debug, Clone, Copy)]
pub enum TaskbarPosition {
    Top,
    Bottom,
    Left,
    Right,
    Hidden,
}

impl Default for TaskbarPosition {
    fn default() -> Self {
        TaskbarPosition::Bottom
    }
}

#[derive(Debug, Clone)]
pub struct Taskbar {
    pub window: NativeWindow,
    position: TaskbarPosition,
}

impl Taskbar {
    pub fn new(window: NativeWindow) -> Self {
        Taskbar {
            window,
            position: TaskbarPosition::default(),
        }
    }

    pub fn get_position(&self) -> TaskbarPosition {
        let tb_rect = self
            .window
            .get_rect()
            .expect("Failed to get rect of taskbar window");

        let display_rect = self
            .window
            .get_display()
            .expect("Failed to get display of taskbar")
            .rect;

        if self.window.is_hidden() {
            TaskbarPosition::Hidden
        } else if tb_rect.left == display_rect.left
            && tb_rect.top == display_rect.top
            && tb_rect.bottom == display_rect.bottom
        {
            TaskbarPosition::Left
        } else if tb_rect.right == display_rect.right
            && tb_rect.top == display_rect.top
            && tb_rect.bottom == display_rect.bottom
        {
            TaskbarPosition::Right
        } else if tb_rect.left == display_rect.left
            && tb_rect.top == display_rect.top
            && tb_rect.right == display_rect.right
        {
            TaskbarPosition::Top
        } else {
            TaskbarPosition::Bottom
        }
    }
}

// pub fn show_taskbars(state: &mut AppState) {
//     for tb in state.get_taskbars() {
//         info!("Showing taskbar {:?}", tb);
//         tb.window.show();
//     }

//     // update_task_bars();
// }
// pub fn hide_taskbars(state: &mut AppState) {
//     for tb in state.get_taskbars() {
//         info!("Hiding taskbar {:?}", tb);
//         tb.window.hide();
//     }

//     // update_task_bars();
// }

// TODO: Find out whether this is still needed
// pub fn update_task_bars() {
//     let taskbars = api::get_taskbars();
//     let multi_monitor = CONFIG.lock().multi_monitor;

//     for mut tb in taskbars {
//         let display = tb
//             .window
//             .get_display()
//             .expect("Failed to get display of taskbar");

//         if (!multi_monitor && display.is_primary()) || multi_monitor {
//             debug!("Initialized {:?})", tb);
//             tb.position = tb.get_position();
//             with_display_by(
//                 |d| d.id == display.id,
//                 |d| d.unwrap().taskbar = Some(tb.clone()),
//             );
//             if multi_monitor {
//                 break;
//             }
//         }
//     }
// }

// fn get_taskbar_position(rect: RECT, hwnd: HWND, hmonitor: i32) -> TaskBarPosition {
//     let is_window_visible = unsafe { IsWindowVisible(hwnd) == 1 };

//     if !is_window_visible {
//         TaskBarPosition::Hidden
//     } else if rect.left == left && rect.top == top && rect.bottom == bottom {
//         TaskBarPosition::Left
//     } else if rect.right == right && rect.top == top && rect.bottom == bottom {
//         TaskBarPosition::Right
//     } else if rect.left == left && rect.top == top && rect.right == right {
//         TaskBarPosition::Top
//     } else {
//         TaskBarPosition::Bottom
//     }
// }
