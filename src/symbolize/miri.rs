use core::ffi::c_void;
use core::marker::PhantomData;

use crate::backtrace::miri::{resolve_addr, Frame};
use crate::symbolize::{ResolveWhat, SymbolName};
use crate::types::BytesOrWideString;

pub unsafe fn resolve(what: ResolveWhat<'_>, cb: &mut dyn FnMut(&super::Symbol)) {
    let sym = match what {
        ResolveWhat::Address(addr) => Symbol {
            inner: resolve_addr(addr),
            _unused: PhantomData,
        },
        ResolveWhat::Frame(frame) => Symbol {
            inner: frame.inner.clone(),
            _unused: PhantomData,
        },
    };
    cb(&super::Symbol { inner: sym })
}

pub struct Symbol<'a> {
    inner: Frame,
    _unused: PhantomData<&'a ()>,
}

impl<'a> Symbol<'a> {
    pub fn name(&self) -> Option<SymbolName<'_>> {
        Some(SymbolName::new(&self.inner.inner.name))
    }

    pub fn addr(&self) -> Option<*mut c_void> {
        Some(self.inner.addr)
    }

    pub fn filename_raw(&self) -> Option<BytesOrWideString<'_>> {
        Some(BytesOrWideString::Bytes(&self.inner.inner.filename))
    }

    pub fn lineno(&self) -> Option<u32> {
        Some(self.inner.inner.lineno)
    }

    pub fn colno(&self) -> Option<u32> {
        Some(self.inner.inner.colno)
    }

    pub fn filename(&self) -> Option<&::std::path::Path> {
        use std::path::Path;
        Some(Path::new(
            std::str::from_utf8(&self.inner.inner.filename).unwrap(),
        ))
    }
}
