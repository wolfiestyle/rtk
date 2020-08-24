use rtk::toplevel::WindowAttributes;
use winit::dpi::PhysicalSize;
use winit::window::WindowBuilder;

pub fn make_win_builder(win_attr: &WindowAttributes) -> WindowBuilder {
    let size = win_attr.size.nonzero_or(rtk::DEFAULT_WINDOW_SIZE);
    let mut win_builder = WindowBuilder::new()
        .with_title(win_attr.title.clone().unwrap_or_else(|| "Window".into()))
        .with_inner_size(PhysicalSize::new(size.w, size.h))
        .with_resizable(win_attr.resizable)
        .with_maximized(win_attr.maximized)
        .with_transparent(win_attr.transparent)
        .with_always_on_top(win_attr.always_on_top)
        .with_decorations(win_attr.decorations);
    if let Some(size) = win_attr.min_size.get_nonzero() {
        win_builder = win_builder.with_min_inner_size(PhysicalSize::new(size.w, size.h));
    }
    if let Some(size) = win_attr.max_size.get_nonzero() {
        win_builder = win_builder.with_max_inner_size(PhysicalSize::new(size.w, size.h));
    }

    win_builder
}
