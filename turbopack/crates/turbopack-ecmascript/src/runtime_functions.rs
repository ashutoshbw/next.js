use std::fmt::{Display, Formatter};

use swc_core::ecma::ast::{Expr, MemberExpr, MemberProp};
use turbopack_core::compile_time_info::FreeVarReference;

#[derive(Clone, Copy)]
pub struct TurbopackRuntimeFunctionShortcut(pub &'static str);

impl Display for TurbopackRuntimeFunctionShortcut {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "__turbopack_context__.{}", self.0)
    }
}

impl Into<FreeVarReference> for TurbopackRuntimeFunctionShortcut {
    fn into(self) -> FreeVarReference {
        FreeVarReference::Member("__turbopack_context__".into(), self.0.into())
    }
}

impl Into<Expr> for TurbopackRuntimeFunctionShortcut {
    fn into(self) -> Expr {
        Expr::Member(MemberExpr {
            obj: Box::new(Expr::Ident("__turbopack_context__".into())),
            prop: MemberProp::Ident(self.0.into()),
            ..Default::default()
        })
    }
}

pub const TURBOPACK_REQUIRE: TurbopackRuntimeFunctionShortcut =
    TurbopackRuntimeFunctionShortcut("r");
pub const TURBOPACK_MODULE_CONTEXT: TurbopackRuntimeFunctionShortcut =
    TurbopackRuntimeFunctionShortcut("f");
pub const TURBOPACK_IMPORT: TurbopackRuntimeFunctionShortcut =
    TurbopackRuntimeFunctionShortcut("i");
pub const TURBOPACK_ESM: TurbopackRuntimeFunctionShortcut = TurbopackRuntimeFunctionShortcut("s");
pub const TURBOPACK_EXPORT_VALUE: TurbopackRuntimeFunctionShortcut =
    TurbopackRuntimeFunctionShortcut("v");
pub const TURBOPACK_EXPORT_NAMESPACE: TurbopackRuntimeFunctionShortcut =
    TurbopackRuntimeFunctionShortcut("n");
pub const TURBOPACK_CACHE: TurbopackRuntimeFunctionShortcut = TurbopackRuntimeFunctionShortcut("c");
pub const TURBOPACK_MODULES: TurbopackRuntimeFunctionShortcut =
    TurbopackRuntimeFunctionShortcut("M");
pub const TURBOPACK_LOAD: TurbopackRuntimeFunctionShortcut = TurbopackRuntimeFunctionShortcut("l");
pub const TURBOPACK_DYNAMIC: TurbopackRuntimeFunctionShortcut =
    TurbopackRuntimeFunctionShortcut("j");
pub const TURBOPACK_RESOLVE_ABSOLUTE_PATH: TurbopackRuntimeFunctionShortcut =
    TurbopackRuntimeFunctionShortcut("P");
pub const TURBOPACK_RELATIVE_URL: TurbopackRuntimeFunctionShortcut =
    TurbopackRuntimeFunctionShortcut("U");
pub const TURBOPACK_RESOLVE_MODULE_ID_PATH: TurbopackRuntimeFunctionShortcut =
    TurbopackRuntimeFunctionShortcut("R");
pub const TURBOPACK_WORKER_BLOB_URL: TurbopackRuntimeFunctionShortcut =
    TurbopackRuntimeFunctionShortcut("b");
pub const TURBOPACK_ASYNC_MODULE: TurbopackRuntimeFunctionShortcut =
    TurbopackRuntimeFunctionShortcut("a");
pub const TURBOPACK_EXTERNAL_REQUIRE: TurbopackRuntimeFunctionShortcut =
    TurbopackRuntimeFunctionShortcut("x");
pub const TURBOPACK_EXTERNAL_IMPORT: TurbopackRuntimeFunctionShortcut =
    TurbopackRuntimeFunctionShortcut("y");
pub const TURBOPACK_REFRESH: TurbopackRuntimeFunctionShortcut =
    TurbopackRuntimeFunctionShortcut("k");
pub const TURBOPACK_REQUIRE_STUB: TurbopackRuntimeFunctionShortcut =
    TurbopackRuntimeFunctionShortcut("z");
pub const TURBOPACK_REQUIRE_REAL: TurbopackRuntimeFunctionShortcut =
    TurbopackRuntimeFunctionShortcut("t");

pub const TUBROPACK_RUNTIME_FUNCTION_SHORTCUTS: [(&str, TurbopackRuntimeFunctionShortcut); 20] = [
    ("__turbopack_require__", TURBOPACK_REQUIRE),
    ("__turbopack_module_context__", TURBOPACK_MODULE_CONTEXT),
    ("__turbopack_import__", TURBOPACK_IMPORT),
    ("__turbopack_esm__", TURBOPACK_ESM),
    ("__turbopack_export_value__", TURBOPACK_EXPORT_VALUE),
    ("__turbopack_export_namespace__", TURBOPACK_EXPORT_NAMESPACE),
    ("__turbopack_cache__", TURBOPACK_CACHE),
    ("__turbopack_modules__", TURBOPACK_MODULES),
    ("__turbopack_load__", TURBOPACK_LOAD),
    ("__turbopack_dynamic__", TURBOPACK_DYNAMIC),
    (
        "__turbopack_resolve_absolute_path__",
        TURBOPACK_RESOLVE_ABSOLUTE_PATH,
    ),
    ("__turbopack_relative_url__", TURBOPACK_RELATIVE_URL),
    (
        "__turbopack_resolve_module_id_path__",
        TURBOPACK_RESOLVE_MODULE_ID_PATH,
    ),
    ("__turbopack_worker_blob_url__", TURBOPACK_WORKER_BLOB_URL),
    ("__turbopack_async_module__", TURBOPACK_ASYNC_MODULE),
    ("__turbopack_external_require__", TURBOPACK_EXTERNAL_REQUIRE),
    ("__turbopack_external_import__", TURBOPACK_EXTERNAL_IMPORT),
    ("__turbopack_refresh__", TURBOPACK_REFRESH),
    ("__turbopack_require_stub__", TURBOPACK_REQUIRE_STUB),
    ("__turbopack_require_real__", TURBOPACK_REQUIRE_REAL),
];
