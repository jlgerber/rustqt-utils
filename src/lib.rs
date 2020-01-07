use log;
use qt_core::{q_io_device::OpenModeFlag, QFile, QFlags, QSize, QString, QTextStream};
use qt_widgets::{
    cpp_core::{CppBox, MutPtr, Ref as QRef, StaticUpcast},
    QDesktopWidget, QHBoxLayout, QMainWindow, QVBoxLayout, QWidget,
};

/// Given an input of &str or String, return a boxed QString
///
/// # Arguments
/// * `input` - A type that implements AsRef<str>
///
/// # Returns
/// * An owned, CppBox wrapped QString
pub fn qs<S: AsRef<str>>(input: S) -> CppBox<QString> {
    QString::from_std_str(input.as_ref())
}

/// Provided a MutPtr of a widget that implements StaticUpcast<QWidget> (and all
/// types that inherit from QWidget do), load the stylesheet
///
/// # Argument
/// * `sheet` - Path to stylesheet, as a &str
/// * `widget` - A MutPtr wrpping a type which implements StaticUpcast<QWidget>
///
/// # Returns
/// * bool indicating success or failure
pub fn load_stylesheet<T>(sheet: &str, widget: MutPtr<T>) -> bool
where
    T: StaticUpcast<QWidget>,
{
    unsafe {
        let mut file = QFile::from_q_string(&QString::from_std_str(sheet));
        if file.open_1a(QFlags::from(OpenModeFlag::ReadOnly)) {
            let mut text_stream = QTextStream::new();
            text_stream.set_device(file.as_mut_ptr());
            let stylesheet = text_stream.read_all();
            T::static_upcast_mut(widget).set_style_sheet(stylesheet.as_ref());
            true
        } else {
            log::warn!("stylesheet not found");
            false
        }
    }
}

/// Resize the window to some scale of the current screen.
///
/// # Arguments
/// * `main_window`: The main window of the gui application
/// * `scale`: A scale factor applied to the full size of the main screen in
/// order to arrive at the requested size
///
/// # Returns
/// * None
pub fn resize_window_to_screen(main_window: &mut MutPtr<QMainWindow>, scale: f32) {
    unsafe {
        let desktop = QDesktopWidget::new();
        let screen_size = desktop.available_geometry();
        let new_size = QSize::new_2a(
            (screen_size.width() as f32 * scale) as i32,
            (screen_size.height() as f32 * scale) as i32,
        );
        main_window.set_geometry_4a(
            ((screen_size.width() - new_size.width()) as f32 / 2.0) as i32,
            ((screen_size.height() - new_size.height()) as f32 / 2.0) as i32,
            new_size.width(),
            new_size.height(),
        );
    }
}

/// Create a QVBoxLayout with zero margin, contents, and spacing
///
/// # Arguments
/// * None
///
/// # Returns
/// * CppBox<QVBoxLayout>
pub fn create_vlayout() -> CppBox<QVBoxLayout> {
    unsafe {
        let mut pc_vlayout = QVBoxLayout::new_0a();
        pc_vlayout.set_margin(0);
        pc_vlayout.set_contents_margins_4a(0, 0, 0, 0);
        pc_vlayout.set_spacing(0);
        pc_vlayout
    }
}

/// Create a QHBoxLayout with zero margin, contents margin, and spacing
///
/// # Arguments
/// * None
///
/// # Returns
/// * CppBox<QHBoxLayout>
pub fn create_hlayout() -> CppBox<QHBoxLayout> {
    unsafe {
        let mut pc_hlayout = QHBoxLayout::new_0a();
        pc_hlayout.set_margin(0);
        pc_hlayout.set_contents_margins_4a(0, 0, 0, 0);
        pc_hlayout.set_spacing(0);
        pc_hlayout
    }
}

/// Produce an owned CppBoxed QString from self
pub trait ToQString {
    fn to_qstring(&self) -> CppBox<QString>;
}

/// Convert to Self from a QString reference
pub trait FromQString {
    fn from_qstring(input: QRef<QString>) -> Self;
}