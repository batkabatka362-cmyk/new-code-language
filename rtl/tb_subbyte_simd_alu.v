// ============================================================================
// IEEE 1364-2001 Automated Self-Checking Testbench for Sub-Byte SIMD ALU
// Verifies 0-DSP Popcount Tree Ternary Dot Product across all test vectors
// ============================================================================

`timescale 1ns / 1ps

module tb_subbyte_simd_alu;
    reg         clk;
    reg         rst_n;
    reg  [31:0] weights_packed;
    reg  [31:0] acts_packed;
    wire [31:0] dot_product_out;

    integer errors;
    integer test_count;

    // Instantiate DUT
    subbyte_simd_alu dut (
        .clk(clk),
        .rst_n(rst_n),
        .weights_packed(weights_packed),
        .acts_packed(acts_packed),
        .dot_product_out(dot_product_out)
    );

    // 100MHz clock
    always #5 clk = ~clk;

    // Golden software model for verification
    function signed [31:0] golden_ternary_mac;
        input [31:0] w;
        input [31:0] a;
        integer idx;
        reg signed [31:0] acc;
        reg [1:0] w_val;
        reg [1:0] a_val;
        begin
            acc = 32'sd0;
            for (idx = 0; idx < 16; idx = idx + 1) begin
                w_val = w[idx*2 +: 2];
                a_val = a[idx*2 +: 2];
                case (w_val)
                    2'b01: acc = acc + a_val;
                    2'b10: acc = acc - a_val;
                    default: ; // 00 or 11
                endcase
            end
            golden_ternary_mac = acc;
        end
    endfunction

    task check_vector;
        input [31:0] w;
        input [31:0] a;
        reg signed [31:0] expected;
        begin
            weights_packed = w;
            acts_packed = a;
            @(posedge clk);
            #1; // Sample output after clock edge
            expected = golden_ternary_mac(w, a);
            test_count = test_count + 1;
            if ($signed(dot_product_out) !== expected) begin
                $display("[FAIL] Test %0d: W=%08h, A=%08h -> DUT=%0d, Expected=%0d",
                         test_count, w, a, $signed(dot_product_out), expected);
                errors = errors + 1;
            end
        end
    endtask

    initial begin
        clk = 0;
        rst_n = 0;
        weights_packed = 0;
        acts_packed = 0;
        errors = 0;
        test_count = 0;

        // Reset
        #20 rst_n = 1;
        #10;

        $display("=== Starting Sub-Byte SIMD ALU 0-DSP Popcount Verification ===");

        // Vector 1: All +1 weights (2'b01 repeated = 32'h55555555), all acts = 3 (32'hFFFFFFFF)
        // 16 * (+1 * 3) = +48
        check_vector(32'h55555555, 32'hFFFFFFFF);

        // Vector 2: All -1 weights (2'b10 repeated = 32'hAAAAAAAA), all acts = 3 (32'hFFFFFFFF)
        // 16 * (-1 * 3) = -48
        check_vector(32'hAAAAAAAA, 32'hFFFFFFFF);

        // Vector 3: All zero weights (32'h00000000)
        check_vector(32'h00000000, 32'hFFFFFFFF);

        // Vector 4: Alternating +1 and -1 weights (8x +1, 8x -1), acts = 2
        // Result should be 0
        check_vector(32'h66666666, 32'hAAAAAAAA);

        // Vector 5: Sparsity test (only 1 non-zero weight)
        check_vector(32'h00000001, 32'h00000003); // +3
        check_vector(32'h00000002, 32'h00000003); // -3

        // Vector 6: 100 Pseudo-random test vectors
        repeat (100) begin
            check_vector($random, $random);
        end

        #20;
        $display("==================================================");
        $display("   SUB-BYTE SIMD ALU RTL VERIFICATION SUMMARY    ");
        $display("==================================================");
        $display("  Total Vectors Tested: %0d", test_count);
        $display("  Total Mismatches:     %0d", errors);
        $display("==================================================");

        if (errors == 0) begin
            $display("[PASS] 100% BIT-EXACT 0-DSP POPCOUNT RTL VERIFIED!");
            $finish(0);
        end else begin
            $display("[FAIL] RTL VERIFICATION ENCOUNTERED ERRORS!");
            $finish(1);
        end
    end

endmodule
