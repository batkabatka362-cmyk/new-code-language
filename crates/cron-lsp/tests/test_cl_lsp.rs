// ============================================================================
// CRON Language Server Protocol (LSP 3.17) Integration Test Suite
// Testing: .cl Machine Code & .cr Blueprint Language Server Features
// ============================================================================

use cron_lsp::LspServer;
use serde_json::json;

#[test]
fn test_integration_lsp_cl_full_lifecycle() {
    let mut server = LspServer::new();

    // 1. Initialize Handshake
    let init_req = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "clientInfo": { "name": "Antigravity IDE", "version": "2.0" }
        }
    });
    let resp_str = server.handle_message(&init_req.to_string()).expect("Init response");
    let resp: serde_json::Value = serde_json::from_str(&resp_str).unwrap();
    assert_eq!(resp["result"]["capabilities"]["textDocumentSync"], 1);
    assert_eq!(resp["result"]["capabilities"]["hoverProvider"], true);
    assert_eq!(resp["result"]["capabilities"]["codeActionProvider"], true);
    assert_eq!(resp["result"]["capabilities"]["documentFormattingProvider"], true);

    // 2. Open Valid .cl Document
    let valid_cl = "\
@CORE(0,0,0,0)
B0000: ADD R0, R1, R2 | NOP | LOAD [R3+0] | ROUTE_DOR +X
B0001: DOT_I2 R4, R5, R6 | NOP | SWIZZLE GF2_4 | NOP
";
    let open_req = json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": "file:///kernel.cl",
                "languageId": "cron-cl",
                "version": 1,
                "text": valid_cl
            }
        }
    });
    let diag_notify_str = server.handle_message(&open_req.to_string()).expect("Diag notification");
    let diag_notify: serde_json::Value = serde_json::from_str(&diag_notify_str).unwrap();
    let diags = diag_notify["params"]["diagnostics"].as_array().unwrap();
    assert!(diags.is_empty(), "Valid .cl program should have 0 diagnostics, found: {:?}", diags);

    // 3. Hover over DOT_I2
    let hover_req = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "textDocument/hover",
        "params": {
            "textDocument": { "uri": "file:///kernel.cl" },
            "position": { "line": 2, "character": 8 }
        }
    });
    let hover_resp_str = server.handle_message(&hover_req.to_string()).expect("Hover response");
    let hover_resp: serde_json::Value = serde_json::from_str(&hover_resp_str).unwrap();
    let hover_md = hover_resp["result"]["contents"]["value"].as_str().unwrap();
    assert!(hover_md.contains("BitNet 1.58b Ternary Dot Product"));
    assert!(hover_md.contains("0.08 pJ"));

    // 4. Hover over SWIZZLE
    let hover_swizzle = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "textDocument/hover",
        "params": {
            "textDocument": { "uri": "file:///kernel.cl" },
            "position": { "line": 2, "character": 35 }
        }
    });
    let sw_resp_str = server.handle_message(&hover_swizzle.to_string()).expect("Hover response");
    let sw_resp: serde_json::Value = serde_json::from_str(&sw_resp_str).unwrap();
    let sw_md = sw_resp["result"]["contents"]["value"].as_str().unwrap();
    assert!(sw_md.contains("SWIZZLE"));
    assert!(sw_md.contains("16 banks") || sw_md.contains("Galois Field"));

    // 5. Autocompletions in MEM slot
    let comp_req = json!({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "textDocument/completion",
        "params": {
            "textDocument": { "uri": "file:///kernel.cl" },
            "position": { "line": 1, "character": 32 }
        }
    });
    let comp_resp_str = server.handle_message(&comp_req.to_string()).expect("Comp response");
    let comp_resp: serde_json::Value = serde_json::from_str(&comp_resp_str).unwrap();
    let items = comp_resp["result"]["items"].as_array().unwrap();
    assert!(items.iter().any(|i| i["label"] == "LOAD"));
    assert!(items.iter().any(|i| i["label"] == "SWIZZLE"));

    // 6. Change document to introduce error (Invalid register R25, port mismatch)
    let bad_cl = "\
@CORE(0,0,0,0)
B0000: LOAD [R1] | NOP | NOP | NOP
B0001: ADD R25, R1, R2 | NOP | LOAD [R1] | ROUTE_DOR +X
";
    let change_req = json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didChange",
        "params": {
            "textDocument": { "uri": "file:///kernel.cl", "version": 2 },
            "contentChanges": [{ "text": bad_cl }]
        }
    });
    let change_notify_str = server.handle_message(&change_req.to_string()).expect("Change notification");
    let change_notify: serde_json::Value = serde_json::from_str(&change_notify_str).unwrap();
    let bad_diags = change_notify["params"]["diagnostics"].as_array().unwrap();
    assert!(bad_diags.iter().any(|d| d["code"] == "CL003" && d["message"].as_str().unwrap().contains("LOAD")));
    assert!(bad_diags.iter().any(|d| d["code"] == "CL004" && d["message"].as_str().unwrap().contains("R25")));

    // 7. Request Code Action to Heal
    let code_action_req = json!({
        "jsonrpc": "2.0",
        "id": 5,
        "method": "textDocument/codeAction",
        "params": {
            "textDocument": { "uri": "file:///kernel.cl" },
            "range": { "start": { "line": 0, "character": 0 }, "end": { "line": 3, "character": 0 } },
            "context": { "diagnostics": bad_diags }
        }
    });
    let ca_resp_str = server.handle_message(&code_action_req.to_string()).expect("CodeAction response");
    let ca_resp: serde_json::Value = serde_json::from_str(&ca_resp_str).unwrap();
    let actions = ca_resp["result"].as_array().unwrap();
    assert!(actions.iter().any(|a| a["title"].as_str().unwrap().contains("Heal .cl Microcode")));

    // 8. Document Formatting
    let format_req = json!({
        "jsonrpc": "2.0",
        "id": 6,
        "method": "textDocument/formatting",
        "params": {
            "textDocument": { "uri": "file:///kernel.cl" },
            "options": { "tabSize": 4, "insertSpaces": true }
        }
    });
    let fmt_resp_str = server.handle_message(&format_req.to_string()).expect("Format response");
    let fmt_resp: serde_json::Value = serde_json::from_str(&fmt_resp_str).unwrap();
    let edits = fmt_resp["result"].as_array().unwrap();
    assert!(!edits.is_empty());
}
