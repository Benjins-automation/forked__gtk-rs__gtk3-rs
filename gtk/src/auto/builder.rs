// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Application;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute, ptr};

glib::wrapper! {
    #[doc(alias = "GtkBuilder")]
    pub struct Builder(Object<ffi::GtkBuilder, ffi::GtkBuilderClass>);

    match fn {
        type_ => || ffi::gtk_builder_get_type(),
    }
}

impl Builder {
    pub const NONE: Option<&'static Builder> = None;

    #[doc(alias = "gtk_builder_new")]
    pub fn new() -> Builder {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_builder_new()) }
    }

    #[doc(alias = "gtk_builder_new_from_resource")]
    #[doc(alias = "new_from_resource")]
    pub fn from_resource(resource_path: &str) -> Builder {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_builder_new_from_resource(
                resource_path.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_builder_new_from_string")]
    #[doc(alias = "new_from_string")]
    pub fn from_string(string: &str) -> Builder {
        assert_initialized_main_thread!();
        let length = string.len() as _;
        unsafe {
            from_glib_full(ffi::gtk_builder_new_from_string(
                string.to_glib_none().0,
                length,
            ))
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

pub trait BuilderExt: IsA<Builder> + 'static {
    //#[doc(alias = "gtk_builder_add_callback_symbol")]
    //fn add_callback_symbol<P: FnOnce() + 'static>(&self, callback_name: &str, callback_symbol: P) {
    //    unsafe { TODO: call ffi:gtk_builder_add_callback_symbol() }
    //}

    //#[doc(alias = "gtk_builder_add_callback_symbols")]
    //fn add_callback_symbols<P: FnOnce() + 'static>(&self, first_callback_name: &str, first_callback_symbol: P, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:gtk_builder_add_callback_symbols() }
    //}

    //#[doc(alias = "gtk_builder_connect_signals")]
    //fn connect_signals(&self, user_data: /*Unimplemented*/Option<Basic: Pointer>) {
    //    unsafe { TODO: call ffi:gtk_builder_connect_signals() }
    //}

    #[doc(alias = "gtk_builder_expose_object")]
    fn expose_object(&self, name: &str, object: &impl IsA<glib::Object>) {
        unsafe {
            ffi::gtk_builder_expose_object(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                object.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_builder_get_application")]
    #[doc(alias = "get_application")]
    fn application(&self) -> Option<Application> {
        unsafe {
            from_glib_none(ffi::gtk_builder_get_application(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_builder_get_objects")]
    #[doc(alias = "get_objects")]
    fn objects(&self) -> Vec<glib::Object> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_builder_get_objects(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_builder_get_translation_domain")]
    #[doc(alias = "get_translation_domain")]
    fn translation_domain(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_builder_get_translation_domain(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_builder_get_type_from_name")]
    #[doc(alias = "get_type_from_name")]
    fn type_from_name(&self, type_name: &str) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gtk_builder_get_type_from_name(
                self.as_ref().to_glib_none().0,
                type_name.to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "gtk_builder_lookup_callback_symbol")]
    //fn lookup_callback_symbol(&self, callback_name: &str) -> Option<Box_<dyn Fn() + 'static>> {
    //    unsafe { TODO: call ffi:gtk_builder_lookup_callback_symbol() }
    //}

    #[doc(alias = "gtk_builder_set_application")]
    fn set_application(&self, application: &impl IsA<Application>) {
        unsafe {
            ffi::gtk_builder_set_application(
                self.as_ref().to_glib_none().0,
                application.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_builder_set_translation_domain")]
    fn set_translation_domain(&self, domain: Option<&str>) {
        unsafe {
            ffi::gtk_builder_set_translation_domain(
                self.as_ref().to_glib_none().0,
                domain.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_builder_value_from_string")]
    fn value_from_string(
        &self,
        pspec: impl AsRef<glib::ParamSpec>,
        string: &str,
    ) -> Result<glib::Value, glib::Error> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let mut error = ptr::null_mut();
            let is_ok = ffi::gtk_builder_value_from_string(
                self.as_ref().to_glib_none().0,
                pspec.as_ref().to_glib_none().0,
                string.to_glib_none().0,
                value.to_glib_none_mut().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(value)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gtk_builder_value_from_string_type")]
    fn value_from_string_type(
        &self,
        type_: glib::types::Type,
        string: &str,
    ) -> Result<glib::Value, glib::Error> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let mut error = ptr::null_mut();
            let is_ok = ffi::gtk_builder_value_from_string_type(
                self.as_ref().to_glib_none().0,
                type_.into_glib(),
                string.to_glib_none().0,
                value.to_glib_none_mut().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(value)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "translation-domain")]
    fn connect_translation_domain_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_translation_domain_trampoline<
            P: IsA<Builder>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkBuilder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Builder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::translation-domain\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_translation_domain_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Builder>> BuilderExt for O {}

impl fmt::Display for Builder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Builder")
    }
}
