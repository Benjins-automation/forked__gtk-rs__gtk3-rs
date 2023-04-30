// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkPlugAccessible")]
    pub struct PlugAccessible(Object<ffi::GtkPlugAccessible, ffi::GtkPlugAccessibleClass>) @extends atk::Object;

    match fn {
        type_ => || ffi::gtk_plug_accessible_get_type(),
    }
}

impl PlugAccessible {
    pub const NONE: Option<&'static PlugAccessible> = None;

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`PlugAccessible`] objects.
    ///
    /// This method returns an instance of [`PlugAccessibleBuilder`](crate::builders::PlugAccessibleBuilder) which can be used to create [`PlugAccessible`] objects.
    pub fn builder() -> PlugAccessibleBuilder {
        PlugAccessibleBuilder::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`PlugAccessible`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PlugAccessibleBuilder {
    builder: glib::object::ObjectBuilder<'static, PlugAccessible>,
}

impl PlugAccessibleBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn accessible_description(self, accessible_description: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("accessible-description", accessible_description.into()),
        }
    }

    pub fn accessible_name(self, accessible_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("accessible-name", accessible_name.into()),
        }
    }

    pub fn accessible_parent(self, accessible_parent: &impl IsA<atk::Object>) -> Self {
        Self {
            builder: self
                .builder
                .property("accessible-parent", accessible_parent.clone().upcast()),
        }
    }

    pub fn accessible_role(self, accessible_role: atk::Role) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    pub fn accessible_table_caption(
        self,
        accessible_table_caption: impl Into<glib::GString>,
    ) -> Self {
        Self {
            builder: self
                .builder
                .property("accessible-table-caption", accessible_table_caption.into()),
        }
    }

    pub fn accessible_table_caption_object(
        self,
        accessible_table_caption_object: &impl IsA<atk::Object>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "accessible-table-caption-object",
                accessible_table_caption_object.clone().upcast(),
            ),
        }
    }

    pub fn accessible_table_column_description(
        self,
        accessible_table_column_description: impl Into<glib::GString>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "accessible-table-column-description",
                accessible_table_column_description.into(),
            ),
        }
    }

    pub fn accessible_table_column_header(
        self,
        accessible_table_column_header: &impl IsA<atk::Object>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "accessible-table-column-header",
                accessible_table_column_header.clone().upcast(),
            ),
        }
    }

    pub fn accessible_table_row_description(
        self,
        accessible_table_row_description: impl Into<glib::GString>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "accessible-table-row-description",
                accessible_table_row_description.into(),
            ),
        }
    }

    pub fn accessible_table_row_header(
        self,
        accessible_table_row_header: &impl IsA<atk::Object>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "accessible-table-row-header",
                accessible_table_row_header.clone().upcast(),
            ),
        }
    }

    pub fn accessible_table_summary(
        self,
        accessible_table_summary: &impl IsA<atk::Object>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "accessible-table-summary",
                accessible_table_summary.clone().upcast(),
            ),
        }
    }

    pub fn accessible_value(self, accessible_value: f64) -> Self {
        Self {
            builder: self.builder.property("accessible-value", accessible_value),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`PlugAccessible`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> PlugAccessible {
        self.builder.build()
    }
}

pub trait PlugAccessibleExt: IsA<PlugAccessible> + 'static {
    #[doc(alias = "gtk_plug_accessible_get_id")]
    #[doc(alias = "get_id")]
    fn id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_plug_accessible_get_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<PlugAccessible>> PlugAccessibleExt for O {}

impl fmt::Display for PlugAccessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PlugAccessible")
    }
}
