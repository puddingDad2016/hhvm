// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

// This module defines a C FFI entry point.

// The function defined here is
// `parser_positioned_full_triva_cpp_ffi`. It returns parse trees as
// JSON strings. It is implemented by calling
// `positioned_full_trivia_parser::parse_script_to_json`.

///`struct CParserEnv` is for passing options through the C FFI.
#[repr(C)]
struct CParserEnv {
    codegen: bool,
    hhvm_compat_mode: bool,
    php5_compat_mode: bool,
    allow_new_attribute_syntax: bool,
    enable_xhp_class_modifier: bool,
    disable_xhp_element_mangling: bool,
    disable_xhp_children_declarations: bool,
    disable_modes: bool,
    disallow_hash_comments: bool,
    disallow_fun_and_cls_meth_pseudo_funcs: bool,
    array_unification: bool,
    interpret_soft_types_as_like_types: bool,
}
impl std::convert::From<&CParserEnv> for parser_core_types::parser_env::ParserEnv {
    fn from(env: &CParserEnv) -> parser_core_types::parser_env::ParserEnv {
        parser_core_types::parser_env::ParserEnv {
            codegen: env.codegen,
            hhvm_compat_mode: env.hhvm_compat_mode,
            php5_compat_mode: env.php5_compat_mode,
            allow_new_attribute_syntax: env.allow_new_attribute_syntax,
            enable_xhp_class_modifier: env.enable_xhp_class_modifier,
            disable_xhp_element_mangling: env.disable_xhp_element_mangling,
            disable_xhp_children_declarations: env.disable_xhp_children_declarations,
            disable_modes: env.disable_modes,
            disallow_hash_comments: env.disallow_hash_comments,
            disallow_fun_and_cls_meth_pseudo_funcs: env.disallow_fun_and_cls_meth_pseudo_funcs,
            array_unification: env.array_unification,
            interpret_soft_types_as_like_types: env.interpret_soft_types_as_like_types,
        }
    }
}

/// Return result of `parse_positioned_full_trivia_cpp_ffi` to Rust.
#[no_mangle]
extern "C" fn parse_positioned_full_trivia_free_string_cpp_ffi(s: *mut libc::c_char) {
    let _ = unsafe { std::ffi::CString::from_raw(s) };
}

/// Calculate a parse tree from source text and render it as json.
#[no_mangle]
extern "C" fn parse_positioned_full_trivia_cpp_ffi(
    filename: *const libc::c_char,
    source_text: *const libc::c_char,
    env: usize,
) -> *const libc::c_char {
    // We rely on the C caller that `filename` be a
    // properly initialized null-terminated C string and we do
    // not check that the bytes it contains are valid UTF-8.
    let filepath = oxidized::relative_path::RelativePath::make(
        oxidized::relative_path::Prefix::Dummy,
        std::path::PathBuf::from(unsafe { cpp_helper::cstr::to_str(filename) }),
    );
    // We rely on the C caller that `text` be a properly iniitalized
    // null-terminated C string.
    let text: &[u8] = unsafe { cpp_helper::cstr::to_u8(source_text) };
    // We rely on the C caller that `env` can be legitmately
    // reinterpreted as a `*const CParserEnv` and that on doing so, it
    // points to a valid properly initialized value.
    let env: parser_core_types::parser_env::ParserEnv = unsafe {
        cpp_helper::from_ptr(
            env,
            <parser_core_types::parser_env::ParserEnv as std::convert::From<&CParserEnv>>::from,
        )
    }
    .unwrap();
    let indexed_source = parser_core_types::indexed_source_text::IndexedSourceText::new(
        parser_core_types::source_text::SourceText::make(ocamlrep::rc::RcOc::new(filepath), text),
    );
    let alloc = bumpalo::Bump::new();
    let mut serializer = serde_json::Serializer::new(std::vec![]);
    let stack_limit: std::option::Option<&stack_limit::StackLimit> = None;
    match positioned_full_trivia_parser::parse_script_to_json(
        &alloc,
        &mut serializer,
        &indexed_source,
        env,
        stack_limit,
    ) {
        Ok(()) =>
        // No runtime assertion is made that `v` contains no 0 bytes.
        unsafe {
            cpp_helper::cstr::from_vec_u8(serializer.into_inner())
        }
        _ => std::ptr::null(),
    }
}
