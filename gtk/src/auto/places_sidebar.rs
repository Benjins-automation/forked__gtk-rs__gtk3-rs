// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Adjustment, Align, Bin, Buildable, Container, CornerType, PlacesOpenFlags, PolicyType,
    ResizeMode, ScrolledWindow, ShadowType, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkPlacesSidebar")]
    pub struct PlacesSidebar(Object<ffi::GtkPlacesSidebar, ffi::GtkPlacesSidebarClass>) @extends ScrolledWindow, Bin, Container, Widget, @implements Buildable;

    match fn {
        type_ => || ffi::gtk_places_sidebar_get_type(),
    }
}

impl PlacesSidebar {
    #[doc(alias = "gtk_places_sidebar_new")]
    pub fn new() -> PlacesSidebar {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_places_sidebar_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`PlacesSidebar`] objects.
    ///
    /// This method returns an instance of [`PlacesSidebarBuilder`](crate::builders::PlacesSidebarBuilder) which can be used to create [`PlacesSidebar`] objects.
    pub fn builder() -> PlacesSidebarBuilder {
        PlacesSidebarBuilder::new()
    }

    #[doc(alias = "gtk_places_sidebar_add_shortcut")]
    pub fn add_shortcut(&self, location: &impl IsA<gio::File>) {
        unsafe {
            ffi::gtk_places_sidebar_add_shortcut(
                self.to_glib_none().0,
                location.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_places_sidebar_get_local_only")]
    #[doc(alias = "get_local_only")]
    pub fn is_local_only(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_local_only(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_places_sidebar_get_location")]
    #[doc(alias = "get_location")]
    pub fn location(&self) -> Option<gio::File> {
        unsafe { from_glib_full(ffi::gtk_places_sidebar_get_location(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_places_sidebar_get_nth_bookmark")]
    #[doc(alias = "get_nth_bookmark")]
    pub fn nth_bookmark(&self, n: i32) -> Option<gio::File> {
        unsafe {
            from_glib_full(ffi::gtk_places_sidebar_get_nth_bookmark(
                self.to_glib_none().0,
                n,
            ))
        }
    }

    #[doc(alias = "gtk_places_sidebar_get_open_flags")]
    #[doc(alias = "get_open_flags")]
    pub fn open_flags(&self) -> PlacesOpenFlags {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_open_flags(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_places_sidebar_get_show_desktop")]
    #[doc(alias = "get_show_desktop")]
    pub fn shows_desktop(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_show_desktop(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_places_sidebar_get_show_enter_location")]
    #[doc(alias = "get_show_enter_location")]
    pub fn shows_enter_location(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_show_enter_location(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_places_sidebar_get_show_other_locations")]
    #[doc(alias = "get_show_other_locations")]
    pub fn shows_other_locations(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_show_other_locations(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_places_sidebar_get_show_recent")]
    #[doc(alias = "get_show_recent")]
    pub fn shows_recent(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_show_recent(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_places_sidebar_get_show_starred_location")]
    #[doc(alias = "get_show_starred_location")]
    pub fn shows_starred_location(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_show_starred_location(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_places_sidebar_get_show_trash")]
    #[doc(alias = "get_show_trash")]
    pub fn shows_trash(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_show_trash(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_places_sidebar_list_shortcuts")]
    pub fn list_shortcuts(&self) -> Vec<gio::File> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_places_sidebar_list_shortcuts(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_places_sidebar_remove_shortcut")]
    pub fn remove_shortcut(&self, location: &impl IsA<gio::File>) {
        unsafe {
            ffi::gtk_places_sidebar_remove_shortcut(
                self.to_glib_none().0,
                location.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_places_sidebar_set_drop_targets_visible")]
    pub fn set_drop_targets_visible(&self, visible: bool, context: &gdk::DragContext) {
        unsafe {
            ffi::gtk_places_sidebar_set_drop_targets_visible(
                self.to_glib_none().0,
                visible.into_glib(),
                context.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_places_sidebar_set_local_only")]
    pub fn set_local_only(&self, local_only: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_local_only(self.to_glib_none().0, local_only.into_glib());
        }
    }

    #[doc(alias = "gtk_places_sidebar_set_location")]
    pub fn set_location(&self, location: Option<&impl IsA<gio::File>>) {
        unsafe {
            ffi::gtk_places_sidebar_set_location(
                self.to_glib_none().0,
                location.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_places_sidebar_set_open_flags")]
    pub fn set_open_flags(&self, flags: PlacesOpenFlags) {
        unsafe {
            ffi::gtk_places_sidebar_set_open_flags(self.to_glib_none().0, flags.into_glib());
        }
    }

    #[doc(alias = "gtk_places_sidebar_set_show_desktop")]
    pub fn set_show_desktop(&self, show_desktop: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_show_desktop(
                self.to_glib_none().0,
                show_desktop.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_places_sidebar_set_show_enter_location")]
    pub fn set_show_enter_location(&self, show_enter_location: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_show_enter_location(
                self.to_glib_none().0,
                show_enter_location.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_places_sidebar_set_show_other_locations")]
    pub fn set_show_other_locations(&self, show_other_locations: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_show_other_locations(
                self.to_glib_none().0,
                show_other_locations.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_places_sidebar_set_show_recent")]
    pub fn set_show_recent(&self, show_recent: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_show_recent(self.to_glib_none().0, show_recent.into_glib());
        }
    }

    #[doc(alias = "gtk_places_sidebar_set_show_starred_location")]
    pub fn set_show_starred_location(&self, show_starred_location: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_show_starred_location(
                self.to_glib_none().0,
                show_starred_location.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_places_sidebar_set_show_trash")]
    pub fn set_show_trash(&self, show_trash: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_show_trash(self.to_glib_none().0, show_trash.into_glib());
        }
    }

    #[doc(alias = "populate-all")]
    pub fn populates_all(&self) -> bool {
        glib::ObjectExt::property(self, "populate-all")
    }

    #[doc(alias = "populate-all")]
    pub fn set_populate_all(&self, populate_all: bool) {
        glib::ObjectExt::set_property(self, "populate-all", populate_all)
    }

    #[doc(alias = "show-connect-to-server")]
    pub fn shows_connect_to_server(&self) -> bool {
        glib::ObjectExt::property(self, "show-connect-to-server")
    }

    #[doc(alias = "show-connect-to-server")]
    pub fn set_show_connect_to_server(&self, show_connect_to_server: bool) {
        glib::ObjectExt::set_property(self, "show-connect-to-server", show_connect_to_server)
    }

    #[doc(alias = "drag-action-ask")]
    pub fn connect_drag_action_ask<F: Fn(&Self, i32) -> i32 + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn drag_action_ask_trampoline<
            F: Fn(&PlacesSidebar, i32) -> i32 + 'static,
        >(
            this: *mut ffi::GtkPlacesSidebar,
            actions: libc::c_int,
            f: glib::ffi::gpointer,
        ) -> libc::c_int {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), actions)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drag-action-ask\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drag_action_ask_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "mount")]
    pub fn connect_mount<F: Fn(&Self, &gio::MountOperation) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn mount_trampoline<
            F: Fn(&PlacesSidebar, &gio::MountOperation) + 'static,
        >(
            this: *mut ffi::GtkPlacesSidebar,
            mount_operation: *mut gio::ffi::GMountOperation,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(mount_operation))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"mount\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    mount_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "open-location")]
    pub fn connect_open_location<F: Fn(&Self, &gio::File, PlacesOpenFlags) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn open_location_trampoline<
            F: Fn(&PlacesSidebar, &gio::File, PlacesOpenFlags) + 'static,
        >(
            this: *mut ffi::GtkPlacesSidebar,
            location: *mut gio::ffi::GFile,
            open_flags: ffi::GtkPlacesOpenFlags,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &from_glib_borrow(location),
                from_glib(open_flags),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"open-location\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    open_location_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    //#[doc(alias = "populate-popup")]
    //pub fn connect_populate_popup<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored selected_volume: Gio.Volume
    //}

    #[doc(alias = "show-enter-location")]
    pub fn connect_show_enter_location<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn show_enter_location_trampoline<F: Fn(&PlacesSidebar) + 'static>(
            this: *mut ffi::GtkPlacesSidebar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"show-enter-location\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    show_enter_location_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-error-message")]
    pub fn connect_show_error_message<F: Fn(&Self, &str, &str) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn show_error_message_trampoline<
            F: Fn(&PlacesSidebar, &str, &str) + 'static,
        >(
            this: *mut ffi::GtkPlacesSidebar,
            primary: *mut libc::c_char,
            secondary: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &glib::GString::from_glib_borrow(primary),
                &glib::GString::from_glib_borrow(secondary),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"show-error-message\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    show_error_message_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-other-locations-with-flags")]
    pub fn connect_show_other_locations_with_flags<F: Fn(&Self, PlacesOpenFlags) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn show_other_locations_with_flags_trampoline<
            F: Fn(&PlacesSidebar, PlacesOpenFlags) + 'static,
        >(
            this: *mut ffi::GtkPlacesSidebar,
            open_flags: ffi::GtkPlacesOpenFlags,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(open_flags))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"show-other-locations-with-flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    show_other_locations_with_flags_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-starred-location")]
    pub fn connect_show_starred_location<F: Fn(&Self, PlacesOpenFlags) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn show_starred_location_trampoline<
            F: Fn(&PlacesSidebar, PlacesOpenFlags) + 'static,
        >(
            this: *mut ffi::GtkPlacesSidebar,
            open_flags: ffi::GtkPlacesOpenFlags,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(open_flags))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"show-starred-location\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    show_starred_location_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "unmount")]
    pub fn connect_unmount<F: Fn(&Self, &gio::MountOperation) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn unmount_trampoline<
            F: Fn(&PlacesSidebar, &gio::MountOperation) + 'static,
        >(
            this: *mut ffi::GtkPlacesSidebar,
            mount_operation: *mut gio::ffi::GMountOperation,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(mount_operation))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"unmount\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    unmount_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "local-only")]
    pub fn connect_local_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_local_only_trampoline<F: Fn(&PlacesSidebar) + 'static>(
            this: *mut ffi::GtkPlacesSidebar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::local-only\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_local_only_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "location")]
    pub fn connect_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_location_trampoline<F: Fn(&PlacesSidebar) + 'static>(
            this: *mut ffi::GtkPlacesSidebar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::location\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_location_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "open-flags")]
    pub fn connect_open_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_open_flags_trampoline<F: Fn(&PlacesSidebar) + 'static>(
            this: *mut ffi::GtkPlacesSidebar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::open-flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_open_flags_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "populate-all")]
    pub fn connect_populate_all_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_populate_all_trampoline<F: Fn(&PlacesSidebar) + 'static>(
            this: *mut ffi::GtkPlacesSidebar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::populate-all\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_populate_all_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-connect-to-server")]
    pub fn connect_show_connect_to_server_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_connect_to_server_trampoline<
            F: Fn(&PlacesSidebar) + 'static,
        >(
            this: *mut ffi::GtkPlacesSidebar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-connect-to-server\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_connect_to_server_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-desktop")]
    pub fn connect_show_desktop_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_desktop_trampoline<F: Fn(&PlacesSidebar) + 'static>(
            this: *mut ffi::GtkPlacesSidebar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-desktop\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_desktop_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-enter-location")]
    pub fn connect_show_enter_location_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_enter_location_trampoline<
            F: Fn(&PlacesSidebar) + 'static,
        >(
            this: *mut ffi::GtkPlacesSidebar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-enter-location\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_enter_location_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-other-locations")]
    pub fn connect_show_other_locations_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_other_locations_trampoline<
            F: Fn(&PlacesSidebar) + 'static,
        >(
            this: *mut ffi::GtkPlacesSidebar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-other-locations\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_other_locations_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-recent")]
    pub fn connect_show_recent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_recent_trampoline<F: Fn(&PlacesSidebar) + 'static>(
            this: *mut ffi::GtkPlacesSidebar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-recent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_recent_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-starred-location")]
    pub fn connect_show_starred_location_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_starred_location_trampoline<
            F: Fn(&PlacesSidebar) + 'static,
        >(
            this: *mut ffi::GtkPlacesSidebar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-starred-location\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_starred_location_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-trash")]
    pub fn connect_show_trash_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_trash_trampoline<F: Fn(&PlacesSidebar) + 'static>(
            this: *mut ffi::GtkPlacesSidebar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-trash\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_trash_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for PlacesSidebar {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`PlacesSidebar`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PlacesSidebarBuilder {
    builder: glib::object::ObjectBuilder<'static, PlacesSidebar>,
}

impl PlacesSidebarBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn local_only(self, local_only: bool) -> Self {
        Self {
            builder: self.builder.property("local-only", local_only),
        }
    }

    pub fn location(self, location: &impl IsA<gio::File>) -> Self {
        Self {
            builder: self.builder.property("location", location.clone().upcast()),
        }
    }

    pub fn open_flags(self, open_flags: PlacesOpenFlags) -> Self {
        Self {
            builder: self.builder.property("open-flags", open_flags),
        }
    }

    pub fn populate_all(self, populate_all: bool) -> Self {
        Self {
            builder: self.builder.property("populate-all", populate_all),
        }
    }

    pub fn show_connect_to_server(self, show_connect_to_server: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("show-connect-to-server", show_connect_to_server),
        }
    }

    pub fn show_desktop(self, show_desktop: bool) -> Self {
        Self {
            builder: self.builder.property("show-desktop", show_desktop),
        }
    }

    pub fn show_enter_location(self, show_enter_location: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("show-enter-location", show_enter_location),
        }
    }

    pub fn show_other_locations(self, show_other_locations: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("show-other-locations", show_other_locations),
        }
    }

    pub fn show_recent(self, show_recent: bool) -> Self {
        Self {
            builder: self.builder.property("show-recent", show_recent),
        }
    }

    pub fn show_starred_location(self, show_starred_location: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("show-starred-location", show_starred_location),
        }
    }

    pub fn show_trash(self, show_trash: bool) -> Self {
        Self {
            builder: self.builder.property("show-trash", show_trash),
        }
    }

    pub fn hadjustment(self, hadjustment: &impl IsA<Adjustment>) -> Self {
        Self {
            builder: self
                .builder
                .property("hadjustment", hadjustment.clone().upcast()),
        }
    }

    pub fn hscrollbar_policy(self, hscrollbar_policy: PolicyType) -> Self {
        Self {
            builder: self
                .builder
                .property("hscrollbar-policy", hscrollbar_policy),
        }
    }

    pub fn kinetic_scrolling(self, kinetic_scrolling: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("kinetic-scrolling", kinetic_scrolling),
        }
    }

    pub fn max_content_height(self, max_content_height: i32) -> Self {
        Self {
            builder: self
                .builder
                .property("max-content-height", max_content_height),
        }
    }

    pub fn max_content_width(self, max_content_width: i32) -> Self {
        Self {
            builder: self
                .builder
                .property("max-content-width", max_content_width),
        }
    }

    pub fn min_content_height(self, min_content_height: i32) -> Self {
        Self {
            builder: self
                .builder
                .property("min-content-height", min_content_height),
        }
    }

    pub fn min_content_width(self, min_content_width: i32) -> Self {
        Self {
            builder: self
                .builder
                .property("min-content-width", min_content_width),
        }
    }

    pub fn overlay_scrolling(self, overlay_scrolling: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("overlay-scrolling", overlay_scrolling),
        }
    }

    pub fn propagate_natural_height(self, propagate_natural_height: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("propagate-natural-height", propagate_natural_height),
        }
    }

    pub fn propagate_natural_width(self, propagate_natural_width: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("propagate-natural-width", propagate_natural_width),
        }
    }

    pub fn shadow_type(self, shadow_type: ShadowType) -> Self {
        Self {
            builder: self.builder.property("shadow-type", shadow_type),
        }
    }

    pub fn vadjustment(self, vadjustment: &impl IsA<Adjustment>) -> Self {
        Self {
            builder: self
                .builder
                .property("vadjustment", vadjustment.clone().upcast()),
        }
    }

    pub fn vscrollbar_policy(self, vscrollbar_policy: PolicyType) -> Self {
        Self {
            builder: self
                .builder
                .property("vscrollbar-policy", vscrollbar_policy),
        }
    }

    pub fn window_placement(self, window_placement: CornerType) -> Self {
        Self {
            builder: self.builder.property("window-placement", window_placement),
        }
    }

    pub fn border_width(self, border_width: u32) -> Self {
        Self {
            builder: self.builder.property("border-width", border_width),
        }
    }

    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn resize_mode(self, resize_mode: ResizeMode) -> Self {
        Self {
            builder: self.builder.property("resize-mode", resize_mode),
        }
    }

    pub fn app_paintable(self, app_paintable: bool) -> Self {
        Self {
            builder: self.builder.property("app-paintable", app_paintable),
        }
    }

    pub fn can_default(self, can_default: bool) -> Self {
        Self {
            builder: self.builder.property("can-default", can_default),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn events(self, events: gdk::EventMask) -> Self {
        Self {
            builder: self.builder.property("events", events),
        }
    }

    pub fn expand(self, expand: bool) -> Self {
        Self {
            builder: self.builder.property("expand", expand),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_default(self, has_default: bool) -> Self {
        Self {
            builder: self.builder.property("has-default", has_default),
        }
    }

    pub fn has_focus(self, has_focus: bool) -> Self {
        Self {
            builder: self.builder.property("has-focus", has_focus),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn is_focus(self, is_focus: bool) -> Self {
        Self {
            builder: self.builder.property("is-focus", is_focus),
        }
    }

    pub fn margin(self, margin: i32) -> Self {
        Self {
            builder: self.builder.property("margin", margin),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn no_show_all(self, no_show_all: bool) -> Self {
        Self {
            builder: self.builder.property("no-show-all", no_show_all),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn parent(self, parent: &impl IsA<Container>) -> Self {
        Self {
            builder: self.builder.property("parent", parent.clone().upcast()),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`PlacesSidebar`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> PlacesSidebar {
        self.builder.build()
    }
}

impl fmt::Display for PlacesSidebar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PlacesSidebar")
    }
}
