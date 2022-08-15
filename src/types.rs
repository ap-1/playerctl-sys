use std::os::raw::{c_int, c_char, c_long};
use glib_sys::{
    GError,
    GList,
    GCompareDataFunc,
    GDestroyNotify,
    GType,
    gpointer,
};
use gobject_sys::{GClosure, GClosureNotifyData, GObject, GTypeClass, GTypeInstance, GValue};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}

type _GClosureNotifyData = GClosureNotifyData;
type _GError = GError;
type _GList = GList;
type _GObject = GObject;
type _GTypeClass = GTypeClass;
type _GTypeInstance = GTypeInstance;
type _GValue = GValue;
type gboolean = c_int;
type gchar = c_char;
type gdouble = f64;
type gint64 = c_long;
