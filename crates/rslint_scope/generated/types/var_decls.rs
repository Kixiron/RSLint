#![allow(
    path_statements,
    unused_imports,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    unused_parens,
    non_shorthand_field_patterns,
    dead_code,
    overflowing_literals,
    unreachable_patterns,
    unused_variables,
    clippy::missing_safety_doc,
    clippy::match_single_binding,
    clippy::ptr_arg,
    clippy::redundant_closure,
    clippy::needless_lifetimes,
    clippy::borrowed_box,
    clippy::map_clone,
    clippy::toplevel_ref_arg,
    clippy::double_parens,
    clippy::collapsible_if,
    clippy::clone_on_copy,
    clippy::unused_unit,
    clippy::deref_addrof,
    clippy::clone_on_copy,
    clippy::needless_return,
    clippy::op_ref,
    clippy::match_like_matches_macro,
    clippy::comparison_chain,
    clippy::len_zero,
    clippy::extra_unused_lifetimes
)]

use ::num::One;
use ::std::ops::Deref;

use ::differential_dataflow::collection;
use ::timely::communication;
use ::timely::dataflow::scopes;
use ::timely::worker;

//use ::serde::de::DeserializeOwned;
use ::differential_datalog::ddval::DDValConvert;
use ::differential_datalog::ddval::DDValue;
use ::differential_datalog::program;
use ::differential_datalog::program::TupleTS;
use ::differential_datalog::program::Weight;
use ::differential_datalog::program::XFormArrangement;
use ::differential_datalog::program::XFormCollection;
use ::differential_datalog::record::FromRecord;
use ::differential_datalog::record::IntoRecord;
use ::differential_datalog::record::Mutator;
use ::serde::Deserialize;
use ::serde::Serialize;

// `usize` and `isize` are builtin Rust types; we therefore declare an alias to DDlog's `usize` and
// `isize`.
pub type std_usize = u64;
pub type std_isize = i64;

