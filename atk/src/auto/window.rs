// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Object;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "AtkWindow")]
    pub struct Window(Interface<ffi::AtkWindow, ffi::AtkWindowIface>) @requires Object;

    match fn {
        type_ => || ffi::atk_window_get_type(),
    }
}

impl Window {
    pub const NONE: Option<&'static Window> = None;
}

pub trait AtkWindowExt: IsA<Window> + 'static {
    #[doc(alias = "activate")]
    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<P: IsA<Window>, F: Fn(&P) + 'static>(
            this: *mut ffi::AtkWindow,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Window::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    activate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "create")]
    fn connect_create<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn create_trampoline<P: IsA<Window>, F: Fn(&P) + 'static>(
            this: *mut ffi::AtkWindow,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Window::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"create\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    create_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "deactivate")]
    fn connect_deactivate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn deactivate_trampoline<P: IsA<Window>, F: Fn(&P) + 'static>(
            this: *mut ffi::AtkWindow,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Window::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"deactivate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    deactivate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "destroy")]
    fn connect_destroy<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn destroy_trampoline<P: IsA<Window>, F: Fn(&P) + 'static>(
            this: *mut ffi::AtkWindow,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Window::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"destroy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    destroy_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "maximize")]
    fn connect_maximize<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn maximize_trampoline<P: IsA<Window>, F: Fn(&P) + 'static>(
            this: *mut ffi::AtkWindow,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Window::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"maximize\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    maximize_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "minimize")]
    fn connect_minimize<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn minimize_trampoline<P: IsA<Window>, F: Fn(&P) + 'static>(
            this: *mut ffi::AtkWindow,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Window::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"minimize\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    minimize_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "move")]
    fn connect_move<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn move_trampoline<P: IsA<Window>, F: Fn(&P) + 'static>(
            this: *mut ffi::AtkWindow,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Window::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    move_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "resize")]
    fn connect_resize<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn resize_trampoline<P: IsA<Window>, F: Fn(&P) + 'static>(
            this: *mut ffi::AtkWindow,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Window::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"resize\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    resize_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "restore")]
    fn connect_restore<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn restore_trampoline<P: IsA<Window>, F: Fn(&P) + 'static>(
            this: *mut ffi::AtkWindow,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Window::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"restore\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    restore_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Window>> AtkWindowExt for O {}

impl fmt::Display for Window {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Window")
    }
}
