// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Object;
use glib::{prelude::*, translate::*};
use std::{fmt, mem};

glib::wrapper! {
    #[doc(alias = "AtkTableCell")]
    pub struct TableCell(Interface<ffi::AtkTableCell, ffi::AtkTableCellIface>) @requires Object;

    match fn {
        type_ => || ffi::atk_table_cell_get_type(),
    }
}

impl TableCell {
    pub const NONE: Option<&'static TableCell> = None;
}

pub trait TableCellExt: IsA<TableCell> + 'static {
    #[doc(alias = "atk_table_cell_get_column_header_cells")]
    #[doc(alias = "get_column_header_cells")]
    fn column_header_cells(&self) -> Vec<Object> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::atk_table_cell_get_column_header_cells(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "atk_table_cell_get_column_span")]
    #[doc(alias = "get_column_span")]
    fn column_span(&self) -> i32 {
        unsafe { ffi::atk_table_cell_get_column_span(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "atk_table_cell_get_position")]
    #[doc(alias = "get_position")]
    fn position(&self) -> Option<(i32, i32)> {
        unsafe {
            let mut row = mem::MaybeUninit::uninit();
            let mut column = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::atk_table_cell_get_position(
                self.as_ref().to_glib_none().0,
                row.as_mut_ptr(),
                column.as_mut_ptr(),
            ));
            if ret {
                Some((row.assume_init(), column.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "atk_table_cell_get_row_column_span")]
    #[doc(alias = "get_row_column_span")]
    fn row_column_span(&self) -> Option<(i32, i32, i32, i32)> {
        unsafe {
            let mut row = mem::MaybeUninit::uninit();
            let mut column = mem::MaybeUninit::uninit();
            let mut row_span = mem::MaybeUninit::uninit();
            let mut column_span = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::atk_table_cell_get_row_column_span(
                self.as_ref().to_glib_none().0,
                row.as_mut_ptr(),
                column.as_mut_ptr(),
                row_span.as_mut_ptr(),
                column_span.as_mut_ptr(),
            ));
            if ret {
                Some((
                    row.assume_init(),
                    column.assume_init(),
                    row_span.assume_init(),
                    column_span.assume_init(),
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "atk_table_cell_get_row_header_cells")]
    #[doc(alias = "get_row_header_cells")]
    fn row_header_cells(&self) -> Vec<Object> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::atk_table_cell_get_row_header_cells(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "atk_table_cell_get_row_span")]
    #[doc(alias = "get_row_span")]
    fn row_span(&self) -> i32 {
        unsafe { ffi::atk_table_cell_get_row_span(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "atk_table_cell_get_table")]
    #[doc(alias = "get_table")]
    fn table(&self) -> Option<Object> {
        unsafe {
            from_glib_full(ffi::atk_table_cell_get_table(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<TableCell>> TableCellExt for O {}

impl fmt::Display for TableCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TableCell")
    }
}
