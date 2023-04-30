// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Align, Buildable, CalendarDisplayOptions, Container, Widget};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkCalendar")]
    pub struct Calendar(Object<ffi::GtkCalendar, ffi::GtkCalendarClass>) @extends Widget, @implements Buildable;

    match fn {
        type_ => || ffi::gtk_calendar_get_type(),
    }
}

impl Calendar {
    pub const NONE: Option<&'static Calendar> = None;

    #[doc(alias = "gtk_calendar_new")]
    pub fn new() -> Calendar {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_calendar_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Calendar`] objects.
    ///
    /// This method returns an instance of [`CalendarBuilder`](crate::builders::CalendarBuilder) which can be used to create [`Calendar`] objects.
    pub fn builder() -> CalendarBuilder {
        CalendarBuilder::new()
    }
}

impl Default for Calendar {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Calendar`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct CalendarBuilder {
    builder: glib::object::ObjectBuilder<'static, Calendar>,
}

impl CalendarBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn day(self, day: i32) -> Self {
        Self {
            builder: self.builder.property("day", day),
        }
    }

    pub fn detail_height_rows(self, detail_height_rows: i32) -> Self {
        Self {
            builder: self
                .builder
                .property("detail-height-rows", detail_height_rows),
        }
    }

    pub fn detail_width_chars(self, detail_width_chars: i32) -> Self {
        Self {
            builder: self
                .builder
                .property("detail-width-chars", detail_width_chars),
        }
    }

    pub fn month(self, month: i32) -> Self {
        Self {
            builder: self.builder.property("month", month),
        }
    }

    pub fn no_month_change(self, no_month_change: bool) -> Self {
        Self {
            builder: self.builder.property("no-month-change", no_month_change),
        }
    }

    pub fn show_day_names(self, show_day_names: bool) -> Self {
        Self {
            builder: self.builder.property("show-day-names", show_day_names),
        }
    }

    pub fn show_details(self, show_details: bool) -> Self {
        Self {
            builder: self.builder.property("show-details", show_details),
        }
    }

    pub fn show_heading(self, show_heading: bool) -> Self {
        Self {
            builder: self.builder.property("show-heading", show_heading),
        }
    }

    pub fn show_week_numbers(self, show_week_numbers: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("show-week-numbers", show_week_numbers),
        }
    }

