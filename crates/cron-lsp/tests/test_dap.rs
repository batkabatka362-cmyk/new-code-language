use cron_lsp::dap::DapServer;
use serde_json::json;
use std::fs;

#[test]
fn test_dap_initialize_and_capabilities() {
    let mut server = DapServer::new();
    let init_req = json!({
        "seq": 1,
        "type": "request",
        "command": "initialize",
        "arguments": {
            "clientID": "vscode",
            "clientName": "Visual Studio Code",
            "adapterID": "cron"
        }
    });

    let msgs = server.handle_message(&init_req.to_string());
    assert_eq!(msgs.len(), 2, "Expected response and initialized event");

    let resp: serde_json::Value = serde_json::from_str(&msgs[0]).unwrap();
    assert_eq!(resp["type"], "response");
    assert_eq!(resp["command"], "initialize");
    assert_eq!(resp["success"], true);
    assert_eq!(resp["body"]["supportsConfigurationDoneRequest"], true);
    assert_eq!(resp["body"]["supportsStepBack"], true);

    let event: serde_json::Value = serde_json::from_str(&msgs[1]).unwrap();
    assert_eq!(event["type"], "event");
    assert_eq!(event["event"], "initialized");
}

#[test]
fn test_dap_launch_step_and_variables_lifecycle() {
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_dap_prog.cl");
    let cl_code = r#"
B0001: '=00#0042> _NO00#000> _NO00#000> _NO00#000>
B0002: _PO01$000> '=02#0064> _NO00#000> _NO00#000>
B0003: _HL00#000! _NO00#000> _NO00#000> _NO00#000>
"#;
    fs::write(&test_file, cl_code).expect("Failed to write temporary .cl file");

    let mut server = DapServer::new();

    // 1. Initialize
    let init_req = json!({ "seq": 1, "type": "request", "command": "initialize" });
    server.handle_message(&init_req.to_string());

    // 2. Launch
    let launch_req = json!({
        "seq": 2,
        "type": "request",
        "command": "launch",
        "arguments": {
            "program": test_file.to_str().unwrap(),
            "stopOnEntry": true,
            "coreId": 0
        }
    });
    let launch_msgs = server.handle_message(&launch_req.to_string());
    assert!(launch_msgs.len() >= 2);
    let stop_ev: serde_json::Value = serde_json::from_str(&launch_msgs[1]).unwrap();
    assert_eq!(stop_ev["event"], "stopped");
    assert_eq!(stop_ev["body"]["reason"], "entry");

    // 3. Threads
    let threads_req = json!({ "seq": 3, "type": "request", "command": "threads" });
    let threads_msgs = server.handle_message(&threads_req.to_string());
    let threads_resp: serde_json::Value = serde_json::from_str(&threads_msgs[0]).unwrap();
    assert_eq!(threads_resp["body"]["threads"][0]["id"], 1);

    // 4. Stack Trace
    let st_req = json!({ "seq": 4, "type": "request", "command": "stackTrace" });
    let st_msgs = server.handle_message(&st_req.to_string());
    let st_resp: serde_json::Value = serde_json::from_str(&st_msgs[0]).unwrap();
    assert_eq!(st_resp["body"]["stackFrames"].as_array().unwrap().len(), 1);

    // 5. Scopes
    let scopes_req = json!({ "seq": 5, "type": "request", "command": "scopes" });
    let scopes_msgs = server.handle_message(&scopes_req.to_string());
    let scopes_resp: serde_json::Value = serde_json::from_str(&scopes_msgs[0]).unwrap();
    let scopes = scopes_resp["body"]["scopes"].as_array().unwrap();
    assert_eq!(scopes.len(), 4);

    // 6. Step Over (next) to execute Cycle 1 ('=00#0042>)
    let next_req = json!({ "seq": 6, "type": "request", "command": "next" });
    let next_msgs = server.handle_message(&next_req.to_string());
    let next_stop: serde_json::Value = serde_json::from_str(&next_msgs[1]).unwrap();
    assert_eq!(next_stop["event"], "stopped");
    assert_eq!(next_stop["body"]["reason"], "step");

    // 7. Check Variables (R0 should now be 0x42 = 66)
    let vars_req = json!({
        "seq": 7,
        "type": "request",
        "command": "variables",
        "arguments": { "variablesReference": 1000 }
    });
    let vars_msgs = server.handle_message(&vars_req.to_string());
    let vars_resp: serde_json::Value = serde_json::from_str(&vars_msgs[0]).unwrap();
    let vars = vars_resp["body"]["variables"].as_array().unwrap();
    assert!(vars.iter().any(|v| v["name"].as_str().unwrap().starts_with("R0") && v["value"].as_str().unwrap().contains("0x00000042")));

    // 8. Set Variable (modify R0 live)
    let set_var_req = json!({
        "seq": 8,
        "type": "request",
        "command": "setVariable",
        "arguments": {
            "variablesReference": 1000,
            "name": "R0",
            "value": "0x1337"
        }
    });
    let set_msgs = server.handle_message(&set_var_req.to_string());
    let set_resp: serde_json::Value = serde_json::from_str(&set_msgs[0]).unwrap();
    assert_eq!(set_resp["success"], true);

    // 9. Step Back (reversible step back to initial state)
    let back_req = json!({ "seq": 9, "type": "request", "command": "stepBack" });
    let back_msgs = server.handle_message(&back_req.to_string());
    let back_resp: serde_json::Value = serde_json::from_str(&back_msgs[0]).unwrap();
    assert_eq!(back_resp["success"], true);

    // Clean up
    let _ = fs::remove_file(&test_file);
}

#[test]
fn test_dap_breakpoint_and_continue() {
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_dap_bp.cl");
    let cl_code = r#"
B0001: '=00#0010> _NO00#000> _NO00#000> _NO00#000>
B0002: '=01#0020> _NO00#000> _NO00#000> _NO00#000>
B0003: '=02#0030> _NO00#000> _NO00#000> _NO00#000>
B0004: _HL00#000! _NO00#000> _NO00#000> _NO00#000>
"#;
    fs::write(&test_file, cl_code).expect("Failed to write temporary .cl file");

    let mut server = DapServer::new();

    // 1. Initialize
    let init_req = json!({ "seq": 1, "type": "request", "command": "initialize" });
    server.handle_message(&init_req.to_string());

    // 2. Launch
    let launch_req = json!({
        "seq": 2,
        "type": "request",
        "command": "launch",
        "arguments": {
            "program": test_file.to_str().unwrap(),
            "stopOnEntry": true
        }
    });
    server.handle_message(&launch_req.to_string());

    // 3. Set breakpoint at cycle 3
    let bp_req = json!({
        "seq": 3,
        "type": "request",
        "command": "setBreakpoints",
        "arguments": {
            "breakpoints": [
                { "line": 3 }
            ]
        }
    });
    let bp_msgs = server.handle_message(&bp_req.to_string());
    let bp_resp: serde_json::Value = serde_json::from_str(&bp_msgs[0]).unwrap();
    assert_eq!(bp_resp["body"]["breakpoints"][0]["verified"], true);

    // 4. Continue until breakpoint
    let cont_req = json!({ "seq": 4, "type": "request", "command": "continue" });
    let cont_msgs = server.handle_message(&cont_req.to_string());
    assert_eq!(cont_msgs.len(), 2);
    let cont_stop: serde_json::Value = serde_json::from_str(&cont_msgs[1]).unwrap();
    assert_eq!(cont_stop["event"], "stopped");
    assert_eq!(cont_stop["body"]["reason"], "breakpoint");

    // Clean up
    let _ = fs::remove_file(&test_file);
}
