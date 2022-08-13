use std::os::raw::{c_int, c_char, c_long, c_ulong, c_uint};
use glib_sys::{
    GError,
    GList,
    GData,
    GCompareDataFunc,
    GDestroyNotify,
    GType,
    gpointer,
};

#[repr(C)]
pub union _GValue__bindgen_ty_1 {
    pub v_int: gint,
    pub v_uint: guint,
    pub v_long: glong,
    pub v_ulong: gulong,
    pub v_int64: gint64,
    pub v_uint64: guint64,
    pub v_float: gfloat,
    pub v_double: gdouble,
    pub v_pointer: gpointer,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}

#[repr(C)]
pub struct _GClosure {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4]>,
    pub marshal: Option<unsafe extern "C" fn(closure: *mut _GClosure, return_value: *mut _GValue__bindgen, n_param_values: guint, param_values: *const _GValue__bindgen, invocation_hint: gpointer, marshal_data: gpointer)>,
    pub data: gpointer,
    pub notifiers: *mut _GClosureNotifyData__bindgen,
}

pub type _GClosureNotify__bindgen = Option<unsafe extern "C" fn(data: gpointer, closure: *mut _GClosure)>;

#[repr(C)]
pub struct _GClosureNotifyData__bindgen {
    pub data: gpointer,
    pub notify: _GClosureNotify__bindgen,
}

#[repr(C)]
pub struct _GObject__bindgen {
    pub g_type_instance: _GTypeInstance__bindgen,
    pub ref_count: c_uint,
    pub qdata: *mut GData,
}

#[repr(C)]
pub struct _GTypeClass__bindgen {
    pub g_type: GType,
}

#[repr(C)]
pub struct _GTypeInstance__bindgen {
    pub g_class: *mut _GTypeClass__bindgen,
}

#[repr(C)]
pub struct _GValue__bindgen {
    pub g_type: GType,
    pub data: [_GValue__bindgen_ty_1; 2],
}

type _GClosureNotifyData = _GClosureNotifyData__bindgen;
type _GError = GError;
type _GList = GList;
type _GObject = _GObject__bindgen;
type _GTypeClass = _GTypeClass__bindgen;
type _GTypeInstance = _GTypeInstance__bindgen;
type _GValue = _GValue__bindgen;
type GClosure = _GClosure;
type GObject = _GObject__bindgen;
type gboolean = c_int;
type gchar = c_char;
type gdouble = f64;
type gfloat = f32;
type gint = c_int;
type gint64 = c_long;
type glong = c_long;
type guint = c_uint;
type guint64 = c_ulong;
type gulong = c_ulong;
