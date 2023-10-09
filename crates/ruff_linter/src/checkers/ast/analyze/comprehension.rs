use ruff_python_ast::Comprehension;
use ruff_python_ast::Stmt;

use crate::checkers::ast::Checker;
use crate::codes::Rule;
use crate::rules::{flake8_bugbear, flake8_simplify};

/// Run lint rules over a [`Comprehension`] syntax nodes.
pub(crate) fn comprehension(comprehension: &Comprehension, checker: &mut Checker) {
    if checker.enabled(Rule::InDictKeys) {
        flake8_simplify::rules::key_in_dict_comprehension(checker, comprehension);
    }

    if checker.enabled(Rule::UnusedLoopControlVariable) {
        let Stmt::Expr(stmt_expr) = checker.semantic.current_statement() else {
            return;
        };

        flake8_bugbear::rules::unused_loop_control_variable_comprehension(
            checker,
            stmt_expr,
            comprehension,
        );
    }
}
