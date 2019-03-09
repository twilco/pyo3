use crate::ffi2::methodobject::PyMethodDef;
use crate::ffi2::object::PyObject;
use crate::ffi2::pyport::Py_ssize_t;
use std::os::raw::{c_char, c_int, c_long};
use std::ptr;

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyArg_Parse")]
    pub fn PyArg_Parse(args: *mut PyObject, format: *const c_char, ...) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyArg_ParseTuple")]
    pub fn PyArg_ParseTuple(args: *mut PyObject, format: *const c_char, ...) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyArg_ParseTupleAndKeywords")]
    pub fn PyArg_ParseTupleAndKeywords(
        args: *mut PyObject,
        kw: *mut PyObject,
        format: *const c_char,
        keywords: *mut *mut c_char,
        ...
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyArg_UnpackTuple")]
    pub fn PyArg_UnpackTuple(
        args: *mut PyObject,
        name: *const c_char,
        min: Py_ssize_t,
        max: Py_ssize_t,
        ...
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPy_BuildValue")]
    pub fn Py_BuildValue(format: *const c_char, ...) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "_PyPy_BuildValue_SizeT")]
    //fn _Py_BuildValue_SizeT(arg1: *const c_char, ...)
    // -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "_PyPyArg_NoKeywords")]
    //fn _PyArg_NoKeywords(funcname: *const c_char,
    //                     kw: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyModule_AddObject")]
    pub fn PyModule_AddObject(
        module: *mut PyObject,
        name: *const c_char,
        value: *mut PyObject,
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyModule_AddIntConstant")]
    pub fn PyModule_AddIntConstant(
        module: *mut PyObject,
        name: *const c_char,
        value: c_long,
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyModule_AddStringConstant")]
    pub fn PyModule_AddStringConstant(
        module: *mut PyObject,
        name: *const c_char,
        value: *const c_char,
    ) -> c_int;

    #[cfg(all(target_pointer_width = "64", not(py_sys_config = "Py_TRACE_REFS")))]
    #[cfg_attr(PyPy, link_name = "_PyPy_InitPyPyModule")]
    fn Py_InitModule4_64(
        name: *const c_char,
        methods: *mut PyMethodDef,
        doc: *const c_char,
        _self: *mut PyObject,
        apiver: c_int,
    ) -> *mut PyObject;

    #[cfg(all(target_pointer_width = "64", py_sys_config = "Py_TRACE_REFS"))]
    fn Py_InitModule4TraceRefs_64(
        name: *const c_char,
        methods: *mut PyMethodDef,
        doc: *const c_char,
        _self: *mut PyObject,
        apiver: c_int,
    ) -> *mut PyObject;

    #[cfg(all(not(target_pointer_width = "64"), not(py_sys_config = "Py_TRACE_REFS")))]
    pub fn Py_InitModule4(
        name: *const c_char,
        methods: *mut PyMethodDef,
        doc: *const c_char,
        _self: *mut PyObject,
        apiver: c_int,
    ) -> *mut PyObject;

    #[cfg(all(not(target_pointer_width = "64"), py_sys_config = "Py_TRACE_REFS"))]
    fn Py_InitModule4TraceRefs(
        name: *const c_char,
        methods: *mut PyMethodDef,
        doc: *const c_char,
        _self: *mut PyObject,
        apiver: c_int,
    ) -> *mut PyObject;
}

pub const PYTHON_API_VERSION: c_int = 1013;

#[cfg(all(target_pointer_width = "64", not(py_sys_config = "Py_TRACE_REFS")))]
#[inline]
pub unsafe fn Py_InitModule4(
    name: *const c_char,
    methods: *mut PyMethodDef,
    doc: *const c_char,
    _self: *mut PyObject,
    apiver: c_int,
) -> *mut PyObject {
    Py_InitModule4_64(name, methods, doc, _self, apiver)
}

#[cfg(all(target_pointer_width = "64", py_sys_config = "Py_TRACE_REFS"))]
#[inline]
pub unsafe fn Py_InitModule4(
    name: *const c_char,
    methods: *mut PyMethodDef,
    doc: *const c_char,
    _self: *mut PyObject,
    apiver: c_int,
) -> *mut PyObject {
    Py_InitModule4TraceRefs_64(name, methods, doc, _self, apiver)
}

#[cfg(all(not(target_pointer_width = "64"), py_sys_config = "Py_TRACE_REFS"))]
#[inline]
pub unsafe fn Py_InitModule4(
    name: *const c_char,
    methods: *mut PyMethodDef,
    doc: *const c_char,
    _self: *mut PyObject,
    apiver: c_int,
) -> *mut PyObject {
    Py_InitModule4TraceRefs(name, methods, doc, _self, apiver)
}

#[inline]
pub unsafe fn Py_InitModule(name: *const c_char, methods: *mut PyMethodDef) -> *mut PyObject {
    Py_InitModule4(
        name,
        methods,
        ptr::null(),
        ptr::null_mut(),
        PYTHON_API_VERSION,
    )
}

#[inline]
pub unsafe fn Py_InitModule3(
    name: *const c_char,
    methods: *mut PyMethodDef,
    doc: *const c_char,
) -> *mut PyObject {
    Py_InitModule4(name, methods, doc, ptr::null_mut(), PYTHON_API_VERSION)
}
