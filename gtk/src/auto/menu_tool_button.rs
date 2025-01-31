// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Actionable, Align, Bin, Buildable, Container, Menu, ResizeMode, ToolButton, ToolItem, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkMenuToolButton")]
    pub struct MenuToolButton(Object<ffi::GtkMenuToolButton, ffi::GtkMenuToolButtonClass>) @extends ToolButton, ToolItem, Bin, Container, Widget, @implements Buildable, Actionable;

    match fn {
        type_ => || ffi::gtk_menu_tool_button_get_type(),
    }
}

impl MenuToolButton {
    pub const NONE: Option<&'static MenuToolButton> = None;

    #[doc(alias = "gtk_menu_tool_button_new")]
    pub fn new(icon_widget: Option<&impl IsA<Widget>>, label: Option<&str>) -> MenuToolButton {
        assert_initialized_main_thread!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_menu_tool_button_new(
                icon_widget.map(|p| p.as_ref()).to_glib_none().0,
                label.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`MenuToolButton`] objects.
    ///
    /// This method returns an instance of [`MenuToolButtonBuilder`](crate::builders::MenuToolButtonBuilder) which can be used to create [`MenuToolButton`] objects.
    pub fn builder() -> MenuToolButtonBuilder {
        MenuToolButtonBuilder::new()
    }
}

impl Default for MenuToolButton {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`MenuToolButton`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct MenuToolButtonBuilder {
    builder: glib::object::ObjectBuilder<'static, MenuToolButton>,
}

impl MenuToolButtonBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn menu(self, menu: &impl IsA<Menu>) -> Self {
        Self {
            builder: self.builder.property("menu", menu.clone().upcast()),
        }
    }

    pub fn icon_name(self, icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("icon-name", icon_name.into()),
        }
    }

    pub fn icon_widget(self, icon_widget: &impl IsA<Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("icon-widget", icon_widget.clone().upcast()),
        }
    }

    pub fn label(self, label: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("label", label.into()),
        }
    }

    pub fn label_widget(self, label_widget: &impl IsA<Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("label-widget", label_widget.clone().upcast()),
        }
    }

    pub fn use_underline(self, use_underline: bool) -> Self {
        Self {
            builder: self.builder.property("use-underline", use_underline),
        }
    }

    pub fn is_important(self, is_important: bool) -> Self {
        Self {
            builder: self.builder.property("is-important", is_important),
        }
    }

    pub fn visible_horizontal(self, visible_horizontal: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("visible-horizontal", visible_horizontal),
        }
    }

    pub fn visible_vertical(self, visible_vertical: bool) -> Self {
        Self {
            builder: self.builder.property("visible-vertical", visible_vertical),
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

    pub fn action_name(self, action_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("action-name", action_name.into()),
        }
    }

    pub fn action_target(self, action_target: &glib::Variant) -> Self {
        Self {
            builder: self
                .builder
                .property("action-target", action_target.clone()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`MenuToolButton`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> MenuToolButton {
        self.builder.build()
    }
}

pub trait MenuToolButtonExt: IsA<MenuToolButton> + 'static {
    #[doc(alias = "gtk_menu_tool_button_get_menu")]
    #[doc(alias = "get_menu")]
    fn menu(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_menu_tool_button_get_menu(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_menu_tool_button_set_arrow_tooltip_markup")]
    fn set_arrow_tooltip_markup(&self, markup: &str) {
        unsafe {
            ffi::gtk_menu_tool_button_set_arrow_tooltip_markup(
                self.as_ref().to_glib_none().0,
                markup.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_menu_tool_button_set_arrow_tooltip_text")]
    fn set_arrow_tooltip_text(&self, text: &str) {
        unsafe {
            ffi::gtk_menu_tool_button_set_arrow_tooltip_text(
                self.as_ref().to_glib_none().0,
                text.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_menu_tool_button_set_menu")]
    fn set_menu(&self, menu: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_menu_tool_button_set_menu(
                self.as_ref().to_glib_none().0,
                menu.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "show-menu")]
    fn connect_show_menu<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn show_menu_trampoline<P: IsA<MenuToolButton>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMenuToolButton,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MenuToolButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"show-menu\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    show_menu_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "menu")]
    fn connect_menu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_menu_trampoline<P: IsA<MenuToolButton>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkMenuToolButton,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MenuToolButton::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::menu\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_menu_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<MenuToolButton>> MenuToolButtonExt for O {}

impl fmt::Display for MenuToolButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MenuToolButton")
    }
}
