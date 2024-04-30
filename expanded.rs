#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[cfg(test)]
mod test {
    use bincode::{Decode, Encode};
    use ww_macro::*;
    use wasm_bindgen::prelude::*;
    pub struct Input(pub i32, pub i32);
    impl ::bincode::Encode for Input {
        fn encode<__E: ::bincode::enc::Encoder>(
            &self,
            encoder: &mut __E,
        ) -> core::result::Result<(), ::bincode::error::EncodeError> {
            ::bincode::Encode::encode(&self.0, encoder)?;
            ::bincode::Encode::encode(&self.1, encoder)?;
            Ok(())
        }
    }
    impl ::bincode::Decode for Input {
        fn decode<__D: ::bincode::de::Decoder>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, ::bincode::error::DecodeError> {
            Ok(Self {
                0: ::bincode::Decode::decode(decoder)?,
                1: ::bincode::Decode::decode(decoder)?,
            })
        }
    }
    impl<'__de> ::bincode::BorrowDecode<'__de> for Input {
        fn borrow_decode<__D: ::bincode::de::BorrowDecoder<'__de>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, ::bincode::error::DecodeError> {
            Ok(Self {
                0: ::bincode::BorrowDecode::borrow_decode(decoder)?,
                1: ::bincode::BorrowDecode::borrow_decode(decoder)?,
            })
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::describe::WasmDescribe for Input {
        fn describe() {
            use wasm_bindgen::__wbindgen_if_not_std;
            use wasm_bindgen::describe::*;
            inform(RUST_STRUCT);
            inform(5u32);
            inform(73u32);
            inform(110u32);
            inform(112u32);
            inform(117u32);
            inform(116u32);
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::IntoWasmAbi for Input {
        type Abi = u32;
        fn into_abi(self) -> u32 {
            use wasm_bindgen::__rt::std::boxed::Box;
            use wasm_bindgen::__rt::WasmRefCell;
            Box::into_raw(Box::new(WasmRefCell::new(self))) as u32
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::FromWasmAbi for Input {
        type Abi = u32;
        unsafe fn from_abi(js: u32) -> Self {
            use wasm_bindgen::__rt::std::boxed::Box;
            use wasm_bindgen::__rt::{assert_not_null, WasmRefCell};
            let ptr = js as *mut WasmRefCell<Input>;
            assert_not_null(ptr);
            let js = Box::from_raw(ptr);
            (*js).borrow_mut();
            js.into_inner()
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::__rt::core::convert::From<Input> for wasm_bindgen::JsValue {
        fn from(value: Input) -> Self {
            let ptr = wasm_bindgen::convert::IntoWasmAbi::into_abi(value);
            #[cfg(
                not(
                    all(
                        target_arch = "wasm32",
                        not(any(target_os = "emscripten", target_os = "wasi"))
                    )
                )
            )]
            unsafe fn __wbg_input_new(_: u32) -> u32 {
                {
                    ::std::rt::begin_panic(
                        "cannot convert to JsValue outside of the wasm target",
                    );
                }
            }
            unsafe {
                <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                    __wbg_input_new(ptr),
                )
            }
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::RefFromWasmAbi for Input {
        type Abi = u32;
        type Anchor = wasm_bindgen::__rt::Ref<'static, Input>;
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let js = js as *mut wasm_bindgen::__rt::WasmRefCell<Input>;
            wasm_bindgen::__rt::assert_not_null(js);
            (*js).borrow()
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::RefMutFromWasmAbi for Input {
        type Abi = u32;
        type Anchor = wasm_bindgen::__rt::RefMut<'static, Input>;
        unsafe fn ref_mut_from_abi(js: Self::Abi) -> Self::Anchor {
            let js = js as *mut wasm_bindgen::__rt::WasmRefCell<Input>;
            wasm_bindgen::__rt::assert_not_null(js);
            (*js).borrow_mut()
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::LongRefFromWasmAbi for Input {
        type Abi = u32;
        type Anchor = wasm_bindgen::__rt::Ref<'static, Input>;
        unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
            <Self as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(js)
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::OptionIntoWasmAbi for Input {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::OptionFromWasmAbi for Input {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    #[allow(clippy::all)]
    impl wasm_bindgen::convert::TryFromJsValue for Input {
        type Error = wasm_bindgen::JsValue;
        fn try_from_js_value(
            value: wasm_bindgen::JsValue,
        ) -> wasm_bindgen::__rt::std::result::Result<Self, Self::Error> {
            let idx = wasm_bindgen::convert::IntoWasmAbi::into_abi(&value);
            #[cfg(
                not(
                    all(
                        target_arch = "wasm32",
                        not(any(target_os = "emscripten", target_os = "wasi"))
                    )
                )
            )]
            unsafe fn __wbg_input_unwrap(_: u32) -> u32 {
                {
                    ::std::rt::begin_panic(
                        "cannot convert from JsValue outside of the wasm target",
                    );
                }
            }
            let ptr = unsafe { __wbg_input_unwrap(idx) };
            if ptr == 0 {
                wasm_bindgen::__rt::std::result::Result::Err(value)
            } else {
                wasm_bindgen::__rt::std::mem::forget(value);
                unsafe {
                    wasm_bindgen::__rt::std::result::Result::Ok(
                        <Self as wasm_bindgen::convert::FromWasmAbi>::from_abi(ptr),
                    )
                }
            }
        }
    }
    impl wasm_bindgen::describe::WasmDescribeVector for Input {
        fn describe_vector() {
            use wasm_bindgen::describe::*;
            inform(VECTOR);
            inform(NAMED_EXTERNREF);
            inform(5u32);
            inform(73u32);
            inform(110u32);
            inform(112u32);
            inform(117u32);
            inform(116u32);
        }
    }
    impl wasm_bindgen::convert::VectorIntoWasmAbi for Input {
        type Abi = <wasm_bindgen::__rt::std::boxed::Box<
            [wasm_bindgen::JsValue],
        > as wasm_bindgen::convert::IntoWasmAbi>::Abi;
        fn vector_into_abi(
            vector: wasm_bindgen::__rt::std::boxed::Box<[Input]>,
        ) -> Self::Abi {
            wasm_bindgen::convert::js_value_vector_into_abi(vector)
        }
    }
    impl wasm_bindgen::convert::VectorFromWasmAbi for Input {
        type Abi = <wasm_bindgen::__rt::std::boxed::Box<
            [wasm_bindgen::JsValue],
        > as wasm_bindgen::convert::FromWasmAbi>::Abi;
        unsafe fn vector_from_abi(
            js: Self::Abi,
        ) -> wasm_bindgen::__rt::std::boxed::Box<[Input]> {
            wasm_bindgen::convert::js_value_vector_from_abi(js)
        }
    }
    #[automatically_derived]
    const _: () = {
        #[doc(hidden)]
        pub unsafe extern "C" fn __wbg_get_input_0(
            js: u32,
        ) -> wasm_bindgen::convert::WasmRet<
            <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        > {
            use wasm_bindgen::__rt::{WasmRefCell, assert_not_null};
            use wasm_bindgen::convert::IntoWasmAbi;
            fn assert_copy<T: Copy>() {}
            assert_copy::<i32>();
            let js = js as *mut WasmRefCell<Input>;
            assert_not_null(js);
            let val = (*js).borrow().0;
            <i32 as IntoWasmAbi>::into_abi(val).into()
        }
    };
    #[automatically_derived]
    const _: () = {
        #[doc(hidden)]
        pub unsafe extern "C" fn __wbg_get_input_1(
            js: u32,
        ) -> wasm_bindgen::convert::WasmRet<
            <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        > {
            use wasm_bindgen::__rt::{WasmRefCell, assert_not_null};
            use wasm_bindgen::convert::IntoWasmAbi;
            fn assert_copy<T: Copy>() {}
            assert_copy::<i32>();
            let js = js as *mut WasmRefCell<Input>;
            assert_not_null(js);
            let val = (*js).borrow().1;
            <i32 as IntoWasmAbi>::into_abi(val).into()
        }
    };
    #[allow(dead_code)]
    pub fn __pow(i__: String) -> String {
        let i: Input = bincode::decode_from_slice(
                i__.as_bytes(),
                bincode::config::standard(),
            )
            .unwrap()
            .0;
        let res = { i.0.pow(i.1 as u32) };
        String::from_utf8(
                bincode::encode_to_vec(&res, bincode::config::standard()).unwrap(),
            )
            .unwrap()
    }
    #[automatically_derived]
    const _: () = {
        pub unsafe extern "C" fn __wasm_bindgen_generated___pow(
            arg0_1: <<String as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
            arg0_2: <<String as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
            arg0_3: <<String as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
            arg0_4: <<String as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
        ) -> wasm_bindgen::convert::WasmRet<
            <String as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
        > {
            let _ret = {
                let arg0 = unsafe {
                    <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                        <<String as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                            arg0_1,
                            arg0_2,
                            arg0_3,
                            arg0_4,
                        ),
                    )
                };
                let _ret = __pow(arg0);
                _ret
            };
            <String as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret).into()
        }
    };
    pub async fn pow(i: Input, c: impl Fn(i32) + 'static) {
        let w = WrappedWorker::<Input, i32>::new("worker.js").await;
        w.run_task("__pow", i, c);
        w.destroy();
    }
}
#[rustc_main]
#[coverage(off)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
