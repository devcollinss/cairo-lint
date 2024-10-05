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
    
    // Print debug information to understand what function is being called
    println!("Function called: {:?}", function_name);
    
    // Check if the function used is `unwrap`
    if function_name == "unwrap" || function_name.ends_with("::unwrap") {
        // Retrieve the stable pointer for diagnostics
        let stable_ptr = expr_function_call.stable_ptr;

        // Push the diagnostic message to the diagnostics vector
        diagnostics.push(PluginDiagnostic {
            stable_ptr: stable_ptr.into(), // Ensure this correctly converts to the expected type
            message: UNWRAP_USED.to_string(), // Use the constant message for consistency
            severity: Severity::Warning,
        });
    } else {
        // Log a message or handle the case where the function is not `unwrap`
        println!("Function '{}' called, no diagnostic triggered.", function_name);
    }
}
