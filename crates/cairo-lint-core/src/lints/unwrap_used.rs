use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::ExprFunctionCall;

pub const UNWRAP_USED: &str = "Use of unwrap() detected. Consider using '?' or 'expect()' instead.";

pub fn check_unwrap_used(
    db: &dyn SemanticGroup,
    expr_function_call: &ExprFunctionCall,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_id = expr_function_call.function;
    let function_name = function_id.name(db);

    // Check if the function used is `unwrap`
    if function_name == "unwrap" {
        // Retrieve diagnostic information
        let stable_ptr = expr_function_call.stable_ptr;
        diagnostics.push(PluginDiagnostic {
            stable_ptr: stable_ptr.into(),
            message: "Use of unwrap() detected. Consider using '?' or 'expect()' instead.".to_string(),
            severity: Severity::Warning,
        });
    }
}
