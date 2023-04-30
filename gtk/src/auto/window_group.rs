// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Widget, Window};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkWindowGroup")]
    pub struct WindowGroup(Object<ffi::GtkWindowGroup, ffi::GtkWindowGroupClass>);

    match fn {
        type_ => || ffi::gtk_window_group_get_type(),
    }
}

impl WindowGroup {
    pub const NONE: Option<&'static WindowGroup> = None;

    #[doc(alias = "gtk_window_group_new")]
    pub fn new() -> WindowGroup {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_window_group_new()) }
    }
}

impl Default for WindowGroup {
    fn default() -> Self {
        Self::new()
    }
}

pub trait WindowGroupExt: IsA<WindowGroup> + 'static {
    #[doc(alias = "gtk_window_group_add_window")]
    fn add_window(&self, window: &impl IsA<Window>) {
        unsafe {
            ffi::gtk_window_group_add_window(
                self.as_ref().to_glib_none().0,
                window.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_window_group_get_current_device_grab")]
    #[doc(alias = "get_current_device_grab")]
    fn current_device_grab(&self, device: &gdk::Device) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_window_group_get_current_device_grab(
                self.as_ref().to_glib_none().0,
                device.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_window_group_get_current_grab")]
    #[doc(alias = "get_current_grab")]
    fn current_grab(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_window_group_get_current_grab(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_window_group_list_windows")]
    fn list_windows(&self) -> Vec<Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_window_group_list_windows(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_window_group_remove_window")]
    fn remove_window(&self, window: &impl IsA<Window>) {
        unsafe {
            ffi::gtk_window_group_remove_window(
                self.as_ref().to_glib_none().0,
                window.as_ref().to_glib_none().0,
            );
        }
    }
}

impl<O: IsA<WindowGroup>> WindowGroupExt for O {}

impl fmt::Display for WindowGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WindowGroup")
    }
}
