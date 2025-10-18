#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(warnings)]

macro_rules! skip_assert_initialized {
    () => {};
}

macro_rules! assert_initialized_main_thread {
    () => {};
}

pub use auto::*;
use ffi;
mod auto;
