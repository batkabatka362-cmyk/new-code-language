// =============================================================================
// hierarchical_tbt_attractor.v
// 3-Tier Hierarchical Geometric Rotor Memory (HTGM) Hardware Accelerator
// Part of DPSTA & Topological Brain Transformer (TBT)
//
// Tier 1: Char-Level Wire-Shift N-gram (rho^0, rho^1, rho^2)
// Tier 2: Word-Level Clifford Rotor Accumulator (4 Resonant Channels)
// Tier 3: Macro-Scale 16-Slot Attractor Ring Buffer with Tropical Phase Resonance
// =============================================================================

`default_nettype none
`timescale 1ns / 1ps

module hierarchical_tbt_attractor #(
    parameter D_BITS          = 512,
    parameter RING_DEPTH      = 16,     // 16 Sentence Attractor Slots
    parameter NUM_STATES      = 94,     // ASCII 33 to 126
    parameter CHANNEL_SHIFT_0 = 17,     // Channel 0 (Scalar Grade)
    parameter CHANNEL_SHIFT_1 = 34,     // Channel 1 (Vector Grade)
    parameter CHANNEL_SHIFT_2 = 68,     // Channel 2 (Bivector Grade)
    parameter CHANNEL_SHIFT_3 = 136     // Channel 3 (Trivector Grade)
)(
    input  wire                 clk,
    input  wire                 rst_n,

    // Stream Input
    input  wire                 char_valid,
    input  wire [6:0]           char_id,        // 0 to 93
    input  wire [D_BITS-1:0]    char_hypervec,  // 512-bit anchor vector from ROM

    // Boundary Signals (can also be decoded internally)
    input  wire                 word_boundary,      // High on space, punctuation
    input  wire                 sentence_boundary,  // High on '.', '!', '?', ';'

    // Telemetry & Attractor Output
    output reg  [D_BITS-1:0]    current_attractor_out,
    output reg  [3:0]           best_slot_index,
    output reg  [9:0]           best_resonance_score, // 0 to 512
    output reg  [6:0]           attractor_target_char,
    output reg  [15:0]          tropical_bias_boost,  // Score injected to DPSTA Core
    output reg                  attractor_valid
);

    // =========================================================================
    // 1. TIER 1: MICRO-SCALE CHAR ROLLING REGISTER
    // =========================================================================
    reg [D_BITS-1:0] char_hist_0; // e(t-2)
    reg [D_BITS-1:0] char_hist_1; // e(t-1)
    reg [D_BITS-1:0] char_hist_2; // e(t)

    // Combinational Wire Rotations (0 logic gate delay)
    wire [D_BITS-1:0] rot_0 = char_hist_0;
    wire [D_BITS-1:0] rot_1 = (char_hist_1 << 19) | (char_hist_1 >> (D_BITS - 19));
    wire [D_BITS-1:0] rot_2 = (char_hist_2 << 38) | (char_hist_2 >> (D_BITS - 38));

    wire [D_BITS-1:0] local_ngram_vec = rot_2 ^ rot_1 ^ rot_0;

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            char_hist_0 <= {D_BITS{1'b0}};
            char_hist_1 <= {D_BITS{1'b0}};
            char_hist_2 <= {D_BITS{1'b0}};
        end else if (char_valid) begin
            char_hist_0 <= char_hist_1;
            char_hist_1 <= char_hist_2;
            char_hist_2 <= char_hypervec;
        end
    end

    // =========================================================================
    // 2. TIER 2: MESO-SCALE WORD & SENTENCE CLIFFORD ROTOR ACCUMULATOR
    // =========================================================================
    reg [D_BITS-1:0] word_accumulator;
    reg [D_BITS-1:0] sentence_accumulator;
    reg [1:0]        channel_select;

    // Resonant Channel Shifter for Word-to-Sentence binding
    reg [D_BITS-1:0] shifted_word;
    always @(*) begin
        case (channel_select)
            2'b00: shifted_word = (word_accumulator << CHANNEL_SHIFT_0) | (word_accumulator >> (D_BITS - CHANNEL_SHIFT_0));
            2'b01: shifted_word = (word_accumulator << CHANNEL_SHIFT_1) | (word_accumulator >> (D_BITS - CHANNEL_SHIFT_1));
            2'b10: shifted_word = (word_accumulator << CHANNEL_SHIFT_2) | (word_accumulator >> (D_BITS - CHANNEL_SHIFT_2));
            2'b11: shifted_word = (word_accumulator << CHANNEL_SHIFT_3) | (word_accumulator >> (D_BITS - CHANNEL_SHIFT_3));
        endcase
    end

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            word_accumulator     <= {D_BITS{1'b0}};
            sentence_accumulator <= {D_BITS{1'b0}};
            channel_select       <= 2'b00;
        end else if (char_valid) begin
            if (word_boundary) begin
                // Bind completed word into sentence accumulator
                sentence_accumulator <= sentence_accumulator ^ shifted_word;
                channel_select       <= channel_select + 1'b1;
                word_accumulator     <= {D_BITS{1'b0}};
            end else begin
                // Accumulate char into word
                word_accumulator     <= word_accumulator ^ char_hypervec;
            end

            if (sentence_boundary) begin
                sentence_accumulator <= {D_BITS{1'b0}};
                channel_select       <= 2'b00;
            end
        end
    end

    // =========================================================================
    // 3. TIER 3: MACRO-SCALE 16-SLOT ATTRACTOR RING BUFFER
    // =========================================================================
    reg [D_BITS-1:0] ring_mem [0:RING_DEPTH-1];
    reg [3:0]        ring_head;
    reg [4:0]        ring_occupancy;
    integer          i;

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            ring_head      <= 4'd0;
            ring_occupancy <= 5'd0;
            for (i = 0; i < RING_DEPTH; i = i + 1) begin
                ring_mem[i] <= {D_BITS{1'b0}};
            end
        end else if (char_valid && sentence_boundary) begin
            // Store finished sentence rotor into ring buffer
            ring_mem[ring_head] <= sentence_accumulator;
            ring_head           <= (ring_head == RING_DEPTH - 1) ? 4'd0 : ring_head + 1'b1;
            if (ring_occupancy < RING_DEPTH) begin
                ring_occupancy <= ring_occupancy + 1'b1;
            end
        end
    end

    // =========================================================================
    // 4. PARALLEL TROPICAL PHASE RESONANCE ENGINE (MAX-PLUS POPCOUNT)
    // =========================================================================
    // Resonate current rolling n-gram against all valid slots in the ring
    wire [D_BITS-1:0] diff_mask [0:RING_DEPTH-1];
    reg  [9:0]        sim_scores [0:RING_DEPTH-1];

    genvar g;
    generate
        for (g = 0; g < RING_DEPTH; g = g + 1) begin : GEN_SIM
            assign diff_mask[g] = local_ngram_vec ^ ring_mem[g];
        end
    endgenerate

    // Bit-count function
    function [9:0] popcount512;
        input [D_BITS-1:0] vec;
        integer b;
        begin
            popcount512 = 10'd0;
            for (b = 0; b < D_BITS; b = b + 1) begin
                popcount512 = popcount512 + vec[b];
            end
        end
    endfunction

    // Pipeline Stage: Calculate Hamming Similarities & Top-1 Arbiter
    reg [3:0] best_idx_comb;
    reg [9:0] max_score_comb;
    integer   s;

    always @(*) begin
        best_idx_comb  = 4'd0;
        max_score_comb = 10'd0;
        for (s = 0; s < RING_DEPTH; s = s + 1) begin
            if (s < ring_occupancy) begin
                // Similarity = 512 - HammingDistance
                sim_scores[s] = 10'd512 - popcount512(diff_mask[s]);
                if (sim_scores[s] > max_score_comb) begin
                    max_score_comb = sim_scores[s];
                    best_idx_comb  = s[3:0];
                end
            end else begin
                sim_scores[s] = 10'd0;
            end
        end
    end

    // Output Registers (Synchronous Latency: 1 такт)
    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            best_slot_index       <= 4'd0;
            best_resonance_score  <= 10'd0;
            current_attractor_out <= {D_BITS{1'b0}};
            tropical_bias_boost   <= 16'd0;
            attractor_valid       <= 1'b0;
            attractor_target_char <= 7'd0;
        end else if (char_valid) begin
            best_slot_index       <= best_idx_comb;
            best_resonance_score  <= max_score_comb;
            current_attractor_out <= ring_mem[best_idx_comb];

            // If resonance exceeds threshold (> 272 / 512, i.e., > 53% correlation)
            if (max_score_comb > 10'd272 && ring_occupancy > 0) begin
                attractor_valid     <= 1'b1;
                // Tropical boost proportional to resonance
                tropical_bias_boost <= (max_score_comb - 10'd256) << 4; // 16x scale
                attractor_target_char <= char_id; // Predicted carrier
            end else begin
                attractor_valid     <= 1'b0;
                tropical_bias_boost <= 16'd0;
                attractor_target_char <= 7'd0;
            end
        end
    end

endmodule
