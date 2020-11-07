use crate::{
    datalog::{DatalogBuilder, DatalogScope},
    AnalyzerInner, Visit,
};
use rslint_core::rule_prelude::ast::{AstChildren, ImportClause, ImportDecl, ModuleItem};
use rslint_parser::ast::NamedImports;
use types::{ImportClause as DatalogImportClause, NamedImport as DatalogNamedImport};

impl<'ddlog> Visit<'ddlog, ModuleItem> for AnalyzerInner {
    type Output = Option<DatalogScope<'ddlog>>;

    fn visit(&self, scope: &dyn DatalogBuilder<'ddlog>, item: ModuleItem) -> Self::Output {
        match item {
            ModuleItem::ImportDecl(import) => {
                self.visit(scope, import);
                None
            }
            ModuleItem::ExportNamed(_export) => {
                // self.visit(scope, export);
                None
            }
            ModuleItem::ExportDefaultDecl(_export) => {
                // self.visit(scope, export);
                None
            }
            ModuleItem::ExportDefaultExpr(_export) => {
                // self.visit(scope, export);
                None
            }
            ModuleItem::ExportWildcard(_export) => {
                // self.visit(scope, export);
                None
            }
            ModuleItem::ExportDecl(_export) => {
                // self.visit(scope, export);
                None
            }
            ModuleItem::Stmt(stmt) => self.visit(scope, stmt).1,
        }
    }
}

impl<'ddlog> Visit<'ddlog, AstChildren<ModuleItem>> for AnalyzerInner {
    type Output = DatalogScope<'ddlog>;

    fn visit(
        &self,
        scope: &dyn DatalogBuilder<'ddlog>,
        items: AstChildren<ModuleItem>,
    ) -> Self::Output {
        let mut scope = scope.scope();
        for item in items {
            if let Some(new_scope) = self.visit(&scope, item) {
                scope = new_scope;
            }
        }

        scope
    }
}

impl<'ddlog> Visit<'ddlog, ImportDecl> for AnalyzerInner {
    type Output = ();

    fn visit(&self, scope: &dyn DatalogBuilder<'ddlog>, import: ImportDecl) -> Self::Output {
        let clauses = self.visit(scope, import.imports());
        scope.import_decl(clauses);
    }
}

impl<'ddlog> Visit<'ddlog, ImportClause> for AnalyzerInner {
    type Output = DatalogImportClause;

    fn visit(&self, scope: &dyn DatalogBuilder<'ddlog>, clause: ImportClause) -> Self::Output {
        match clause {
            ImportClause::WildcardImport(wildcard) => DatalogImportClause::WildcardImport {
                alias: self.visit(scope, wildcard.alias()).into(),
            },
            ImportClause::NamedImports(named) => DatalogImportClause::GroupedImport {
                imports: self.visit(scope, named).into(),
            },
            ImportClause::Name(name) => DatalogImportClause::SingleImport {
                name: self.visit(scope, name),
            },
        }
    }
}

impl<'ddlog> Visit<'ddlog, AstChildren<ImportClause>> for AnalyzerInner {
    type Output = Vec<DatalogImportClause>;

    fn visit(
        &self,
        scope: &dyn DatalogBuilder<'ddlog>,
        imports: AstChildren<ImportClause>,
    ) -> Self::Output {
        imports.map(|import| self.visit(scope, import)).collect()
    }
}

impl<'ddlog> Visit<'ddlog, NamedImports> for AnalyzerInner {
    type Output = Vec<DatalogNamedImport>;

    fn visit(&self, scope: &dyn DatalogBuilder<'ddlog>, imports: NamedImports) -> Self::Output {
        imports
            .specifiers()
            .map(|spec| DatalogNamedImport {
                name: self.visit(scope, spec.name()).into(),
                alias: self.visit(scope, spec.alias()).into(),
            })
            .collect()
    }
}