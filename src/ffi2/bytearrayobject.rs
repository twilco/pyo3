use ffi2::object::*;
use ffi2::pyport::Py_ssize_t;
use std::os::raw::{c_char, c_int};

/*#[repr(C)]
#[deriving(Copy)]
struct PyByteArrayObject {
    #[cfg(py_sys_config="Py_TRACE_REFS")]
    pub _ob_next: *mut PyObject,
    #[cfg(py_sys_config="Py_TRACE_REFS")]
    pub _ob_prev: *mut PyObject,
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut PyTypeObject,
    pub ob_size: Py_ssize_t,
    pub ob_exports: c_int,
    pub ob_alloc: Py_ssize_t,
    pub ob_bytes: *mut c_char,
}*/

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyByteArray_Type")]
    pub static mut PyByteArray_Type: PyTypeObject;
    pub static mut PyByteArrayIter_Type: PyTypeObject;
}

#[cfg_attr(PyPy, link_name = "PyPyByteArray_Check")]
pub unsafe fn PyByteArray_Check(op: *mut PyObject) -> c_int {
    PyObject_TypeCheck(op, &mut PyByteArray_Type)
}

#[cfg_attr(PyPy, link_name = "PyPyByteArray_CheckExact")]
pub unsafe fn PyByteArray_CheckExact(op: *mut PyObject) -> c_int {
    let u: *mut PyTypeObject = &mut PyByteArray_Type;
    (Py_TYPE(op) == u) as c_int
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyByteArray_FromObject")]
    pub fn PyByteArray_FromObject(o: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyByteArray_Concat")]
    pub fn PyByteArray_Concat(a: *mut PyObject, b: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyByteArray_FromStringAndSize")]
    pub fn PyByteArray_FromStringAndSize(string: *const c_char, len: Py_ssize_t) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyByteArray_Size")]
    pub fn PyByteArray_Size(bytearray: *mut PyObject) -> Py_ssize_t;
    #[cfg_attr(PyPy, link_name = "PyPyByteArray_AsString")]
    pub fn PyByteArray_AsString(bytearray: *mut PyObject) -> *mut c_char;
    #[cfg_attr(PyPy, link_name = "PyPyByteArray_Resize")]
    pub fn PyByteArray_Resize(bytearray: *mut PyObject, len: Py_ssize_t) -> c_int;
}

#[inline(always)]
pub unsafe fn PyByteArray_AS_STRING(o: *mut PyObject) -> *mut c_char {
    PyByteArray_AsString(o)
    // #define PyByteArray_AS_STRING(self) \
    //   (assert(PyByteArray_Check(self)), \
    //    Py_SIZE(self) ? ((PyByteArrayObject *)(self))->ob_bytes : _PyByteArray_empty_string)
}

#[inline(always)]
pub unsafe fn PyByteArray_GET_SIZE(o: *mut PyObject) -> Py_ssize_t {
    // #define PyByteArray_GET_SIZE(self)  (assert(PyByteArray_Check(self)),Py_SIZE(self))
    PyByteArray_Size(o)
}
