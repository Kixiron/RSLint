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

#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct DeclarationInDescendent {
    pub file: types__ast::FileId,
    pub scope: types__ast::ScopeId,
    pub name: types__ast::Name,
    pub id: types__ast::AnyId,
}
impl abomonation::Abomonation for DeclarationInDescendent {}
::differential_datalog::decl_struct_from_record!(DeclarationInDescendent["outputs::no_shadow::DeclarationInDescendent"]<>, ["outputs::no_shadow::DeclarationInDescendent"][4]{[0]file["file"]: types__ast::FileId, [1]scope["scope"]: types__ast::ScopeId, [2]name["name"]: types__ast::Name, [3]id["id"]: types__ast::AnyId});
::differential_datalog::decl_struct_into_record!(DeclarationInDescendent, ["outputs::no_shadow::DeclarationInDescendent"]<>, file, scope, name, id);
#[rustfmt::skip] ::differential_datalog::decl_record_mutator_struct!(DeclarationInDescendent, <>, file: types__ast::FileId, scope: types__ast::ScopeId, name: types__ast::Name, id: types__ast::AnyId);
impl ::std::fmt::Display for DeclarationInDescendent {
    fn fmt(&self, __formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            DeclarationInDescendent {
                file,
                scope,
                name,
                id,
            } => {
                __formatter.write_str("outputs::no_shadow::DeclarationInDescendent{")?;
                ::std::fmt::Debug::fmt(file, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(scope, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(name, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(id, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl ::std::fmt::Debug for DeclarationInDescendent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self, f)
    }
}
#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct NoShadow {
    pub variable: types__ast::Name,
    pub original: ddlog_std::tuple2<types__ast::AnyId, types__ast::Span>,
    pub shadower: ddlog_std::tuple2<types__ast::AnyId, types__ast::Span>,
    pub implicit: bool,
    pub file: types__ast::FileId,
}
impl abomonation::Abomonation for NoShadow {}
::differential_datalog::decl_struct_from_record!(NoShadow["outputs::no_shadow::NoShadow"]<>, ["outputs::no_shadow::NoShadow"][5]{[0]variable["variable"]: types__ast::Name, [1]original["original"]: ddlog_std::tuple2<types__ast::AnyId, types__ast::Span>, [2]shadower["shadower"]: ddlog_std::tuple2<types__ast::AnyId, types__ast::Span>, [3]implicit["implicit"]: bool, [4]file["file"]: types__ast::FileId});
::differential_datalog::decl_struct_into_record!(NoShadow, ["outputs::no_shadow::NoShadow"]<>, variable, original, shadower, implicit, file);
#[rustfmt::skip] ::differential_datalog::decl_record_mutator_struct!(NoShadow, <>, variable: types__ast::Name, original: ddlog_std::tuple2<types__ast::AnyId, types__ast::Span>, shadower: ddlog_std::tuple2<types__ast::AnyId, types__ast::Span>, implicit: bool, file: types__ast::FileId);
impl ::std::fmt::Display for NoShadow {
    fn fmt(&self, __formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            NoShadow {
                variable,
                original,
                shadower,
                implicit,
                file,
            } => {
                __formatter.write_str("outputs::no_shadow::NoShadow{")?;
                ::std::fmt::Debug::fmt(variable, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(original, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(shadower, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(implicit, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(file, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl ::std::fmt::Debug for NoShadow {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self, f)
    }
}
#[derive(Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct ScopeOfDecl {
    pub file: types__ast::FileId,
    pub scope: types__ast::ScopeId,
    pub declared: types__ast::AnyId,
}
impl abomonation::Abomonation for ScopeOfDecl {}
::differential_datalog::decl_struct_from_record!(ScopeOfDecl["outputs::no_shadow::ScopeOfDecl"]<>, ["outputs::no_shadow::ScopeOfDecl"][3]{[0]file["file"]: types__ast::FileId, [1]scope["scope"]: types__ast::ScopeId, [2]declared["declared"]: types__ast::AnyId});
::differential_datalog::decl_struct_into_record!(ScopeOfDecl, ["outputs::no_shadow::ScopeOfDecl"]<>, file, scope, declared);
#[rustfmt::skip] ::differential_datalog::decl_record_mutator_struct!(ScopeOfDecl, <>, file: types__ast::FileId, scope: types__ast::ScopeId, declared: types__ast::AnyId);
impl ::std::fmt::Display for ScopeOfDecl {
    fn fmt(&self, __formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            ScopeOfDecl {
                file,
                scope,
                declared,
            } => {
                __formatter.write_str("outputs::no_shadow::ScopeOfDecl{")?;
                ::std::fmt::Debug::fmt(file, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(scope, __formatter)?;
                __formatter.write_str(",")?;
                ::std::fmt::Debug::fmt(declared, __formatter)?;
                __formatter.write_str("}")
            }
        }
    }
}
impl ::std::fmt::Debug for ScopeOfDecl {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self, f)
    }
}
pub static __Arng_outputs_no_shadow_ScopeOfDecl_0: ::once_cell::sync::Lazy<program::Arrangement> =
    ::once_cell::sync::Lazy::new(|| program::Arrangement::Map {
        name: std::borrow::Cow::from(
            r###"(outputs::no_shadow::ScopeOfDecl{.file=(_0: ast::FileId), .scope=(_: ast::ScopeId), .declared=(_1: ast::AnyId)}: outputs::no_shadow::ScopeOfDecl) /*join*/"###,
        ),
        afun: {
            fn __f(__v: DDValue) -> Option<(DDValue, DDValue)> {
                let __cloned = __v.clone();
                match <ScopeOfDecl>::from_ddvalue(__v) {
                    ScopeOfDecl {
                        file: ref _0,
                        scope: _,
                        declared: ref _1,
                    } => Some((ddlog_std::tuple2((*_0).clone(), (*_1).clone())).into_ddvalue()),
                    _ => None,
                }
                .map(|x| (x, __cloned))
            }
            __f
        },
        queryable: false,
    });
pub static __Arng_outputs_no_shadow_ScopeOfDecl_1: ::once_cell::sync::Lazy<program::Arrangement> =
    ::once_cell::sync::Lazy::new(|| program::Arrangement::Map {
        name: std::borrow::Cow::from(
            r###"(outputs::no_shadow::ScopeOfDecl{.file=(_0: ast::FileId), .scope=(_: ast::ScopeId), .declared=(_: ast::AnyId)}: outputs::no_shadow::ScopeOfDecl) /*join*/"###,
        ),
        afun: {
            fn __f(__v: DDValue) -> Option<(DDValue, DDValue)> {
                let __cloned = __v.clone();
                match <ScopeOfDecl>::from_ddvalue(__v) {
                    ScopeOfDecl {
                        file: ref _0,
                        scope: _,
                        declared: _,
                    } => Some(((*_0).clone()).into_ddvalue()),
                    _ => None,
                }
                .map(|x| (x, __cloned))
            }
            __f
        },
        queryable: false,
    });
pub static __Arng_outputs_no_shadow_DeclarationInDescendent_0: ::once_cell::sync::Lazy<
    program::Arrangement,
> = ::once_cell::sync::Lazy::new(|| program::Arrangement::Map {
    name: std::borrow::Cow::from(
        r###"(outputs::no_shadow::DeclarationInDescendent{.file=(_1: ast::FileId), .scope=(_0: ast::ScopeId), .name=(_: internment::Intern<string>), .id=(_: ast::AnyId)}: outputs::no_shadow::DeclarationInDescendent) /*join*/"###,
    ),
    afun: {
        fn __f(__v: DDValue) -> Option<(DDValue, DDValue)> {
            let __cloned = __v.clone();
            match <DeclarationInDescendent>::from_ddvalue(__v) {
                DeclarationInDescendent {
                    file: ref _1,
                    scope: ref _0,
                    name: _,
                    id: _,
                } => Some((ddlog_std::tuple2((*_0).clone(), (*_1).clone())).into_ddvalue()),
                _ => None,
            }
            .map(|x| (x, __cloned))
        }
        __f
    },
    queryable: false,
});
pub static __Arng_outputs_no_shadow_DeclarationInDescendent_1: ::once_cell::sync::Lazy<
    program::Arrangement,
> = ::once_cell::sync::Lazy::new(|| program::Arrangement::Map {
    name: std::borrow::Cow::from(
        r###"(outputs::no_shadow::DeclarationInDescendent{.file=(_0: ast::FileId), .scope=(_1: ast::ScopeId), .name=(_2: internment::Intern<string>), .id=(_: ast::AnyId)}: outputs::no_shadow::DeclarationInDescendent) /*join*/"###,
    ),
    afun: {
        fn __f(__v: DDValue) -> Option<(DDValue, DDValue)> {
            let __cloned = __v.clone();
            match <DeclarationInDescendent>::from_ddvalue(__v) {
                DeclarationInDescendent {
                    file: ref _0,
                    scope: ref _1,
                    name: ref _2,
                    id: _,
                } => Some(
                    (ddlog_std::tuple3((*_0).clone(), (*_1).clone(), (*_2).clone())).into_ddvalue(),
                ),
                _ => None,
            }
            .map(|x| (x, __cloned))
        }
        __f
    },
    queryable: false,
});
pub static __Rule_outputs_no_shadow_ScopeOfDecl_0: ::once_cell::sync::Lazy<program::Rule> =
    ::once_cell::sync::Lazy::new(
        || /* outputs::no_shadow::ScopeOfDecl[(outputs::no_shadow::ScopeOfDecl{.file=file, .scope=scope, .declared=declared}: outputs::no_shadow::ScopeOfDecl)] :- __Prefix_8[((file: ast::FileId), (config: config::Config))], var_decls::VariableDeclarations[(var_decls::VariableDeclarations{.file=(file: ast::FileId), .name=(_: internment::Intern<string>), .scope=(var_decls::Unhoistable{.scope=(scope: ast::ScopeId)}: var_decls::DeclarationScope), .declared_in=(declared: ast::AnyId), .meta=(_: ddlog_std::Ref<var_decls::VariableMeta>)}: var_decls::VariableDeclarations)], (not (ast::is_global(declared))). */
                                                                                                                             program::Rule::ArrangementRule {
                                                                                                                                 description: std::borrow::Cow::from( "outputs::no_shadow::ScopeOfDecl(.file=file, .scope=scope, .declared=declared) :- __Prefix_8[(file, config)], var_decls::VariableDeclarations(.file=file, .name=_, .scope=var_decls::Unhoistable{.scope=scope}, .declared_in=declared, .meta=_), (not (ast::is_global(declared)))."),
                                                                                                                                 arr: ( 6, 0),
                                                                                                                                 xform: XFormArrangement::Join{
                                                                                                                                            description: std::borrow::Cow::from("__Prefix_8[(file, config)], var_decls::VariableDeclarations(.file=file, .name=_, .scope=var_decls::Unhoistable{.scope=scope}, .declared_in=declared, .meta=_)"),
                                                                                                                                            ffun: None,
                                                                                                                                            arrangement: (86,4),
                                                                                                                                            jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                            {
                                                                                                                                                let (ref file, ref config) = match *<ddlog_std::tuple2<types__ast::FileId, types__config::Config>>::from_ddvalue_ref(__v1) {
                                                                                                                                                    ddlog_std::tuple2(ref file, ref config) => ((*file).clone(), (*config).clone()),
                                                                                                                                                    _ => return None
                                                                                                                                                };
                                                                                                                                                let (ref scope, ref declared) = match *<crate::var_decls::VariableDeclarations>::from_ddvalue_ref(__v2) {
                                                                                                                                                    crate::var_decls::VariableDeclarations{file: _, name: _, scope: crate::var_decls::DeclarationScope::Unhoistable{scope: ref scope}, declared_in: ref declared, meta: _} => ((*scope).clone(), (*declared).clone()),
                                                                                                                                                    _ => return None
                                                                                                                                                };
                                                                                                                                                if !(!types__ast::is_global(declared)) {return None;};
                                                                                                                                                Some(((ScopeOfDecl{file: (*file).clone(), scope: (*scope).clone(), declared: (*declared).clone()})).into_ddvalue())
                                                                                                                                            }
                                                                                                                                            __f},
                                                                                                                                            next: Box::new(None)
                                                                                                                                        }
                                                                                                                             },
    );
pub static __Rule_outputs_no_shadow_ScopeOfDecl_1: ::once_cell::sync::Lazy<program::Rule> =
    ::once_cell::sync::Lazy::new(
        || /* outputs::no_shadow::ScopeOfDecl[(outputs::no_shadow::ScopeOfDecl{.file=file, .scope=scope, .declared=declared}: outputs::no_shadow::ScopeOfDecl)] :- __Prefix_8[((file: ast::FileId), (config: config::Config))], var_decls::VariableDeclarations[(var_decls::VariableDeclarations{.file=(file: ast::FileId), .name=(_: internment::Intern<string>), .scope=(var_decls::Hoistable{.hoisted=(hoisted_scope: ast::ScopeId), .unhoisted=(unhoisted_scope: ast::ScopeId)}: var_decls::DeclarationScope), .declared_in=(declared: ast::AnyId), .meta=(_: ddlog_std::Ref<var_decls::VariableMeta>)}: var_decls::VariableDeclarations)], (not (ast::is_global(declared))), ((var scope: ast::ScopeId) = if (config::no_shadow_hoisting(config)) {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     hoisted_scope
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 } else {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       unhoisted_scope
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   }). */
                                                                                                                             program::Rule::ArrangementRule {
                                                                                                                                 description: std::borrow::Cow::from( "outputs::no_shadow::ScopeOfDecl(.file=file, .scope=scope, .declared=declared) :- __Prefix_8[(file, config)], var_decls::VariableDeclarations(.file=file, .name=_, .scope=var_decls::Hoistable{.hoisted=hoisted_scope, .unhoisted=unhoisted_scope}, .declared_in=declared, .meta=_), (not (ast::is_global(declared))), (var scope = if (config::no_shadow_hoisting(config)) {\n                                                                                                                                                                                                                                                                                                                                       hoisted_scope\n                                                                                                                                                                                                                                                                                                                                   } else {\n                                                                                                                                                                                                                                                                                                                                         unhoisted_scope\n                                                                                                                                                                                                                                                                                                                                     })."),
                                                                                                                                 arr: ( 6, 0),
                                                                                                                                 xform: XFormArrangement::Join{
                                                                                                                                            description: std::borrow::Cow::from("__Prefix_8[(file, config)], var_decls::VariableDeclarations(.file=file, .name=_, .scope=var_decls::Hoistable{.hoisted=hoisted_scope, .unhoisted=unhoisted_scope}, .declared_in=declared, .meta=_)"),
                                                                                                                                            ffun: None,
                                                                                                                                            arrangement: (86,5),
                                                                                                                                            jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                            {
                                                                                                                                                let (ref file, ref config) = match *<ddlog_std::tuple2<types__ast::FileId, types__config::Config>>::from_ddvalue_ref(__v1) {
                                                                                                                                                    ddlog_std::tuple2(ref file, ref config) => ((*file).clone(), (*config).clone()),
                                                                                                                                                    _ => return None
                                                                                                                                                };
                                                                                                                                                let (ref hoisted_scope, ref unhoisted_scope, ref declared) = match *<crate::var_decls::VariableDeclarations>::from_ddvalue_ref(__v2) {
                                                                                                                                                    crate::var_decls::VariableDeclarations{file: _, name: _, scope: crate::var_decls::DeclarationScope::Hoistable{hoisted: ref hoisted_scope, unhoisted: ref unhoisted_scope}, declared_in: ref declared, meta: _} => ((*hoisted_scope).clone(), (*unhoisted_scope).clone(), (*declared).clone()),
                                                                                                                                                    _ => return None
                                                                                                                                                };
                                                                                                                                                if !(!types__ast::is_global(declared)) {return None;};
                                                                                                                                                let ref scope: types__ast::ScopeId = match if types__config::no_shadow_hoisting(config) {
                                                                                                                                                                                               (*hoisted_scope).clone()
                                                                                                                                                                                           } else {
                                                                                                                                                                                               (*unhoisted_scope).clone()
                                                                                                                                                                                           } {
                                                                                                                                                    scope => scope,
                                                                                                                                                    _ => return None
                                                                                                                                                };
                                                                                                                                                Some(((ScopeOfDecl{file: (*file).clone(), scope: (*scope).clone(), declared: (*declared).clone()})).into_ddvalue())
                                                                                                                                            }
                                                                                                                                            __f},
                                                                                                                                            next: Box::new(None)
                                                                                                                                        }
                                                                                                                             },
    );
pub static __Rule_outputs_no_shadow_DeclarationInDescendent_0: ::once_cell::sync::Lazy<
    program::Rule,
> = ::once_cell::sync::Lazy::new(
    || /* outputs::no_shadow::DeclarationInDescendent[(outputs::no_shadow::DeclarationInDescendent{.file=file, .scope=scope, .name=name, .id=id}: outputs::no_shadow::DeclarationInDescendent)] :- outputs::no_shadow::ScopeOfDecl[(outputs::no_shadow::ScopeOfDecl{.file=(file: ast::FileId), .scope=(scope: ast::ScopeId), .declared=(id: ast::AnyId)}: outputs::no_shadow::ScopeOfDecl)], var_decls::VariableDeclarations[(var_decls::VariableDeclarations{.file=(file: ast::FileId), .name=(name: internment::Intern<string>), .scope=(_: var_decls::DeclarationScope), .declared_in=(id: ast::AnyId), .meta=(_: ddlog_std::Ref<var_decls::VariableMeta>)}: var_decls::VariableDeclarations)]. */
                                                                                                                                         program::Rule::ArrangementRule {
                                                                                                                                             description: std::borrow::Cow::from( "outputs::no_shadow::DeclarationInDescendent(.file=file, .scope=scope, .name=name, .id=id) :- outputs::no_shadow::ScopeOfDecl(.file=file, .scope=scope, .declared=id), var_decls::VariableDeclarations(.file=file, .name=name, .scope=_, .declared_in=id, .meta=_)."),
                                                                                                                                             arr: ( 67, 0),
                                                                                                                                             xform: XFormArrangement::Join{
                                                                                                                                                        description: std::borrow::Cow::from("outputs::no_shadow::ScopeOfDecl(.file=file, .scope=scope, .declared=id), var_decls::VariableDeclarations(.file=file, .name=name, .scope=_, .declared_in=id, .meta=_)"),
                                                                                                                                                        ffun: None,
                                                                                                                                                        arrangement: (86,1),
                                                                                                                                                        jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                                        {
                                                                                                                                                            let (ref file, ref scope, ref id) = match *<ScopeOfDecl>::from_ddvalue_ref(__v1) {
                                                                                                                                                                ScopeOfDecl{file: ref file, scope: ref scope, declared: ref id} => ((*file).clone(), (*scope).clone(), (*id).clone()),
                                                                                                                                                                _ => return None
                                                                                                                                                            };
                                                                                                                                                            let ref name = match *<crate::var_decls::VariableDeclarations>::from_ddvalue_ref(__v2) {
                                                                                                                                                                crate::var_decls::VariableDeclarations{file: _, name: ref name, scope: _, declared_in: _, meta: _} => (*name).clone(),
                                                                                                                                                                _ => return None
                                                                                                                                                            };
                                                                                                                                                            Some(((DeclarationInDescendent{file: (*file).clone(), scope: (*scope).clone(), name: (*name).clone(), id: (*id).clone()})).into_ddvalue())
                                                                                                                                                        }
                                                                                                                                                        __f},
                                                                                                                                                        next: Box::new(None)
                                                                                                                                                    }
                                                                                                                                         },
);
pub static __Rule_outputs_no_shadow_DeclarationInDescendent_1: ::once_cell::sync::Lazy<
    program::Rule,
> = ::once_cell::sync::Lazy::new(
    || /* outputs::no_shadow::DeclarationInDescendent[(outputs::no_shadow::DeclarationInDescendent{.file=file, .scope=parent, .name=name, .id=id}: outputs::no_shadow::DeclarationInDescendent)] :- outputs::no_shadow::DeclarationInDescendent[(outputs::no_shadow::DeclarationInDescendent{.file=(file: ast::FileId), .scope=(child: ast::ScopeId), .name=(name: internment::Intern<string>), .id=(id: ast::AnyId)}: outputs::no_shadow::DeclarationInDescendent)], inputs::InputScope[(inputs::InputScope{.parent=(parent: ast::ScopeId), .child=(child: ast::ScopeId), .file=(file: ast::FileId)}: inputs::InputScope)]. */
                                                                                                                                         program::Rule::ArrangementRule {
                                                                                                                                             description: std::borrow::Cow::from( "outputs::no_shadow::DeclarationInDescendent(.file=file, .scope=parent, .name=name, .id=id) :- outputs::no_shadow::DeclarationInDescendent(.file=file, .scope=child, .name=name, .id=id), inputs::InputScope(.parent=parent, .child=child, .file=file)."),
                                                                                                                                             arr: ( 65, 0),
                                                                                                                                             xform: XFormArrangement::Join{
                                                                                                                                                        description: std::borrow::Cow::from("outputs::no_shadow::DeclarationInDescendent(.file=file, .scope=child, .name=name, .id=id), inputs::InputScope(.parent=parent, .child=child, .file=file)"),
                                                                                                                                                        ffun: None,
                                                                                                                                                        arrangement: (41,0),
                                                                                                                                                        jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                                        {
                                                                                                                                                            let (ref file, ref child, ref name, ref id) = match *<DeclarationInDescendent>::from_ddvalue_ref(__v1) {
                                                                                                                                                                DeclarationInDescendent{file: ref file, scope: ref child, name: ref name, id: ref id} => ((*file).clone(), (*child).clone(), (*name).clone(), (*id).clone()),
                                                                                                                                                                _ => return None
                                                                                                                                                            };
                                                                                                                                                            let ref parent = match *<types__inputs::InputScope>::from_ddvalue_ref(__v2) {
                                                                                                                                                                types__inputs::InputScope{parent: ref parent, child: _, file: _} => (*parent).clone(),
                                                                                                                                                                _ => return None
                                                                                                                                                            };
                                                                                                                                                            Some(((DeclarationInDescendent{file: (*file).clone(), scope: (*parent).clone(), name: (*name).clone(), id: (*id).clone()})).into_ddvalue())
                                                                                                                                                        }
                                                                                                                                                        __f},
                                                                                                                                                        next: Box::new(None)
                                                                                                                                                    }
                                                                                                                                         },
);
pub static __Rule_outputs_no_shadow_NoShadow_0: ::once_cell::sync::Lazy<program::Rule> =
    ::once_cell::sync::Lazy::new(
        || /* outputs::no_shadow::NoShadow[(outputs::no_shadow::NoShadow{.variable=name, .original=(shadowed_id, shadowed_span), .shadower=(shadower_id, shadower_span), .implicit=false, .file=file}: outputs::no_shadow::NoShadow)] :- __Prefix_8[((file: ast::FileId), (config: config::Config))], outputs::no_shadow::ScopeOfDecl[(outputs::no_shadow::ScopeOfDecl{.file=(file: ast::FileId), .scope=(shadowed_scope: ast::ScopeId), .declared=(shadowed_id: ast::AnyId)}: outputs::no_shadow::ScopeOfDecl)], var_decls::VariableDeclarations[(var_decls::VariableDeclarations{.file=(file: ast::FileId), .name=(name: internment::Intern<string>), .scope=(shadowed_scope_raw: var_decls::DeclarationScope), .declared_in=(shadowed_id: ast::AnyId), .meta=((&(var_decls::VariableMeta{.is_function_argument=(_: bool), .implicitly_declared=false, .declaration_span=(ddlog_std::Some{.x=(shadowed_span: ast::Span)}: ddlog_std::Option<ast::Span>)}: var_decls::VariableMeta)): ddlog_std::Ref<var_decls::VariableMeta>)}: var_decls::VariableDeclarations)], outputs::no_shadow::DeclarationInDescendent[(outputs::no_shadow::DeclarationInDescendent{.file=(file: ast::FileId), .scope=(shadowed_scope: ast::ScopeId), .name=(name: internment::Intern<string>), .id=(shadower_id: ast::AnyId)}: outputs::no_shadow::DeclarationInDescendent)], (shadowed_id != shadower_id), var_decls::VariableDeclarations[(var_decls::VariableDeclarations{.file=(file: ast::FileId), .name=(name: internment::Intern<string>), .scope=(shadower_scope_raw: var_decls::DeclarationScope), .declared_in=(shadower_id: ast::AnyId), .meta=((&(var_decls::VariableMeta{.is_function_argument=(_: bool), .implicitly_declared=false, .declaration_span=(ddlog_std::Some{.x=(shadower_span: ast::Span)}: ddlog_std::Option<ast::Span>)}: var_decls::VariableMeta)): ddlog_std::Ref<var_decls::VariableMeta>)}: var_decls::VariableDeclarations)], match (((config::no_shadow_hoisting(config)), (var_decls::is_hoistable(shadower_scope_raw)), (var_decls::is_hoistable(shadowed_scope_raw)))) {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             (true, true, true) -> (shadower_span < shadowed_span),
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             (true, false, true) -> (shadower_span < shadowed_span),
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             (true, true, false) -> (shadower_span < shadowed_span),
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             ((_: bool), (_: bool), (_: bool)) -> true
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         }. */
                                                                                                                          program::Rule::ArrangementRule {
                                                                                                                              description: std::borrow::Cow::from( "outputs::no_shadow::NoShadow(.variable=name, .original=(shadowed_id, shadowed_span), .shadower=(shadower_id, shadower_span), .implicit=false, .file=file) :- __Prefix_8[(file, config)], outputs::no_shadow::ScopeOfDecl(.file=file, .scope=shadowed_scope, .declared=shadowed_id), var_decls::VariableDeclarations(.file=file, .name=name, .scope=shadowed_scope_raw, .declared_in=shadowed_id, .meta=(&var_decls::VariableMeta{.is_function_argument=_, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=shadowed_span}})), outputs::no_shadow::DeclarationInDescendent(.file=file, .scope=shadowed_scope, .name=name, .id=shadower_id), (shadowed_id != shadower_id), var_decls::VariableDeclarations(.file=file, .name=name, .scope=shadower_scope_raw, .declared_in=shadower_id, .meta=(&var_decls::VariableMeta{.is_function_argument=_, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=shadower_span}})), match (((config::no_shadow_hoisting(config)), (var_decls::is_hoistable(shadower_scope_raw)), (var_decls::is_hoistable(shadowed_scope_raw)))) {\n                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       (true, true, true) -> (shadower_span < shadowed_span),\n                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       (true, false, true) -> (shadower_span < shadowed_span),\n                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       (true, true, false) -> (shadower_span < shadowed_span),\n                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       (_, _, _) -> true\n                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   }."),
                                                                                                                              arr: ( 6, 0),
                                                                                                                              xform: XFormArrangement::Join{
                                                                                                                                         description: std::borrow::Cow::from("__Prefix_8[(file, config)], outputs::no_shadow::ScopeOfDecl(.file=file, .scope=shadowed_scope, .declared=shadowed_id)"),
                                                                                                                                         ffun: None,
                                                                                                                                         arrangement: (67,1),
                                                                                                                                         jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                         {
                                                                                                                                             let (ref file, ref config) = match *<ddlog_std::tuple2<types__ast::FileId, types__config::Config>>::from_ddvalue_ref(__v1) {
                                                                                                                                                 ddlog_std::tuple2(ref file, ref config) => ((*file).clone(), (*config).clone()),
                                                                                                                                                 _ => return None
                                                                                                                                             };
                                                                                                                                             let (ref shadowed_scope, ref shadowed_id) = match *<ScopeOfDecl>::from_ddvalue_ref(__v2) {
                                                                                                                                                 ScopeOfDecl{file: _, scope: ref shadowed_scope, declared: ref shadowed_id} => ((*shadowed_scope).clone(), (*shadowed_id).clone()),
                                                                                                                                                 _ => return None
                                                                                                                                             };
                                                                                                                                             Some((ddlog_std::tuple4((*file).clone(), (*config).clone(), (*shadowed_scope).clone(), (*shadowed_id).clone())).into_ddvalue())
                                                                                                                                         }
                                                                                                                                         __f},
                                                                                                                                         next: Box::new(Some(XFormCollection::Arrange {
                                                                                                                                                                 description: std::borrow::Cow::from("arrange __Prefix_8[(file, config)], outputs::no_shadow::ScopeOfDecl(.file=file, .scope=shadowed_scope, .declared=shadowed_id) by (file, shadowed_id)"),
                                                                                                                                                                 afun: {fn __f(__v: DDValue) -> Option<(DDValue,DDValue)>
                                                                                                                                                                 {
                                                                                                                                                                     let ddlog_std::tuple4(ref file, ref config, ref shadowed_scope, ref shadowed_id) = *<ddlog_std::tuple4<types__ast::FileId, types__config::Config, types__ast::ScopeId, types__ast::AnyId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                     Some(((ddlog_std::tuple2((*file).clone(), (*shadowed_id).clone())).into_ddvalue(), (ddlog_std::tuple4((*file).clone(), (*config).clone(), (*shadowed_scope).clone(), (*shadowed_id).clone())).into_ddvalue()))
                                                                                                                                                                 }
                                                                                                                                                                 __f},
                                                                                                                                                                 next: Box::new(XFormArrangement::Join{
                                                                                                                                                                                    description: std::borrow::Cow::from("__Prefix_8[(file, config)], outputs::no_shadow::ScopeOfDecl(.file=file, .scope=shadowed_scope, .declared=shadowed_id), var_decls::VariableDeclarations(.file=file, .name=name, .scope=shadowed_scope_raw, .declared_in=shadowed_id, .meta=(&var_decls::VariableMeta{.is_function_argument=_, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=shadowed_span}}))"),
                                                                                                                                                                                    ffun: None,
                                                                                                                                                                                    arrangement: (86,2),
                                                                                                                                                                                    jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                                                                    {
                                                                                                                                                                                        let ddlog_std::tuple4(ref file, ref config, ref shadowed_scope, ref shadowed_id) = *<ddlog_std::tuple4<types__ast::FileId, types__config::Config, types__ast::ScopeId, types__ast::AnyId>>::from_ddvalue_ref( __v1 );
                                                                                                                                                                                        let (ref name, ref shadowed_scope_raw, ref shadowed_span) = match *<crate::var_decls::VariableDeclarations>::from_ddvalue_ref(__v2) {
                                                                                                                                                                                            crate::var_decls::VariableDeclarations{file: _, name: ref name, scope: ref shadowed_scope_raw, declared_in: _, meta: ref _0_} => match ((*_0_)).deref() {
                                                                                                                                                                                                                                                                                                                                 crate::var_decls::VariableMeta{is_function_argument: _, implicitly_declared: _, declaration_span: ddlog_std::Option::Some{x: shadowed_span}} => ((*name).clone(), (*shadowed_scope_raw).clone(), (*shadowed_span).clone()),
                                                                                                                                                                                                                                                                                                                                 _ => return None
                                                                                                                                                                                                                                                                                                                             },
                                                                                                                                                                                            _ => return None
                                                                                                                                                                                        };
                                                                                                                                                                                        Some((ddlog_std::tuple7((*file).clone(), (*config).clone(), (*shadowed_scope).clone(), (*shadowed_id).clone(), (*name).clone(), (*shadowed_scope_raw).clone(), (*shadowed_span).clone())).into_ddvalue())
                                                                                                                                                                                    }
                                                                                                                                                                                    __f},
                                                                                                                                                                                    next: Box::new(Some(XFormCollection::Arrange {
                                                                                                                                                                                                            description: std::borrow::Cow::from("arrange __Prefix_8[(file, config)], outputs::no_shadow::ScopeOfDecl(.file=file, .scope=shadowed_scope, .declared=shadowed_id), var_decls::VariableDeclarations(.file=file, .name=name, .scope=shadowed_scope_raw, .declared_in=shadowed_id, .meta=(&var_decls::VariableMeta{.is_function_argument=_, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=shadowed_span}})) by (file, shadowed_scope, name)"),
                                                                                                                                                                                                            afun: {fn __f(__v: DDValue) -> Option<(DDValue,DDValue)>
                                                                                                                                                                                                            {
                                                                                                                                                                                                                let ddlog_std::tuple7(ref file, ref config, ref shadowed_scope, ref shadowed_id, ref name, ref shadowed_scope_raw, ref shadowed_span) = *<ddlog_std::tuple7<types__ast::FileId, types__config::Config, types__ast::ScopeId, types__ast::AnyId, internment::Intern<String>, crate::var_decls::DeclarationScope, types__ast::Span>>::from_ddvalue_ref( &__v );
                                                                                                                                                                                                                Some(((ddlog_std::tuple3((*file).clone(), (*shadowed_scope).clone(), (*name).clone())).into_ddvalue(), (ddlog_std::tuple6((*file).clone(), (*config).clone(), (*shadowed_id).clone(), (*name).clone(), (*shadowed_scope_raw).clone(), (*shadowed_span).clone())).into_ddvalue()))
                                                                                                                                                                                                            }
                                                                                                                                                                                                            __f},
                                                                                                                                                                                                            next: Box::new(XFormArrangement::Join{
                                                                                                                                                                                                                               description: std::borrow::Cow::from("__Prefix_8[(file, config)], outputs::no_shadow::ScopeOfDecl(.file=file, .scope=shadowed_scope, .declared=shadowed_id), var_decls::VariableDeclarations(.file=file, .name=name, .scope=shadowed_scope_raw, .declared_in=shadowed_id, .meta=(&var_decls::VariableMeta{.is_function_argument=_, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=shadowed_span}})), outputs::no_shadow::DeclarationInDescendent(.file=file, .scope=shadowed_scope, .name=name, .id=shadower_id)"),
                                                                                                                                                                                                                               ffun: None,
                                                                                                                                                                                                                               arrangement: (65,1),
                                                                                                                                                                                                                               jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                                                                                                               {
                                                                                                                                                                                                                                   let ddlog_std::tuple6(ref file, ref config, ref shadowed_id, ref name, ref shadowed_scope_raw, ref shadowed_span) = *<ddlog_std::tuple6<types__ast::FileId, types__config::Config, types__ast::AnyId, internment::Intern<String>, crate::var_decls::DeclarationScope, types__ast::Span>>::from_ddvalue_ref( __v1 );
                                                                                                                                                                                                                                   let ref shadower_id = match *<DeclarationInDescendent>::from_ddvalue_ref(__v2) {
                                                                                                                                                                                                                                       DeclarationInDescendent{file: _, scope: _, name: _, id: ref shadower_id} => (*shadower_id).clone(),
                                                                                                                                                                                                                                       _ => return None
                                                                                                                                                                                                                                   };
                                                                                                                                                                                                                                   if !((&*shadowed_id) != (&*shadower_id)) {return None;};
                                                                                                                                                                                                                                   Some((ddlog_std::tuple7((*file).clone(), (*config).clone(), (*shadowed_id).clone(), (*name).clone(), (*shadowed_scope_raw).clone(), (*shadowed_span).clone(), (*shadower_id).clone())).into_ddvalue())
                                                                                                                                                                                                                               }
                                                                                                                                                                                                                               __f},
                                                                                                                                                                                                                               next: Box::new(Some(XFormCollection::Arrange {
                                                                                                                                                                                                                                                       description: std::borrow::Cow::from("arrange __Prefix_8[(file, config)], outputs::no_shadow::ScopeOfDecl(.file=file, .scope=shadowed_scope, .declared=shadowed_id), var_decls::VariableDeclarations(.file=file, .name=name, .scope=shadowed_scope_raw, .declared_in=shadowed_id, .meta=(&var_decls::VariableMeta{.is_function_argument=_, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=shadowed_span}})), outputs::no_shadow::DeclarationInDescendent(.file=file, .scope=shadowed_scope, .name=name, .id=shadower_id), (shadowed_id != shadower_id) by (file, name, shadower_id)"),
                                                                                                                                                                                                                                                       afun: {fn __f(__v: DDValue) -> Option<(DDValue,DDValue)>
                                                                                                                                                                                                                                                       {
                                                                                                                                                                                                                                                           let ddlog_std::tuple7(ref file, ref config, ref shadowed_id, ref name, ref shadowed_scope_raw, ref shadowed_span, ref shadower_id) = *<ddlog_std::tuple7<types__ast::FileId, types__config::Config, types__ast::AnyId, internment::Intern<String>, crate::var_decls::DeclarationScope, types__ast::Span, types__ast::AnyId>>::from_ddvalue_ref( &__v );
                                                                                                                                                                                                                                                           Some(((ddlog_std::tuple3((*file).clone(), (*name).clone(), (*shadower_id).clone())).into_ddvalue(), (ddlog_std::tuple7((*file).clone(), (*config).clone(), (*shadowed_id).clone(), (*name).clone(), (*shadowed_scope_raw).clone(), (*shadowed_span).clone(), (*shadower_id).clone())).into_ddvalue()))
                                                                                                                                                                                                                                                       }
                                                                                                                                                                                                                                                       __f},
                                                                                                                                                                                                                                                       next: Box::new(XFormArrangement::Join{
                                                                                                                                                                                                                                                                          description: std::borrow::Cow::from("__Prefix_8[(file, config)], outputs::no_shadow::ScopeOfDecl(.file=file, .scope=shadowed_scope, .declared=shadowed_id), var_decls::VariableDeclarations(.file=file, .name=name, .scope=shadowed_scope_raw, .declared_in=shadowed_id, .meta=(&var_decls::VariableMeta{.is_function_argument=_, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=shadowed_span}})), outputs::no_shadow::DeclarationInDescendent(.file=file, .scope=shadowed_scope, .name=name, .id=shadower_id), (shadowed_id != shadower_id), var_decls::VariableDeclarations(.file=file, .name=name, .scope=shadower_scope_raw, .declared_in=shadower_id, .meta=(&var_decls::VariableMeta{.is_function_argument=_, .implicitly_declared=false, .declaration_span=ddlog_std::Some{.x=shadower_span}}))"),
                                                                                                                                                                                                                                                                          ffun: None,
                                                                                                                                                                                                                                                                          arrangement: (86,3),
                                                                                                                                                                                                                                                                          jfun: {fn __f(_: &DDValue ,__v1: &DDValue,__v2: &DDValue) -> Option<DDValue>
                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                              let ddlog_std::tuple7(ref file, ref config, ref shadowed_id, ref name, ref shadowed_scope_raw, ref shadowed_span, ref shadower_id) = *<ddlog_std::tuple7<types__ast::FileId, types__config::Config, types__ast::AnyId, internment::Intern<String>, crate::var_decls::DeclarationScope, types__ast::Span, types__ast::AnyId>>::from_ddvalue_ref( __v1 );
                                                                                                                                                                                                                                                                              let (ref shadower_scope_raw, ref shadower_span) = match *<crate::var_decls::VariableDeclarations>::from_ddvalue_ref(__v2) {
                                                                                                                                                                                                                                                                                  crate::var_decls::VariableDeclarations{file: _, name: _, scope: ref shadower_scope_raw, declared_in: _, meta: ref _0_} => match ((*_0_)).deref() {
                                                                                                                                                                                                                                                                                                                                                                                                                crate::var_decls::VariableMeta{is_function_argument: _, implicitly_declared: _, declaration_span: ddlog_std::Option::Some{x: shadower_span}} => ((*shadower_scope_raw).clone(), (*shadower_span).clone()),
                                                                                                                                                                                                                                                                                                                                                                                                                _ => return None
                                                                                                                                                                                                                                                                                                                                                                                                            },
                                                                                                                                                                                                                                                                                  _ => return None
                                                                                                                                                                                                                                                                              };
                                                                                                                                                                                                                                                                              if !match ddlog_std::tuple3(types__config::no_shadow_hoisting(config), crate::var_decls::is_hoistable(shadower_scope_raw), crate::var_decls::is_hoistable(shadowed_scope_raw)) {
                                                                                                                                                                                                                                                                                      ddlog_std::tuple3(true, true, true) => ((&*shadower_span) < (&*shadowed_span)),
                                                                                                                                                                                                                                                                                      ddlog_std::tuple3(true, false, true) => ((&*shadower_span) < (&*shadowed_span)),
                                                                                                                                                                                                                                                                                      ddlog_std::tuple3(true, true, false) => ((&*shadower_span) < (&*shadowed_span)),
                                                                                                                                                                                                                                                                                      ddlog_std::tuple3(_, _, _) => true
                                                                                                                                                                                                                                                                                  } {return None;};
                                                                                                                                                                                                                                                                              Some(((NoShadow{variable: (*name).clone(), original: ddlog_std::tuple2((*shadowed_id).clone(), (*shadowed_span).clone()), shadower: ddlog_std::tuple2((*shadower_id).clone(), (*shadower_span).clone()), implicit: false, file: (*file).clone()})).into_ddvalue())
                                                                                                                                                                                                                                                                          }
                                                                                                                                                                                                                                                                          __f},
                                                                                                                                                                                                                                                                          next: Box::new(None)
                                                                                                                                                                                                                                                                      })
                                                                                                                                                                                                                                                   }))
                                                                                                                                                                                                                           })
                                                                                                                                                                                                        }))
                                                                                                                                                                                })
                                                                                                                                                             }))
                                                                                                                                     }
                                                                                                                          },
    );
