/* Copyright 2016 Jordan Miner
 *
 * Licensed under the MIT license <LICENSE or
 * http://opensource.org/licenses/MIT>. This file may not be copied,
 * modified, or distributed except according to those terms.
 */

use super::control_prelude::*;

#[derive(Clone)]
pub struct List(HandleRc);

impl List {
    pub fn new() -> Self {
        unsafe {
            ::iup_open();
            let ih = IupList(ptr::null_mut());
            List(HandleRc::new(ih))
        }
    }

    pub fn dropdown(&self) -> bool {
        unsafe {
            get_str_attribute_slice(self.handle(), "DROPDOWN\0") == "YES"
        }
    }

    pub fn set_dropdown(&self, dropdown: bool) -> &Self {
        set_str_attribute(self.handle(), "DROPDOWN\0", if dropdown { "YES\0" } else { "NO\0" });
        self
    }

    pub fn edit_box(&self) -> bool {
        unsafe {
            get_str_attribute_slice(self.handle(), "EDITBOX\0") == "YES"
        }
    }

    pub fn set_edit_box(&self, dropdown: bool) -> &Self {
        set_str_attribute(self.handle(), "EDITBOX\0", if dropdown { "YES\0" } else { "NO\0" });
        self
    }

    pub fn multiple(&self) -> bool {
        unsafe {
            get_str_attribute_slice(self.handle(), "MULTIPLE\0") == "YES"
        }
    }

    pub fn set_multiple(&self, multiple: bool) -> &Self {
        set_str_attribute(self.handle(), "MULTIPLE\0", if multiple { "YES\0" } else { "NO\0" });
        self
    }

    // An `index` of 0 is the first item.
    pub fn item(&self, index: usize) -> String {
        get_str_attribute(self.handle(), &format!("{}\0", index + 1))
    }

    // An `index` of 0 is the first item.
    pub fn set_item(&self, index: usize, text: &str) -> &Self {
        set_str_attribute(self.handle(), &format!("{}\0", index + 1), text);
        self
    }

    pub fn append_item(&self, text: &str) -> &Self {
        set_str_attribute(self.handle(), "APPENDITEM\0", text);
        self
    }

    // An `index` of 0 is the first item.
    pub fn insert_item(&self, index: usize, text: &str) -> &Self {
        set_str_attribute(self.handle(), &format!("INSERTITEM{}\0", index + 1), text);
        self
    }

    // An `index` of 0 is the first item.
    pub fn remove_item(&self, index: usize) -> &Self {
        set_str_attribute(self.handle(), "REMOVEITEM\0", &format!("{}\0", index + 1));
        self
    }

    // An `index` of 0 is the first item.
    pub fn clear(&self) -> &Self {
        set_str_attribute(self.handle(), "REMOVEITEM\0", "ALL\0");
        self
    }

    pub fn count(&self) -> usize {
        unsafe {
            let s = get_str_attribute_slice(self.handle(), "COUNT\0");
            s.parse().unwrap()
        }
    }

    /// Panics if `edit_box` is false.
    pub fn value_text(&self) -> String {
        assert!(self.edit_box());
        get_str_attribute(self.handle(), "VALUE\0")
    }

    /// Returns the index of the selected item or `None` if no item is selected.
    ///
    /// Panics if `edit_box` is true or `multiple` is true.
    pub fn value_single(&self) -> Option<usize> {
        assert!(!self.edit_box());
        assert!(!self.multiple());

        unsafe {
            let s = get_str_attribute_slice(self.handle(), "VALUE\0");
            s.parse::<usize>().ok().into_iter().filter(|i| *i != 0).next().map(|i| i - 1)
        }
    }

    /// Returns the indexes of all selected items.
    ///
    /// Panics if `edit_box` is true or `multiple` is false.
    pub fn value_multiple(&self) -> Vec<usize> {
        assert!(!self.edit_box());
        assert!(self.multiple());

        unsafe {
            let s = get_str_attribute_slice(self.handle(), "VALUE\0");
            let sel = s.as_bytes().iter().enumerate().filter(|&(_, c)| *c == b'+').map(|(i, _)| i);
            sel.collect()
        }
    }

    // visible_items
    // visible_columns
    // visible_lines
}

impl_control_traits!(List);

impl ActiveAttribute for List {}
impl MinMaxSizeAttribute for List {}
impl VisibleAttribute for List {}

impl MenuCommonCallbacks for List {}