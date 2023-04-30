// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Layer, RelationSet, RelationType, Role, State, StateSet};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "AtkObject")]
    pub struct Object(Object<ffi::AtkObject, ffi::AtkObjectClass>);

    match fn {
        type_ => || ffi::atk_object_get_type(),
    }
}

impl Object {
    pub const NONE: Option<&'static Object> = None;
}

pub trait AtkObjectExt: IsA<Object> + 'static {
    #[doc(alias = "atk_object_add_relationship")]
    fn add_relationship(&self, relationship: RelationType, target: &impl IsA<Object>) -> bool {
        unsafe {
            from_glib(ffi::atk_object_add_relationship(
                self.as_ref().to_glib_none().0,
                relationship.into_glib(),
                target.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v2_34")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_34")))]
    #[doc(alias = "atk_object_get_accessible_id")]
    #[doc(alias = "get_accessible_id")]
    fn accessible_id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::atk_object_get_accessible_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "atk_object_get_description")]
    #[doc(alias = "get_description")]
    fn description(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::atk_object_get_description(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "atk_object_get_index_in_parent")]
    #[doc(alias = "get_index_in_parent")]
    fn index_in_parent(&self) -> i32 {
        unsafe { ffi::atk_object_get_index_in_parent(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "atk_object_get_layer")]
    #[doc(alias = "get_layer")]
    fn layer(&self) -> Layer {
        unsafe { from_glib(ffi::atk_object_get_layer(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "atk_object_get_mdi_zorder")]
    #[doc(alias = "get_mdi_zorder")]
    fn mdi_zorder(&self) -> i32 {
        unsafe { ffi::atk_object_get_mdi_zorder(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "atk_object_get_n_accessible_children")]
    #[doc(alias = "get_n_accessible_children")]
    fn n_accessible_children(&self) -> i32 {
        unsafe { ffi::atk_object_get_n_accessible_children(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "atk_object_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::atk_object_get_name(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "atk_object_get_object_locale")]
    #[doc(alias = "get_object_locale")]
    fn object_locale(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::atk_object_get_object_locale(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "atk_object_get_parent")]
    #[doc(alias = "get_parent")]
    #[must_use]
    fn parent(&self) -> Option<Object> {
        unsafe { from_glib_none(ffi::atk_object_get_parent(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "atk_object_get_role")]
    #[doc(alias = "get_role")]
    fn role(&self) -> Role {
        unsafe { from_glib(ffi::atk_object_get_role(self.as_ref().to_glib_none().0)) }
    }

    //#[doc(alias = "atk_object_initialize")]
    //fn initialize(&self, data: /*Unimplemented*/Option<Basic: Pointer>) {
    //    unsafe { TODO: call ffi:atk_object_initialize() }
    //}

    #[doc(alias = "atk_object_notify_state_change")]
    fn notify_state_change(&self, state: State, value: bool) {
        unsafe {
            ffi::atk_object_notify_state_change(
                self.as_ref().to_glib_none().0,
                state,
                value.into_glib(),
            );
        }
    }

    #[doc(alias = "atk_object_peek_parent")]
    #[must_use]
    fn peek_parent(&self) -> Option<Object> {
        unsafe { from_glib_none(ffi::atk_object_peek_parent(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "atk_object_ref_accessible_child")]
    #[must_use]
    fn ref_accessible_child(&self, i: i32) -> Option<Object> {
        unsafe {
            from_glib_full(ffi::atk_object_ref_accessible_child(
                self.as_ref().to_glib_none().0,
                i,
            ))
        }
    }

    #[doc(alias = "atk_object_ref_relation_set")]
    fn ref_relation_set(&self) -> Option<RelationSet> {
        unsafe {
            from_glib_full(ffi::atk_object_ref_relation_set(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "atk_object_ref_state_set")]
    fn ref_state_set(&self) -> Option<StateSet> {
        unsafe {
            from_glib_full(ffi::atk_object_ref_state_set(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "atk_object_remove_relationship")]
    fn remove_relationship(&self, relationship: RelationType, target: &impl IsA<Object>) -> bool {
        unsafe {
            from_glib(ffi::atk_object_remove_relationship(
                self.as_ref().to_glib_none().0,
                relationship.into_glib(),
                target.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v2_34")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_34")))]
    #[doc(alias = "atk_object_set_accessible_id")]
    fn set_accessible_id(&self, name: &str) {
        unsafe {
            ffi::atk_object_set_accessible_id(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "atk_object_set_description")]
    fn set_description(&self, description: &str) {
        unsafe {
            ffi::atk_object_set_description(
                self.as_ref().to_glib_none().0,
                description.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "atk_object_set_name")]
    fn set_name(&self, name: &str) {
        unsafe {
            ffi::atk_object_set_name(self.as_ref().to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[doc(alias = "atk_object_set_parent")]
    fn set_parent(&self, parent: &impl IsA<Object>) {
        unsafe {
            ffi::atk_object_set_parent(
                self.as_ref().to_glib_none().0,
                parent.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "atk_object_set_role")]
    fn set_role(&self, role: Role) {
        unsafe {
            ffi::atk_object_set_role(self.as_ref().to_glib_none().0, role.into_glib());
        }
    }

    #[doc(alias = "accessible-component-layer")]
    fn accessible_component_layer(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "accessible-component-layer")
    }

    #[doc(alias = "accessible-component-mdi-zorder")]
    fn accessible_component_mdi_zorder(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "accessible-component-mdi-zorder")
    }

    #[doc(alias = "accessible-description")]
    fn accessible_description(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "accessible-description")
    }

    #[doc(alias = "accessible-description")]
    fn set_accessible_description(&self, accessible_description: Option<&str>) {
        glib::ObjectExt::set_property(
            self.as_ref(),
            "accessible-description",
            accessible_description,
        )
    }

    #[doc(alias = "accessible-hypertext-nlinks")]
    fn accessible_hypertext_nlinks(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "accessible-hypertext-nlinks")
    }

    #[doc(alias = "accessible-name")]
    fn accessible_name(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "accessible-name")
    }

    #[doc(alias = "accessible-name")]
    fn set_accessible_name(&self, accessible_name: Option<&str>) {
        glib::ObjectExt::set_property(self.as_ref(), "accessible-name", accessible_name)
    }

    #[doc(alias = "accessible-parent")]
    fn accessible_parent(&self) -> Option<Object> {
        glib::ObjectExt::property(self.as_ref(), "accessible-parent")
    }

    #[doc(alias = "accessible-parent")]
    fn set_accessible_parent<P: IsA<Object>>(&self, accessible_parent: Option<&P>) {
        glib::ObjectExt::set_property(self.as_ref(), "accessible-parent", accessible_parent)
    }

    #[doc(alias = "accessible-role")]
    fn accessible_role(&self) -> Role {
        glib::ObjectExt::property(self.as_ref(), "accessible-role")
    }

    #[doc(alias = "accessible-role")]
    fn set_accessible_role(&self, accessible_role: Role) {
        glib::ObjectExt::set_property(self.as_ref(), "accessible-role", accessible_role)
    }

    #[doc(alias = "accessible-table-caption")]
    fn accessible_table_caption(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "accessible-table-caption")
    }

    #[doc(alias = "accessible-table-caption")]
    fn set_accessible_table_caption(&self, accessible_table_caption: Option<&str>) {
        glib::ObjectExt::set_property(
            self.as_ref(),
            "accessible-table-caption",
            accessible_table_caption,
        )
    }

    #[doc(alias = "accessible-table-caption-object")]
    fn accessible_table_caption_object(&self) -> Option<Object> {
        glib::ObjectExt::property(self.as_ref(), "accessible-table-caption-object")
    }

    #[doc(alias = "accessible-table-caption-object")]
    fn set_accessible_table_caption_object<P: IsA<Object>>(
        &self,
        accessible_table_caption_object: Option<&P>,
    ) {
        glib::ObjectExt::set_property(
            self.as_ref(),
            "accessible-table-caption-object",
            accessible_table_caption_object,
        )
    }

    #[doc(alias = "accessible-table-column-description")]
    fn accessible_table_column_description(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "accessible-table-column-description")
    }

    #[doc(alias = "accessible-table-column-description")]
    fn set_accessible_table_column_description(
        &self,
        accessible_table_column_description: Option<&str>,
    ) {
        glib::ObjectExt::set_property(
            self.as_ref(),
            "accessible-table-column-description",
            accessible_table_column_description,
        )
    }

    #[doc(alias = "accessible-table-column-header")]
    fn accessible_table_column_header(&self) -> Option<Object> {
        glib::ObjectExt::property(self.as_ref(), "accessible-table-column-header")
    }

    #[doc(alias = "accessible-table-column-header")]
    fn set_accessible_table_column_header<P: IsA<Object>>(
        &self,
        accessible_table_column_header: Option<&P>,
    ) {
        glib::ObjectExt::set_property(
            self.as_ref(),
            "accessible-table-column-header",
            accessible_table_column_header,
        )
    }

    #[doc(alias = "accessible-table-row-description")]
    fn accessible_table_row_description(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "accessible-table-row-description")
    }

    #[doc(alias = "accessible-table-row-description")]
    fn set_accessible_table_row_description(&self, accessible_table_row_description: Option<&str>) {
        glib::ObjectExt::set_property(
            self.as_ref(),
            "accessible-table-row-description",
            accessible_table_row_description,
        )
    }

    #[doc(alias = "accessible-table-row-header")]
    fn accessible_table_row_header(&self) -> Option<Object> {
        glib::ObjectExt::property(self.as_ref(), "accessible-table-row-header")
    }

    #[doc(alias = "accessible-table-row-header")]
    fn set_accessible_table_row_header<P: IsA<Object>>(
        &self,
        accessible_table_row_header: Option<&P>,
    ) {
        glib::ObjectExt::set_property(
            self.as_ref(),
            "accessible-table-row-header",
            accessible_table_row_header,
        )
    }

    #[doc(alias = "accessible-table-summary")]
    fn accessible_table_summary(&self) -> Option<Object> {
        glib::ObjectExt::property(self.as_ref(), "accessible-table-summary")
    }

    #[doc(alias = "accessible-table-summary")]
    fn set_accessible_table_summary<P: IsA<Object>>(&self, accessible_table_summary: Option<&P>) {
        glib::ObjectExt::set_property(
            self.as_ref(),
            "accessible-table-summary",
            accessible_table_summary,
        )
    }

    #[doc(alias = "accessible-value")]
    fn accessible_value(&self) -> f64 {
        glib::ObjectExt::property(self.as_ref(), "accessible-value")
    }

    #[doc(alias = "accessible-value")]
    fn set_accessible_value(&self, accessible_value: f64) {
        glib::ObjectExt::set_property(self.as_ref(), "accessible-value", accessible_value)
    }

    #[doc(alias = "active-descendant-changed")]
    fn connect_active_descendant_changed<F: Fn(&Self, &Object) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn active_descendant_changed_trampoline<
            P: IsA<Object>,
            F: Fn(&P, &Object) + 'static,
        >(
            this: *mut ffi::AtkObject,
            arg1: *mut ffi::AtkObject,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Object::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(arg1),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            let detailed_signal_name =
                detail.map(|name| format!("active-descendant-changed::{name}\0"));
            let signal_name: &[u8] = detailed_signal_name
                .as_ref()
                .map_or(&b"active-descendant-changed\0"[..], |n| n.as_bytes());
            connect_raw(
                self.as_ptr() as *mut _,
                signal_name.as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    active_descendant_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v2_46")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_46")))]
    #[doc(alias = "announcement")]
    fn connect_announcement<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn announcement_trampoline<P: IsA<Object>, F: Fn(&P, &str) + 'static>(
            this: *mut ffi::AtkObject,
            arg1: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Object::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(arg1),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"announcement\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    announcement_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "children-changed")]
    fn connect_children_changed<F: Fn(&Self, u32, &Object) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn children_changed_trampoline<
            P: IsA<Object>,
            F: Fn(&P, u32, &Object) + 'static,
        >(
            this: *mut ffi::AtkObject,
            arg1: libc::c_uint,
            arg2: *mut ffi::AtkObject,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Object::from_glib_borrow(this).unsafe_cast_ref(),
                arg1,
                &from_glib_borrow(arg2),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            let detailed_signal_name = detail.map(|name| format!("children-changed::{name}\0"));
            let signal_name: &[u8] = detailed_signal_name
                .as_ref()
                .map_or(&b"children-changed\0"[..], |n| n.as_bytes());
            connect_raw(
                self.as_ptr() as *mut _,
                signal_name.as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    children_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    //#[doc(alias = "property-change")]
    //fn connect_property_change<Unsupported or ignored types>(&self, detail: Option<&str>, f: F) -> SignalHandlerId {
    //    Ignored arg1: Atk.PropertyValues
    //}

    #[doc(alias = "state-change")]
    fn connect_state_change<F: Fn(&Self, &str, bool) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn state_change_trampoline<
            P: IsA<Object>,
            F: Fn(&P, &str, bool) + 'static,
        >(
            this: *mut ffi::AtkObject,
            arg1: *mut libc::c_char,
            arg2: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Object::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(arg1),
                from_glib(arg2),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            let detailed_signal_name = detail.map(|name| format!("state-change::{name}\0"));
            let signal_name: &[u8] = detailed_signal_name
                .as_ref()
                .map_or(&b"state-change\0"[..], |n| n.as_bytes());
            connect_raw(
                self.as_ptr() as *mut _,
                signal_name.as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    state_change_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "visible-data-changed")]
    fn connect_visible_data_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn visible_data_changed_trampoline<
            P: IsA<Object>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AtkObject,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"visible-data-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    visible_data_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accessible-component-layer")]
    fn connect_accessible_component_layer_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_accessible_component_layer_trampoline<
            P: IsA<Object>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AtkObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accessible-component-layer\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accessible_component_layer_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accessible-component-mdi-zorder")]
    fn connect_accessible_component_mdi_zorder_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_accessible_component_mdi_zorder_trampoline<
            P: IsA<Object>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AtkObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accessible-component-mdi-zorder\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accessible_component_mdi_zorder_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accessible-description")]
    fn connect_accessible_description_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_accessible_description_trampoline<
            P: IsA<Object>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AtkObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accessible-description\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accessible_description_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accessible-hypertext-nlinks")]
    fn connect_accessible_hypertext_nlinks_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_accessible_hypertext_nlinks_trampoline<
            P: IsA<Object>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AtkObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accessible-hypertext-nlinks\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accessible_hypertext_nlinks_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accessible-name")]
    fn connect_accessible_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accessible_name_trampoline<
            P: IsA<Object>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AtkObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accessible-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accessible_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accessible-parent")]
    fn connect_accessible_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accessible_parent_trampoline<
            P: IsA<Object>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AtkObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accessible-parent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accessible_parent_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accessible-role")]
    fn connect_accessible_role_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accessible_role_trampoline<
            P: IsA<Object>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AtkObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accessible-role\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accessible_role_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accessible-table-caption")]
    fn connect_accessible_table_caption_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_accessible_table_caption_trampoline<
            P: IsA<Object>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AtkObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accessible-table-caption\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accessible_table_caption_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accessible-table-caption-object")]
    fn connect_accessible_table_caption_object_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_accessible_table_caption_object_trampoline<
            P: IsA<Object>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AtkObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accessible-table-caption-object\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accessible_table_caption_object_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accessible-table-column-description")]
    fn connect_accessible_table_column_description_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_accessible_table_column_description_trampoline<
            P: IsA<Object>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AtkObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accessible-table-column-description\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accessible_table_column_description_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accessible-table-column-header")]
    fn connect_accessible_table_column_header_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_accessible_table_column_header_trampoline<
            P: IsA<Object>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AtkObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accessible-table-column-header\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accessible_table_column_header_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accessible-table-row-description")]
    fn connect_accessible_table_row_description_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_accessible_table_row_description_trampoline<
            P: IsA<Object>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AtkObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accessible-table-row-description\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accessible_table_row_description_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accessible-table-row-header")]
    fn connect_accessible_table_row_header_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_accessible_table_row_header_trampoline<
            P: IsA<Object>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AtkObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accessible-table-row-header\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accessible_table_row_header_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accessible-table-summary")]
    fn connect_accessible_table_summary_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_accessible_table_summary_trampoline<
            P: IsA<Object>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AtkObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accessible-table-summary\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accessible_table_summary_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accessible-value")]
    fn connect_accessible_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accessible_value_trampoline<
            P: IsA<Object>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::AtkObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accessible-value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accessible_value_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Object>> AtkObjectExt for O {}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Object")
    }
}