#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum DeclarationScope {
    Unhoistable {
        scope: types__ast::ScopeId,
    },
    Hoistable {
        hoisted: types__ast::ScopeId,
        unhoisted: types__ast::ScopeId,
    },
}
impl abomonation::Abomonation for DeclarationScope {}
::differential_datalog::decl_enum_from_record!(DeclarationScope["var_decls::DeclarationScope"]<>, Unhoistable["var_decls::Unhoistable"][1]{[0]scope["scope"]: types__ast::ScopeId}, Hoistable["var_decls::Hoistable"][2]{[0]hoisted["hoisted"]: types__ast::ScopeId, [1]unhoisted["unhoisted"]: types__ast::ScopeId});
::differential_datalog::decl_enum_into_record!(DeclarationScope<>, Unhoistable["var_decls::Unhoistable"]{scope}, Hoistable["var_decls::Hoistable"]{hoisted, unhoisted});
#[rustfmt::skip] ::differential_datalog::decl_record_mutator_enum!(DeclarationScope<>, Unhoistable{scope: types__ast::ScopeId}, Hoistable{hoisted: types__ast::ScopeId, unhoisted: types__ast::ScopeId});
impl ::std::fmt::Display for DeclarationScope {
    fn fmt(&self, __formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            DeclarationScope::Unhoistable { scope } => {
                __formatter.write_str("var_decls::Unhoistable{")?;
                ::std::fmt::Debug::fmt(scope, __formatter)?;
                __formatter.write_str("}")
            }
            DeclarationScope::Hoistable { hoisted, unhoisted } => {
                __formatter.write_str("var_decls::Hoistable{")?;
                ::std::fmt::Debug::fmt(hoisted, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(unhoisted, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl ::std::fmt::Debug for DeclarationScope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self, f)
    }
}
impl ::std::default::Default for DeclarationScope {
    fn default() -> Self {
        DeclarationScope::Unhoistable {
            scope: ::std::default::Default::default(),
        }
    }
}
#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct VariableDeclarations {
    pub file: types__ast::FileId,
    pub name: types__ast::Name,
    pub scope: DeclarationScope,
    pub declared_in: types__ast::AnyId,
    pub meta: ddlog_std::Ref<VariableMeta>,
}
impl abomonation::Abomonation for VariableDeclarations {}
::differential_datalog::decl_struct_from_record!(VariableDeclarations["var_decls::VariableDeclarations"]<>, ["var_decls::VariableDeclarations"][5]{[0]file["file"]: types__ast::FileId, [1]name["name"]: types__ast::Name, [2]scope["scope"]: DeclarationScope, [3]declared_in["declared_in"]: types__ast::AnyId, [4]meta["meta"]: ddlog_std::Ref<VariableMeta>});
::differential_datalog::decl_struct_into_record!(VariableDeclarations, ["var_decls::VariableDeclarations"]<>, file, name, scope, declared_in, meta);
#[rustfmt::skip] ::differential_datalog::decl_record_mutator_struct!(VariableDeclarations, <>, file: types__ast::FileId, name: types__ast::Name, scope: DeclarationScope, declared_in: types__ast::AnyId, meta: ddlog_std::Ref<VariableMeta>);
impl ::std::fmt::Display for VariableDeclarations {
    fn fmt(&self, __formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            VariableDeclarations {
                file,
                name,
                scope,
                declared_in,
                meta,
            } => {
                __formatter.write_str("var_decls::VariableDeclarations{")?;
                ::std::fmt::Debug::fmt(file, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(name, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(scope, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(declared_in, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(meta, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl ::std::fmt::Debug for VariableDeclarations {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self, f)
    }
}
#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct VariableMeta {
    pub is_function_argument: bool,
    pub implicitly_declared: bool,
    pub declaration_span: ddlog_std::Option<types__ast::Span>,
}
impl abomonation::Abomonation for VariableMeta {}
::differential_datalog::decl_struct_from_record!(VariableMeta["var_decls::VariableMeta"]<>, ["var_decls::VariableMeta"][3]{[0]is_function_argument["is_function_argument"]: bool, [1]implicitly_declared["implicitly_declared"]: bool, [2]declaration_span["declaration_span"]: ddlog_std::Option<types__ast::Span>});
::differential_datalog::decl_struct_into_record!(VariableMeta, ["var_decls::VariableMeta"]<>, is_function_argument, implicitly_declared, declaration_span);
#[rustfmt::skip] ::differential_datalog::decl_record_mutator_struct!(VariableMeta, <>, is_function_argument: bool, implicitly_declared: bool, declaration_span: ddlog_std::Option<types__ast::Span>);
impl ::std::fmt::Display for VariableMeta {
    fn fmt(&self, __formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            VariableMeta {
                is_function_argument,
                implicitly_declared,
                declaration_span,
            } => {
                __formatter.write_str("var_decls::VariableMeta{")?;
                ::std::fmt::Debug::fmt(is_function_argument, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(implicitly_declared, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(declaration_span, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl ::std::fmt::Debug for VariableMeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self, f)
    }
}
pub fn hoisted_scope(scope: &DeclarationScope) -> types__ast::ScopeId {
    match (*scope) {
        DeclarationScope::Unhoistable { scope: ref scope } => (*scope).clone(),
        DeclarationScope::Hoistable {
            hoisted: ref hoisted,
            unhoisted: _,
        } => (*hoisted).clone(),
    }
}
pub fn is_hoistable(scope: &DeclarationScope) -> bool {
    match (*scope) {
        DeclarationScope::Unhoistable { scope: _ } => false,
        DeclarationScope::Hoistable {
            hoisted: _,
            unhoisted: _,
        } => true,
    }
}
pub fn is_unhoistable(scope: &DeclarationScope) -> bool {
    match (*scope) {
        DeclarationScope::Unhoistable { scope: _ } => true,
        DeclarationScope::Hoistable {
            hoisted: _,
            unhoisted: _,
        } => false,
    }
}
pub fn unhoisted_scope(scope: &DeclarationScope) -> types__ast::ScopeId {
    match (*scope) {
        DeclarationScope::Unhoistable { scope: ref scope } => (*scope).clone(),
        DeclarationScope::Hoistable {
            hoisted: _,
            unhoisted: ref unhoisted,
        } => (*unhoisted).clone(),
    }
}
pub static __Arng_var_decls_VariableDeclarations_0: ::once_cell::sync::Lazy<program::Arrangement> =
    ::once_cell::sync::Lazy::new(|| program::Arrangement::Map {
        name: std::borrow::Cow::from(
            r###"(var_decls::VariableDeclarations{.file=(_0: ast::FileId), .name=(_1: internment::Intern<string>), .scope=(_: var_decls::DeclarationScope), .declared_in=(_2: ast::AnyId), .meta=(_: ddlog_std::Ref<var_decls::VariableMeta>)}: var_decls::VariableDeclarations) /*join*/"###,
        ),
        afun: {
            fn __f(__v: DDValue) -> Option<(DDValue, DDValue)> {
                let __cloned = __v.clone();
                match <VariableDeclarations>::from_ddvalue(__v) {
                    VariableDeclarations {
                        file: ref _0,
                        name: ref _1,
                        scope: _,
                        declared_in: ref _2,
                        meta: _,
                    } => Some(
                        (ddlog_std::tuple3((*_0).clone(), (*_1).clone(), (*_2).clone()))
                            .into_ddvalue(),
                    ),
                    _ => None,
                }
                .map(|x| (x, __cloned))
            }
            __f
        },
        queryable: false,
    });
pub static __Arng_var_decls_VariableDeclarations_1: ::once_cell::sync::Lazy<program::Arrangement> =
    ::once_cell::sync::Lazy::new(|| program::Arrangement::Map {
        name: std::borrow::Cow::from(
            r###"(var_decls::VariableDeclarations{.file=(_0: ast::FileId), .name=(_: internment::Intern<string>), .scope=(_: var_decls::DeclarationScope), .declared_in=(_1: ast::AnyId), .meta=(_: ddlog_std::Ref<var_decls::VariableMeta>)}: var_decls::VariableDeclarations) /*join*/"###,
        ),
        afun: {
            fn __f(__v: DDValue) -> Option<(DDValue, DDValue)> {
                let __cloned = __v.clone();
                match <VariableDeclarations>::from_ddvalue(__v) {
                    VariableDeclarations {
                        file: ref _0,
                        name: _,
                        scope: _,
                        declared_in: ref _1,
                        meta: _,
                    } => Some((ddlog_std::tuple2((*_0).clone(), (*_1).clone())).into_ddvalue()),
                    _ => None,
                }
                .map(|x| (x, __cloned))
            }
            __f
        },
        queryable: false,
    });
pub static __Arng_var_decls_VariableDeclarations_2: ::once_cell::sync::Lazy<program::Arrangement> =
    ::once_cell::sync::Lazy::new(|| program::Arrangement::Map {
        name: std::borrow::Cow::from(
            r###"(var_decls::VariableDeclarations{.file=(_0: ast::FileId), .name=(_: internment::Intern<string>), .scope=(_: var_decls::DeclarationScope), .declared_in=(_1: ast::AnyId), .meta=((&(var_decls::VariableMeta{.is_function_argument=(_: bool), .implicitly_declared=false, .declaration_span=(ddlog_std::Some{.x=(_: ast::Span)}: ddlog_std::Option<ast::Span>)}: var_decls::VariableMeta)): ddlog_std::Ref<var_decls::VariableMeta>)}: var_decls::VariableDeclarations) /*join*/"###,
        ),
        afun: {
            fn __f(__v: DDValue) -> Option<(DDValue, DDValue)> {
                let __cloned = __v.clone();
                match <VariableDeclarations>::from_ddvalue(__v) {
                    VariableDeclarations {
                        file: ref _0,
                        name: _,
                        scope: _,
                        declared_in: ref _1,
                        meta: ref _0_,
                    } => match (*_0_).deref() {
                        VariableMeta {
                            is_function_argument: _,
                            implicitly_declared: false,
                            declaration_span: ddlog_std::Option::Some { x: _ },
                        } => Some((ddlog_std::tuple2((*_0).clone(), (*_1).clone())).into_ddvalue()),
                        _ => None,
                    },
                    _ => None,
                }
                .map(|x| (x, __cloned))
            }
            __f
        },
        queryable: false,
    });
pub static __Arng_var_decls_VariableDeclarations_3: ::once_cell::sync::Lazy<program::Arrangement> =
    ::once_cell::sync::Lazy::new(|| program::Arrangement::Map {
        name: std::borrow::Cow::from(
            r###"(var_decls::VariableDeclarations{.file=(_0: ast::FileId), .name=(_1: internment::Intern<string>), .scope=(_: var_decls::DeclarationScope), .declared_in=(_2: ast::AnyId), .meta=((&(var_decls::VariableMeta{.is_function_argument=(_: bool), .implicitly_declared=false, .declaration_span=(ddlog_std::Some{.x=(_: ast::Span)}: ddlog_std::Option<ast::Span>)}: var_decls::VariableMeta)): ddlog_std::Ref<var_decls::VariableMeta>)}: var_decls::VariableDeclarations) /*join*/"###,
        ),
        afun: {
            fn __f(__v: DDValue) -> Option<(DDValue, DDValue)> {
                let __cloned = __v.clone();
                match <VariableDeclarations>::from_ddvalue(__v) {
                    VariableDeclarations {
                        file: ref _0,
                        name: ref _1,
                        scope: _,
                        declared_in: ref _2,
                        meta: ref _0_,
                    } => match (*_0_).deref() {
                        VariableMeta {
                            is_function_argument: _,
                            implicitly_declared: false,
                            declaration_span: ddlog_std::Option::Some { x: _ },
                        } => Some(
                            (ddlog_std::tuple3((*_0).clone(), (*_1).clone(), (*_2).clone()))
                                .into_ddvalue(),
                        ),
                        _ => None,
                    },
                    _ => None,
                }
                .map(|x| (x, __cloned))
            }
            __f
        },
        queryable: false,
    });
pub static __Arng_var_decls_VariableDeclarations_4: ::once_cell::sync::Lazy<program::Arrangement> =
    ::once_cell::sync::Lazy::new(|| program::Arrangement::Map {
        name: std::borrow::Cow::from(
            r###"(var_decls::VariableDeclarations{.file=(_0: ast::FileId), .name=(_: internment::Intern<string>), .scope=(var_decls::Unhoistable{.scope=(_: ast::ScopeId)}: var_decls::DeclarationScope), .declared_in=(_: ast::AnyId), .meta=(_: ddlog_std::Ref<var_decls::VariableMeta>)}: var_decls::VariableDeclarations) /*join*/"###,
        ),
        afun: {
            fn __f(__v: DDValue) -> Option<(DDValue, DDValue)> {
                let __cloned = __v.clone();
                match <VariableDeclarations>::from_ddvalue(__v) {
                    VariableDeclarations {
                        file: ref _0,
                        name: _,
                        scope: DeclarationScope::Unhoistable { scope: _ },
                        declared_in: _,
                        meta: _,
                    } => Some(((*_0).clone()).into_ddvalue()),
                    _ => None,
                }
                .map(|x| (x, __cloned))
            }
            __f
        },
        queryable: false,
    });
pub static __Arng_var_decls_VariableDeclarations_5: ::once_cell::sync::Lazy<program::Arrangement> =
    ::once_cell::sync::Lazy::new(|| program::Arrangement::Map {
        name: std::borrow::Cow::from(
            r###"(var_decls::VariableDeclarations{.file=(_0: ast::FileId), .name=(_: internment::Intern<string>), .scope=(var_decls::Hoistable{.hoisted=(_: ast::ScopeId), .unhoisted=(_: ast::ScopeId)}: var_decls::DeclarationScope), .declared_in=(_: ast::AnyId), .meta=(_: ddlog_std::Ref<var_decls::VariableMeta>)}: var_decls::VariableDeclarations) /*join*/"###,
        ),
        afun: {
            fn __f(__v: DDValue) -> Option<(DDValue, DDValue)> {
                let __cloned = __v.clone();
                match <VariableDeclarations>::from_ddvalue(__v) {
                    VariableDeclarations {
                        file: ref _0,
                        name: _,
                        scope:
                            DeclarationScope::Hoistable {
                                hoisted: _,
                                unhoisted: _,
                            },
                        declared_in: _,
                        meta: _,
                    } => Some(((*_0).clone()).into_ddvalue()),
                    _ => None,
                }
                .map(|x| (x, __cloned))
            }
            __f
        },
        queryable: false,
    });
pub static __Arng_var_decls_VariableDeclarations_6: ::once_cell::sync::Lazy<program::Arrangement> =
    ::once_cell::sync::Lazy::new(|| program::Arrangement::Map {
        name: std::borrow::Cow::from(
            r###"(var_decls::VariableDeclarations{.file=(_0: ast::FileId), .name=(_: internment::Intern<string>), .scope=(_: var_decls::DeclarationScope), .declared_in=(_: ast::AnyId), .meta=((&(var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=(ddlog_std::Some{.x=(_: ast::Span)}: ddlog_std::Option<ast::Span>)}: var_decls::VariableMeta)): ddlog_std::Ref<var_decls::VariableMeta>)}: var_decls::VariableDeclarations) /*join*/"###,
        ),
        afun: {
            fn __f(__v: DDValue) -> Option<(DDValue, DDValue)> {
                let __cloned = __v.clone();
                match <VariableDeclarations>::from_ddvalue(__v) {
                    VariableDeclarations {
                        file: ref _0,
                        name: _,
                        scope: _,
                        declared_in: _,
                        meta: ref _0_,
                    } => match (*_0_).deref() {
                        VariableMeta {
                            is_function_argument: false,
                            implicitly_declared: false,
                            declaration_span: ddlog_std::Option::Some { x: _ },
                        } => Some(((*_0).clone()).into_ddvalue()),
                        _ => None,
                    },
                    _ => None,
                }
                .map(|x| (x, __cloned))
            }
            __f
        },
        queryable: false,
    });
pub static __Arng_var_decls_VariableDeclarations_7: ::once_cell::sync::Lazy<program::Arrangement> =
    ::once_cell::sync::Lazy::new(|| program::Arrangement::Map {
        name: std::borrow::Cow::from(
            r###"(var_decls::VariableDeclarations{.file=(_0: ast::FileId), .name=(_: internment::Intern<string>), .scope=(_: var_decls::DeclarationScope), .declared_in=(_: ast::AnyId), .meta=((&(var_decls::VariableMeta{.is_function_argument=true, .implicitly_declared=false, .declaration_span=(ddlog_std::Some{.x=(_: ast::Span)}: ddlog_std::Option<ast::Span>)}: var_decls::VariableMeta)): ddlog_std::Ref<var_decls::VariableMeta>)}: var_decls::VariableDeclarations) /*join*/"###,
        ),
        afun: {
            fn __f(__v: DDValue) -> Option<(DDValue, DDValue)> {
                let __cloned = __v.clone();
                match <VariableDeclarations>::from_ddvalue(__v) {
                    VariableDeclarations {
                        file: ref _0,
                        name: _,
                        scope: _,
                        declared_in: _,
                        meta: ref _0_,
                    } => match (*_0_).deref() {
                        VariableMeta {
                            is_function_argument: true,
                            implicitly_declared: false,
                            declaration_span: ddlog_std::Option::Some { x: _ },
                        } => Some(((*_0).clone()).into_ddvalue()),
                        _ => None,
                    },
                    _ => None,
                }
                .map(|x| (x, __cloned))
            }
            __f
        },
        queryable: false,
    });
pub static __Arng_var_decls_VariableDeclarations_8: ::once_cell::sync::Lazy<program::Arrangement> =
    ::once_cell::sync::Lazy::new(|| program::Arrangement::Map {
        name: std::borrow::Cow::from(
            r###"(var_decls::VariableDeclarations{.file=(_0: ast::FileId), .name=(_: internment::Intern<string>), .scope=(_: var_decls::DeclarationScope), .declared_in=(ast::AnyIdGlobal{.global=(_: ast::GlobalId)}: ast::AnyId), .meta=((&(var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=(ddlog_std::Some{.x=(_: ast::Span)}: ddlog_std::Option<ast::Span>)}: var_decls::VariableMeta)): ddlog_std::Ref<var_decls::VariableMeta>)}: var_decls::VariableDeclarations) /*join*/"###,
        ),
        afun: {
            fn __f(__v: DDValue) -> Option<(DDValue, DDValue)> {
                let __cloned = __v.clone();
                match <VariableDeclarations>::from_ddvalue(__v) {
                    VariableDeclarations {
                        file: ref _0,
                        name: _,
                        scope: _,
                        declared_in: types__ast::AnyId::AnyIdGlobal { global: _ },
                        meta: ref _0_,
                    } => match (*_0_).deref() {
                        VariableMeta {
                            is_function_argument: false,
                            implicitly_declared: false,
                            declaration_span: ddlog_std::Option::Some { x: _ },
                        } => Some(((*_0).clone()).into_ddvalue()),
                        _ => None,
                    },
                    _ => None,
                }
                .map(|x| (x, __cloned))
            }
            __f
        },
        queryable: false,
    });
pub static __Rule_var_decls_VariableDeclarations_0: ::once_cell::sync::Lazy<program::Rule> =
    ::once_cell::sync::Lazy::new(
        || /* var_decls::VariableDeclarations[(var_decls::VariableDeclarations{.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta}: var_decls::VariableDeclarations)] :- inputs::File[(inputs::File{.id=(file: ast::FileId), .kind=(_: ast::FileKind), .top_level_scope=(file_scope: ast::ScopeId), .config=(_: config::Config)}: inputs::File)], inputs::ImplicitGlobal[(inputs::ImplicitGlobal{.id=(global: ast::GlobalId), .name=(name: internment::Intern<string>), .privileges=(_: ast::GlobalPriv)}: inputs::ImplicitGlobal)], ((var meta: ddlog_std::Ref<var_decls::VariableMeta>) = ((ddlog_std::ref_new: function(var_decls::VariableMeta):ddlog_std::Ref<var_decls::VariableMeta>)((var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=(ddlog_std::None{}: ddlog_std::Option<ast::Span>)}: var_decls::VariableMeta)))), ((var id: ast::AnyId) = (ast::AnyIdGlobal{.global=global}: ast::AnyId)), ((var scope: var_decls::DeclarationScope) = (var_decls::Unhoistable{.scope=file_scope}: var_decls::DeclarationScope)). */
                                                                                                                              program::Rule::ArrangementRule {
                                                                                                                                  description: std::borrow::Cow::from( "var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::File(.id=file, .kind=_, .top_level_scope=file_scope, .config=_), inputs::ImplicitGlobal(.id=global, .name=name, .privileges=_), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=ddlog_std::None{}}))), (var id = ast::AnyIdGlobal{.global=global}), (var scope = var_decls::Unhoistable{.scope=file_scope})."),
                                                                                                                                  arr: ( 29, 1),
                                                                                                                                  xform: XFormArrangement::Join{
                                                                                                                                             description: std::borrow::Cow::from("inputs::File(.id=file, .kind=_, .top_level_scope=file_scope, .config=_), inputs::ImplicitGlobal(.id=global, .name=name, .privileges=_)"),
                                                                                                                                             ffun: None,
                                                                                                                                             arrangement: (37,0),
                                                                                                                                             jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                             {
                                                                                                                                                 let (ref file, ref file_scope) = match *<types__inputs::File>::from_ddvalue_ref(__v1) {
                                                                                                                                                     types__inputs::File{id: ref file, kind: _, top_level_scope: ref file_scope, config: _} => ((*file).clone(), (*file_scope).clone()),
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 let (ref global, ref name) = match *<types__inputs::ImplicitGlobal>::from_ddvalue_ref(__v2) {
                                                                                                                                                     types__inputs::ImplicitGlobal{id: ref global, name: ref name, privileges: _} => ((*global).clone(), (*name).clone()),
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 let ref meta: ddlog_std::Ref<VariableMeta> = match (*(&*crate::__STATIC_0)).clone() {
                                                                                                                                                     meta => meta,
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 let ref id: types__ast::AnyId = match (types__ast::AnyId::AnyIdGlobal{global: (*global).clone()}) {
                                                                                                                                                     id => id,
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 let ref scope: DeclarationScope = match (DeclarationScope::Unhoistable{scope: (*file_scope).clone()}) {
                                                                                                                                                     scope => scope,
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 Some(((VariableDeclarations{file: (*file).clone(), name: (*name).clone(), scope: (*scope).clone(), declared_in: (*id).clone(), meta: (*meta).clone()})).into_ddvalue())
                                                                                                                                             }
                                                                                                                                             __f},
                                                                                                                                             next: Box::new(None)
                                                                                                                                         }
                                                                                                                              },
    );
pub static __Rule_var_decls_VariableDeclarations_1: ::once_cell::sync::Lazy<program::Rule> =
    ::once_cell::sync::Lazy::new(
        || /* var_decls::VariableDeclarations[(var_decls::VariableDeclarations{.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta}: var_decls::VariableDeclarations)] :- inputs::UserGlobal[(inputs::UserGlobal{.id=(global: ast::GlobalId), .file=(file: ast::FileId), .name=(name: internment::Intern<string>), .privileges=(_: ast::GlobalPriv)}: inputs::UserGlobal)], inputs::File[(inputs::File{.id=(file: ast::FileId), .kind=(_: ast::FileKind), .top_level_scope=(file_scope: ast::ScopeId), .config=(_: config::Config)}: inputs::File)], ((var meta: ddlog_std::Ref<var_decls::VariableMeta>) = ((ddlog_std::ref_new: function(var_decls::VariableMeta):ddlog_std::Ref<var_decls::VariableMeta>)((var_decls::VariableMeta{.is_function_argument=true, .implicitly_declared=false, .declaration_span=(ddlog_std::None{}: ddlog_std::Option<ast::Span>)}: var_decls::VariableMeta)))), ((var id: ast::AnyId) = (ast::AnyIdGlobal{.global=global}: ast::AnyId)), ((var scope: var_decls::DeclarationScope) = (var_decls::Unhoistable{.scope=file_scope}: var_decls::DeclarationScope)). */
                                                                                                                              program::Rule::ArrangementRule {
                                                                                                                                  description: std::borrow::Cow::from( "var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::UserGlobal(.id=global, .file=file, .name=name, .privileges=_), inputs::File(.id=file, .kind=_, .top_level_scope=file_scope, .config=_), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=true, .implicitly_declared=false, .declaration_span=ddlog_std::None{}}))), (var id = ast::AnyIdGlobal{.global=global}), (var scope = var_decls::Unhoistable{.scope=file_scope})."),
                                                                                                                                  arr: ( 56, 0),
                                                                                                                                  xform: XFormArrangement::Join{
                                                                                                                                             description: std::borrow::Cow::from("inputs::UserGlobal(.id=global, .file=file, .name=name, .privileges=_), inputs::File(.id=file, .kind=_, .top_level_scope=file_scope, .config=_)"),
                                                                                                                                             ffun: None,
                                                                                                                                             arrangement: (29,0),
                                                                                                                                             jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                             {
                                                                                                                                                 let (ref global, ref file, ref name) = match *<types__inputs::UserGlobal>::from_ddvalue_ref(__v1) {
                                                                                                                                                     types__inputs::UserGlobal{id: ref global, file: ref file, name: ref name, privileges: _} => ((*global).clone(), (*file).clone(), (*name).clone()),
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 let ref file_scope = match *<types__inputs::File>::from_ddvalue_ref(__v2) {
                                                                                                                                                     types__inputs::File{id: _, kind: _, top_level_scope: ref file_scope, config: _} => (*file_scope).clone(),
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 let ref meta: ddlog_std::Ref<VariableMeta> = match (*(&*crate::__STATIC_1)).clone() {
                                                                                                                                                     meta => meta,
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 let ref id: types__ast::AnyId = match (types__ast::AnyId::AnyIdGlobal{global: (*global).clone()}) {
                                                                                                                                                     id => id,
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 let ref scope: DeclarationScope = match (DeclarationScope::Unhoistable{scope: (*file_scope).clone()}) {
                                                                                                                                                     scope => scope,
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 Some(((VariableDeclarations{file: (*file).clone(), name: (*name).clone(), scope: (*scope).clone(), declared_in: (*id).clone(), meta: (*meta).clone()})).into_ddvalue())
                                                                                                                                             }
                                                                                                                                             __f},
                                                                                                                                             next: Box::new(None)
                                                                                                                                         }
                                                                                                                              },
    );
pub static __Rule_var_decls_VariableDeclarations_2: ::once_cell::sync::Lazy<program::Rule> =
    ::once_cell::sync::Lazy::new(
        || /* var_decls::VariableDeclarations[(var_decls::VariableDeclarations{.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta}: var_decls::VariableDeclarations)] :- inputs::ImportDecl[(inputs::ImportDecl{.id=(import_id: ast::ImportId), .file=(file: ast::FileId), .clause=(clause: ast::ImportClause)}: inputs::ImportDecl)], inputs::File[(inputs::File{.id=(file: ast::FileId), .kind=(_: ast::FileKind), .top_level_scope=(file_scope: ast::ScopeId), .config=(_: config::Config)}: inputs::File)], var imported = FlatMap((ast::free_variables(clause))), ((ast::Spanned{.data=(var name: internment::Intern<string>), .span=(var span: ast::Span)}: ast::Spanned<internment::Intern<string>>) = imported), ((var meta: ddlog_std::Ref<var_decls::VariableMeta>) = ((ddlog_std::ref_new: function(var_decls::VariableMeta):ddlog_std::Ref<var_decls::VariableMeta>)((var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=(ddlog_std::Some{.x=span}: ddlog_std::Option<ast::Span>)}: var_decls::VariableMeta)))), ((var id: ast::AnyId) = (ast::AnyIdImport{.import_=import_id}: ast::AnyId)), ((var scope: var_decls::DeclarationScope) = (var_decls::Unhoistable{.scope=file_scope}: var_decls::DeclarationScope)). */
                                                                                                                              program::Rule::ArrangementRule {
                                                                                                                                  description: std::borrow::Cow::from( "var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::ImportDecl(.id=import_id, .file=file, .clause=clause), inputs::File(.id=file, .kind=_, .top_level_scope=file_scope, .config=_), var imported = FlatMap((ast::free_variables(clause))), (ast::Spanned{.data=var name, .span=var span} = imported), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdImport{.import_=import_id}), (var scope = var_decls::Unhoistable{.scope=file_scope})."),
                                                                                                                                  arr: ( 38, 0),
                                                                                                                                  xform: XFormArrangement::Join{
                                                                                                                                             description: std::borrow::Cow::from("inputs::ImportDecl(.id=import_id, .file=file, .clause=clause), inputs::File(.id=file, .kind=_, .top_level_scope=file_scope, .config=_)"),
                                                                                                                                             ffun: None,
                                                                                                                                             arrangement: (29,0),
                                                                                                                                             jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                             {
                                                                                                                                                 let (ref import_id, ref file, ref clause) = match *<types__inputs::ImportDecl>::from_ddvalue_ref(__v1) {
                                                                                                                                                     types__inputs::ImportDecl{id: ref import_id, file: ref file, clause: ref clause} => ((*import_id).clone(), (*file).clone(), (*clause).clone()),
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 let ref file_scope = match *<types__inputs::File>::from_ddvalue_ref(__v2) {
                                                                                                                                                     types__inputs::File{id: _, kind: _, top_level_scope: ref file_scope, config: _} => (*file_scope).clone(),
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 Some((ddlog_std::tuple4((*import_id).clone(), (*file).clone(), (*clause).clone(), (*file_scope).clone())).into_ddvalue())
                                                                                                                                             }
                                                                                                                                             __f},
                                                                                                                                             next: Box::new(Some(XFormCollection::FlatMap{
                                                                                                                                                                     description: std::borrow::Cow::from("inputs::ImportDecl(.id=import_id, .file=file, .clause=clause), inputs::File(.id=file, .kind=_, .top_level_scope=file_scope, .config=_), var imported = FlatMap((ast::free_variables(clause)))"),
                                                                                                                                                                     fmfun: {fn __f(__v: DDValue) -> Option<Box<dyn Iterator<Item=DDValue>>>
                                                                                                                                                                     {
                                                                                                                                                                         let ddlog_std::tuple4(ref import_id, ref file, ref clause, ref file_scope) = *<ddlog_std::tuple4<types__ast::ImportId, types__ast::FileId, types__ast::ImportClause, types__ast::ScopeId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                         let __flattened = types__ast::free_variables(clause);
                                                                                                                                                                         let import_id = (*import_id).clone();
                                                                                                                                                                         let file = (*file).clone();
                                                                                                                                                                         let file_scope = (*file_scope).clone();
                                                                                                                                                                         Some(Box::new(__flattened.into_iter().map(move |imported|(ddlog_std::tuple4(imported.clone(), import_id.clone(), file.clone(), file_scope.clone())).into_ddvalue())))
                                                                                                                                                                     }
                                                                                                                                                                     __f},
                                                                                                                                                                     next: Box::new(Some(XFormCollection::FilterMap{
                                                                                                                                                                                             description: std::borrow::Cow::from("head of var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::ImportDecl(.id=import_id, .file=file, .clause=clause), inputs::File(.id=file, .kind=_, .top_level_scope=file_scope, .config=_), var imported = FlatMap((ast::free_variables(clause))), (ast::Spanned{.data=var name, .span=var span} = imported), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdImport{.import_=import_id}), (var scope = var_decls::Unhoistable{.scope=file_scope})."),
                                                                                                                                                                                             fmfun: {fn __f(__v: DDValue) -> Option<DDValue>
                                                                                                                                                                                             {
                                                                                                                                                                                                 let ddlog_std::tuple4(ref imported, ref import_id, ref file, ref file_scope) = *<ddlog_std::tuple4<types__ast::Spanned<internment::Intern<String>>, types__ast::ImportId, types__ast::FileId, types__ast::ScopeId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                                                 let (ref name, ref span): (internment::Intern<String>, types__ast::Span) = match (*imported).clone() {
                                                                                                                                                                                                     types__ast::Spanned{data: name, span: span} => (name, span),
                                                                                                                                                                                                     _ => return None
                                                                                                                                                                                                 };
                                                                                                                                                                                                 let ref meta: ddlog_std::Ref<VariableMeta> = match ddlog_std::ref_new((&(VariableMeta{is_function_argument: false, implicitly_declared: false, declaration_span: (ddlog_std::Option::Some{x: (*span).clone()})}))) {
                                                                                                                                                                                                     meta => meta,
                                                                                                                                                                                                     _ => return None
                                                                                                                                                                                                 };
                                                                                                                                                                                                 let ref id: types__ast::AnyId = match (types__ast::AnyId::AnyIdImport{import_: (*import_id).clone()}) {
                                                                                                                                                                                                     id => id,
                                                                                                                                                                                                     _ => return None
                                                                                                                                                                                                 };
                                                                                                                                                                                                 let ref scope: DeclarationScope = match (DeclarationScope::Unhoistable{scope: (*file_scope).clone()}) {
                                                                                                                                                                                                     scope => scope,
                                                                                                                                                                                                     _ => return None
                                                                                                                                                                                                 };
                                                                                                                                                                                                 Some(((VariableDeclarations{file: (*file).clone(), name: (*name).clone(), scope: (*scope).clone(), declared_in: (*id).clone(), meta: (*meta).clone()})).into_ddvalue())
                                                                                                                                                                                             }
                                                                                                                                                                                             __f},
                                                                                                                                                                                             next: Box::new(None)
                                                                                                                                                                                         }))
                                                                                                                                                                 }))
                                                                                                                                         }
                                                                                                                              },
    );
pub static __Rule_var_decls_VariableDeclarations_3: ::once_cell::sync::Lazy<program::Rule> =
    ::once_cell::sync::Lazy::new(
        || /* var_decls::VariableDeclarations[(var_decls::VariableDeclarations{.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta}: var_decls::VariableDeclarations)] :- inputs::Class[(inputs::Class{.id=(class: ast::ClassId), .file=(file: ast::FileId), .name=(ddlog_std::Some{.x=(ast::Spanned{.data=(name: internment::Intern<string>), .span=(span: ast::Span)}: ast::Spanned<internment::Intern<string>>)}: ddlog_std::Option<ast::Spanned<ast::Name>>), .parent=(_: ddlog_std::Option<ast::ExprId>), .elements=(_: ddlog_std::Option<ddlog_std::Vec<ast::IClassElement>>), .scope=(class_scope: ast::ScopeId), .exported=(_: bool)}: inputs::Class)], ((var meta: ddlog_std::Ref<var_decls::VariableMeta>) = ((ddlog_std::ref_new: function(var_decls::VariableMeta):ddlog_std::Ref<var_decls::VariableMeta>)((var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=(ddlog_std::Some{.x=span}: ddlog_std::Option<ast::Span>)}: var_decls::VariableMeta)))), ((var id: ast::AnyId) = (ast::AnyIdClass{.class=class}: ast::AnyId)), ((var scope: var_decls::DeclarationScope) = (var_decls::Unhoistable{.scope=class_scope}: var_decls::DeclarationScope)). */
                                                                                                                              program::Rule::CollectionRule {
                                                                                                                                  description: std::borrow::Cow::from("var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::Class(.id=class, .file=file, .name=ddlog_std::Some{.x=ast::Spanned{.data=name, .span=span}}, .parent=_, .elements=_, .scope=class_scope, .exported=_), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdClass{.class=class}), (var scope = var_decls::Unhoistable{.scope=class_scope})."),
                                                                                                                                  rel: 17,
                                                                                                                                  xform: Some(XFormCollection::FilterMap{
                                                                                                                                                  description: std::borrow::Cow::from("head of var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::Class(.id=class, .file=file, .name=ddlog_std::Some{.x=ast::Spanned{.data=name, .span=span}}, .parent=_, .elements=_, .scope=class_scope, .exported=_), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdClass{.class=class}), (var scope = var_decls::Unhoistable{.scope=class_scope})."),
                                                                                                                                                  fmfun: {fn __f(__v: DDValue) -> Option<DDValue>
                                                                                                                                                  {
                                                                                                                                                      let (ref class, ref file, ref name, ref span, ref class_scope) = match *<types__inputs::Class>::from_ddvalue_ref(&__v) {
                                                                                                                                                          types__inputs::Class{id: ref class, file: ref file, name: ddlog_std::Option::Some{x: types__ast::Spanned{data: ref name, span: ref span}}, parent: _, elements: _, scope: ref class_scope, exported: _} => ((*class).clone(), (*file).clone(), (*name).clone(), (*span).clone(), (*class_scope).clone()),
                                                                                                                                                          _ => return None
                                                                                                                                                      };
                                                                                                                                                      let ref meta: ddlog_std::Ref<VariableMeta> = match ddlog_std::ref_new((&(VariableMeta{is_function_argument: false, implicitly_declared: false, declaration_span: (ddlog_std::Option::Some{x: (*span).clone()})}))) {
                                                                                                                                                          meta => meta,
                                                                                                                                                          _ => return None
                                                                                                                                                      };
                                                                                                                                                      let ref id: types__ast::AnyId = match (types__ast::AnyId::AnyIdClass{class: (*class).clone()}) {
                                                                                                                                                          id => id,
                                                                                                                                                          _ => return None
                                                                                                                                                      };
                                                                                                                                                      let ref scope: DeclarationScope = match (DeclarationScope::Unhoistable{scope: (*class_scope).clone()}) {
                                                                                                                                                          scope => scope,
                                                                                                                                                          _ => return None
                                                                                                                                                      };
                                                                                                                                                      Some(((VariableDeclarations{file: (*file).clone(), name: (*name).clone(), scope: (*scope).clone(), declared_in: (*id).clone(), meta: (*meta).clone()})).into_ddvalue())
                                                                                                                                                  }
                                                                                                                                                  __f},
                                                                                                                                                  next: Box::new(None)
                                                                                                                                              })
                                                                                                                              },
    );
pub static __Rule_var_decls_VariableDeclarations_4: ::once_cell::sync::Lazy<program::Rule> =
    ::once_cell::sync::Lazy::new(
        || /* var_decls::VariableDeclarations[(var_decls::VariableDeclarations{.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta}: var_decls::VariableDeclarations)] :- inputs::LetDecl[(inputs::LetDecl{.stmt_id=(stmt: ast::StmtId), .file=(file: ast::FileId), .pattern=(ddlog_std::Some{.x=(pat: internment::Intern<ast::Pattern>)}: ddlog_std::Option<ast::IPattern>), .value=(_: ddlog_std::Option<ast::ExprId>), .exported=(_: bool)}: inputs::LetDecl)], inputs::Statement[(inputs::Statement{.id=(stmt: ast::StmtId), .file=(file: ast::FileId), .kind=(_: ast::StmtKind), .scope=(stmt_scope: ast::ScopeId), .span=(_: ast::Span)}: inputs::Statement)], var bound = FlatMap(((ast::bound_vars: function(internment::Intern<ast::Pattern>):ddlog_std::Vec<ast::Spanned<ast::Name>>)(pat))), ((ast::Spanned{.data=(var name: internment::Intern<string>), .span=(var span: ast::Span)}: ast::Spanned<internment::Intern<string>>) = bound), ((var meta: ddlog_std::Ref<var_decls::VariableMeta>) = ((ddlog_std::ref_new: function(var_decls::VariableMeta):ddlog_std::Ref<var_decls::VariableMeta>)((var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=(ddlog_std::Some{.x=span}: ddlog_std::Option<ast::Span>)}: var_decls::VariableMeta)))), ((var id: ast::AnyId) = (ast::AnyIdStmt{.stmt=stmt}: ast::AnyId)), ((var scope: var_decls::DeclarationScope) = (var_decls::Unhoistable{.scope=stmt_scope}: var_decls::DeclarationScope)). */
                                                                                                                              program::Rule::ArrangementRule {
                                                                                                                                  description: std::borrow::Cow::from( "var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::LetDecl(.stmt_id=stmt, .file=file, .pattern=ddlog_std::Some{.x=pat}, .value=_, .exported=_), inputs::Statement(.id=stmt, .file=file, .kind=_, .scope=stmt_scope, .span=_), var bound = FlatMap((ast::bound_vars(pat))), (ast::Spanned{.data=var name, .span=var span} = bound), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdStmt{.stmt=stmt}), (var scope = var_decls::Unhoistable{.scope=stmt_scope})."),
                                                                                                                                  arr: ( 43, 0),
                                                                                                                                  xform: XFormArrangement::Join{
                                                                                                                                             description: std::borrow::Cow::from("inputs::LetDecl(.stmt_id=stmt, .file=file, .pattern=ddlog_std::Some{.x=pat}, .value=_, .exported=_), inputs::Statement(.id=stmt, .file=file, .kind=_, .scope=stmt_scope, .span=_)"),
                                                                                                                                             ffun: None,
                                                                                                                                             arrangement: (48,0),
                                                                                                                                             jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                             {
                                                                                                                                                 let (ref stmt, ref file, ref pat) = match *<types__inputs::LetDecl>::from_ddvalue_ref(__v1) {
                                                                                                                                                     types__inputs::LetDecl{stmt_id: ref stmt, file: ref file, pattern: ddlog_std::Option::Some{x: ref pat}, value: _, exported: _} => ((*stmt).clone(), (*file).clone(), (*pat).clone()),
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 let ref stmt_scope = match *<types__inputs::Statement>::from_ddvalue_ref(__v2) {
                                                                                                                                                     types__inputs::Statement{id: _, file: _, kind: _, scope: ref stmt_scope, span: _} => (*stmt_scope).clone(),
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 Some((ddlog_std::tuple4((*stmt).clone(), (*file).clone(), (*pat).clone(), (*stmt_scope).clone())).into_ddvalue())
                                                                                                                                             }
                                                                                                                                             __f},
                                                                                                                                             next: Box::new(Some(XFormCollection::FlatMap{
                                                                                                                                                                     description: std::borrow::Cow::from("inputs::LetDecl(.stmt_id=stmt, .file=file, .pattern=ddlog_std::Some{.x=pat}, .value=_, .exported=_), inputs::Statement(.id=stmt, .file=file, .kind=_, .scope=stmt_scope, .span=_), var bound = FlatMap((ast::bound_vars(pat)))"),
                                                                                                                                                                     fmfun: {fn __f(__v: DDValue) -> Option<Box<dyn Iterator<Item=DDValue>>>
                                                                                                                                                                     {
                                                                                                                                                                         let ddlog_std::tuple4(ref stmt, ref file, ref pat, ref stmt_scope) = *<ddlog_std::tuple4<types__ast::StmtId, types__ast::FileId, internment::Intern<types__ast::Pattern>, types__ast::ScopeId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                         let __flattened = types__ast::bound_vars_internment_Intern__ast_Pattern_ddlog_std_Vec__ast_Spanned__internment_Intern____Stringval(pat);
                                                                                                                                                                         let stmt = (*stmt).clone();
                                                                                                                                                                         let file = (*file).clone();
                                                                                                                                                                         let stmt_scope = (*stmt_scope).clone();
                                                                                                                                                                         Some(Box::new(__flattened.into_iter().map(move |bound|(ddlog_std::tuple4(bound.clone(), stmt.clone(), file.clone(), stmt_scope.clone())).into_ddvalue())))
                                                                                                                                                                     }
                                                                                                                                                                     __f},
                                                                                                                                                                     next: Box::new(Some(XFormCollection::FilterMap{
                                                                                                                                                                                             description: std::borrow::Cow::from("head of var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::LetDecl(.stmt_id=stmt, .file=file, .pattern=ddlog_std::Some{.x=pat}, .value=_, .exported=_), inputs::Statement(.id=stmt, .file=file, .kind=_, .scope=stmt_scope, .span=_), var bound = FlatMap((ast::bound_vars(pat))), (ast::Spanned{.data=var name, .span=var span} = bound), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdStmt{.stmt=stmt}), (var scope = var_decls::Unhoistable{.scope=stmt_scope})."),
                                                                                                                                                                                             fmfun: {fn __f(__v: DDValue) -> Option<DDValue>
                                                                                                                                                                                             {
                                                                                                                                                                                                 let ddlog_std::tuple4(ref bound, ref stmt, ref file, ref stmt_scope) = *<ddlog_std::tuple4<types__ast::Spanned<internment::Intern<String>>, types__ast::StmtId, types__ast::FileId, types__ast::ScopeId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                                                 let (ref name, ref span): (internment::Intern<String>, types__ast::Span) = match (*bound).clone() {
                                                                                                                                                                                                     types__ast::Spanned{data: name, span: span} => (name, span),
                                                                                                                                                                                                     _ => return None
                                                                                                                                                                                                 };
                                                                                                                                                                                                 let ref meta: ddlog_std::Ref<VariableMeta> = match ddlog_std::ref_new((&(VariableMeta{is_function_argument: false, implicitly_declared: false, declaration_span: (ddlog_std::Option::Some{x: (*span).clone()})}))) {
                                                                                                                                                                                                     meta => meta,
                                                                                                                                                                                                     _ => return None
                                                                                                                                                                                                 };
                                                                                                                                                                                                 let ref id: types__ast::AnyId = match (types__ast::AnyId::AnyIdStmt{stmt: (*stmt).clone()}) {
                                                                                                                                                                                                     id => id,
                                                                                                                                                                                                     _ => return None
                                                                                                                                                                                                 };
                                                                                                                                                                                                 let ref scope: DeclarationScope = match (DeclarationScope::Unhoistable{scope: (*stmt_scope).clone()}) {
                                                                                                                                                                                                     scope => scope,
                                                                                                                                                                                                     _ => return None
                                                                                                                                                                                                 };
                                                                                                                                                                                                 Some(((VariableDeclarations{file: (*file).clone(), name: (*name).clone(), scope: (*scope).clone(), declared_in: (*id).clone(), meta: (*meta).clone()})).into_ddvalue())
                                                                                                                                                                                             }
                                                                                                                                                                                             __f},
                                                                                                                                                                                             next: Box::new(None)
                                                                                                                                                                                         }))
                                                                                                                                                                 }))
                                                                                                                                         }
                                                                                                                              },
    );
pub static __Rule_var_decls_VariableDeclarations_5: ::once_cell::sync::Lazy<program::Rule> =
    ::once_cell::sync::Lazy::new(
        || /* var_decls::VariableDeclarations[(var_decls::VariableDeclarations{.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta}: var_decls::VariableDeclarations)] :- inputs::ConstDecl[(inputs::ConstDecl{.stmt_id=(stmt: ast::StmtId), .file=(file: ast::FileId), .pattern=(ddlog_std::Some{.x=(pat: internment::Intern<ast::Pattern>)}: ddlog_std::Option<ast::IPattern>), .value=(_: ddlog_std::Option<ast::ExprId>), .exported=(_: bool)}: inputs::ConstDecl)], inputs::Statement[(inputs::Statement{.id=(stmt: ast::StmtId), .file=(file: ast::FileId), .kind=(_: ast::StmtKind), .scope=(stmt_scope: ast::ScopeId), .span=(_: ast::Span)}: inputs::Statement)], var bound = FlatMap(((ast::bound_vars: function(internment::Intern<ast::Pattern>):ddlog_std::Vec<ast::Spanned<ast::Name>>)(pat))), ((ast::Spanned{.data=(var name: internment::Intern<string>), .span=(var span: ast::Span)}: ast::Spanned<internment::Intern<string>>) = bound), ((var meta: ddlog_std::Ref<var_decls::VariableMeta>) = ((ddlog_std::ref_new: function(var_decls::VariableMeta):ddlog_std::Ref<var_decls::VariableMeta>)((var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=(ddlog_std::Some{.x=span}: ddlog_std::Option<ast::Span>)}: var_decls::VariableMeta)))), ((var id: ast::AnyId) = (ast::AnyIdStmt{.stmt=stmt}: ast::AnyId)), ((var scope: var_decls::DeclarationScope) = (var_decls::Unhoistable{.scope=stmt_scope}: var_decls::DeclarationScope)). */
                                                                                                                              program::Rule::ArrangementRule {
                                                                                                                                  description: std::borrow::Cow::from( "var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::ConstDecl(.stmt_id=stmt, .file=file, .pattern=ddlog_std::Some{.x=pat}, .value=_, .exported=_), inputs::Statement(.id=stmt, .file=file, .kind=_, .scope=stmt_scope, .span=_), var bound = FlatMap((ast::bound_vars(pat))), (ast::Spanned{.data=var name, .span=var span} = bound), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdStmt{.stmt=stmt}), (var scope = var_decls::Unhoistable{.scope=stmt_scope})."),
                                                                                                                                  arr: ( 19, 0),
                                                                                                                                  xform: XFormArrangement::Join{
                                                                                                                                             description: std::borrow::Cow::from("inputs::ConstDecl(.stmt_id=stmt, .file=file, .pattern=ddlog_std::Some{.x=pat}, .value=_, .exported=_), inputs::Statement(.id=stmt, .file=file, .kind=_, .scope=stmt_scope, .span=_)"),
                                                                                                                                             ffun: None,
                                                                                                                                             arrangement: (48,0),
                                                                                                                                             jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                             {
                                                                                                                                                 let (ref stmt, ref file, ref pat) = match *<types__inputs::ConstDecl>::from_ddvalue_ref(__v1) {
                                                                                                                                                     types__inputs::ConstDecl{stmt_id: ref stmt, file: ref file, pattern: ddlog_std::Option::Some{x: ref pat}, value: _, exported: _} => ((*stmt).clone(), (*file).clone(), (*pat).clone()),
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 let ref stmt_scope = match *<types__inputs::Statement>::from_ddvalue_ref(__v2) {
                                                                                                                                                     types__inputs::Statement{id: _, file: _, kind: _, scope: ref stmt_scope, span: _} => (*stmt_scope).clone(),
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 Some((ddlog_std::tuple4((*stmt).clone(), (*file).clone(), (*pat).clone(), (*stmt_scope).clone())).into_ddvalue())
                                                                                                                                             }
                                                                                                                                             __f},
                                                                                                                                             next: Box::new(Some(XFormCollection::FlatMap{
                                                                                                                                                                     description: std::borrow::Cow::from("inputs::ConstDecl(.stmt_id=stmt, .file=file, .pattern=ddlog_std::Some{.x=pat}, .value=_, .exported=_), inputs::Statement(.id=stmt, .file=file, .kind=_, .scope=stmt_scope, .span=_), var bound = FlatMap((ast::bound_vars(pat)))"),
                                                                                                                                                                     fmfun: {fn __f(__v: DDValue) -> Option<Box<dyn Iterator<Item=DDValue>>>
                                                                                                                                                                     {
                                                                                                                                                                         let ddlog_std::tuple4(ref stmt, ref file, ref pat, ref stmt_scope) = *<ddlog_std::tuple4<types__ast::StmtId, types__ast::FileId, internment::Intern<types__ast::Pattern>, types__ast::ScopeId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                         let __flattened = types__ast::bound_vars_internment_Intern__ast_Pattern_ddlog_std_Vec__ast_Spanned__internment_Intern____Stringval(pat);
                                                                                                                                                                         let stmt = (*stmt).clone();
                                                                                                                                                                         let file = (*file).clone();
                                                                                                                                                                         let stmt_scope = (*stmt_scope).clone();
                                                                                                                                                                         Some(Box::new(__flattened.into_iter().map(move |bound|(ddlog_std::tuple4(bound.clone(), stmt.clone(), file.clone(), stmt_scope.clone())).into_ddvalue())))
                                                                                                                                                                     }
                                                                                                                                                                     __f},
                                                                                                                                                                     next: Box::new(Some(XFormCollection::FilterMap{
                                                                                                                                                                                             description: std::borrow::Cow::from("head of var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::ConstDecl(.stmt_id=stmt, .file=file, .pattern=ddlog_std::Some{.x=pat}, .value=_, .exported=_), inputs::Statement(.id=stmt, .file=file, .kind=_, .scope=stmt_scope, .span=_), var bound = FlatMap((ast::bound_vars(pat))), (ast::Spanned{.data=var name, .span=var span} = bound), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdStmt{.stmt=stmt}), (var scope = var_decls::Unhoistable{.scope=stmt_scope})."),
                                                                                                                                                                                             fmfun: {fn __f(__v: DDValue) -> Option<DDValue>
                                                                                                                                                                                             {
                                                                                                                                                                                                 let ddlog_std::tuple4(ref bound, ref stmt, ref file, ref stmt_scope) = *<ddlog_std::tuple4<types__ast::Spanned<internment::Intern<String>>, types__ast::StmtId, types__ast::FileId, types__ast::ScopeId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                                                 let (ref name, ref span): (internment::Intern<String>, types__ast::Span) = match (*bound).clone() {
                                                                                                                                                                                                     types__ast::Spanned{data: name, span: span} => (name, span),
                                                                                                                                                                                                     _ => return None
                                                                                                                                                                                                 };
                                                                                                                                                                                                 let ref meta: ddlog_std::Ref<VariableMeta> = match ddlog_std::ref_new((&(VariableMeta{is_function_argument: false, implicitly_declared: false, declaration_span: (ddlog_std::Option::Some{x: (*span).clone()})}))) {
                                                                                                                                                                                                     meta => meta,
                                                                                                                                                                                                     _ => return None
                                                                                                                                                                                                 };
                                                                                                                                                                                                 let ref id: types__ast::AnyId = match (types__ast::AnyId::AnyIdStmt{stmt: (*stmt).clone()}) {
                                                                                                                                                                                                     id => id,
                                                                                                                                                                                                     _ => return None
                                                                                                                                                                                                 };
                                                                                                                                                                                                 let ref scope: DeclarationScope = match (DeclarationScope::Unhoistable{scope: (*stmt_scope).clone()}) {
                                                                                                                                                                                                     scope => scope,
                                                                                                                                                                                                     _ => return None
                                                                                                                                                                                                 };
                                                                                                                                                                                                 Some(((VariableDeclarations{file: (*file).clone(), name: (*name).clone(), scope: (*scope).clone(), declared_in: (*id).clone(), meta: (*meta).clone()})).into_ddvalue())
                                                                                                                                                                                             }
                                                                                                                                                                                             __f},
                                                                                                                                                                                             next: Box::new(None)
                                                                                                                                                                                         }))
                                                                                                                                                                 }))
                                                                                                                                         }
                                                                                                                              },
    );
pub static __Rule_var_decls_VariableDeclarations_6: ::once_cell::sync::Lazy<program::Rule> =
    ::once_cell::sync::Lazy::new(
        || /* var_decls::VariableDeclarations[(var_decls::VariableDeclarations{.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta}: var_decls::VariableDeclarations)] :- inputs::VarDecl[(inputs::VarDecl{.stmt_id=(stmt: ast::StmtId), .file=(file: ast::FileId), .pattern=(ddlog_std::Some{.x=(pat: internment::Intern<ast::Pattern>)}: ddlog_std::Option<ast::IPattern>), .value=(_: ddlog_std::Option<ast::ExprId>), .exported=(_: bool)}: inputs::VarDecl)], inputs::Statement[(inputs::Statement{.id=(stmt: ast::StmtId), .file=(file: ast::FileId), .kind=(_: ast::StmtKind), .scope=(stmt_scope: ast::ScopeId), .span=(_: ast::Span)}: inputs::Statement)], scopes::FunctionLevelScope[(scopes::FunctionLevelScope{.scope=(stmt_scope: ast::ScopeId), .nearest=(nearest_scope: ast::ScopeId), .file=(file: ast::FileId), .id=(_: ast::AnyId)}: scopes::FunctionLevelScope)], var bound = FlatMap(((ast::bound_vars: function(internment::Intern<ast::Pattern>):ddlog_std::Vec<ast::Spanned<ast::Name>>)(pat))), ((ast::Spanned{.data=(var name: internment::Intern<string>), .span=(var span: ast::Span)}: ast::Spanned<internment::Intern<string>>) = bound), ((var meta: ddlog_std::Ref<var_decls::VariableMeta>) = ((ddlog_std::ref_new: function(var_decls::VariableMeta):ddlog_std::Ref<var_decls::VariableMeta>)((var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=(ddlog_std::Some{.x=span}: ddlog_std::Option<ast::Span>)}: var_decls::VariableMeta)))), ((var id: ast::AnyId) = (ast::AnyIdStmt{.stmt=stmt}: ast::AnyId)), ((var scope: var_decls::DeclarationScope) = (var_decls::Hoistable{.hoisted=nearest_scope, .unhoisted=stmt_scope}: var_decls::DeclarationScope)). */
                                                                                                                              program::Rule::ArrangementRule {
                                                                                                                                  description: std::borrow::Cow::from( "var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::VarDecl(.stmt_id=stmt, .file=file, .pattern=ddlog_std::Some{.x=pat}, .value=_, .exported=_), inputs::Statement(.id=stmt, .file=file, .kind=_, .scope=stmt_scope, .span=_), scopes::FunctionLevelScope(.scope=stmt_scope, .nearest=nearest_scope, .file=file, .id=_), var bound = FlatMap((ast::bound_vars(pat))), (ast::Spanned{.data=var name, .span=var span} = bound), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdStmt{.stmt=stmt}), (var scope = var_decls::Hoistable{.hoisted=nearest_scope, .unhoisted=stmt_scope})."),
                                                                                                                                  arr: ( 57, 0),
                                                                                                                                  xform: XFormArrangement::Join{
                                                                                                                                             description: std::borrow::Cow::from("inputs::VarDecl(.stmt_id=stmt, .file=file, .pattern=ddlog_std::Some{.x=pat}, .value=_, .exported=_), inputs::Statement(.id=stmt, .file=file, .kind=_, .scope=stmt_scope, .span=_)"),
                                                                                                                                             ffun: None,
                                                                                                                                             arrangement: (48,0),
                                                                                                                                             jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                             {
                                                                                                                                                 let (ref stmt, ref file, ref pat) = match *<types__inputs::VarDecl>::from_ddvalue_ref(__v1) {
                                                                                                                                                     types__inputs::VarDecl{stmt_id: ref stmt, file: ref file, pattern: ddlog_std::Option::Some{x: ref pat}, value: _, exported: _} => ((*stmt).clone(), (*file).clone(), (*pat).clone()),
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 let ref stmt_scope = match *<types__inputs::Statement>::from_ddvalue_ref(__v2) {
                                                                                                                                                     types__inputs::Statement{id: _, file: _, kind: _, scope: ref stmt_scope, span: _} => (*stmt_scope).clone(),
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 Some((ddlog_std::tuple4((*stmt).clone(), (*file).clone(), (*pat).clone(), (*stmt_scope).clone())).into_ddvalue())
                                                                                                                                             }
                                                                                                                                             __f},
                                                                                                                                             next: Box::new(Some(XFormCollection::Arrange {
                                                                                                                                                                     description: std::borrow::Cow::from("arrange inputs::VarDecl(.stmt_id=stmt, .file=file, .pattern=ddlog_std::Some{.x=pat}, .value=_, .exported=_), inputs::Statement(.id=stmt, .file=file, .kind=_, .scope=stmt_scope, .span=_) by (stmt_scope, file)"),
                                                                                                                                                                     afun: {fn __f(__v: DDValue) -> Option<(DDValue,DDValue)>
                                                                                                                                                                     {
                                                                                                                                                                         let ddlog_std::tuple4(ref stmt, ref file, ref pat, ref stmt_scope) = *<ddlog_std::tuple4<types__ast::StmtId, types__ast::FileId, internment::Intern<types__ast::Pattern>, types__ast::ScopeId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                         Some(((ddlog_std::tuple2((*stmt_scope).clone(), (*file).clone())).into_ddvalue(), (ddlog_std::tuple4((*stmt).clone(), (*file).clone(), (*pat).clone(), (*stmt_scope).clone())).into_ddvalue()))
                                                                                                                                                                     }
                                                                                                                                                                     __f},
                                                                                                                                                                     next: Box::new(XFormArrangement::Join{
                                                                                                                                                                                        description: std::borrow::Cow::from("inputs::VarDecl(.stmt_id=stmt, .file=file, .pattern=ddlog_std::Some{.x=pat}, .value=_, .exported=_), inputs::Statement(.id=stmt, .file=file, .kind=_, .scope=stmt_scope, .span=_), scopes::FunctionLevelScope(.scope=stmt_scope, .nearest=nearest_scope, .file=file, .id=_)"),
                                                                                                                                                                                        ffun: None,
                                                                                                                                                                                        arrangement: (80,0),
                                                                                                                                                                                        jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                                                                        {
                                                                                                                                                                                            let ddlog_std::tuple4(ref stmt, ref file, ref pat, ref stmt_scope) = *<ddlog_std::tuple4<types__ast::StmtId, types__ast::FileId, internment::Intern<types__ast::Pattern>, types__ast::ScopeId>>::from_ddvalue_ref( __v1 );
                                                                                                                                                                                            let ref nearest_scope = match *<types__scopes::FunctionLevelScope>::from_ddvalue_ref(__v2) {
                                                                                                                                                                                                types__scopes::FunctionLevelScope{scope: _, nearest: ref nearest_scope, file: _, id: _} => (*nearest_scope).clone(),
                                                                                                                                                                                                _ => return None
                                                                                                                                                                                            };
                                                                                                                                                                                            Some((ddlog_std::tuple5((*stmt).clone(), (*file).clone(), (*pat).clone(), (*stmt_scope).clone(), (*nearest_scope).clone())).into_ddvalue())
                                                                                                                                                                                        }
                                                                                                                                                                                        __f},
                                                                                                                                                                                        next: Box::new(Some(XFormCollection::FlatMap{
                                                                                                                                                                                                                description: std::borrow::Cow::from("inputs::VarDecl(.stmt_id=stmt, .file=file, .pattern=ddlog_std::Some{.x=pat}, .value=_, .exported=_), inputs::Statement(.id=stmt, .file=file, .kind=_, .scope=stmt_scope, .span=_), scopes::FunctionLevelScope(.scope=stmt_scope, .nearest=nearest_scope, .file=file, .id=_), var bound = FlatMap((ast::bound_vars(pat)))"),
                                                                                                                                                                                                                fmfun: {fn __f(__v: DDValue) -> Option<Box<dyn Iterator<Item=DDValue>>>
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    let ddlog_std::tuple5(ref stmt, ref file, ref pat, ref stmt_scope, ref nearest_scope) = *<ddlog_std::tuple5<types__ast::StmtId, types__ast::FileId, internment::Intern<types__ast::Pattern>, types__ast::ScopeId, types__ast::ScopeId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                                                                    let __flattened = types__ast::bound_vars_internment_Intern__ast_Pattern_ddlog_std_Vec__ast_Spanned__internment_Intern____Stringval(pat);
                                                                                                                                                                                                                    let stmt = (*stmt).clone();
                                                                                                                                                                                                                    let file = (*file).clone();
                                                                                                                                                                                                                    let stmt_scope = (*stmt_scope).clone();
                                                                                                                                                                                                                    let nearest_scope = (*nearest_scope).clone();
                                                                                                                                                                                                                    Some(Box::new(__flattened.into_iter().map(move |bound|(ddlog_std::tuple5(bound.clone(), stmt.clone(), file.clone(), stmt_scope.clone(), nearest_scope.clone())).into_ddvalue())))
                                                                                                                                                                                                                }
                                                                                                                                                                                                                __f},
                                                                                                                                                                                                                next: Box::new(Some(XFormCollection::FilterMap{
                                                                                                                                                                                                                                        description: std::borrow::Cow::from("head of var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::VarDecl(.stmt_id=stmt, .file=file, .pattern=ddlog_std::Some{.x=pat}, .value=_, .exported=_), inputs::Statement(.id=stmt, .file=file, .kind=_, .scope=stmt_scope, .span=_), scopes::FunctionLevelScope(.scope=stmt_scope, .nearest=nearest_scope, .file=file, .id=_), var bound = FlatMap((ast::bound_vars(pat))), (ast::Spanned{.data=var name, .span=var span} = bound), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdStmt{.stmt=stmt}), (var scope = var_decls::Hoistable{.hoisted=nearest_scope, .unhoisted=stmt_scope})."),
                                                                                                                                                                                                                                        fmfun: {fn __f(__v: DDValue) -> Option<DDValue>
                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                            let ddlog_std::tuple5(ref bound, ref stmt, ref file, ref stmt_scope, ref nearest_scope) = *<ddlog_std::tuple5<types__ast::Spanned<internment::Intern<String>>, types__ast::StmtId, types__ast::FileId, types__ast::ScopeId, types__ast::ScopeId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                                                                                            let (ref name, ref span): (internment::Intern<String>, types__ast::Span) = match (*bound).clone() {
                                                                                                                                                                                                                                                types__ast::Spanned{data: name, span: span} => (name, span),
                                                                                                                                                                                                                                                _ => return None
                                                                                                                                                                                                                                            };
                                                                                                                                                                                                                                            let ref meta: ddlog_std::Ref<VariableMeta> = match ddlog_std::ref_new((&(VariableMeta{is_function_argument: false, implicitly_declared: false, declaration_span: (ddlog_std::Option::Some{x: (*span).clone()})}))) {
                                                                                                                                                                                                                                                meta => meta,
                                                                                                                                                                                                                                                _ => return None
                                                                                                                                                                                                                                            };
                                                                                                                                                                                                                                            let ref id: types__ast::AnyId = match (types__ast::AnyId::AnyIdStmt{stmt: (*stmt).clone()}) {
                                                                                                                                                                                                                                                id => id,
                                                                                                                                                                                                                                                _ => return None
                                                                                                                                                                                                                                            };
                                                                                                                                                                                                                                            let ref scope: DeclarationScope = match (DeclarationScope::Hoistable{hoisted: (*nearest_scope).clone(), unhoisted: (*stmt_scope).clone()}) {
                                                                                                                                                                                                                                                scope => scope,
                                                                                                                                                                                                                                                _ => return None
                                                                                                                                                                                                                                            };
                                                                                                                                                                                                                                            Some(((VariableDeclarations{file: (*file).clone(), name: (*name).clone(), scope: (*scope).clone(), declared_in: (*id).clone(), meta: (*meta).clone()})).into_ddvalue())
                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                        __f},
                                                                                                                                                                                                                                        next: Box::new(None)
                                                                                                                                                                                                                                    }))
                                                                                                                                                                                                            }))
                                                                                                                                                                                    })
                                                                                                                                                                 }))
                                                                                                                                         }
                                                                                                                              },
    );
pub static __Rule_var_decls_VariableDeclarations_7: ::once_cell::sync::Lazy<program::Rule> =
    ::once_cell::sync::Lazy::new(
        || /* var_decls::VariableDeclarations[(var_decls::VariableDeclarations{.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta}: var_decls::VariableDeclarations)] :- inputs::Function[(inputs::Function{.id=(func: ast::FuncId), .file=(file: ast::FileId), .name=(ddlog_std::Some{.x=(ast::Spanned{.data=(name: internment::Intern<string>), .span=(span: ast::Span)}: ast::Spanned<internment::Intern<string>>)}: ddlog_std::Option<ast::Spanned<ast::Name>>), .scope=(func_scope: ast::ScopeId), .body=(_: ast::ScopeId), .exported=(_: bool)}: inputs::Function)], scopes::FunctionLevelScope[(scopes::FunctionLevelScope{.scope=(func_scope: ast::ScopeId), .nearest=(nearest_scope: ast::ScopeId), .file=(file: ast::FileId), .id=(_: ast::AnyId)}: scopes::FunctionLevelScope)], ((var meta: ddlog_std::Ref<var_decls::VariableMeta>) = ((ddlog_std::ref_new: function(var_decls::VariableMeta):ddlog_std::Ref<var_decls::VariableMeta>)((var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=(ddlog_std::Some{.x=span}: ddlog_std::Option<ast::Span>)}: var_decls::VariableMeta)))), ((var id: ast::AnyId) = (ast::AnyIdFunc{.func=func}: ast::AnyId)), ((var scope: var_decls::DeclarationScope) = (var_decls::Hoistable{.hoisted=nearest_scope, .unhoisted=func_scope}: var_decls::DeclarationScope)). */
                                                                                                                              program::Rule::ArrangementRule {
                                                                                                                                  description: std::borrow::Cow::from( "var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::Function(.id=func, .file=file, .name=ddlog_std::Some{.x=ast::Spanned{.data=name, .span=span}}, .scope=func_scope, .body=_, .exported=_), scopes::FunctionLevelScope(.scope=func_scope, .nearest=nearest_scope, .file=file, .id=_), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdFunc{.func=func}), (var scope = var_decls::Hoistable{.hoisted=nearest_scope, .unhoisted=func_scope})."),
                                                                                                                                  arr: ( 34, 2),
                                                                                                                                  xform: XFormArrangement::Join{
                                                                                                                                             description: std::borrow::Cow::from("inputs::Function(.id=func, .file=file, .name=ddlog_std::Some{.x=ast::Spanned{.data=name, .span=span}}, .scope=func_scope, .body=_, .exported=_), scopes::FunctionLevelScope(.scope=func_scope, .nearest=nearest_scope, .file=file, .id=_)"),
                                                                                                                                             ffun: None,
                                                                                                                                             arrangement: (80,0),
                                                                                                                                             jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                             {
                                                                                                                                                 let (ref func, ref file, ref name, ref span, ref func_scope) = match *<types__inputs::Function>::from_ddvalue_ref(__v1) {
                                                                                                                                                     types__inputs::Function{id: ref func, file: ref file, name: ddlog_std::Option::Some{x: types__ast::Spanned{data: ref name, span: ref span}}, scope: ref func_scope, body: _, exported: _} => ((*func).clone(), (*file).clone(), (*name).clone(), (*span).clone(), (*func_scope).clone()),
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 let ref nearest_scope = match *<types__scopes::FunctionLevelScope>::from_ddvalue_ref(__v2) {
                                                                                                                                                     types__scopes::FunctionLevelScope{scope: _, nearest: ref nearest_scope, file: _, id: _} => (*nearest_scope).clone(),
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 let ref meta: ddlog_std::Ref<VariableMeta> = match ddlog_std::ref_new((&(VariableMeta{is_function_argument: false, implicitly_declared: false, declaration_span: (ddlog_std::Option::Some{x: (*span).clone()})}))) {
                                                                                                                                                     meta => meta,
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 let ref id: types__ast::AnyId = match (types__ast::AnyId::AnyIdFunc{func: (*func).clone()}) {
                                                                                                                                                     id => id,
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 let ref scope: DeclarationScope = match (DeclarationScope::Hoistable{hoisted: (*nearest_scope).clone(), unhoisted: (*func_scope).clone()}) {
                                                                                                                                                     scope => scope,
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 Some(((VariableDeclarations{file: (*file).clone(), name: (*name).clone(), scope: (*scope).clone(), declared_in: (*id).clone(), meta: (*meta).clone()})).into_ddvalue())
                                                                                                                                             }
                                                                                                                                             __f},
                                                                                                                                             next: Box::new(None)
                                                                                                                                         }
                                                                                                                              },
    );
pub static __Rule_var_decls_VariableDeclarations_8: ::once_cell::sync::Lazy<program::Rule> =
    ::once_cell::sync::Lazy::new(
        || /* var_decls::VariableDeclarations[(var_decls::VariableDeclarations{.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta}: var_decls::VariableDeclarations)] :- inputs::FunctionArg[(inputs::FunctionArg{.parent_func=(func: ast::FuncId), .file=(file: ast::FileId), .pattern=(pat: internment::Intern<ast::Pattern>), .implicit=(implicit: bool)}: inputs::FunctionArg)], inputs::Function[(inputs::Function{.id=(func: ast::FuncId), .file=(file: ast::FileId), .name=(_: ddlog_std::Option<ast::Spanned<ast::Name>>), .scope=(_: ast::ScopeId), .body=(body: ast::ScopeId), .exported=(_: bool)}: inputs::Function)], var bound = FlatMap(((ast::bound_vars: function(internment::Intern<ast::Pattern>):ddlog_std::Vec<ast::Spanned<ast::Name>>)(pat))), ((ast::Spanned{.data=(var name: internment::Intern<string>), .span=(var span: ast::Span)}: ast::Spanned<internment::Intern<string>>) = bound), ((var meta: ddlog_std::Ref<var_decls::VariableMeta>) = ((ddlog_std::ref_new: function(var_decls::VariableMeta):ddlog_std::Ref<var_decls::VariableMeta>)((var_decls::VariableMeta{.is_function_argument=true, .implicitly_declared=implicit, .declaration_span=(ddlog_std::Some{.x=span}: ddlog_std::Option<ast::Span>)}: var_decls::VariableMeta)))), ((var id: ast::AnyId) = (ast::AnyIdFunc{.func=func}: ast::AnyId)), ((var scope: var_decls::DeclarationScope) = (var_decls::Unhoistable{.scope=body}: var_decls::DeclarationScope)). */
                                                                                                                              program::Rule::ArrangementRule {
                                                                                                                                  description: std::borrow::Cow::from( "var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::FunctionArg(.parent_func=func, .file=file, .pattern=pat, .implicit=implicit), inputs::Function(.id=func, .file=file, .name=_, .scope=_, .body=body, .exported=_), var bound = FlatMap((ast::bound_vars(pat))), (ast::Spanned{.data=var name, .span=var span} = bound), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=true, .implicitly_declared=implicit, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdFunc{.func=func}), (var scope = var_decls::Unhoistable{.scope=body})."),
                                                                                                                                  arr: ( 35, 0),
                                                                                                                                  xform: XFormArrangement::Join{
                                                                                                                                             description: std::borrow::Cow::from("inputs::FunctionArg(.parent_func=func, .file=file, .pattern=pat, .implicit=implicit), inputs::Function(.id=func, .file=file, .name=_, .scope=_, .body=body, .exported=_)"),
                                                                                                                                             ffun: None,
                                                                                                                                             arrangement: (34,3),
                                                                                                                                             jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                             {
                                                                                                                                                 let (ref func, ref file, ref pat, ref implicit) = match *<types__inputs::FunctionArg>::from_ddvalue_ref(__v1) {
                                                                                                                                                     types__inputs::FunctionArg{parent_func: ref func, file: ref file, pattern: ref pat, implicit: ref implicit} => ((*func).clone(), (*file).clone(), (*pat).clone(), (*implicit).clone()),
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 let ref body = match *<types__inputs::Function>::from_ddvalue_ref(__v2) {
                                                                                                                                                     types__inputs::Function{id: _, file: _, name: _, scope: _, body: ref body, exported: _} => (*body).clone(),
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 Some((ddlog_std::tuple5((*func).clone(), (*file).clone(), (*pat).clone(), (*implicit).clone(), (*body).clone())).into_ddvalue())
                                                                                                                                             }
                                                                                                                                             __f},
                                                                                                                                             next: Box::new(Some(XFormCollection::FlatMap{
                                                                                                                                                                     description: std::borrow::Cow::from("inputs::FunctionArg(.parent_func=func, .file=file, .pattern=pat, .implicit=implicit), inputs::Function(.id=func, .file=file, .name=_, .scope=_, .body=body, .exported=_), var bound = FlatMap((ast::bound_vars(pat)))"),
                                                                                                                                                                     fmfun: {fn __f(__v: DDValue) -> Option<Box<dyn Iterator<Item=DDValue>>>
                                                                                                                                                                     {
                                                                                                                                                                         let ddlog_std::tuple5(ref func, ref file, ref pat, ref implicit, ref body) = *<ddlog_std::tuple5<types__ast::FuncId, types__ast::FileId, internment::Intern<types__ast::Pattern>, bool, types__ast::ScopeId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                         let __flattened = types__ast::bound_vars_internment_Intern__ast_Pattern_ddlog_std_Vec__ast_Spanned__internment_Intern____Stringval(pat);
                                                                                                                                                                         let func = (*func).clone();
                                                                                                                                                                         let file = (*file).clone();
                                                                                                                                                                         let implicit = (*implicit).clone();
                                                                                                                                                                         let body = (*body).clone();
                                                                                                                                                                         Some(Box::new(__flattened.into_iter().map(move |bound|(ddlog_std::tuple5(bound.clone(), func.clone(), file.clone(), implicit.clone(), body.clone())).into_ddvalue())))
                                                                                                                                                                     }
                                                                                                                                                                     __f},
                                                                                                                                                                     next: Box::new(Some(XFormCollection::FilterMap{
                                                                                                                                                                                             description: std::borrow::Cow::from("head of var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::FunctionArg(.parent_func=func, .file=file, .pattern=pat, .implicit=implicit), inputs::Function(.id=func, .file=file, .name=_, .scope=_, .body=body, .exported=_), var bound = FlatMap((ast::bound_vars(pat))), (ast::Spanned{.data=var name, .span=var span} = bound), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=true, .implicitly_declared=implicit, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdFunc{.func=func}), (var scope = var_decls::Unhoistable{.scope=body})."),
                                                                                                                                                                                             fmfun: {fn __f(__v: DDValue) -> Option<DDValue>
                                                                                                                                                                                             {
                                                                                                                                                                                                 let ddlog_std::tuple5(ref bound, ref func, ref file, ref implicit, ref body) = *<ddlog_std::tuple5<types__ast::Spanned<internment::Intern<String>>, types__ast::FuncId, types__ast::FileId, bool, types__ast::ScopeId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                                                 let (ref name, ref span): (internment::Intern<String>, types__ast::Span) = match (*bound).clone() {
                                                                                                                                                                                                     types__ast::Spanned{data: name, span: span} => (name, span),
                                                                                                                                                                                                     _ => return None
                                                                                                                                                                                                 };
                                                                                                                                                                                                 let ref meta: ddlog_std::Ref<VariableMeta> = match ddlog_std::ref_new((&(VariableMeta{is_function_argument: true, implicitly_declared: (*implicit).clone(), declaration_span: (ddlog_std::Option::Some{x: (*span).clone()})}))) {
                                                                                                                                                                                                     meta => meta,
                                                                                                                                                                                                     _ => return None
                                                                                                                                                                                                 };
                                                                                                                                                                                                 let ref id: types__ast::AnyId = match (types__ast::AnyId::AnyIdFunc{func: (*func).clone()}) {
                                                                                                                                                                                                     id => id,
                                                                                                                                                                                                     _ => return None
                                                                                                                                                                                                 };
                                                                                                                                                                                                 let ref scope: DeclarationScope = match (DeclarationScope::Unhoistable{scope: (*body).clone()}) {
                                                                                                                                                                                                     scope => scope,
                                                                                                                                                                                                     _ => return None
                                                                                                                                                                                                 };
                                                                                                                                                                                                 Some(((VariableDeclarations{file: (*file).clone(), name: (*name).clone(), scope: (*scope).clone(), declared_in: (*id).clone(), meta: (*meta).clone()})).into_ddvalue())
                                                                                                                                                                                             }
                                                                                                                                                                                             __f},
                                                                                                                                                                                             next: Box::new(None)
                                                                                                                                                                                         }))
                                                                                                                                                                 }))
                                                                                                                                         }
                                                                                                                              },
    );
pub static __Rule_var_decls_VariableDeclarations_9: ::once_cell::sync::Lazy<program::Rule> =
    ::once_cell::sync::Lazy::new(
        || /* var_decls::VariableDeclarations[(var_decls::VariableDeclarations{.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta}: var_decls::VariableDeclarations)] :- inputs::ArrowParam[(inputs::ArrowParam{.expr_id=(expr: ast::ExprId), .file=(file: ast::FileId), .param=(pat: internment::Intern<ast::Pattern>)}: inputs::ArrowParam)], inputs::Arrow[(inputs::Arrow{.expr_id=(expr: ast::ExprId), .file=(file: ast::FileId), .body=(ddlog_std::Some{.x=((_: ddlog_std::Either<ast::ExprId,ast::StmtId>), (body_scope: ast::ScopeId))}: ddlog_std::Option<(ddlog_std::Either<ast::ExprId,ast::StmtId>, ast::ScopeId)>)}: inputs::Arrow)], var bound = FlatMap(((ast::bound_vars: function(internment::Intern<ast::Pattern>):ddlog_std::Vec<ast::Spanned<ast::Name>>)(pat))), ((ast::Spanned{.data=(var name: internment::Intern<string>), .span=(var span: ast::Span)}: ast::Spanned<internment::Intern<string>>) = bound), ((var meta: ddlog_std::Ref<var_decls::VariableMeta>) = ((ddlog_std::ref_new: function(var_decls::VariableMeta):ddlog_std::Ref<var_decls::VariableMeta>)((var_decls::VariableMeta{.is_function_argument=true, .implicitly_declared=false, .declaration_span=(ddlog_std::Some{.x=span}: ddlog_std::Option<ast::Span>)}: var_decls::VariableMeta)))), ((var id: ast::AnyId) = (ast::AnyIdExpr{.expr=expr}: ast::AnyId)), ((var scope: var_decls::DeclarationScope) = (var_decls::Unhoistable{.scope=body_scope}: var_decls::DeclarationScope)). */
                                                                                                                              program::Rule::ArrangementRule {
                                                                                                                                  description: std::borrow::Cow::from( "var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::ArrowParam(.expr_id=expr, .file=file, .param=pat), inputs::Arrow(.expr_id=expr, .file=file, .body=ddlog_std::Some{.x=(_, body_scope)}), var bound = FlatMap((ast::bound_vars(pat))), (ast::Spanned{.data=var name, .span=var span} = bound), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=true, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdExpr{.expr=expr}), (var scope = var_decls::Unhoistable{.scope=body_scope})."),
                                                                                                                                  arr: ( 10, 0),
                                                                                                                                  xform: XFormArrangement::Join{
                                                                                                                                             description: std::borrow::Cow::from("inputs::ArrowParam(.expr_id=expr, .file=file, .param=pat), inputs::Arrow(.expr_id=expr, .file=file, .body=ddlog_std::Some{.x=(_, body_scope)})"),
                                                                                                                                             ffun: None,
                                                                                                                                             arrangement: (9,1),
                                                                                                                                             jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                             {
                                                                                                                                                 let (ref expr, ref file, ref pat) = match *<types__inputs::ArrowParam>::from_ddvalue_ref(__v1) {
                                                                                                                                                     types__inputs::ArrowParam{expr_id: ref expr, file: ref file, param: ref pat} => ((*expr).clone(), (*file).clone(), (*pat).clone()),
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 let ref body_scope = match *<types__inputs::Arrow>::from_ddvalue_ref(__v2) {
                                                                                                                                                     types__inputs::Arrow{expr_id: _, file: _, body: ddlog_std::Option::Some{x: ddlog_std::tuple2(_, ref body_scope)}} => (*body_scope).clone(),
                                                                                                                                                     _ => return None
                                                                                                                                                 };
                                                                                                                                                 Some((ddlog_std::tuple4((*expr).clone(), (*file).clone(), (*pat).clone(), (*body_scope).clone())).into_ddvalue())
                                                                                                                                             }
                                                                                                                                             __f},
                                                                                                                                             next: Box::new(Some(XFormCollection::FlatMap{
                                                                                                                                                                     description: std::borrow::Cow::from("inputs::ArrowParam(.expr_id=expr, .file=file, .param=pat), inputs::Arrow(.expr_id=expr, .file=file, .body=ddlog_std::Some{.x=(_, body_scope)}), var bound = FlatMap((ast::bound_vars(pat)))"),
                                                                                                                                                                     fmfun: {fn __f(__v: DDValue) -> Option<Box<dyn Iterator<Item=DDValue>>>
                                                                                                                                                                     {
                                                                                                                                                                         let ddlog_std::tuple4(ref expr, ref file, ref pat, ref body_scope) = *<ddlog_std::tuple4<types__ast::ExprId, types__ast::FileId, internment::Intern<types__ast::Pattern>, types__ast::ScopeId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                         let __flattened = types__ast::bound_vars_internment_Intern__ast_Pattern_ddlog_std_Vec__ast_Spanned__internment_Intern____Stringval(pat);
                                                                                                                                                                         let expr = (*expr).clone();
                                                                                                                                                                         let file = (*file).clone();
                                                                                                                                                                         let body_scope = (*body_scope).clone();
                                                                                                                                                                         Some(Box::new(__flattened.into_iter().map(move |bound|(ddlog_std::tuple4(bound.clone(), expr.clone(), file.clone(), body_scope.clone())).into_ddvalue())))
                                                                                                                                                                     }
                                                                                                                                                                     __f},
                                                                                                                                                                     next: Box::new(Some(XFormCollection::FilterMap{
                                                                                                                                                                                             description: std::borrow::Cow::from("head of var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::ArrowParam(.expr_id=expr, .file=file, .param=pat), inputs::Arrow(.expr_id=expr, .file=file, .body=ddlog_std::Some{.x=(_, body_scope)}), var bound = FlatMap((ast::bound_vars(pat))), (ast::Spanned{.data=var name, .span=var span} = bound), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=true, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdExpr{.expr=expr}), (var scope = var_decls::Unhoistable{.scope=body_scope})."),
                                                                                                                                                                                             fmfun: {fn __f(__v: DDValue) -> Option<DDValue>
                                                                                                                                                                                             {
                                                                                                                                                                                                 let ddlog_std::tuple4(ref bound, ref expr, ref file, ref body_scope) = *<ddlog_std::tuple4<types__ast::Spanned<internment::Intern<String>>, types__ast::ExprId, types__ast::FileId, types__ast::ScopeId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                                                 let (ref name, ref span): (internment::Intern<String>, types__ast::Span) = match (*bound).clone() {
                                                                                                                                                                                                     types__ast::Spanned{data: name, span: span} => (name, span),
                                                                                                                                                                                                     _ => return None
                                                                                                                                                                                                 };
                                                                                                                                                                                                 let ref meta: ddlog_std::Ref<VariableMeta> = match ddlog_std::ref_new((&(VariableMeta{is_function_argument: true, implicitly_declared: false, declaration_span: (ddlog_std::Option::Some{x: (*span).clone()})}))) {
                                                                                                                                                                                                     meta => meta,
                                                                                                                                                                                                     _ => return None
                                                                                                                                                                                                 };
                                                                                                                                                                                                 let ref id: types__ast::AnyId = match (types__ast::AnyId::AnyIdExpr{expr: (*expr).clone()}) {
                                                                                                                                                                                                     id => id,
                                                                                                                                                                                                     _ => return None
                                                                                                                                                                                                 };
                                                                                                                                                                                                 let ref scope: DeclarationScope = match (DeclarationScope::Unhoistable{scope: (*body_scope).clone()}) {
                                                                                                                                                                                                     scope => scope,
                                                                                                                                                                                                     _ => return None
                                                                                                                                                                                                 };
                                                                                                                                                                                                 Some(((VariableDeclarations{file: (*file).clone(), name: (*name).clone(), scope: (*scope).clone(), declared_in: (*id).clone(), meta: (*meta).clone()})).into_ddvalue())
                                                                                                                                                                                             }
                                                                                                                                                                                             __f},
                                                                                                                                                                                             next: Box::new(None)
                                                                                                                                                                                         }))
                                                                                                                                                                 }))
                                                                                                                                         }
                                                                                                                              },
    );
pub static __Rule_var_decls_VariableDeclarations_10: ::once_cell::sync::Lazy<program::Rule> =
    ::once_cell::sync::Lazy::new(
        || /* var_decls::VariableDeclarations[(var_decls::VariableDeclarations{.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta}: var_decls::VariableDeclarations)] :- inputs::InlineFunc[(inputs::InlineFunc{.expr_id=(expr: ast::ExprId), .file=(file: ast::FileId), .name=(ddlog_std::Some{.x=(ast::Spanned{.data=(name: internment::Intern<string>), .span=(span: ast::Span)}: ast::Spanned<internment::Intern<string>>)}: ddlog_std::Option<ast::Spanned<ast::Name>>), .body=(ddlog_std::Some{.x=(body: ast::StmtId)}: ddlog_std::Option<ast::StmtId>)}: inputs::InlineFunc)], inputs::Statement[(inputs::Statement{.id=(body: ast::StmtId), .file=(file: ast::FileId), .kind=(_: ast::StmtKind), .scope=(body_scope: ast::ScopeId), .span=(_: ast::Span)}: inputs::Statement)], ((var meta: ddlog_std::Ref<var_decls::VariableMeta>) = ((ddlog_std::ref_new: function(var_decls::VariableMeta):ddlog_std::Ref<var_decls::VariableMeta>)((var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=true, .declaration_span=(ddlog_std::Some{.x=span}: ddlog_std::Option<ast::Span>)}: var_decls::VariableMeta)))), ((var id: ast::AnyId) = (ast::AnyIdExpr{.expr=expr}: ast::AnyId)), ((var scope: var_decls::DeclarationScope) = (var_decls::Unhoistable{.scope=body_scope}: var_decls::DeclarationScope)). */
                                                                                                                               program::Rule::ArrangementRule {
                                                                                                                                   description: std::borrow::Cow::from( "var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::InlineFunc(.expr_id=expr, .file=file, .name=ddlog_std::Some{.x=ast::Spanned{.data=name, .span=span}}, .body=ddlog_std::Some{.x=body}), inputs::Statement(.id=body, .file=file, .kind=_, .scope=body_scope, .span=_), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=true, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdExpr{.expr=expr}), (var scope = var_decls::Unhoistable{.scope=body_scope})."),
                                                                                                                                   arr: ( 39, 2),
                                                                                                                                   xform: XFormArrangement::Join{
                                                                                                                                              description: std::borrow::Cow::from("inputs::InlineFunc(.expr_id=expr, .file=file, .name=ddlog_std::Some{.x=ast::Spanned{.data=name, .span=span}}, .body=ddlog_std::Some{.x=body}), inputs::Statement(.id=body, .file=file, .kind=_, .scope=body_scope, .span=_)"),
                                                                                                                                              ffun: None,
                                                                                                                                              arrangement: (48,0),
                                                                                                                                              jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                              {
                                                                                                                                                  let (ref expr, ref file, ref name, ref span, ref body) = match *<types__inputs::InlineFunc>::from_ddvalue_ref(__v1) {
                                                                                                                                                      types__inputs::InlineFunc{expr_id: ref expr, file: ref file, name: ddlog_std::Option::Some{x: types__ast::Spanned{data: ref name, span: ref span}}, body: ddlog_std::Option::Some{x: ref body}} => ((*expr).clone(), (*file).clone(), (*name).clone(), (*span).clone(), (*body).clone()),
                                                                                                                                                      _ => return None
                                                                                                                                                  };
                                                                                                                                                  let ref body_scope = match *<types__inputs::Statement>::from_ddvalue_ref(__v2) {
                                                                                                                                                      types__inputs::Statement{id: _, file: _, kind: _, scope: ref body_scope, span: _} => (*body_scope).clone(),
                                                                                                                                                      _ => return None
                                                                                                                                                  };
                                                                                                                                                  let ref meta: ddlog_std::Ref<VariableMeta> = match ddlog_std::ref_new((&(VariableMeta{is_function_argument: false, implicitly_declared: true, declaration_span: (ddlog_std::Option::Some{x: (*span).clone()})}))) {
                                                                                                                                                      meta => meta,
                                                                                                                                                      _ => return None
                                                                                                                                                  };
                                                                                                                                                  let ref id: types__ast::AnyId = match (types__ast::AnyId::AnyIdExpr{expr: (*expr).clone()}) {
                                                                                                                                                      id => id,
                                                                                                                                                      _ => return None
                                                                                                                                                  };
                                                                                                                                                  let ref scope: DeclarationScope = match (DeclarationScope::Unhoistable{scope: (*body_scope).clone()}) {
                                                                                                                                                      scope => scope,
                                                                                                                                                      _ => return None
                                                                                                                                                  };
                                                                                                                                                  Some(((VariableDeclarations{file: (*file).clone(), name: (*name).clone(), scope: (*scope).clone(), declared_in: (*id).clone(), meta: (*meta).clone()})).into_ddvalue())
                                                                                                                                              }
                                                                                                                                              __f},
                                                                                                                                              next: Box::new(None)
                                                                                                                                          }
                                                                                                                               },
    );
pub static __Rule_var_decls_VariableDeclarations_11: ::once_cell::sync::Lazy<program::Rule> =
    ::once_cell::sync::Lazy::new(
        || /* var_decls::VariableDeclarations[(var_decls::VariableDeclarations{.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta}: var_decls::VariableDeclarations)] :- inputs::InlineFuncParam[(inputs::InlineFuncParam{.expr_id=(expr: ast::ExprId), .file=(file: ast::FileId), .param=(pat: internment::Intern<ast::Pattern>)}: inputs::InlineFuncParam)], inputs::InlineFunc[(inputs::InlineFunc{.expr_id=(expr: ast::ExprId), .file=(file: ast::FileId), .name=(_: ddlog_std::Option<ast::Spanned<ast::Name>>), .body=(ddlog_std::Some{.x=(body: ast::StmtId)}: ddlog_std::Option<ast::StmtId>)}: inputs::InlineFunc)], inputs::Statement[(inputs::Statement{.id=(body: ast::StmtId), .file=(file: ast::FileId), .kind=(_: ast::StmtKind), .scope=(body_scope: ast::ScopeId), .span=(_: ast::Span)}: inputs::Statement)], var bound = FlatMap(((ast::bound_vars: function(internment::Intern<ast::Pattern>):ddlog_std::Vec<ast::Spanned<ast::Name>>)(pat))), ((ast::Spanned{.data=(var name: internment::Intern<string>), .span=(var span: ast::Span)}: ast::Spanned<internment::Intern<string>>) = bound), ((var meta: ddlog_std::Ref<var_decls::VariableMeta>) = ((ddlog_std::ref_new: function(var_decls::VariableMeta):ddlog_std::Ref<var_decls::VariableMeta>)((var_decls::VariableMeta{.is_function_argument=true, .implicitly_declared=false, .declaration_span=(ddlog_std::Some{.x=span}: ddlog_std::Option<ast::Span>)}: var_decls::VariableMeta)))), ((var id: ast::AnyId) = (ast::AnyIdExpr{.expr=expr}: ast::AnyId)), ((var scope: var_decls::DeclarationScope) = (var_decls::Unhoistable{.scope=body_scope}: var_decls::DeclarationScope)). */
                                                                                                                               program::Rule::ArrangementRule {
                                                                                                                                   description: std::borrow::Cow::from( "var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::InlineFuncParam(.expr_id=expr, .file=file, .param=pat), inputs::InlineFunc(.expr_id=expr, .file=file, .name=_, .body=ddlog_std::Some{.x=body}), inputs::Statement(.id=body, .file=file, .kind=_, .scope=body_scope, .span=_), var bound = FlatMap((ast::bound_vars(pat))), (ast::Spanned{.data=var name, .span=var span} = bound), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=true, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdExpr{.expr=expr}), (var scope = var_decls::Unhoistable{.scope=body_scope})."),
                                                                                                                                   arr: ( 40, 0),
                                                                                                                                   xform: XFormArrangement::Join{
                                                                                                                                              description: std::borrow::Cow::from("inputs::InlineFuncParam(.expr_id=expr, .file=file, .param=pat), inputs::InlineFunc(.expr_id=expr, .file=file, .name=_, .body=ddlog_std::Some{.x=body})"),
                                                                                                                                              ffun: None,
                                                                                                                                              arrangement: (39,3),
                                                                                                                                              jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                              {
                                                                                                                                                  let (ref expr, ref file, ref pat) = match *<types__inputs::InlineFuncParam>::from_ddvalue_ref(__v1) {
                                                                                                                                                      types__inputs::InlineFuncParam{expr_id: ref expr, file: ref file, param: ref pat} => ((*expr).clone(), (*file).clone(), (*pat).clone()),
                                                                                                                                                      _ => return None
                                                                                                                                                  };
                                                                                                                                                  let ref body = match *<types__inputs::InlineFunc>::from_ddvalue_ref(__v2) {
                                                                                                                                                      types__inputs::InlineFunc{expr_id: _, file: _, name: _, body: ddlog_std::Option::Some{x: ref body}} => (*body).clone(),
                                                                                                                                                      _ => return None
                                                                                                                                                  };
                                                                                                                                                  Some((ddlog_std::tuple4((*expr).clone(), (*file).clone(), (*pat).clone(), (*body).clone())).into_ddvalue())
                                                                                                                                              }
                                                                                                                                              __f},
                                                                                                                                              next: Box::new(Some(XFormCollection::Arrange {
                                                                                                                                                                      description: std::borrow::Cow::from("arrange inputs::InlineFuncParam(.expr_id=expr, .file=file, .param=pat), inputs::InlineFunc(.expr_id=expr, .file=file, .name=_, .body=ddlog_std::Some{.x=body}) by (body, file)"),
                                                                                                                                                                      afun: {fn __f(__v: DDValue) -> Option<(DDValue,DDValue)>
                                                                                                                                                                      {
                                                                                                                                                                          let ddlog_std::tuple4(ref expr, ref file, ref pat, ref body) = *<ddlog_std::tuple4<types__ast::ExprId, types__ast::FileId, internment::Intern<types__ast::Pattern>, types__ast::StmtId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                          Some(((ddlog_std::tuple2((*body).clone(), (*file).clone())).into_ddvalue(), (ddlog_std::tuple3((*expr).clone(), (*file).clone(), (*pat).clone())).into_ddvalue()))
                                                                                                                                                                      }
                                                                                                                                                                      __f},
                                                                                                                                                                      next: Box::new(XFormArrangement::Join{
                                                                                                                                                                                         description: std::borrow::Cow::from("inputs::InlineFuncParam(.expr_id=expr, .file=file, .param=pat), inputs::InlineFunc(.expr_id=expr, .file=file, .name=_, .body=ddlog_std::Some{.x=body}), inputs::Statement(.id=body, .file=file, .kind=_, .scope=body_scope, .span=_)"),
                                                                                                                                                                                         ffun: None,
                                                                                                                                                                                         arrangement: (48,0),
                                                                                                                                                                                         jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                                                                         {
                                                                                                                                                                                             let ddlog_std::tuple3(ref expr, ref file, ref pat) = *<ddlog_std::tuple3<types__ast::ExprId, types__ast::FileId, internment::Intern<types__ast::Pattern>>>::from_ddvalue_ref( __v1 );
                                                                                                                                                                                             let ref body_scope = match *<types__inputs::Statement>::from_ddvalue_ref(__v2) {
                                                                                                                                                                                                 types__inputs::Statement{id: _, file: _, kind: _, scope: ref body_scope, span: _} => (*body_scope).clone(),
                                                                                                                                                                                                 _ => return None
                                                                                                                                                                                             };
                                                                                                                                                                                             Some((ddlog_std::tuple4((*expr).clone(), (*file).clone(), (*pat).clone(), (*body_scope).clone())).into_ddvalue())
                                                                                                                                                                                         }
                                                                                                                                                                                         __f},
                                                                                                                                                                                         next: Box::new(Some(XFormCollection::FlatMap{
                                                                                                                                                                                                                 description: std::borrow::Cow::from("inputs::InlineFuncParam(.expr_id=expr, .file=file, .param=pat), inputs::InlineFunc(.expr_id=expr, .file=file, .name=_, .body=ddlog_std::Some{.x=body}), inputs::Statement(.id=body, .file=file, .kind=_, .scope=body_scope, .span=_), var bound = FlatMap((ast::bound_vars(pat)))"),
                                                                                                                                                                                                                 fmfun: {fn __f(__v: DDValue) -> Option<Box<dyn Iterator<Item=DDValue>>>
                                                                                                                                                                                                                 {
                                                                                                                                                                                                                     let ddlog_std::tuple4(ref expr, ref file, ref pat, ref body_scope) = *<ddlog_std::tuple4<types__ast::ExprId, types__ast::FileId, internment::Intern<types__ast::Pattern>, types__ast::ScopeId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                                                                     let __flattened = types__ast::bound_vars_internment_Intern__ast_Pattern_ddlog_std_Vec__ast_Spanned__internment_Intern____Stringval(pat);
                                                                                                                                                                                                                     let expr = (*expr).clone();
                                                                                                                                                                                                                     let file = (*file).clone();
                                                                                                                                                                                                                     let body_scope = (*body_scope).clone();
                                                                                                                                                                                                                     Some(Box::new(__flattened.into_iter().map(move |bound|(ddlog_std::tuple4(bound.clone(), expr.clone(), file.clone(), body_scope.clone())).into_ddvalue())))
                                                                                                                                                                                                                 }
                                                                                                                                                                                                                 __f},
                                                                                                                                                                                                                 next: Box::new(Some(XFormCollection::FilterMap{
                                                                                                                                                                                                                                         description: std::borrow::Cow::from("head of var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::InlineFuncParam(.expr_id=expr, .file=file, .param=pat), inputs::InlineFunc(.expr_id=expr, .file=file, .name=_, .body=ddlog_std::Some{.x=body}), inputs::Statement(.id=body, .file=file, .kind=_, .scope=body_scope, .span=_), var bound = FlatMap((ast::bound_vars(pat))), (ast::Spanned{.data=var name, .span=var span} = bound), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=true, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdExpr{.expr=expr}), (var scope = var_decls::Unhoistable{.scope=body_scope})."),
                                                                                                                                                                                                                                         fmfun: {fn __f(__v: DDValue) -> Option<DDValue>
                                                                                                                                                                                                                                         {
                                                                                                                                                                                                                                             let ddlog_std::tuple4(ref bound, ref expr, ref file, ref body_scope) = *<ddlog_std::tuple4<types__ast::Spanned<internment::Intern<String>>, types__ast::ExprId, types__ast::FileId, types__ast::ScopeId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                                                                                             let (ref name, ref span): (internment::Intern<String>, types__ast::Span) = match (*bound).clone() {
                                                                                                                                                                                                                                                 types__ast::Spanned{data: name, span: span} => (name, span),
                                                                                                                                                                                                                                                 _ => return None
                                                                                                                                                                                                                                             };
                                                                                                                                                                                                                                             let ref meta: ddlog_std::Ref<VariableMeta> = match ddlog_std::ref_new((&(VariableMeta{is_function_argument: true, implicitly_declared: false, declaration_span: (ddlog_std::Option::Some{x: (*span).clone()})}))) {
                                                                                                                                                                                                                                                 meta => meta,
                                                                                                                                                                                                                                                 _ => return None
                                                                                                                                                                                                                                             };
                                                                                                                                                                                                                                             let ref id: types__ast::AnyId = match (types__ast::AnyId::AnyIdExpr{expr: (*expr).clone()}) {
                                                                                                                                                                                                                                                 id => id,
                                                                                                                                                                                                                                                 _ => return None
                                                                                                                                                                                                                                             };
                                                                                                                                                                                                                                             let ref scope: DeclarationScope = match (DeclarationScope::Unhoistable{scope: (*body_scope).clone()}) {
                                                                                                                                                                                                                                                 scope => scope,
                                                                                                                                                                                                                                                 _ => return None
                                                                                                                                                                                                                                             };
                                                                                                                                                                                                                                             Some(((VariableDeclarations{file: (*file).clone(), name: (*name).clone(), scope: (*scope).clone(), declared_in: (*id).clone(), meta: (*meta).clone()})).into_ddvalue())
                                                                                                                                                                                                                                         }
                                                                                                                                                                                                                                         __f},
                                                                                                                                                                                                                                         next: Box::new(None)
                                                                                                                                                                                                                                     }))
                                                                                                                                                                                                             }))
                                                                                                                                                                                     })
                                                                                                                                                                  }))
                                                                                                                                          }
                                                                                                                               },
    );
pub static __Rule_var_decls_VariableDeclarations_12: ::once_cell::sync::Lazy<program::Rule> =
    ::once_cell::sync::Lazy::new(
        || /* var_decls::VariableDeclarations[(var_decls::VariableDeclarations{.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta}: var_decls::VariableDeclarations)] :- inputs::Try[(inputs::Try{.stmt_id=(stmt: ast::StmtId), .file=(file: ast::FileId), .body=(_: ddlog_std::Option<ast::StmtId>), .handler=(ast::TryHandler{.error=(ddlog_std::Some{.x=(error: internment::Intern<ast::Pattern>)}: ddlog_std::Option<ast::IPattern>), .body=(ddlog_std::Some{.x=(body: ast::StmtId)}: ddlog_std::Option<ast::StmtId>)}: ast::TryHandler), .finalizer=(_: ddlog_std::Option<ast::StmtId>)}: inputs::Try)], inputs::Statement[(inputs::Statement{.id=(body: ast::StmtId), .file=(file: ast::FileId), .kind=(_: ast::StmtKind), .scope=(stmt_scope: ast::ScopeId), .span=(_: ast::Span)}: inputs::Statement)], var bound = FlatMap(((ast::bound_vars: function(internment::Intern<ast::Pattern>):ddlog_std::Vec<ast::Spanned<ast::Name>>)(error))), ((ast::Spanned{.data=(var name: internment::Intern<string>), .span=(var span: ast::Span)}: ast::Spanned<internment::Intern<string>>) = bound), ((var meta: ddlog_std::Ref<var_decls::VariableMeta>) = ((ddlog_std::ref_new: function(var_decls::VariableMeta):ddlog_std::Ref<var_decls::VariableMeta>)((var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=(ddlog_std::Some{.x=span}: ddlog_std::Option<ast::Span>)}: var_decls::VariableMeta)))), ((var id: ast::AnyId) = (ast::AnyIdStmt{.stmt=stmt}: ast::AnyId)), ((var scope: var_decls::DeclarationScope) = (var_decls::Unhoistable{.scope=stmt_scope}: var_decls::DeclarationScope)). */
                                                                                                                               program::Rule::ArrangementRule {
                                                                                                                                   description: std::borrow::Cow::from( "var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::Try(.stmt_id=stmt, .file=file, .body=_, .handler=ast::TryHandler{.error=ddlog_std::Some{.x=error}, .body=ddlog_std::Some{.x=body}}, .finalizer=_), inputs::Statement(.id=body, .file=file, .kind=_, .scope=stmt_scope, .span=_), var bound = FlatMap((ast::bound_vars(error))), (ast::Spanned{.data=var name, .span=var span} = bound), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdStmt{.stmt=stmt}), (var scope = var_decls::Unhoistable{.scope=stmt_scope})."),
                                                                                                                                   arr: ( 54, 0),
                                                                                                                                   xform: XFormArrangement::Join{
                                                                                                                                              description: std::borrow::Cow::from("inputs::Try(.stmt_id=stmt, .file=file, .body=_, .handler=ast::TryHandler{.error=ddlog_std::Some{.x=error}, .body=ddlog_std::Some{.x=body}}, .finalizer=_), inputs::Statement(.id=body, .file=file, .kind=_, .scope=stmt_scope, .span=_)"),
                                                                                                                                              ffun: None,
                                                                                                                                              arrangement: (48,0),
                                                                                                                                              jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                              {
                                                                                                                                                  let (ref stmt, ref file, ref error, ref body) = match *<types__inputs::Try>::from_ddvalue_ref(__v1) {
                                                                                                                                                      types__inputs::Try{stmt_id: ref stmt, file: ref file, body: _, handler: types__ast::TryHandler{error: ddlog_std::Option::Some{x: ref error}, body: ddlog_std::Option::Some{x: ref body}}, finalizer: _} => ((*stmt).clone(), (*file).clone(), (*error).clone(), (*body).clone()),
                                                                                                                                                      _ => return None
                                                                                                                                                  };
                                                                                                                                                  let ref stmt_scope = match *<types__inputs::Statement>::from_ddvalue_ref(__v2) {
                                                                                                                                                      types__inputs::Statement{id: _, file: _, kind: _, scope: ref stmt_scope, span: _} => (*stmt_scope).clone(),
                                                                                                                                                      _ => return None
                                                                                                                                                  };
                                                                                                                                                  Some((ddlog_std::tuple4((*stmt).clone(), (*file).clone(), (*error).clone(), (*stmt_scope).clone())).into_ddvalue())
                                                                                                                                              }
                                                                                                                                              __f},
                                                                                                                                              next: Box::new(Some(XFormCollection::FlatMap{
                                                                                                                                                                      description: std::borrow::Cow::from("inputs::Try(.stmt_id=stmt, .file=file, .body=_, .handler=ast::TryHandler{.error=ddlog_std::Some{.x=error}, .body=ddlog_std::Some{.x=body}}, .finalizer=_), inputs::Statement(.id=body, .file=file, .kind=_, .scope=stmt_scope, .span=_), var bound = FlatMap((ast::bound_vars(error)))"),
                                                                                                                                                                      fmfun: {fn __f(__v: DDValue) -> Option<Box<dyn Iterator<Item=DDValue>>>
                                                                                                                                                                      {
                                                                                                                                                                          let ddlog_std::tuple4(ref stmt, ref file, ref error, ref stmt_scope) = *<ddlog_std::tuple4<types__ast::StmtId, types__ast::FileId, internment::Intern<types__ast::Pattern>, types__ast::ScopeId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                          let __flattened = types__ast::bound_vars_internment_Intern__ast_Pattern_ddlog_std_Vec__ast_Spanned__internment_Intern____Stringval(error);
                                                                                                                                                                          let stmt = (*stmt).clone();
                                                                                                                                                                          let file = (*file).clone();
                                                                                                                                                                          let stmt_scope = (*stmt_scope).clone();
                                                                                                                                                                          Some(Box::new(__flattened.into_iter().map(move |bound|(ddlog_std::tuple4(bound.clone(), stmt.clone(), file.clone(), stmt_scope.clone())).into_ddvalue())))
                                                                                                                                                                      }
                                                                                                                                                                      __f},
                                                                                                                                                                      next: Box::new(Some(XFormCollection::FilterMap{
                                                                                                                                                                                              description: std::borrow::Cow::from("head of var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::Try(.stmt_id=stmt, .file=file, .body=_, .handler=ast::TryHandler{.error=ddlog_std::Some{.x=error}, .body=ddlog_std::Some{.x=body}}, .finalizer=_), inputs::Statement(.id=body, .file=file, .kind=_, .scope=stmt_scope, .span=_), var bound = FlatMap((ast::bound_vars(error))), (ast::Spanned{.data=var name, .span=var span} = bound), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdStmt{.stmt=stmt}), (var scope = var_decls::Unhoistable{.scope=stmt_scope})."),
                                                                                                                                                                                              fmfun: {fn __f(__v: DDValue) -> Option<DDValue>
                                                                                                                                                                                              {
                                                                                                                                                                                                  let ddlog_std::tuple4(ref bound, ref stmt, ref file, ref stmt_scope) = *<ddlog_std::tuple4<types__ast::Spanned<internment::Intern<String>>, types__ast::StmtId, types__ast::FileId, types__ast::ScopeId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                                                  let (ref name, ref span): (internment::Intern<String>, types__ast::Span) = match (*bound).clone() {
                                                                                                                                                                                                      types__ast::Spanned{data: name, span: span} => (name, span),
                                                                                                                                                                                                      _ => return None
                                                                                                                                                                                                  };
                                                                                                                                                                                                  let ref meta: ddlog_std::Ref<VariableMeta> = match ddlog_std::ref_new((&(VariableMeta{is_function_argument: false, implicitly_declared: false, declaration_span: (ddlog_std::Option::Some{x: (*span).clone()})}))) {
                                                                                                                                                                                                      meta => meta,
                                                                                                                                                                                                      _ => return None
                                                                                                                                                                                                  };
                                                                                                                                                                                                  let ref id: types__ast::AnyId = match (types__ast::AnyId::AnyIdStmt{stmt: (*stmt).clone()}) {
                                                                                                                                                                                                      id => id,
                                                                                                                                                                                                      _ => return None
                                                                                                                                                                                                  };
                                                                                                                                                                                                  let ref scope: DeclarationScope = match (DeclarationScope::Unhoistable{scope: (*stmt_scope).clone()}) {
                                                                                                                                                                                                      scope => scope,
                                                                                                                                                                                                      _ => return None
                                                                                                                                                                                                  };
                                                                                                                                                                                                  Some(((VariableDeclarations{file: (*file).clone(), name: (*name).clone(), scope: (*scope).clone(), declared_in: (*id).clone(), meta: (*meta).clone()})).into_ddvalue())
                                                                                                                                                                                              }
                                                                                                                                                                                              __f},
                                                                                                                                                                                              next: Box::new(None)
                                                                                                                                                                                          }))
                                                                                                                                                                  }))
                                                                                                                                          }
                                                                                                                               },
    );
pub static __Rule_var_decls_VariableDeclarations_13: ::once_cell::sync::Lazy<program::Rule> =
    ::once_cell::sync::Lazy::new(
        || /* var_decls::VariableDeclarations[(var_decls::VariableDeclarations{.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta}: var_decls::VariableDeclarations)] :- inputs::Class[(inputs::Class{.id=(class: ast::ClassId), .file=(file: ast::FileId), .name=(_: ddlog_std::Option<ast::Spanned<ast::Name>>), .parent=(_: ddlog_std::Option<ast::ExprId>), .elements=(ddlog_std::Some{.x=(elements: ddlog_std::Vec<ast::IClassElement>)}: ddlog_std::Option<ddlog_std::Vec<ast::IClassElement>>), .scope=(_: ast::ScopeId), .exported=(_: bool)}: inputs::Class)], var elem = FlatMap(elements), ((ddlog_std::Some{.x=((var params: ddlog_std::Vec<ast::FuncParam>), (var body: ast::StmtId))}: ddlog_std::Option<(ddlog_std::Vec<ast::FuncParam>, ast::StmtId)>) = ((ast::method_comps: function(ast::ClassElement):ddlog_std::Option<(ddlog_std::Vec<ast::FuncParam>, ast::StmtId)>)(((internment::ival: function(internment::Intern<ast::ClassElement>):ast::ClassElement)(elem))))), inputs::Statement[(inputs::Statement{.id=(body: ast::StmtId), .file=(file: ast::FileId), .kind=(_: ast::StmtKind), .scope=(stmt_scope: ast::ScopeId), .span=(_: ast::Span)}: inputs::Statement)], var bound = FlatMap(((vec::flatmap: function(ddlog_std::Vec<ast::FuncParam>, function(ast::FuncParam):ddlog_std::Vec<(ast::Spanned<internment::Intern<string>>, bool)>):ddlog_std::Vec<(ast::Spanned<internment::Intern<string>>, bool)>)(params, (function(param: ast::FuncParam):ddlog_std::Vec<(ast::Spanned<internment::Intern<string>>, bool)>{((ast::bound_vars: function(ast::FuncParam):ddlog_std::Vec<(ast::Spanned<internment::Intern<string>>, bool)>)(param))})))), (((ast::Spanned{.data=(var name: internment::Intern<string>), .span=(var span: ast::Span)}: ast::Spanned<internment::Intern<string>>), (var implicit: bool)) = bound), ((var meta: ddlog_std::Ref<var_decls::VariableMeta>) = ((ddlog_std::ref_new: function(var_decls::VariableMeta):ddlog_std::Ref<var_decls::VariableMeta>)((var_decls::VariableMeta{.is_function_argument=true, .implicitly_declared=implicit, .declaration_span=(ddlog_std::Some{.x=span}: ddlog_std::Option<ast::Span>)}: var_decls::VariableMeta)))), ((var id: ast::AnyId) = (ast::AnyIdClass{.class=class}: ast::AnyId)), ((var scope: var_decls::DeclarationScope) = (var_decls::Unhoistable{.scope=stmt_scope}: var_decls::DeclarationScope)). */
                                                                                                                               program::Rule::CollectionRule {
                                                                                                                                   description: std::borrow::Cow::from("var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::Class(.id=class, .file=file, .name=_, .parent=_, .elements=ddlog_std::Some{.x=elements}, .scope=_, .exported=_), var elem = FlatMap(elements), (ddlog_std::Some{.x=(var params, var body)} = (ast::method_comps((internment::ival(elem))))), inputs::Statement(.id=body, .file=file, .kind=_, .scope=stmt_scope, .span=_), var bound = FlatMap((vec::flatmap(params, (function(param: ast::FuncParam):ddlog_std::Vec<(ast::Spanned<internment::Intern<string>>, bool)>{(ast::bound_vars(param))})))), ((ast::Spanned{.data=var name, .span=var span}, var implicit) = bound), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=true, .implicitly_declared=implicit, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdClass{.class=class}), (var scope = var_decls::Unhoistable{.scope=stmt_scope})."),
                                                                                                                                   rel: 17,
                                                                                                                                   xform: Some(XFormCollection::FlatMap{
                                                                                                                                                   description: std::borrow::Cow::from("inputs::Class(.id=class, .file=file, .name=_, .parent=_, .elements=ddlog_std::Some{.x=elements}, .scope=_, .exported=_), var elem = FlatMap(elements)"),
                                                                                                                                                   fmfun: {fn __f(__v: DDValue) -> Option<Box<dyn Iterator<Item=DDValue>>>
                                                                                                                                                   {
                                                                                                                                                       let (ref class, ref file, ref elements) = match *<types__inputs::Class>::from_ddvalue_ref(&__v) {
                                                                                                                                                           types__inputs::Class{id: ref class, file: ref file, name: _, parent: _, elements: ddlog_std::Option::Some{x: ref elements}, scope: _, exported: _} => ((*class).clone(), (*file).clone(), (*elements).clone()),
                                                                                                                                                           _ => return None
                                                                                                                                                       };
                                                                                                                                                       let __flattened = (*elements).clone();
                                                                                                                                                       let class = (*class).clone();
                                                                                                                                                       let file = (*file).clone();
                                                                                                                                                       Some(Box::new(__flattened.into_iter().map(move |elem|(ddlog_std::tuple3(elem.clone(), class.clone(), file.clone())).into_ddvalue())))
                                                                                                                                                   }
                                                                                                                                                   __f},
                                                                                                                                                   next: Box::new(Some(XFormCollection::Arrange {
                                                                                                                                                                           description: std::borrow::Cow::from("arrange inputs::Class(.id=class, .file=file, .name=_, .parent=_, .elements=ddlog_std::Some{.x=elements}, .scope=_, .exported=_), var elem = FlatMap(elements) by (body, file)"),
                                                                                                                                                                           afun: {fn __f(__v: DDValue) -> Option<(DDValue,DDValue)>
                                                                                                                                                                           {
                                                                                                                                                                               let ddlog_std::tuple3(ref elem, ref class, ref file) = *<ddlog_std::tuple3<internment::Intern<types__ast::ClassElement>, types__ast::ClassId, types__ast::FileId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                               let (ref params, ref body): (ddlog_std::Vec<types__ast::FuncParam>, types__ast::StmtId) = match types__ast::method_comps_ast_ClassElement_ddlog_std_Option____Tuple2__ddlog_std_Vec__ast_FuncParam_ast_StmtId(internment::ival(elem)) {
                                                                                                                                                                                   ddlog_std::Option::Some{x: ddlog_std::tuple2(params, body)} => (params, body),
                                                                                                                                                                                   _ => return None
                                                                                                                                                                               };
                                                                                                                                                                               Some(((ddlog_std::tuple2((*body).clone(), (*file).clone())).into_ddvalue(), (ddlog_std::tuple3((*class).clone(), (*file).clone(), (*params).clone())).into_ddvalue()))
                                                                                                                                                                           }
                                                                                                                                                                           __f},
                                                                                                                                                                           next: Box::new(XFormArrangement::Join{
                                                                                                                                                                                              description: std::borrow::Cow::from("inputs::Class(.id=class, .file=file, .name=_, .parent=_, .elements=ddlog_std::Some{.x=elements}, .scope=_, .exported=_), var elem = FlatMap(elements), (ddlog_std::Some{.x=(var params, var body)} = (ast::method_comps((internment::ival(elem))))), inputs::Statement(.id=body, .file=file, .kind=_, .scope=stmt_scope, .span=_)"),
                                                                                                                                                                                              ffun: None,
                                                                                                                                                                                              arrangement: (48,0),
                                                                                                                                                                                              jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                                                                              {
                                                                                                                                                                                                  let ddlog_std::tuple3(ref class, ref file, ref params) = *<ddlog_std::tuple3<types__ast::ClassId, types__ast::FileId, ddlog_std::Vec<types__ast::FuncParam>>>::from_ddvalue_ref( __v1 );
                                                                                                                                                                                                  let ref stmt_scope = match *<types__inputs::Statement>::from_ddvalue_ref(__v2) {
                                                                                                                                                                                                      types__inputs::Statement{id: _, file: _, kind: _, scope: ref stmt_scope, span: _} => (*stmt_scope).clone(),
                                                                                                                                                                                                      _ => return None
                                                                                                                                                                                                  };
                                                                                                                                                                                                  Some((ddlog_std::tuple4((*class).clone(), (*file).clone(), (*params).clone(), (*stmt_scope).clone())).into_ddvalue())
                                                                                                                                                                                              }
                                                                                                                                                                                              __f},
                                                                                                                                                                                              next: Box::new(Some(XFormCollection::FlatMap{
                                                                                                                                                                                                                      description: std::borrow::Cow::from("inputs::Class(.id=class, .file=file, .name=_, .parent=_, .elements=ddlog_std::Some{.x=elements}, .scope=_, .exported=_), var elem = FlatMap(elements), (ddlog_std::Some{.x=(var params, var body)} = (ast::method_comps((internment::ival(elem))))), inputs::Statement(.id=body, .file=file, .kind=_, .scope=stmt_scope, .span=_), var bound = FlatMap((vec::flatmap(params, (function(param: ast::FuncParam):ddlog_std::Vec<(ast::Spanned<internment::Intern<string>>, bool)>{(ast::bound_vars(param))}))))"),
                                                                                                                                                                                                                      fmfun: {fn __f(__v: DDValue) -> Option<Box<dyn Iterator<Item=DDValue>>>
                                                                                                                                                                                                                      {
                                                                                                                                                                                                                          let ddlog_std::tuple4(ref class, ref file, ref params, ref stmt_scope) = *<ddlog_std::tuple4<types__ast::ClassId, types__ast::FileId, ddlog_std::Vec<types__ast::FuncParam>, types__ast::ScopeId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                                                                          let __flattened = types__vec::flatmap::<types__ast::FuncParam, ddlog_std::tuple2<types__ast::Spanned<internment::Intern<String>>, bool>>(params, (&{
                                                                                                                                                                                                                                                                                                                                                                                 (Box::new(::ddlog_rt::ClosureImpl{
                                                                                                                                                                                                                                                                                                                                                                                     description: "(function(param: ast::FuncParam):ddlog_std::Vec<(ast::Spanned<internment::Intern<string>>, bool)>{(ast::bound_vars(param))})",
                                                                                                                                                                                                                                                                                                                                                                                     captured: (),
                                                                                                                                                                                                                                                                                                                                                                                     f: {
                                                                                                                                                                                                                                                                                                                                                                                            fn __f(__args:*const types__ast::FuncParam, __captured: &()) -> ddlog_std::Vec<ddlog_std::tuple2<types__ast::Spanned<internment::Intern<String>>, bool>>
                                                                                                                                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                                                                                                                                let param = unsafe{&*__args};
                                                                                                                                                                                                                                                                                                                                                                                                types__ast::bound_vars_ast_FuncParam_ddlog_std_Vec____Tuple2__ast_Spanned__internment_Intern____Stringval___Boolval(param)
                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                            __f
                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                 }) as Box<dyn ::ddlog_rt::Closure<(*const types__ast::FuncParam), ddlog_std::Vec<ddlog_std::tuple2<types__ast::Spanned<internment::Intern<String>>, bool>>>>)
                                                                                                                                                                                                                                                                                                                                                                             }));
                                                                                                                                                                                                                          let class = (*class).clone();
                                                                                                                                                                                                                          let file = (*file).clone();
                                                                                                                                                                                                                          let stmt_scope = (*stmt_scope).clone();
                                                                                                                                                                                                                          Some(Box::new(__flattened.into_iter().map(move |bound|(ddlog_std::tuple4(bound.clone(), class.clone(), file.clone(), stmt_scope.clone())).into_ddvalue())))
                                                                                                                                                                                                                      }
                                                                                                                                                                                                                      __f},
                                                                                                                                                                                                                      next: Box::new(Some(XFormCollection::FilterMap{
                                                                                                                                                                                                                                              description: std::borrow::Cow::from("head of var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::Class(.id=class, .file=file, .name=_, .parent=_, .elements=ddlog_std::Some{.x=elements}, .scope=_, .exported=_), var elem = FlatMap(elements), (ddlog_std::Some{.x=(var params, var body)} = (ast::method_comps((internment::ival(elem))))), inputs::Statement(.id=body, .file=file, .kind=_, .scope=stmt_scope, .span=_), var bound = FlatMap((vec::flatmap(params, (function(param: ast::FuncParam):ddlog_std::Vec<(ast::Spanned<internment::Intern<string>>, bool)>{(ast::bound_vars(param))})))), ((ast::Spanned{.data=var name, .span=var span}, var implicit) = bound), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=true, .implicitly_declared=implicit, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdClass{.class=class}), (var scope = var_decls::Unhoistable{.scope=stmt_scope})."),
                                                                                                                                                                                                                                              fmfun: {fn __f(__v: DDValue) -> Option<DDValue>
                                                                                                                                                                                                                                              {
                                                                                                                                                                                                                                                  let ddlog_std::tuple4(ref bound, ref class, ref file, ref stmt_scope) = *<ddlog_std::tuple4<ddlog_std::tuple2<types__ast::Spanned<internment::Intern<String>>, bool>, types__ast::ClassId, types__ast::FileId, types__ast::ScopeId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                                                                                                  let (ref name, ref span, ref implicit): (internment::Intern<String>, types__ast::Span, bool) = match (*bound).clone() {
                                                                                                                                                                                                                                                      ddlog_std::tuple2(types__ast::Spanned{data: name, span: span}, implicit) => (name, span, implicit),
                                                                                                                                                                                                                                                      _ => return None
                                                                                                                                                                                                                                                  };
                                                                                                                                                                                                                                                  let ref meta: ddlog_std::Ref<VariableMeta> = match ddlog_std::ref_new((&(VariableMeta{is_function_argument: true, implicitly_declared: (*implicit).clone(), declaration_span: (ddlog_std::Option::Some{x: (*span).clone()})}))) {
                                                                                                                                                                                                                                                      meta => meta,
                                                                                                                                                                                                                                                      _ => return None
                                                                                                                                                                                                                                                  };
                                                                                                                                                                                                                                                  let ref id: types__ast::AnyId = match (types__ast::AnyId::AnyIdClass{class: (*class).clone()}) {
                                                                                                                                                                                                                                                      id => id,
                                                                                                                                                                                                                                                      _ => return None
                                                                                                                                                                                                                                                  };
                                                                                                                                                                                                                                                  let ref scope: DeclarationScope = match (DeclarationScope::Unhoistable{scope: (*stmt_scope).clone()}) {
                                                                                                                                                                                                                                                      scope => scope,
                                                                                                                                                                                                                                                      _ => return None
                                                                                                                                                                                                                                                  };
                                                                                                                                                                                                                                                  Some(((VariableDeclarations{file: (*file).clone(), name: (*name).clone(), scope: (*scope).clone(), declared_in: (*id).clone(), meta: (*meta).clone()})).into_ddvalue())
                                                                                                                                                                                                                                              }
                                                                                                                                                                                                                                              __f},
                                                                                                                                                                                                                                              next: Box::new(None)
                                                                                                                                                                                                                                          }))
                                                                                                                                                                                                                  }))
                                                                                                                                                                                          })
                                                                                                                                                                       }))
                                                                                                                                               })
                                                                                                                               },
    );
pub static __Rule_var_decls_VariableDeclarations_14: ::once_cell::sync::Lazy<program::Rule> =
    ::once_cell::sync::Lazy::new(
        || /* var_decls::VariableDeclarations[(var_decls::VariableDeclarations{.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta}: var_decls::VariableDeclarations)] :- inputs::Property[(inputs::Property{.expr_id=(expr: ast::ExprId), .file=(file: ast::FileId), .key=(_: ddlog_std::Option<ast::PropertyKey>), .val=(ddlog_std::Some{.x=(val: ast::PropertyVal)}: ddlog_std::Option<ast::PropertyVal>)}: inputs::Property)], ((ddlog_std::Some{.x=((var params: ddlog_std::Vec<ast::FuncParam>), (var body: ast::StmtId))}: ddlog_std::Option<(ddlog_std::Vec<ast::FuncParam>, ast::StmtId)>) = ((ast::method_comps: function(ast::PropertyVal):ddlog_std::Option<(ddlog_std::Vec<ast::FuncParam>, ast::StmtId)>)(val))), inputs::Statement[(inputs::Statement{.id=(body: ast::StmtId), .file=(file: ast::FileId), .kind=(_: ast::StmtKind), .scope=(stmt_scope: ast::ScopeId), .span=(_: ast::Span)}: inputs::Statement)], var bound = FlatMap(((vec::flatmap: function(ddlog_std::Vec<ast::FuncParam>, function(ast::FuncParam):ddlog_std::Vec<(ast::Spanned<internment::Intern<string>>, bool)>):ddlog_std::Vec<(ast::Spanned<internment::Intern<string>>, bool)>)(params, (function(param: ast::FuncParam):ddlog_std::Vec<(ast::Spanned<internment::Intern<string>>, bool)>{((ast::bound_vars: function(ast::FuncParam):ddlog_std::Vec<(ast::Spanned<internment::Intern<string>>, bool)>)(param))})))), (((ast::Spanned{.data=(var name: internment::Intern<string>), .span=(var span: ast::Span)}: ast::Spanned<internment::Intern<string>>), (var implicit: bool)) = bound), ((var meta: ddlog_std::Ref<var_decls::VariableMeta>) = ((ddlog_std::ref_new: function(var_decls::VariableMeta):ddlog_std::Ref<var_decls::VariableMeta>)((var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=implicit, .declaration_span=(ddlog_std::Some{.x=span}: ddlog_std::Option<ast::Span>)}: var_decls::VariableMeta)))), ((var id: ast::AnyId) = (ast::AnyIdExpr{.expr=expr}: ast::AnyId)), ((var scope: var_decls::DeclarationScope) = (var_decls::Unhoistable{.scope=stmt_scope}: var_decls::DeclarationScope)). */
                                                                                                                               program::Rule::CollectionRule {
                                                                                                                                   description: std::borrow::Cow::from("var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::Property(.expr_id=expr, .file=file, .key=_, .val=ddlog_std::Some{.x=val}), (ddlog_std::Some{.x=(var params, var body)} = (ast::method_comps(val))), inputs::Statement(.id=body, .file=file, .kind=_, .scope=stmt_scope, .span=_), var bound = FlatMap((vec::flatmap(params, (function(param: ast::FuncParam):ddlog_std::Vec<(ast::Spanned<internment::Intern<string>>, bool)>{(ast::bound_vars(param))})))), ((ast::Spanned{.data=var name, .span=var span}, var implicit) = bound), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=implicit, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdExpr{.expr=expr}), (var scope = var_decls::Unhoistable{.scope=stmt_scope})."),
                                                                                                                                   rel: 46,
                                                                                                                                   xform: Some(XFormCollection::Arrange {
                                                                                                                                                   description: std::borrow::Cow::from("arrange inputs::Property(.expr_id=expr, .file=file, .key=_, .val=ddlog_std::Some{.x=val}) by (body, file)"),
                                                                                                                                                   afun: {fn __f(__v: DDValue) -> Option<(DDValue,DDValue)>
                                                                                                                                                   {
                                                                                                                                                       let (ref expr, ref file, ref val) = match *<types__inputs::Property>::from_ddvalue_ref(&__v) {
                                                                                                                                                           types__inputs::Property{expr_id: ref expr, file: ref file, key: _, val: ddlog_std::Option::Some{x: ref val}} => ((*expr).clone(), (*file).clone(), (*val).clone()),
                                                                                                                                                           _ => return None
                                                                                                                                                       };
                                                                                                                                                       let (ref params, ref body): (ddlog_std::Vec<types__ast::FuncParam>, types__ast::StmtId) = match types__ast::method_comps_ast_PropertyVal_ddlog_std_Option____Tuple2__ddlog_std_Vec__ast_FuncParam_ast_StmtId(val) {
                                                                                                                                                           ddlog_std::Option::Some{x: ddlog_std::tuple2(params, body)} => (params, body),
                                                                                                                                                           _ => return None
                                                                                                                                                       };
                                                                                                                                                       Some(((ddlog_std::tuple2((*body).clone(), (*file).clone())).into_ddvalue(), (ddlog_std::tuple3((*expr).clone(), (*file).clone(), (*params).clone())).into_ddvalue()))
                                                                                                                                                   }
                                                                                                                                                   __f},
                                                                                                                                                   next: Box::new(XFormArrangement::Join{
                                                                                                                                                                      description: std::borrow::Cow::from("inputs::Property(.expr_id=expr, .file=file, .key=_, .val=ddlog_std::Some{.x=val}), (ddlog_std::Some{.x=(var params, var body)} = (ast::method_comps(val))), inputs::Statement(.id=body, .file=file, .kind=_, .scope=stmt_scope, .span=_)"),
                                                                                                                                                                      ffun: None,
                                                                                                                                                                      arrangement: (48,0),
                                                                                                                                                                      jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                                                      {
                                                                                                                                                                          let ddlog_std::tuple3(ref expr, ref file, ref params) = *<ddlog_std::tuple3<types__ast::ExprId, types__ast::FileId, ddlog_std::Vec<types__ast::FuncParam>>>::from_ddvalue_ref( __v1 );
                                                                                                                                                                          let ref stmt_scope = match *<types__inputs::Statement>::from_ddvalue_ref(__v2) {
                                                                                                                                                                              types__inputs::Statement{id: _, file: _, kind: _, scope: ref stmt_scope, span: _} => (*stmt_scope).clone(),
                                                                                                                                                                              _ => return None
                                                                                                                                                                          };
                                                                                                                                                                          Some((ddlog_std::tuple4((*expr).clone(), (*file).clone(), (*params).clone(), (*stmt_scope).clone())).into_ddvalue())
                                                                                                                                                                      }
                                                                                                                                                                      __f},
                                                                                                                                                                      next: Box::new(Some(XFormCollection::FlatMap{
                                                                                                                                                                                              description: std::borrow::Cow::from("inputs::Property(.expr_id=expr, .file=file, .key=_, .val=ddlog_std::Some{.x=val}), (ddlog_std::Some{.x=(var params, var body)} = (ast::method_comps(val))), inputs::Statement(.id=body, .file=file, .kind=_, .scope=stmt_scope, .span=_), var bound = FlatMap((vec::flatmap(params, (function(param: ast::FuncParam):ddlog_std::Vec<(ast::Spanned<internment::Intern<string>>, bool)>{(ast::bound_vars(param))}))))"),
                                                                                                                                                                                              fmfun: {fn __f(__v: DDValue) -> Option<Box<dyn Iterator<Item=DDValue>>>
                                                                                                                                                                                              {
                                                                                                                                                                                                  let ddlog_std::tuple4(ref expr, ref file, ref params, ref stmt_scope) = *<ddlog_std::tuple4<types__ast::ExprId, types__ast::FileId, ddlog_std::Vec<types__ast::FuncParam>, types__ast::ScopeId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                                                  let __flattened = types__vec::flatmap::<types__ast::FuncParam, ddlog_std::tuple2<types__ast::Spanned<internment::Intern<String>>, bool>>(params, (&{
                                                                                                                                                                                                                                                                                                                                                         (Box::new(::ddlog_rt::ClosureImpl{
                                                                                                                                                                                                                                                                                                                                                             description: "(function(param: ast::FuncParam):ddlog_std::Vec<(ast::Spanned<internment::Intern<string>>, bool)>{(ast::bound_vars(param))})",
                                                                                                                                                                                                                                                                                                                                                             captured: (),
                                                                                                                                                                                                                                                                                                                                                             f: {
                                                                                                                                                                                                                                                                                                                                                                    fn __f(__args:*const types__ast::FuncParam, __captured: &()) -> ddlog_std::Vec<ddlog_std::tuple2<types__ast::Spanned<internment::Intern<String>>, bool>>
                                                                                                                                                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                                                                                                                                                        let param = unsafe{&*__args};
                                                                                                                                                                                                                                                                                                                                                                        types__ast::bound_vars_ast_FuncParam_ddlog_std_Vec____Tuple2__ast_Spanned__internment_Intern____Stringval___Boolval(param)
                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                    __f
                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                         }) as Box<dyn ::ddlog_rt::Closure<(*const types__ast::FuncParam), ddlog_std::Vec<ddlog_std::tuple2<types__ast::Spanned<internment::Intern<String>>, bool>>>>)
                                                                                                                                                                                                                                                                                                                                                     }));
                                                                                                                                                                                                  let expr = (*expr).clone();
                                                                                                                                                                                                  let file = (*file).clone();
                                                                                                                                                                                                  let stmt_scope = (*stmt_scope).clone();
                                                                                                                                                                                                  Some(Box::new(__flattened.into_iter().map(move |bound|(ddlog_std::tuple4(bound.clone(), expr.clone(), file.clone(), stmt_scope.clone())).into_ddvalue())))
                                                                                                                                                                                              }
                                                                                                                                                                                              __f},
                                                                                                                                                                                              next: Box::new(Some(XFormCollection::FilterMap{
                                                                                                                                                                                                                      description: std::borrow::Cow::from("head of var_decls::VariableDeclarations(.file=file, .name=name, .scope=scope, .declared_in=id, .meta=meta) :- inputs::Property(.expr_id=expr, .file=file, .key=_, .val=ddlog_std::Some{.x=val}), (ddlog_std::Some{.x=(var params, var body)} = (ast::method_comps(val))), inputs::Statement(.id=body, .file=file, .kind=_, .scope=stmt_scope, .span=_), var bound = FlatMap((vec::flatmap(params, (function(param: ast::FuncParam):ddlog_std::Vec<(ast::Spanned<internment::Intern<string>>, bool)>{(ast::bound_vars(param))})))), ((ast::Spanned{.data=var name, .span=var span}, var implicit) = bound), (var meta = (ddlog_std::ref_new(var_decls::VariableMeta{.is_function_argument=false, .implicitly_declared=implicit, .declaration_span=ddlog_std::Some{.x=span}}))), (var id = ast::AnyIdExpr{.expr=expr}), (var scope = var_decls::Unhoistable{.scope=stmt_scope})."),
                                                                                                                                                                                                                      fmfun: {fn __f(__v: DDValue) -> Option<DDValue>
                                                                                                                                                                                                                      {
                                                                                                                                                                                                                          let ddlog_std::tuple4(ref bound, ref expr, ref file, ref stmt_scope) = *<ddlog_std::tuple4<ddlog_std::tuple2<types__ast::Spanned<internment::Intern<String>>, bool>, types__ast::ExprId, types__ast::FileId, types__ast::ScopeId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                                                                          let (ref name, ref span, ref implicit): (internment::Intern<String>, types__ast::Span, bool) = match (*bound).clone() {
                                                                                                                                                                                                                              ddlog_std::tuple2(types__ast::Spanned{data: name, span: span}, implicit) => (name, span, implicit),
                                                                                                                                                                                                                              _ => return None
                                                                                                                                                                                                                          };
                                                                                                                                                                                                                          let ref meta: ddlog_std::Ref<VariableMeta> = match ddlog_std::ref_new((&(VariableMeta{is_function_argument: false, implicitly_declared: (*implicit).clone(), declaration_span: (ddlog_std::Option::Some{x: (*span).clone()})}))) {
                                                                                                                                                                                                                              meta => meta,
                                                                                                                                                                                                                              _ => return None
                                                                                                                                                                                                                          };
                                                                                                                                                                                                                          let ref id: types__ast::AnyId = match (types__ast::AnyId::AnyIdExpr{expr: (*expr).clone()}) {
                                                                                                                                                                                                                              id => id,
                                                                                                                                                                                                                              _ => return None
                                                                                                                                                                                                                          };
                                                                                                                                                                                                                          let ref scope: DeclarationScope = match (DeclarationScope::Unhoistable{scope: (*stmt_scope).clone()}) {
                                                                                                                                                                                                                              scope => scope,
                                                                                                                                                                                                                              _ => return None
                                                                                                                                                                                                                          };
                                                                                                                                                                                                                          Some(((VariableDeclarations{file: (*file).clone(), name: (*name).clone(), scope: (*scope).clone(), declared_in: (*id).clone(), meta: (*meta).clone()})).into_ddvalue())
                                                                                                                                                                                                                      }
                                                                                                                                                                                                                      __f},
                                                                                                                                                                                                                      next: Box::new(None)
                                                                                                                                                                                                                  }))
                                                                                                                                                                                          }))
                                                                                                                                                                  })
                                                                                                                                               })
                                                                                                                               },
    );