    pub fn year(self, year: i32) -> Self {
        Self {
            builder: self.builder.property("year", year),
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
    /// Build the [`Calendar`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Calendar {
        self.builder.build()
    }
}

pub trait CalendarExt: IsA<Calendar> + 'static {
    #[doc(alias = "gtk_calendar_clear_marks")]
    fn clear_marks(&self) {
        unsafe {
            ffi::gtk_calendar_clear_marks(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_calendar_get_date")]
    #[doc(alias = "get_date")]
    fn date(&self) -> (u32, u32, u32) {
        unsafe {
            let mut year = mem::MaybeUninit::uninit();
            let mut month = mem::MaybeUninit::uninit();
            let mut day = mem::MaybeUninit::uninit();
            ffi::gtk_calendar_get_date(
                self.as_ref().to_glib_none().0,
                year.as_mut_ptr(),
                month.as_mut_ptr(),
                day.as_mut_ptr(),
            );
            (year.assume_init(), month.assume_init(), day.assume_init())
        }
    }

    #[doc(alias = "gtk_calendar_get_day_is_marked")]
    #[doc(alias = "get_day_is_marked")]
    fn day_is_marked(&self, day: u32) -> bool {
        unsafe {
            from_glib(ffi::gtk_calendar_get_day_is_marked(
                self.as_ref().to_glib_none().0,
                day,
            ))
        }
    }

    #[doc(alias = "gtk_calendar_get_detail_height_rows")]
    #[doc(alias = "get_detail_height_rows")]
    fn detail_height_rows(&self) -> i32 {
        unsafe { ffi::gtk_calendar_get_detail_height_rows(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gtk_calendar_get_detail_width_chars")]
    #[doc(alias = "get_detail_width_chars")]
    fn detail_width_chars(&self) -> i32 {
        unsafe { ffi::gtk_calendar_get_detail_width_chars(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gtk_calendar_get_display_options")]
    #[doc(alias = "get_display_options")]
    fn display_options(&self) -> CalendarDisplayOptions {
        unsafe {
            from_glib(ffi::gtk_calendar_get_display_options(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_calendar_mark_day")]
    fn mark_day(&self, day: u32) {
        unsafe {
            ffi::gtk_calendar_mark_day(self.as_ref().to_glib_none().0, day);
        }
    }

    #[doc(alias = "gtk_calendar_select_day")]
    fn select_day(&self, day: u32) {
        unsafe {
            ffi::gtk_calendar_select_day(self.as_ref().to_glib_none().0, day);
        }
    }

    #[doc(alias = "gtk_calendar_select_month")]
    fn select_month(&self, month: u32, year: u32) {
        unsafe {
            ffi::gtk_calendar_select_month(self.as_ref().to_glib_none().0, month, year);
        }
    }

    #[doc(alias = "gtk_calendar_set_detail_func")]
    fn set_detail_func<P: Fn(&Calendar, u32, u32, u32) -> Option<String> + 'static>(
        &self,
        func: P,
    ) {
        let func_data: Box_<P> = Box_::new(func);
        unsafe extern "C" fn func_func<
            P: Fn(&Calendar, u32, u32, u32) -> Option<String> + 'static,
        >(
            calendar: *mut ffi::GtkCalendar,
            year: libc::c_uint,
            month: libc::c_uint,
            day: libc::c_uint,
            user_data: glib::ffi::gpointer,
        ) -> *mut libc::c_char {
            let calendar = from_glib_borrow(calendar);
            let callback: &P = &*(user_data as *mut _);
            (*callback)(&calendar, year, month, day).to_glib_full()
        }
        let func = Some(func_func::<P> as _);
        unsafe extern "C" fn destroy_func<
            P: Fn(&Calendar, u32, u32, u32) -> Option<String> + 'static,
        >(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = func_data;
        unsafe {
            ffi::gtk_calendar_set_detail_func(
                self.as_ref().to_glib_none().0,
                func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[doc(alias = "gtk_calendar_set_detail_height_rows")]
    fn set_detail_height_rows(&self, rows: i32) {
        unsafe {
            ffi::gtk_calendar_set_detail_height_rows(self.as_ref().to_glib_none().0, rows);
        }
    }

    #[doc(alias = "gtk_calendar_set_detail_width_chars")]
    fn set_detail_width_chars(&self, chars: i32) {
        unsafe {
            ffi::gtk_calendar_set_detail_width_chars(self.as_ref().to_glib_none().0, chars);
        }
    }

    #[doc(alias = "gtk_calendar_set_display_options")]
    fn set_display_options(&self, flags: CalendarDisplayOptions) {
        unsafe {
            ffi::gtk_calendar_set_display_options(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_calendar_unmark_day")]
    fn unmark_day(&self, day: u32) {
        unsafe {
            ffi::gtk_calendar_unmark_day(self.as_ref().to_glib_none().0, day);
        }
    }

    fn day(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "day")
    }

    fn set_day(&self, day: i32) {
        glib::ObjectExt::set_property(self.as_ref(), "day", day)
    }

    fn month(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "month")
    }

    fn set_month(&self, month: i32) {
        glib::ObjectExt::set_property(self.as_ref(), "month", month)
    }

    #[doc(alias = "no-month-change")]
    fn is_no_month_change(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "no-month-change")
    }

    #[doc(alias = "no-month-change")]
    fn set_no_month_change(&self, no_month_change: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "no-month-change", no_month_change)
    }

    #[doc(alias = "show-day-names")]
    fn shows_day_names(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "show-day-names")
    }

    #[doc(alias = "show-day-names")]
    fn set_show_day_names(&self, show_day_names: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "show-day-names", show_day_names)
    }

    #[doc(alias = "show-details")]
    fn shows_details(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "show-details")
    }

    #[doc(alias = "show-details")]
    fn set_show_details(&self, show_details: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "show-details", show_details)
    }

    #[doc(alias = "show-heading")]
    fn shows_heading(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "show-heading")
    }

    #[doc(alias = "show-heading")]
    fn set_show_heading(&self, show_heading: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "show-heading", show_heading)
    }

    #[doc(alias = "show-week-numbers")]
    fn shows_week_numbers(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "show-week-numbers")
    }

    #[doc(alias = "show-week-numbers")]
    fn set_show_week_numbers(&self, show_week_numbers: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "show-week-numbers", show_week_numbers)
    }

    fn year(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "year")
    }

    fn set_year(&self, year: i32) {
        glib::ObjectExt::set_property(self.as_ref(), "year", year)
    }

    #[doc(alias = "day-selected")]
    fn connect_day_selected<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn day_selected_trampoline<P: IsA<Calendar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"day-selected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    day_selected_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "day-selected-double-click")]
    fn connect_day_selected_double_click<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn day_selected_double_click_trampoline<
            P: IsA<Calendar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"day-selected-double-click\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    day_selected_double_click_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "month-changed")]
    fn connect_month_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn month_changed_trampoline<P: IsA<Calendar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"month-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    month_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "next-month")]
    fn connect_next_month<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn next_month_trampoline<P: IsA<Calendar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"next-month\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    next_month_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "next-year")]
    fn connect_next_year<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn next_year_trampoline<P: IsA<Calendar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"next-year\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    next_year_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "prev-month")]
    fn connect_prev_month<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn prev_month_trampoline<P: IsA<Calendar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"prev-month\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    prev_month_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "prev-year")]
    fn connect_prev_year<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn prev_year_trampoline<P: IsA<Calendar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"prev-year\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    prev_year_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "day")]
    fn connect_day_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_day_trampoline<P: IsA<Calendar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::day\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_day_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "detail-height-rows")]
    fn connect_detail_height_rows_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_detail_height_rows_trampoline<
            P: IsA<Calendar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::detail-height-rows\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_detail_height_rows_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "detail-width-chars")]
    fn connect_detail_width_chars_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_detail_width_chars_trampoline<
            P: IsA<Calendar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::detail-width-chars\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_detail_width_chars_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "month")]
    fn connect_month_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_month_trampoline<P: IsA<Calendar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::month\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_month_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "no-month-change")]
    fn connect_no_month_change_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_no_month_change_trampoline<
            P: IsA<Calendar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::no-month-change\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_no_month_change_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-day-names")]
    fn connect_show_day_names_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_day_names_trampoline<
            P: IsA<Calendar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-day-names\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_day_names_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-details")]
    fn connect_show_details_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_details_trampoline<
            P: IsA<Calendar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-details\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_details_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-heading")]
    fn connect_show_heading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_heading_trampoline<
            P: IsA<Calendar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-heading\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_heading_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-week-numbers")]
    fn connect_show_week_numbers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_week_numbers_trampoline<
            P: IsA<Calendar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-week-numbers\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_week_numbers_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "year")]
    fn connect_year_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_year_trampoline<P: IsA<Calendar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::year\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_year_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Calendar>> CalendarExt for O {}

impl fmt::Display for Calendar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Calendar")
    }
}
