// =============================================================================
// clifford_rotor_core.v
// Clifford Rotor Machine Code (CL v1.0) 4-Slot VLIW Processing Core
//
// Features:
//   - 40-Byte (320-bit) Base-94 ASCII Instruction Input
//   - 4 Concurrent Execution Slots: Scalar, Vector, Bivector, Trivector/HDC
//   - 16 x 512-bit Multivector Register File
//   - 1-Cycle Zero-Bubble Execution Pipeline
// =============================================================================

`default_nettype none
`timescale 1ns / 1ps

module clifford_rotor_core #(
    parameter D_BITS       = 512,
    parameter NUM_REGS     = 16,
    parameter BASE_CHAR    = 8'd33  // ASCII '!'
)(
    input  wire                 clk,
    input  wire                 rst_n,

    // Instruction Interface (40 ASCII Bytes = 320 bits)
    input  wire                 bundle_valid,
    input  wire [319:0]         bundle_ascii, // [319:240]=Slot0, [239:160]=Slot1, [159:80]=Slot2, [79:0]=Slot3

    // External Interface / Telemetry
    output reg  [6:0]           emitted_char,
    output reg                  emitted_valid,
    output reg  [15:0]          cycle_counter,
    output wire [3:0]           slot_active_status
);

    // =========================================================================
    // 1. BASE-94 TO 64-BIT HARDWARE DECODER FUNCTION
    // =========================================================================
    // Takes 10 ASCII characters (80 bits) and decodes to a 64-bit micro-op
    function [63:0] decode_slot;
        input [79:0] raw_chars; // 10 chars * 8 bits
        reg [63:0] accum;
        integer k;
        reg [6:0] digit;
        begin
            accum = 64'd0;
            for (k = 9; k >= 0; k = k - 1) begin
                digit = raw_chars[k*8 +: 8] - BASE_CHAR;
                accum = (accum * 64'd94) + {57'd0, digit};
            end
            decode_slot = accum;
        end
    endfunction

    // Decode 4 slots combinationally
    wire [63:0] slot0_op = decode_slot(bundle_ascii[319:240]);
    wire [63:0] slot1_op = decode_slot(bundle_ascii[239:160]);
    wire [63:0] slot2_op = decode_slot(bundle_ascii[159:80]);
    wire [63:0] slot3_op = decode_slot(bundle_ascii[79:0]);

    // Field extraction for each slot
    wire [5:0]  s0_opcode = slot0_op[63:58];
    wire [3:0]  s0_pred   = slot0_op[57:54];
    wire [3:0]  s0_rd     = slot0_op[53:50];
    wire [3:0]  s0_rs1    = slot0_op[49:46];
    wire [3:0]  s0_rs2    = slot0_op[45:42];
    wire [17:0] s0_imm    = slot0_op[17:0];

    wire [5:0]  s1_opcode = slot1_op[63:58];
    wire [3:0]  s1_rd     = slot1_op[53:50];
    wire [3:0]  s1_rs1    = slot1_op[49:46];
    wire [3:0]  s1_rs2    = slot1_op[45:42];
    wire [15:0] s1_theta  = slot1_op[33:18];

    wire [5:0]  s2_opcode = slot2_op[63:58];
    wire [3:0]  s2_rd     = slot2_op[53:50];
    wire [3:0]  s2_rs1    = slot2_op[49:46];
    wire [3:0]  s2_rs2    = slot2_op[45:42];

    wire [5:0]  s3_opcode = slot3_op[63:58];
    wire [3:0]  s3_rd     = slot3_op[53:50];
    wire [3:0]  s3_rs1    = slot3_op[49:46];
    wire [3:0]  s3_rs2    = slot3_op[45:42];
    wire [17:0] s3_imm    = slot3_op[17:0];

    assign slot_active_status = { (s3_opcode != 6'd0), (s2_opcode != 6'd0),
                                  (s1_opcode != 6'd0), (s0_opcode != 6'd0) };

    // =========================================================================
    // 2. 16 x 512-BIT MULTIVECTOR REGISTER FILE
    // =========================================================================
    reg [D_BITS-1:0] reg_file [0:NUM_REGS-1];
    integer r;

    // Read Data buses
    wire [D_BITS-1:0] s0_r1_data = reg_file[s0_rs1];
    wire [D_BITS-1:0] s0_r2_data = reg_file[s0_rs2];
    wire [D_BITS-1:0] s1_r1_data = reg_file[s1_rs1];
    wire [D_BITS-1:0] s2_r1_data = reg_file[s2_rs1];
    wire [D_BITS-1:0] s2_r2_data = reg_file[s2_rs2];
    wire [D_BITS-1:0] s3_r1_data = reg_file[s3_rs1];
    wire [D_BITS-1:0] s3_r2_data = reg_file[s3_rs2];

    // =========================================================================
    // 3. 4-WAY PARALLEL GEOMETRIC ALUs
    // =========================================================================
    // ALU 0: Scalar / Linear Ops (CLIFFORD_ADD, CLIFFORD_SUB)
    reg [D_BITS-1:0] alu0_result;
    always @(*) begin
        case (s0_opcode)
            6'h10:   alu0_result = s0_r1_data + s0_r2_data; // CLIFFORD_ADD
            6'h11:   alu0_result = s0_r1_data - s0_r2_data; // CLIFFORD_SUB
            default: alu0_result = s0_r1_data;
        endcase
    end

    // ALU 1: Vector / Rotor Sandwich (Wire Shift + LUT Sine/Cosine)
    reg [D_BITS-1:0] alu1_result;
    always @(*) begin
        case (s1_opcode)
            6'h14: begin // ROT_SANDWICH
                // Wire planar shift by theta[5:0]
                alu1_result = (s1_r1_data << s1_theta[5:0]) | (s1_r1_data >> (D_BITS - s1_theta[5:0]));
            end
            default: alu1_result = s1_r1_data;
        endcase
    end

    // ALU 2: Bivector / Exterior & Tropical Ops (WEDGE_PROD, TROPIC_MAX)
    reg [D_BITS-1:0] alu2_result;
    always @(*) begin
        case (s2_opcode)
            6'h12:   alu2_result = s2_r1_data ^ s2_r2_data; // Simplified Exterior Wedge
            6'h30:   alu2_result = (s2_r1_data > s2_r2_data) ? s2_r1_data : s2_r2_data; // TROPIC_MAX
            default: alu2_result = s2_r1_data;
        endcase
    end

    // ALU 3: Trivector / HDC Ops (HDC_BIND, EMIT_CHAR)
    reg [D_BITS-1:0] alu3_result;
    always @(*) begin
        case (s3_opcode)
            6'h20:   alu3_result = s3_r1_data ^ s3_r2_data; // HDC_BIND (XOR)
            default: alu3_result = s3_r1_data;
        endcase
    end

    // =========================================================================
    // 4. 1-CYCLE REGISTER COMMIT & CONTROL
    // =========================================================================
    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            cycle_counter <= 16'd0;
            emitted_char  <= 7'd0;
            emitted_valid <= 1'b0;
            for (r = 0; r < NUM_REGS; r = r + 1) begin
                reg_file[r] <= {D_BITS{1'b0}};
            end
        end else if (bundle_valid) begin
            cycle_counter <= cycle_counter + 1'b1;

            // Commit Slot 0
            if (s0_opcode == 6'h01) begin // EMIT_CHAR
                emitted_char  <= s0_imm[6:0];
                emitted_valid <= 1'b1;
            end else if (s0_opcode != 6'd0 && s0_rd != 4'd0) begin
                reg_file[s0_rd] <= alu0_result;
            end

            // Commit Slot 1
            if (s1_opcode != 6'd0 && s1_rd != 4'd0) begin
                reg_file[s1_rd] <= alu1_result;
            end

            // Commit Slot 2
            if (s2_opcode != 6'd0 && s2_rd != 4'd0) begin
                reg_file[s2_rd] <= alu2_result;
            end

            // Commit Slot 3
            if (s3_opcode == 6'h01) begin // EMIT_CHAR in Slot 3
                emitted_char  <= s3_imm[6:0];
                emitted_valid <= 1'b1;
            end else if (s3_opcode != 6'd0 && s3_rd != 4'd0) begin
                reg_file[s3_rd] <= alu3_result;
            end
        end else begin
            emitted_valid <= 1'b0;
        end
    end

endmodule
