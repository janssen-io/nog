use super::{Component, ComponentText};

pub fn create() -> Component {
    Component::new("CurrentWindow", |ctx| {
        Ok(vec![ComponentText::new().with_display_text(
            ctx.state
                .get_display_by_id(ctx.display.id)
                .and_then(|d| d.get_focused_grid())
                .and_then(|g| g.get_focused_window())
                .map(|w| w.title.clone())
                .unwrap_or("".into()),
        )])
    })
}