extern crate gtk;
extern crate gtk_sys;
extern crate gdk_sys;
extern crate glib;

use gtk::prelude::*;

use gtk::{Window, WindowType, EventBox, Label};

use glib::translate::ToGlibPtr;

// WTF
#[repr(C)]
pub struct StaticCString(*const u8);
unsafe impl Sync for StaticCString {}

#[no_mangle]
pub static CONST_C_STR: StaticCString =
StaticCString(b"text/plain\0" as *const u8);

static mut DND_TARGET_ENTRY: gtk_sys::GtkTargetEntry =
gtk_sys::GtkTargetEntry {
    target: StaticCString as *mut i8,
    flags: 0,
    info: 0,
};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);

    let cover_event_box = EventBox::new();
    window.add(&cover_event_box);
    unsafe {
        // this compiles but doesn't work
        gtk_sys::gtk_drag_dest_set(cover_event_box.to_glib_none().0,
                                   gtk_sys::GTK_DEST_DEFAULT_ALL,
                                   &mut DND_TARGET_ENTRY,
                                   1,
                                   gdk_sys::GDK_ACTION_COPY);
    };
    cover_event_box.connect_drag_data_received(|_, _, _, _, _, _, _| {
        println!("yay!");
    });

    let label = Label::new(Some("test"));

    cover_event_box.add(&label);

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
