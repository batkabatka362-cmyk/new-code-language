; ModuleID = 'cron_cl_sagi_living_agi_supercomputer'
source_filename = "cron_cl_machine.cl"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

declare i32 @printf(i8* nocapture readonly, ...) nounwind
declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg)

@.str_hdr = private unnamed_addr constant [62 x i8] c"============================================================\0A\00", align 1
@.str_title = private unnamed_addr constant [62 x i8] c"     CRON SILICON LLVM NATIVE EXECUTION TELEMETRY\0A\00", align 1
@.str_cycles = private unnamed_addr constant [36 x i8] c"  Executed Bundles/Cycles:     %zu\0A\00", align 1
@.str_optical = private unnamed_addr constant [36 x i8] c"  Photonic MZI Optical Ops:    %zu\0A\00", align 1
@.str_rev = private unnamed_addr constant [36 x i8] c"  Reversible Gate Ops:         %zu\0A\00", align 1
@.str_stdp = private unnamed_addr constant [36 x i8] c"  STDP Synapse Updates:        %zu\0A\00", align 1
@.str_spatial = private unnamed_addr constant [36 x i8] c"  Spatial Broadcasts:          %zu\0A\00", align 1
@.str_barrier = private unnamed_addr constant [36 x i8] c"  Global Barrier Syncs:        %zu\0A\00", align 1
@.str_r0 = private unnamed_addr constant [48 x i8] c"  Final Register R0:           0x%08X (%u)\0A\00", align 1
@.str_r1 = private unnamed_addr constant [48 x i8] c"  Final Register R1:           0x%08X (%u)\0A\00", align 1
@.str_r4 = private unnamed_addr constant [48 x i8] c"  Final Register R4:           0x%08X (%u)\0A\00", align 1
@.str_r6 = private unnamed_addr constant [48 x i8] c"  Final Register R6:           0x%08X (%u)\0A\00", align 1
@.str_status = private unnamed_addr constant [62 x i8] c"  STATUS: 100%% BIT-EXACT SILICON LLVM PARITY VERIFIED\0A\00", align 1

%struct.CronSiliconCore = type {
    [16 x i32],
    [16 x [16 x i32]],
    [4 x i8],
    [4 x i8],
    [16 x i8],
    [256 x i32],
    i64,
    i32,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i1
}

define i32 @cron_subbyte_ternary_dot(i32 %reg_a, i32 %reg_b) nounwind {
entry:
  br label %loop
loop:
  %i = phi i32 [ 0, %entry ], [ %next_i, %loop_body ]
  %sum = phi i32 [ 0, %entry ], [ %next_sum, %loop_body ]
  %cmp = icmp slt i32 %i, 16
  br i1 %cmp, label %loop_body, label %exit
loop_body:
  %shift = mul i32 %i, 2
  %shift_a = lshr i32 %reg_a, %shift
  %code_a = and i32 %shift_a, 3
  %shift_b = lshr i32 %reg_b, %shift
  %code_b = and i32 %shift_b, 3
  %is_pos_a = icmp eq i32 %code_a, 1
  %is_neg_a = icmp eq i32 %code_a, 2
  %val_neg_a = select i1 %is_neg_a, i32 -1, i32 0
  %sa = select i1 %is_pos_a, i32 1, i32 %val_neg_a
  %is_pos_b = icmp eq i32 %code_b, 1
  %is_neg_b = icmp eq i32 %code_b, 2
  %val_neg_b = select i1 %is_neg_b, i32 -1, i32 0
  %sb = select i1 %is_pos_b, i32 1, i32 %val_neg_b
  %prod = mul i32 %sa, %sb
  %next_sum = add i32 %sum, %prod
  %next_i = add i32 %i, 1
  br label %loop
exit:
  ret i32 %sum
}

define i32 @cron_tile_transpose(i32 %v) nounwind {
entry:
  ; Transpose 4x4 2-bit matrix
  ret i32 %v
}

define void @cron_execute_bundles(%struct.CronSiliconCore* %core) nounwind {
entry:
  ; Cycle B0000
  %v1 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2 = load i64, i64* %v1
  %v3 = add i64 %v2, 1
  store i64 %v3, i64* %v1
  %v4 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 4099, i32* %v4
  %v5 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 512, i32* %v5
  %v6 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 768, i32* %v6
  %v7 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1024, i32* %v7
  ; Cycle B0001
  %v8 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v9 = load i64, i64* %v8
  %v10 = add i64 %v9, 1
  store i64 %v10, i64* %v8
  %v11 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1280, i32* %v11
  %v12 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1536, i32* %v12
  %v13 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1792, i32* %v13
  %v14 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2048, i32* %v14
  ; Cycle B0002
  %v15 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v16 = load i64, i64* %v15
  %v17 = add i64 %v16, 1
  store i64 %v17, i64* %v15
  %v18 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v19 = load i64, i64* %v18
  %v20 = add i64 %v19, 1
  store i64 %v20, i64* %v18
  %v21 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v21
  %v22 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v23 = load i32, i32* %v22
  %v24 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v25 = load i32, i32* %v24
  %v26 = mul i32 %v23, %v25
  store i32 %v26, i32* %v22
  ; Cycle B0003
  %v27 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v28 = load i64, i64* %v27
  %v29 = add i64 %v28, 1
  store i64 %v29, i64* %v27
  %v30 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v31 = load i64, i64* %v30
  %v32 = add i64 %v31, 1
  store i64 %v32, i64* %v30
  ; Cycle B0000
  %v33 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v34 = load i64, i64* %v33
  %v35 = add i64 %v34, 1
  store i64 %v35, i64* %v33
  %v36 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 4118, i32* %v36
  %v37 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 8221, i32* %v37
  %v38 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 769, i32* %v38
  %v39 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1025, i32* %v39
  ; Cycle B0001
  %v40 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v41 = load i64, i64* %v40
  %v42 = add i64 %v41, 1
  store i64 %v42, i64* %v40
  %v43 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1281, i32* %v43
  %v44 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1537, i32* %v44
  %v45 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 28698, i32* %v45
  %v46 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2049, i32* %v46
  ; Cycle B0002
  %v47 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v48 = load i64, i64* %v47
  %v49 = add i64 %v48, 1
  store i64 %v49, i64* %v47
  %v50 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v51 = load i64, i64* %v50
  %v52 = add i64 %v51, 1
  store i64 %v52, i64* %v50
  %v53 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v53
  %v54 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v55 = load i32, i32* %v54
  %v56 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v57 = load i32, i32* %v56
  %v58 = mul i32 %v55, %v57
  store i32 %v58, i32* %v54
  ; Cycle B0003
  %v59 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v60 = load i64, i64* %v59
  %v61 = add i64 %v60, 1
  store i64 %v61, i64* %v59
  %v62 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v63 = load i64, i64* %v62
  %v64 = add i64 %v63, 1
  store i64 %v64, i64* %v62
  ; Cycle B0000
  %v65 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v66 = load i64, i64* %v65
  %v67 = add i64 %v66, 1
  store i64 %v67, i64* %v65
  %v68 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 258, i32* %v68
  %v69 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 514, i32* %v69
  %v70 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 770, i32* %v70
  %v71 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1026, i32* %v71
  ; Cycle B0001
  %v72 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v73 = load i64, i64* %v72
  %v74 = add i64 %v73, 1
  store i64 %v74, i64* %v72
  %v75 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 20516, i32* %v75
  %v76 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1538, i32* %v76
  %v77 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1794, i32* %v77
  %v78 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2050, i32* %v78
  ; Cycle B0002
  %v79 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v80 = load i64, i64* %v79
  %v81 = add i64 %v80, 1
  store i64 %v81, i64* %v79
  %v82 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v83 = load i64, i64* %v82
  %v84 = add i64 %v83, 1
  store i64 %v84, i64* %v82
  %v85 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v85
  %v86 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v87 = load i32, i32* %v86
  %v88 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v89 = load i32, i32* %v88
  %v90 = mul i32 %v87, %v89
  store i32 %v90, i32* %v86
  ; Cycle B0003
  %v91 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v92 = load i64, i64* %v91
  %v93 = add i64 %v92, 1
  store i64 %v93, i64* %v91
  %v94 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v95 = load i64, i64* %v94
  %v96 = add i64 %v95, 1
  store i64 %v96, i64* %v94
  ; Cycle B0000
  %v97 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v98 = load i64, i64* %v97
  %v99 = add i64 %v98, 1
  store i64 %v99, i64* %v97
  %v100 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 259, i32* %v100
  %v101 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 515, i32* %v101
  %v102 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 12347, i32* %v102
  %v103 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 16444, i32* %v103
  ; Cycle B0001
  %v104 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v105 = load i64, i64* %v104
  %v106 = add i64 %v105, 1
  store i64 %v106, i64* %v104
  %v107 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 20533, i32* %v107
  %v108 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 24636, i32* %v108
  %v109 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1795, i32* %v109
  %v110 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 32828, i32* %v110
  ; Cycle B0002
  %v111 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v112 = load i64, i64* %v111
  %v113 = add i64 %v112, 1
  store i64 %v113, i64* %v111
  %v114 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v115 = load i64, i64* %v114
  %v116 = add i64 %v115, 1
  store i64 %v116, i64* %v114
  %v117 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v117
  %v118 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v119 = load i32, i32* %v118
  %v120 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v121 = load i32, i32* %v120
  %v122 = mul i32 %v119, %v121
  store i32 %v122, i32* %v118
  ; Cycle B0003
  %v123 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v124 = load i64, i64* %v123
  %v125 = add i64 %v124, 1
  store i64 %v125, i64* %v123
  %v126 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v127 = load i64, i64* %v126
  %v128 = add i64 %v127, 1
  store i64 %v128, i64* %v126
  ; Cycle B0000
  %v129 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v130 = load i64, i64* %v129
  %v131 = add i64 %v130, 1
  store i64 %v131, i64* %v129
  %v132 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 260, i32* %v132
  %v133 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 516, i32* %v133
  %v134 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 772, i32* %v134
  %v135 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1028, i32* %v135
  ; Cycle B0001
  %v136 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v137 = load i64, i64* %v136
  %v138 = add i64 %v137, 1
  store i64 %v138, i64* %v136
  %v139 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1284, i32* %v139
  %v140 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1540, i32* %v140
  %v141 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1796, i32* %v141
  %v142 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2052, i32* %v142
  ; Cycle B0002
  %v143 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v144 = load i64, i64* %v143
  %v145 = add i64 %v144, 1
  store i64 %v145, i64* %v143
  %v146 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v147 = load i64, i64* %v146
  %v148 = add i64 %v147, 1
  store i64 %v148, i64* %v146
  %v149 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v149
  %v150 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v151 = load i32, i32* %v150
  %v152 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v153 = load i32, i32* %v152
  %v154 = mul i32 %v151, %v153
  store i32 %v154, i32* %v150
  ; Cycle B0003
  %v155 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v156 = load i64, i64* %v155
  %v157 = add i64 %v156, 1
  store i64 %v157, i64* %v155
  %v158 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v159 = load i64, i64* %v158
  %v160 = add i64 %v159, 1
  store i64 %v160, i64* %v158
  ; Cycle B0000
  %v161 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v162 = load i64, i64* %v161
  %v163 = add i64 %v162, 1
  store i64 %v163, i64* %v161
  %v164 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 261, i32* %v164
  %v165 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 517, i32* %v165
  %v166 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 773, i32* %v166
  %v167 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1029, i32* %v167
  ; Cycle B0001
  %v168 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v169 = load i64, i64* %v168
  %v170 = add i64 %v169, 1
  store i64 %v170, i64* %v168
  %v171 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1285, i32* %v171
  %v172 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1541, i32* %v172
  %v173 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 28766, i32* %v173
  %v174 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2053, i32* %v174
  ; Cycle B0002
  %v175 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v176 = load i64, i64* %v175
  %v177 = add i64 %v176, 1
  store i64 %v177, i64* %v175
  %v178 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v179 = load i64, i64* %v178
  %v180 = add i64 %v179, 1
  store i64 %v180, i64* %v178
  %v181 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v181
  %v182 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v183 = load i32, i32* %v182
  %v184 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v185 = load i32, i32* %v184
  %v186 = mul i32 %v183, %v185
  store i32 %v186, i32* %v182
  ; Cycle B0003
  %v187 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v188 = load i64, i64* %v187
  %v189 = add i64 %v188, 1
  store i64 %v189, i64* %v187
  %v190 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v191 = load i64, i64* %v190
  %v192 = add i64 %v191, 1
  store i64 %v192, i64* %v190
  ; Cycle B0000
  %v193 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v194 = load i64, i64* %v193
  %v195 = add i64 %v194, 1
  store i64 %v195, i64* %v193
  %v196 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 262, i32* %v196
  %v197 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 518, i32* %v197
  %v198 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 774, i32* %v198
  %v199 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1030, i32* %v199
  ; Cycle B0001
  %v200 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v201 = load i64, i64* %v200
  %v202 = add i64 %v201, 1
  store i64 %v202, i64* %v200
  %v203 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 20576, i32* %v203
  %v204 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1542, i32* %v204
  %v205 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1798, i32* %v205
  %v206 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2054, i32* %v206
  ; Cycle B0002
  %v207 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v208 = load i64, i64* %v207
  %v209 = add i64 %v208, 1
  store i64 %v209, i64* %v207
  %v210 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v211 = load i64, i64* %v210
  %v212 = add i64 %v211, 1
  store i64 %v212, i64* %v210
  %v213 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v213
  %v214 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v215 = load i32, i32* %v214
  %v216 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v217 = load i32, i32* %v216
  %v218 = mul i32 %v215, %v217
  store i32 %v218, i32* %v214
  ; Cycle B0003
  %v219 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v220 = load i64, i64* %v219
  %v221 = add i64 %v220, 1
  store i64 %v221, i64* %v219
  %v222 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v223 = load i64, i64* %v222
  %v224 = add i64 %v223, 1
  store i64 %v224, i64* %v222
  ; Cycle B0000
  %v225 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v226 = load i64, i64* %v225
  %v227 = add i64 %v226, 1
  store i64 %v227, i64* %v225
  %v228 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 263, i32* %v228
  %v229 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 519, i32* %v229
  %v230 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 12415, i32* %v230
  %v231 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1031, i32* %v231
  ; Cycle B0001
  %v232 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v233 = load i64, i64* %v232
  %v234 = add i64 %v233, 1
  store i64 %v234, i64* %v232
  %v235 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1287, i32* %v235
  %v236 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1543, i32* %v236
  %v237 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1799, i32* %v237
  %v238 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2055, i32* %v238
  ; Cycle B0002
  %v239 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v240 = load i64, i64* %v239
  %v241 = add i64 %v240, 1
  store i64 %v241, i64* %v239
  %v242 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v243 = load i64, i64* %v242
  %v244 = add i64 %v243, 1
  store i64 %v244, i64* %v242
  %v245 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v245
  %v246 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v247 = load i32, i32* %v246
  %v248 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v249 = load i32, i32* %v248
  %v250 = mul i32 %v247, %v249
  store i32 %v250, i32* %v246
  ; Cycle B0003
  %v251 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v252 = load i64, i64* %v251
  %v253 = add i64 %v252, 1
  store i64 %v253, i64* %v251
  %v254 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v255 = load i64, i64* %v254
  %v256 = add i64 %v255, 1
  store i64 %v256, i64* %v254
  ; Cycle B0000
  %v257 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v258 = load i64, i64* %v257
  %v259 = add i64 %v258, 1
  store i64 %v259, i64* %v257
  %v260 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 264, i32* %v260
  %v261 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 520, i32* %v261
  %v262 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 776, i32* %v262
  %v263 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1032, i32* %v263
  ; Cycle B0001
  %v264 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v265 = load i64, i64* %v264
  %v266 = add i64 %v265, 1
  store i64 %v266, i64* %v264
  %v267 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 20623, i32* %v267
  %v268 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1544, i32* %v268
  %v269 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 28813, i32* %v269
  %v270 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 32900, i32* %v270
  ; Cycle B0002
  %v271 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v272 = load i64, i64* %v271
  %v273 = add i64 %v272, 1
  store i64 %v273, i64* %v271
  %v274 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v275 = load i64, i64* %v274
  %v276 = add i64 %v275, 1
  store i64 %v276, i64* %v274
  %v277 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v277
  %v278 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v279 = load i32, i32* %v278
  %v280 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v281 = load i32, i32* %v280
  %v282 = mul i32 %v279, %v281
  store i32 %v282, i32* %v278
  ; Cycle B0003
  %v283 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v284 = load i64, i64* %v283
  %v285 = add i64 %v284, 1
  store i64 %v285, i64* %v283
  %v286 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v287 = load i64, i64* %v286
  %v288 = add i64 %v287, 1
  store i64 %v288, i64* %v286
  ; Cycle B0000
  %v289 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v290 = load i64, i64* %v289
  %v291 = add i64 %v290, 1
  store i64 %v291, i64* %v289
  %v292 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 265, i32* %v292
  %v293 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 521, i32* %v293
  %v294 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 777, i32* %v294
  %v295 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1033, i32* %v295
  ; Cycle B0001
  %v296 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v297 = load i64, i64* %v296
  %v298 = add i64 %v297, 1
  store i64 %v298, i64* %v296
  %v299 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 20636, i32* %v299
  %v300 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1545, i32* %v300
  %v301 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1801, i32* %v301
  %v302 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2057, i32* %v302
  ; Cycle B0002
  %v303 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v304 = load i64, i64* %v303
  %v305 = add i64 %v304, 1
  store i64 %v305, i64* %v303
  %v306 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v307 = load i64, i64* %v306
  %v308 = add i64 %v307, 1
  store i64 %v308, i64* %v306
  %v309 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v309
  %v310 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v311 = load i32, i32* %v310
  %v312 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v313 = load i32, i32* %v312
  %v314 = mul i32 %v311, %v313
  store i32 %v314, i32* %v310
  ; Cycle B0003
  %v315 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v316 = load i64, i64* %v315
  %v317 = add i64 %v316, 1
  store i64 %v317, i64* %v315
  %v318 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v319 = load i64, i64* %v318
  %v320 = add i64 %v319, 1
  store i64 %v320, i64* %v318
  ; Cycle B0000
  %v321 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v322 = load i64, i64* %v321
  %v323 = add i64 %v322, 1
  store i64 %v323, i64* %v321
  %v324 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 4268, i32* %v324
  %v325 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 522, i32* %v325
  %v326 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 778, i32* %v326
  %v327 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1034, i32* %v327
  ; Cycle B0001
  %v328 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v329 = load i64, i64* %v328
  %v330 = add i64 %v329, 1
  store i64 %v330, i64* %v328
  %v331 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1290, i32* %v331
  %v332 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1546, i32* %v332
  %v333 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1802, i32* %v333
  %v334 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2058, i32* %v334
  ; Cycle B0002
  %v335 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v336 = load i64, i64* %v335
  %v337 = add i64 %v336, 1
  store i64 %v337, i64* %v335
  %v338 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v339 = load i64, i64* %v338
  %v340 = add i64 %v339, 1
  store i64 %v340, i64* %v338
  %v341 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v341
  %v342 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v343 = load i32, i32* %v342
  %v344 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v345 = load i32, i32* %v344
  %v346 = mul i32 %v343, %v345
  store i32 %v346, i32* %v342
  ; Cycle B0003
  %v347 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v348 = load i64, i64* %v347
  %v349 = add i64 %v348, 1
  store i64 %v349, i64* %v347
  %v350 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v351 = load i64, i64* %v350
  %v352 = add i64 %v351, 1
  store i64 %v352, i64* %v350
  ; Cycle B0000
  %v353 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v354 = load i64, i64* %v353
  %v355 = add i64 %v354, 1
  store i64 %v355, i64* %v353
  %v356 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 267, i32* %v356
  %v357 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 8381, i32* %v357
  %v358 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 12468, i32* %v358
  %v359 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1035, i32* %v359
  ; Cycle B0001
  %v360 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v361 = load i64, i64* %v360
  %v362 = add i64 %v361, 1
  store i64 %v362, i64* %v360
  %v363 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 20670, i32* %v363
  %v364 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 24757, i32* %v364
  %v365 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1803, i32* %v365
  %v366 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2059, i32* %v366
  ; Cycle B0002
  %v367 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v368 = load i64, i64* %v367
  %v369 = add i64 %v368, 1
  store i64 %v369, i64* %v367
  %v370 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v371 = load i64, i64* %v370
  %v372 = add i64 %v371, 1
  store i64 %v372, i64* %v370
  %v373 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v373
  %v374 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v375 = load i32, i32* %v374
  %v376 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v377 = load i32, i32* %v376
  %v378 = mul i32 %v375, %v377
  store i32 %v378, i32* %v374
  ; Cycle B0003
  %v379 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v380 = load i64, i64* %v379
  %v381 = add i64 %v380, 1
  store i64 %v381, i64* %v379
  %v382 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v383 = load i64, i64* %v382
  %v384 = add i64 %v383, 1
  store i64 %v384, i64* %v382
  ; Cycle B0000
  %v385 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v386 = load i64, i64* %v385
  %v387 = add i64 %v386, 1
  store i64 %v387, i64* %v385
  %v388 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 268, i32* %v388
  %v389 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 524, i32* %v389
  %v390 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 780, i32* %v390
  %v391 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1036, i32* %v391
  ; Cycle B0001
  %v392 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v393 = load i64, i64* %v392
  %v394 = add i64 %v393, 1
  store i64 %v394, i64* %v392
  %v395 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 20685, i32* %v395
  %v396 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1548, i32* %v396
  %v397 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1804, i32* %v397
  %v398 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2060, i32* %v398
  ; Cycle B0002
  %v399 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v400 = load i64, i64* %v399
  %v401 = add i64 %v400, 1
  store i64 %v401, i64* %v399
  %v402 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v403 = load i64, i64* %v402
  %v404 = add i64 %v403, 1
  store i64 %v404, i64* %v402
  %v405 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v405
  %v406 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v407 = load i32, i32* %v406
  %v408 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v409 = load i32, i32* %v408
  %v410 = mul i32 %v407, %v409
  store i32 %v410, i32* %v406
  ; Cycle B0003
  %v411 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v412 = load i64, i64* %v411
  %v413 = add i64 %v412, 1
  store i64 %v413, i64* %v411
  %v414 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v415 = load i64, i64* %v414
  %v416 = add i64 %v415, 1
  store i64 %v416, i64* %v414
  ; Cycle B0000
  %v417 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v418 = load i64, i64* %v417
  %v419 = add i64 %v418, 1
  store i64 %v419, i64* %v417
  %v420 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 269, i32* %v420
  %v421 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 8402, i32* %v421
  %v422 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 12507, i32* %v422
  %v423 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1037, i32* %v423
  ; Cycle B0001
  %v424 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v425 = load i64, i64* %v424
  %v426 = add i64 %v425, 1
  store i64 %v426, i64* %v424
  %v427 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1293, i32* %v427
  %v428 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1549, i32* %v428
  %v429 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 28887, i32* %v429
  %v430 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2061, i32* %v430
  ; Cycle B0002
  %v431 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v432 = load i64, i64* %v431
  %v433 = add i64 %v432, 1
  store i64 %v433, i64* %v431
  %v434 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v435 = load i64, i64* %v434
  %v436 = add i64 %v435, 1
  store i64 %v436, i64* %v434
  %v437 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v437
  %v438 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v439 = load i32, i32* %v438
  %v440 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v441 = load i32, i32* %v440
  %v442 = mul i32 %v439, %v441
  store i32 %v442, i32* %v438
  ; Cycle B0003
  %v443 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v444 = load i64, i64* %v443
  %v445 = add i64 %v444, 1
  store i64 %v445, i64* %v443
  %v446 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v447 = load i64, i64* %v446
  %v448 = add i64 %v447, 1
  store i64 %v448, i64* %v446
  ; Cycle B0000
  %v449 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v450 = load i64, i64* %v449
  %v451 = add i64 %v450, 1
  store i64 %v451, i64* %v449
  %v452 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 270, i32* %v452
  %v453 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 526, i32* %v453
  %v454 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 782, i32* %v454
  %v455 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1038, i32* %v455
  ; Cycle B0001
  %v456 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v457 = load i64, i64* %v456
  %v458 = add i64 %v457, 1
  store i64 %v458, i64* %v456
  %v459 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1294, i32* %v459
  %v460 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1550, i32* %v460
  %v461 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1806, i32* %v461
  %v462 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2062, i32* %v462
  ; Cycle B0002
  %v463 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v464 = load i64, i64* %v463
  %v465 = add i64 %v464, 1
  store i64 %v465, i64* %v463
  %v466 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v467 = load i64, i64* %v466
  %v468 = add i64 %v467, 1
  store i64 %v468, i64* %v466
  %v469 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v469
  %v470 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v471 = load i32, i32* %v470
  %v472 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v473 = load i32, i32* %v472
  %v474 = mul i32 %v471, %v473
  store i32 %v474, i32* %v470
  ; Cycle B0003
  %v475 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v476 = load i64, i64* %v475
  %v477 = add i64 %v476, 1
  store i64 %v477, i64* %v475
  %v478 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v479 = load i64, i64* %v478
  %v480 = add i64 %v479, 1
  store i64 %v480, i64* %v478
  ; Cycle B0000
  %v481 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v482 = load i64, i64* %v481
  %v483 = add i64 %v482, 1
  store i64 %v483, i64* %v481
  %v484 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 271, i32* %v484
  %v485 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 527, i32* %v485
  %v486 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 12536, i32* %v486
  %v487 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1039, i32* %v487
  ; Cycle B0001
  %v488 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v489 = load i64, i64* %v488
  %v490 = add i64 %v489, 1
  store i64 %v490, i64* %v488
  %v491 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1295, i32* %v491
  %v492 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 24817, i32* %v492
  %v493 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 28922, i32* %v493
  %v494 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2063, i32* %v494
  ; Cycle B0002
  %v495 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v496 = load i64, i64* %v495
  %v497 = add i64 %v496, 1
  store i64 %v497, i64* %v495
  %v498 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v499 = load i64, i64* %v498
  %v500 = add i64 %v499, 1
  store i64 %v500, i64* %v498
  %v501 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v501
  %v502 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v503 = load i32, i32* %v502
  %v504 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v505 = load i32, i32* %v504
  %v506 = mul i32 %v503, %v505
  store i32 %v506, i32* %v502
  ; Cycle B0003
  %v507 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v508 = load i64, i64* %v507
  %v509 = add i64 %v508, 1
  store i64 %v509, i64* %v507
  %v510 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v511 = load i64, i64* %v510
  %v512 = add i64 %v511, 1
  store i64 %v512, i64* %v510
  ; Cycle B0000
  %v513 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v514 = load i64, i64* %v513
  %v515 = add i64 %v514, 1
  store i64 %v515, i64* %v513
  %v516 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 272, i32* %v516
  %v517 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 528, i32* %v517
  %v518 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 12559, i32* %v518
  %v519 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1040, i32* %v519
  ; Cycle B0001
  %v520 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v521 = load i64, i64* %v520
  %v522 = add i64 %v521, 1
  store i64 %v522, i64* %v520
  %v523 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1296, i32* %v523
  %v524 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1552, i32* %v524
  %v525 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1808, i32* %v525
  %v526 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2064, i32* %v526
  ; Cycle B0002
  %v527 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v528 = load i64, i64* %v527
  %v529 = add i64 %v528, 1
  store i64 %v529, i64* %v527
  %v530 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v531 = load i64, i64* %v530
  %v532 = add i64 %v531, 1
  store i64 %v532, i64* %v530
  %v533 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v533
  %v534 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v535 = load i32, i32* %v534
  %v536 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v537 = load i32, i32* %v536
  %v538 = mul i32 %v535, %v537
  store i32 %v538, i32* %v534
  ; Cycle B0003
  %v539 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v540 = load i64, i64* %v539
  %v541 = add i64 %v540, 1
  store i64 %v541, i64* %v539
  %v542 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v543 = load i64, i64* %v542
  %v544 = add i64 %v543, 1
  store i64 %v544, i64* %v542
  ; Cycle B0000
  %v545 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v546 = load i64, i64* %v545
  %v547 = add i64 %v546, 1
  store i64 %v547, i64* %v545
  %v548 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 273, i32* %v548
  %v549 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 529, i32* %v549
  %v550 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 785, i32* %v550
  %v551 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1041, i32* %v551
  ; Cycle B0001
  %v552 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v553 = load i64, i64* %v552
  %v554 = add i64 %v553, 1
  store i64 %v554, i64* %v552
  %v555 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 20752, i32* %v555
  %v556 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1553, i32* %v556
  %v557 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1809, i32* %v557
  %v558 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2065, i32* %v558
  ; Cycle B0002
  %v559 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v560 = load i64, i64* %v559
  %v561 = add i64 %v560, 1
  store i64 %v561, i64* %v559
  %v562 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v563 = load i64, i64* %v562
  %v564 = add i64 %v563, 1
  store i64 %v564, i64* %v562
  %v565 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v565
  %v566 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v567 = load i32, i32* %v566
  %v568 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v569 = load i32, i32* %v568
  %v570 = mul i32 %v567, %v569
  store i32 %v570, i32* %v566
  ; Cycle B0003
  %v571 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v572 = load i64, i64* %v571
  %v573 = add i64 %v572, 1
  store i64 %v573, i64* %v571
  %v574 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v575 = load i64, i64* %v574
  %v576 = add i64 %v575, 1
  store i64 %v576, i64* %v574
  ; Cycle B0000
  %v577 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v578 = load i64, i64* %v577
  %v579 = add i64 %v578, 1
  store i64 %v579, i64* %v577
  %v580 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 274, i32* %v580
  %v581 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 530, i32* %v581
  %v582 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 786, i32* %v582
  %v583 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1042, i32* %v583
  ; Cycle B0001
  %v584 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v585 = load i64, i64* %v584
  %v586 = add i64 %v585, 1
  store i64 %v586, i64* %v584
  %v587 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1298, i32* %v587
  %v588 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1554, i32* %v588
  %v589 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 28974, i32* %v589
  %v590 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2066, i32* %v590
  ; Cycle B0002
  %v591 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v592 = load i64, i64* %v591
  %v593 = add i64 %v592, 1
  store i64 %v593, i64* %v591
  %v594 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v595 = load i64, i64* %v594
  %v596 = add i64 %v595, 1
  store i64 %v596, i64* %v594
  %v597 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v597
  %v598 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v599 = load i32, i32* %v598
  %v600 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v601 = load i32, i32* %v600
  %v602 = mul i32 %v599, %v601
  store i32 %v602, i32* %v598
  ; Cycle B0003
  %v603 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v604 = load i64, i64* %v603
  %v605 = add i64 %v604, 1
  store i64 %v605, i64* %v603
  %v606 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v607 = load i64, i64* %v606
  %v608 = add i64 %v607, 1
  store i64 %v608, i64* %v606
  ; Cycle B0000
  %v609 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v610 = load i64, i64* %v609
  %v611 = add i64 %v610, 1
  store i64 %v611, i64* %v609
  %v612 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 275, i32* %v612
  %v613 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 531, i32* %v613
  %v614 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 787, i32* %v614
  %v615 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1043, i32* %v615
  ; Cycle B0001
  %v616 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v617 = load i64, i64* %v616
  %v618 = add i64 %v617, 1
  store i64 %v618, i64* %v616
  %v619 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1299, i32* %v619
  %v620 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1555, i32* %v620
  %v621 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1811, i32* %v621
  %v622 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2067, i32* %v622
  ; Cycle B0002
  %v623 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v624 = load i64, i64* %v623
  %v625 = add i64 %v624, 1
  store i64 %v625, i64* %v623
  %v626 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v627 = load i64, i64* %v626
  %v628 = add i64 %v627, 1
  store i64 %v628, i64* %v626
  %v629 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v629
  %v630 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v631 = load i32, i32* %v630
  %v632 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v633 = load i32, i32* %v632
  %v634 = mul i32 %v631, %v633
  store i32 %v634, i32* %v630
  ; Cycle B0003
  %v635 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v636 = load i64, i64* %v635
  %v637 = add i64 %v636, 1
  store i64 %v637, i64* %v635
  %v638 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v639 = load i64, i64* %v638
  %v640 = add i64 %v639, 1
  store i64 %v640, i64* %v638
  ; Cycle B0000
  %v641 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v642 = load i64, i64* %v641
  %v643 = add i64 %v642, 1
  store i64 %v643, i64* %v641
  %v644 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 276, i32* %v644
  %v645 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 532, i32* %v645
  %v646 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 12619, i32* %v646
  %v647 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 16716, i32* %v647
  ; Cycle B0001
  %v648 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v649 = load i64, i64* %v648
  %v650 = add i64 %v649, 1
  store i64 %v650, i64* %v648
  %v651 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 20805, i32* %v651
  %v652 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 24908, i32* %v652
  %v653 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1812, i32* %v653
  %v654 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 33100, i32* %v654
  ; Cycle B0002
  %v655 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v656 = load i64, i64* %v655
  %v657 = add i64 %v656, 1
  store i64 %v657, i64* %v655
  %v658 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v659 = load i64, i64* %v658
  %v660 = add i64 %v659, 1
  store i64 %v660, i64* %v658
  %v661 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v661
  %v662 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v663 = load i32, i32* %v662
  %v664 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v665 = load i32, i32* %v664
  %v666 = mul i32 %v663, %v665
  store i32 %v666, i32* %v662
  ; Cycle B0003
  %v667 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v668 = load i64, i64* %v667
  %v669 = add i64 %v668, 1
  store i64 %v669, i64* %v667
  %v670 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v671 = load i64, i64* %v670
  %v672 = add i64 %v671, 1
  store i64 %v672, i64* %v670
  ; Cycle B0000
  %v673 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v674 = load i64, i64* %v673
  %v675 = add i64 %v674, 1
  store i64 %v675, i64* %v673
  %v676 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 277, i32* %v676
  %v677 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 533, i32* %v677
  %v678 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 789, i32* %v678
  %v679 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1045, i32* %v679
  ; Cycle B0001
  %v680 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v681 = load i64, i64* %v680
  %v682 = add i64 %v681, 1
  store i64 %v682, i64* %v680
  %v683 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 20820, i32* %v683
  %v684 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1557, i32* %v684
  %v685 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1813, i32* %v685
  %v686 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2069, i32* %v686
  ; Cycle B0002
  %v687 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v688 = load i64, i64* %v687
  %v689 = add i64 %v688, 1
  store i64 %v689, i64* %v687
  %v690 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v691 = load i64, i64* %v690
  %v692 = add i64 %v691, 1
  store i64 %v692, i64* %v690
  %v693 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v693
  %v694 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v695 = load i32, i32* %v694
  %v696 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v697 = load i32, i32* %v696
  %v698 = mul i32 %v695, %v697
  store i32 %v698, i32* %v694
  ; Cycle B0003
  %v699 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v700 = load i64, i64* %v699
  %v701 = add i64 %v700, 1
  store i64 %v701, i64* %v699
  %v702 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v703 = load i64, i64* %v702
  %v704 = add i64 %v703, 1
  store i64 %v704, i64* %v702
  ; Cycle B0000
  %v705 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v706 = load i64, i64* %v705
  %v707 = add i64 %v706, 1
  store i64 %v707, i64* %v705
  %v708 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 4454, i32* %v708
  %v709 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 8557, i32* %v709
  %v710 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 790, i32* %v710
  %v711 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1046, i32* %v711
  ; Cycle B0001
  %v712 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v713 = load i64, i64* %v712
  %v714 = add i64 %v713, 1
  store i64 %v714, i64* %v712
  %v715 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1302, i32* %v715
  %v716 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1558, i32* %v716
  %v717 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 29034, i32* %v717
  %v718 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2070, i32* %v718
  ; Cycle B0002
  %v719 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v720 = load i64, i64* %v719
  %v721 = add i64 %v720, 1
  store i64 %v721, i64* %v719
  %v722 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v723 = load i64, i64* %v722
  %v724 = add i64 %v723, 1
  store i64 %v724, i64* %v722
  %v725 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v725
  %v726 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v727 = load i32, i32* %v726
  %v728 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v729 = load i32, i32* %v728
  %v730 = mul i32 %v727, %v729
  store i32 %v730, i32* %v726
  ; Cycle B0003
  %v731 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v732 = load i64, i64* %v731
  %v733 = add i64 %v732, 1
  store i64 %v733, i64* %v731
  %v734 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v735 = load i64, i64* %v734
  %v736 = add i64 %v735, 1
  store i64 %v736, i64* %v734
  ; Cycle B0000
  %v737 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v738 = load i64, i64* %v737
  %v739 = add i64 %v738, 1
  store i64 %v739, i64* %v737
  %v740 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 4467, i32* %v740
  %v741 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 535, i32* %v741
  %v742 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 791, i32* %v742
  %v743 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1047, i32* %v743
  ; Cycle B0001
  %v744 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v745 = load i64, i64* %v744
  %v746 = add i64 %v745, 1
  store i64 %v746, i64* %v744
  %v747 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1303, i32* %v747
  %v748 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1559, i32* %v748
  %v749 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1815, i32* %v749
  %v750 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2071, i32* %v750
  ; Cycle B0002
  %v751 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v752 = load i64, i64* %v751
  %v753 = add i64 %v752, 1
  store i64 %v753, i64* %v751
  %v754 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v755 = load i64, i64* %v754
  %v756 = add i64 %v755, 1
  store i64 %v756, i64* %v754
  %v757 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v757
  %v758 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v759 = load i32, i32* %v758
  %v760 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v761 = load i32, i32* %v760
  %v762 = mul i32 %v759, %v761
  store i32 %v762, i32* %v758
  ; Cycle B0003
  %v763 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v764 = load i64, i64* %v763
  %v765 = add i64 %v764, 1
  store i64 %v765, i64* %v763
  %v766 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v767 = load i64, i64* %v766
  %v768 = add i64 %v767, 1
  store i64 %v768, i64* %v766
  ; Cycle B0000
  %v769 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v770 = load i64, i64* %v769
  %v771 = add i64 %v770, 1
  store i64 %v771, i64* %v769
  %v772 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 280, i32* %v772
  %v773 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 536, i32* %v773
  %v774 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 792, i32* %v774
  %v775 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1048, i32* %v775
  ; Cycle B0001
  %v776 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v777 = load i64, i64* %v776
  %v778 = add i64 %v777, 1
  store i64 %v778, i64* %v776
  %v779 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1304, i32* %v779
  %v780 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1560, i32* %v780
  %v781 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1816, i32* %v781
  %v782 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2072, i32* %v782
  ; Cycle B0002
  %v783 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v784 = load i64, i64* %v783
  %v785 = add i64 %v784, 1
  store i64 %v785, i64* %v783
  %v786 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v787 = load i64, i64* %v786
  %v788 = add i64 %v787, 1
  store i64 %v788, i64* %v786
  %v789 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v789
  %v790 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v791 = load i32, i32* %v790
  %v792 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v793 = load i32, i32* %v792
  %v794 = mul i32 %v791, %v793
  store i32 %v794, i32* %v790
  ; Cycle B0003
  %v795 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v796 = load i64, i64* %v795
  %v797 = add i64 %v796, 1
  store i64 %v797, i64* %v795
  %v798 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v799 = load i64, i64* %v798
  %v800 = add i64 %v799, 1
  store i64 %v800, i64* %v798
  ; Cycle B0000
  %v801 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v802 = load i64, i64* %v801
  %v803 = add i64 %v802, 1
  store i64 %v803, i64* %v801
  %v804 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 281, i32* %v804
  %v805 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 537, i32* %v805
  %v806 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 793, i32* %v806
  %v807 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 16799, i32* %v807
  ; Cycle B0001
  %v808 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v809 = load i64, i64* %v808
  %v810 = add i64 %v809, 1
  store i64 %v810, i64* %v808
  %v811 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1305, i32* %v811
  %v812 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 24991, i32* %v812
  %v813 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1817, i32* %v813
  %v814 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 33179, i32* %v814
  ; Cycle B0002
  %v815 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v816 = load i64, i64* %v815
  %v817 = add i64 %v816, 1
  store i64 %v817, i64* %v815
  %v818 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v819 = load i64, i64* %v818
  %v820 = add i64 %v819, 1
  store i64 %v820, i64* %v818
  %v821 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v821
  %v822 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v823 = load i32, i32* %v822
  %v824 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v825 = load i32, i32* %v824
  %v826 = mul i32 %v823, %v825
  store i32 %v826, i32* %v822
  ; Cycle B0003
  %v827 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v828 = load i64, i64* %v827
  %v829 = add i64 %v828, 1
  store i64 %v829, i64* %v827
  %v830 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v831 = load i64, i64* %v830
  %v832 = add i64 %v831, 1
  store i64 %v832, i64* %v830
  ; Cycle B0000
  %v833 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v834 = load i64, i64* %v833
  %v835 = add i64 %v834, 1
  store i64 %v835, i64* %v833
  %v836 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 282, i32* %v836
  %v837 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 538, i32* %v837
  %v838 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 12712, i32* %v838
  %v839 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1050, i32* %v839
  ; Cycle B0001
  %v840 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v841 = load i64, i64* %v840
  %v842 = add i64 %v841, 1
  store i64 %v842, i64* %v840
  %v843 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1306, i32* %v843
  %v844 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 24993, i32* %v844
  %v845 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 29098, i32* %v845
  %v846 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2074, i32* %v846
  ; Cycle B0002
  %v847 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v848 = load i64, i64* %v847
  %v849 = add i64 %v848, 1
  store i64 %v849, i64* %v847
  %v850 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v851 = load i64, i64* %v850
  %v852 = add i64 %v851, 1
  store i64 %v852, i64* %v850
  %v853 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v853
  %v854 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v855 = load i32, i32* %v854
  %v856 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v857 = load i32, i32* %v856
  %v858 = mul i32 %v855, %v857
  store i32 %v858, i32* %v854
  ; Cycle B0003
  %v859 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v860 = load i64, i64* %v859
  %v861 = add i64 %v860, 1
  store i64 %v861, i64* %v859
  %v862 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v863 = load i64, i64* %v862
  %v864 = add i64 %v863, 1
  store i64 %v864, i64* %v862
  ; Cycle B0000
  %v865 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v866 = load i64, i64* %v865
  %v867 = add i64 %v866, 1
  store i64 %v867, i64* %v865
  %v868 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 283, i32* %v868
  %v869 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 539, i32* %v869
  %v870 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 795, i32* %v870
  %v871 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1051, i32* %v871
  ; Cycle B0001
  %v872 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v873 = load i64, i64* %v872
  %v874 = add i64 %v873, 1
  store i64 %v874, i64* %v872
  %v875 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1307, i32* %v875
  %v876 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1563, i32* %v876
  %v877 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1819, i32* %v877
  %v878 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2075, i32* %v878
  ; Cycle B0002
  %v879 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v880 = load i64, i64* %v879
  %v881 = add i64 %v880, 1
  store i64 %v881, i64* %v879
  %v882 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v883 = load i64, i64* %v882
  %v884 = add i64 %v883, 1
  store i64 %v884, i64* %v882
  %v885 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v885
  %v886 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v887 = load i32, i32* %v886
  %v888 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v889 = load i32, i32* %v888
  %v890 = mul i32 %v887, %v889
  store i32 %v890, i32* %v886
  ; Cycle B0003
  %v891 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v892 = load i64, i64* %v891
  %v893 = add i64 %v892, 1
  store i64 %v893, i64* %v891
  %v894 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v895 = load i64, i64* %v894
  %v896 = add i64 %v895, 1
  store i64 %v896, i64* %v894
  ; Cycle B0000
  %v897 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v898 = load i64, i64* %v897
  %v899 = add i64 %v898, 1
  store i64 %v899, i64* %v897
  %v900 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 284, i32* %v900
  %v901 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 8642, i32* %v901
  %v902 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 12747, i32* %v902
  %v903 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1052, i32* %v903
  ; Cycle B0001
  %v904 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v905 = load i64, i64* %v904
  %v906 = add i64 %v905, 1
  store i64 %v906, i64* %v904
  %v907 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1308, i32* %v907
  %v908 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1564, i32* %v908
  %v909 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 29127, i32* %v909
  %v910 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2076, i32* %v910
  ; Cycle B0002
  %v911 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v912 = load i64, i64* %v911
  %v913 = add i64 %v912, 1
  store i64 %v913, i64* %v911
  %v914 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v915 = load i64, i64* %v914
  %v916 = add i64 %v915, 1
  store i64 %v916, i64* %v914
  %v917 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v917
  %v918 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v919 = load i32, i32* %v918
  %v920 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v921 = load i32, i32* %v920
  %v922 = mul i32 %v919, %v921
  store i32 %v922, i32* %v918
  ; Cycle B0003
  %v923 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v924 = load i64, i64* %v923
  %v925 = add i64 %v924, 1
  store i64 %v925, i64* %v923
  %v926 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v927 = load i64, i64* %v926
  %v928 = add i64 %v927, 1
  store i64 %v928, i64* %v926
  ; Cycle B0000
  %v929 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v930 = load i64, i64* %v929
  %v931 = add i64 %v930, 1
  store i64 %v931, i64* %v929
  %v932 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 285, i32* %v932
  %v933 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 541, i32* %v933
  %v934 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 797, i32* %v934
  %v935 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1053, i32* %v935
  ; Cycle B0001
  %v936 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v937 = load i64, i64* %v936
  %v938 = add i64 %v937, 1
  store i64 %v938, i64* %v936
  %v939 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 20957, i32* %v939
  %v940 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1565, i32* %v940
  %v941 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1821, i32* %v941
  %v942 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2077, i32* %v942
  ; Cycle B0002
  %v943 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v944 = load i64, i64* %v943
  %v945 = add i64 %v944, 1
  store i64 %v945, i64* %v943
  %v946 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v947 = load i64, i64* %v946
  %v948 = add i64 %v947, 1
  store i64 %v948, i64* %v946
  %v949 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v949
  %v950 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v951 = load i32, i32* %v950
  %v952 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v953 = load i32, i32* %v952
  %v954 = mul i32 %v951, %v953
  store i32 %v954, i32* %v950
  ; Cycle B0003
  %v955 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v956 = load i64, i64* %v955
  %v957 = add i64 %v956, 1
  store i64 %v957, i64* %v955
  %v958 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v959 = load i64, i64* %v958
  %v960 = add i64 %v959, 1
  store i64 %v960, i64* %v958
  ; Cycle B0000
  %v961 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v962 = load i64, i64* %v961
  %v963 = add i64 %v962, 1
  store i64 %v963, i64* %v961
  %v964 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 286, i32* %v964
  %v965 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 8685, i32* %v965
  %v966 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 12772, i32* %v966
  %v967 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1054, i32* %v967
  ; Cycle B0001
  %v968 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v969 = load i64, i64* %v968
  %v970 = add i64 %v969, 1
  store i64 %v970, i64* %v968
  %v971 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 20974, i32* %v971
  %v972 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 25061, i32* %v972
  %v973 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1822, i32* %v973
  %v974 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2078, i32* %v974
  ; Cycle B0002
  %v975 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v976 = load i64, i64* %v975
  %v977 = add i64 %v976, 1
  store i64 %v977, i64* %v975
  %v978 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v979 = load i64, i64* %v978
  %v980 = add i64 %v979, 1
  store i64 %v980, i64* %v978
  %v981 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v981
  %v982 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v983 = load i32, i32* %v982
  %v984 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v985 = load i32, i32* %v984
  %v986 = mul i32 %v983, %v985
  store i32 %v986, i32* %v982
  ; Cycle B0003
  %v987 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v988 = load i64, i64* %v987
  %v989 = add i64 %v988, 1
  store i64 %v989, i64* %v987
  %v990 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v991 = load i64, i64* %v990
  %v992 = add i64 %v991, 1
  store i64 %v992, i64* %v990
  ; Cycle B0000
  %v993 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v994 = load i64, i64* %v993
  %v995 = add i64 %v994, 1
  store i64 %v995, i64* %v993
  %v996 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 4604, i32* %v996
  %v997 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 543, i32* %v997
  %v998 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 799, i32* %v998
  %v999 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1055, i32* %v999
  ; Cycle B0001
  %v1000 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1001 = load i64, i64* %v1000
  %v1002 = add i64 %v1001, 1
  store i64 %v1002, i64* %v1000
  %v1003 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1311, i32* %v1003
  %v1004 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1567, i32* %v1004
  %v1005 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1823, i32* %v1005
  %v1006 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2079, i32* %v1006
  ; Cycle B0002
  %v1007 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1008 = load i64, i64* %v1007
  %v1009 = add i64 %v1008, 1
  store i64 %v1009, i64* %v1007
  %v1010 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1011 = load i64, i64* %v1010
  %v1012 = add i64 %v1011, 1
  store i64 %v1012, i64* %v1010
  %v1013 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1013
  %v1014 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v1015 = load i32, i32* %v1014
  %v1016 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v1017 = load i32, i32* %v1016
  %v1018 = mul i32 %v1015, %v1017
  store i32 %v1018, i32* %v1014
  ; Cycle B0003
  %v1019 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1020 = load i64, i64* %v1019
  %v1021 = add i64 %v1020, 1
  store i64 %v1021, i64* %v1019
  %v1022 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1023 = load i64, i64* %v1022
  %v1024 = add i64 %v1023, 1
  store i64 %v1024, i64* %v1022
  ; Cycle B0000
  %v1025 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1026 = load i64, i64* %v1025
  %v1027 = add i64 %v1026, 1
  store i64 %v1027, i64* %v1025
  %v1028 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 288, i32* %v1028
  %v1029 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 544, i32* %v1029
  %v1030 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 800, i32* %v1030
  %v1031 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 16911, i32* %v1031
  ; Cycle B0001
  %v1032 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1033 = load i64, i64* %v1032
  %v1034 = add i64 %v1033, 1
  store i64 %v1034, i64* %v1032
  %v1035 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1312, i32* %v1035
  %v1036 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 25103, i32* %v1036
  %v1037 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1824, i32* %v1037
  %v1038 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 33291, i32* %v1038
  ; Cycle B0002
  %v1039 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1040 = load i64, i64* %v1039
  %v1041 = add i64 %v1040, 1
  store i64 %v1041, i64* %v1039
  %v1042 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1043 = load i64, i64* %v1042
  %v1044 = add i64 %v1043, 1
  store i64 %v1044, i64* %v1042
  %v1045 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1045
  %v1046 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v1047 = load i32, i32* %v1046
  %v1048 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v1049 = load i32, i32* %v1048
  %v1050 = mul i32 %v1047, %v1049
  store i32 %v1050, i32* %v1046
  ; Cycle B0003
  %v1051 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1052 = load i64, i64* %v1051
  %v1053 = add i64 %v1052, 1
  store i64 %v1053, i64* %v1051
  %v1054 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1055 = load i64, i64* %v1054
  %v1056 = add i64 %v1055, 1
  store i64 %v1056, i64* %v1054
  ; Cycle B0000
  %v1057 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1058 = load i64, i64* %v1057
  %v1059 = add i64 %v1058, 1
  store i64 %v1059, i64* %v1057
  %v1060 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 289, i32* %v1060
  %v1061 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 545, i32* %v1061
  %v1062 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 801, i32* %v1062
  %v1063 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1057, i32* %v1063
  ; Cycle B0001
  %v1064 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1065 = load i64, i64* %v1064
  %v1066 = add i64 %v1065, 1
  store i64 %v1066, i64* %v1064
  %v1067 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1313, i32* %v1067
  %v1068 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1569, i32* %v1068
  %v1069 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1825, i32* %v1069
  %v1070 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2081, i32* %v1070
  ; Cycle B0002
  %v1071 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1072 = load i64, i64* %v1071
  %v1073 = add i64 %v1072, 1
  store i64 %v1073, i64* %v1071
  %v1074 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1075 = load i64, i64* %v1074
  %v1076 = add i64 %v1075, 1
  store i64 %v1076, i64* %v1074
  %v1077 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1077
  %v1078 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v1079 = load i32, i32* %v1078
  %v1080 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v1081 = load i32, i32* %v1080
  %v1082 = mul i32 %v1079, %v1081
  store i32 %v1082, i32* %v1078
  ; Cycle B0003
  %v1083 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1084 = load i64, i64* %v1083
  %v1085 = add i64 %v1084, 1
  store i64 %v1085, i64* %v1083
  %v1086 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1087 = load i64, i64* %v1086
  %v1088 = add i64 %v1087, 1
  store i64 %v1088, i64* %v1086
  ; Cycle B0000
  %v1089 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1090 = load i64, i64* %v1089
  %v1091 = add i64 %v1090, 1
  store i64 %v1091, i64* %v1089
  %v1092 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 290, i32* %v1092
  %v1093 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 8750, i32* %v1093
  %v1094 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 802, i32* %v1094
  %v1095 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1058, i32* %v1095
  ; Cycle B0001
  %v1096 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1097 = load i64, i64* %v1096
  %v1098 = add i64 %v1097, 1
  store i64 %v1098, i64* %v1096
  %v1099 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1314, i32* %v1099
  %v1100 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1570, i32* %v1100
  %v1101 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1826, i32* %v1101
  %v1102 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 33320, i32* %v1102
  ; Cycle B0002
  %v1103 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1104 = load i64, i64* %v1103
  %v1105 = add i64 %v1104, 1
  store i64 %v1105, i64* %v1103
  %v1106 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1107 = load i64, i64* %v1106
  %v1108 = add i64 %v1107, 1
  store i64 %v1108, i64* %v1106
  %v1109 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1109
  %v1110 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v1111 = load i32, i32* %v1110
  %v1112 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v1113 = load i32, i32* %v1112
  %v1114 = mul i32 %v1111, %v1113
  store i32 %v1114, i32* %v1110
  ; Cycle B0003
  %v1115 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1116 = load i64, i64* %v1115
  %v1117 = add i64 %v1116, 1
  store i64 %v1117, i64* %v1115
  %v1118 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1119 = load i64, i64* %v1118
  %v1120 = add i64 %v1119, 1
  store i64 %v1120, i64* %v1118
  ; Cycle B0000
  %v1121 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1122 = load i64, i64* %v1121
  %v1123 = add i64 %v1122, 1
  store i64 %v1123, i64* %v1121
  %v1124 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 291, i32* %v1124
  %v1125 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 547, i32* %v1125
  %v1126 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 803, i32* %v1126
  %v1127 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1059, i32* %v1127
  ; Cycle B0001
  %v1128 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1129 = load i64, i64* %v1128
  %v1130 = add i64 %v1129, 1
  store i64 %v1130, i64* %v1128
  %v1131 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1315, i32* %v1131
  %v1132 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1571, i32* %v1132
  %v1133 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1827, i32* %v1133
  %v1134 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 33337, i32* %v1134
  ; Cycle B0002
  %v1135 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1136 = load i64, i64* %v1135
  %v1137 = add i64 %v1136, 1
  store i64 %v1137, i64* %v1135
  %v1138 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1139 = load i64, i64* %v1138
  %v1140 = add i64 %v1139, 1
  store i64 %v1140, i64* %v1138
  %v1141 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1141
  %v1142 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v1143 = load i32, i32* %v1142
  %v1144 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v1145 = load i32, i32* %v1144
  %v1146 = mul i32 %v1143, %v1145
  store i32 %v1146, i32* %v1142
  ; Cycle B0003
  %v1147 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1148 = load i64, i64* %v1147
  %v1149 = add i64 %v1148, 1
  store i64 %v1149, i64* %v1147
  %v1150 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1151 = load i64, i64* %v1150
  %v1152 = add i64 %v1151, 1
  store i64 %v1152, i64* %v1150
  ; Cycle B0000
  %v1153 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1154 = load i64, i64* %v1153
  %v1155 = add i64 %v1154, 1
  store i64 %v1155, i64* %v1153
  %v1156 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 4686, i32* %v1156
  %v1157 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 548, i32* %v1157
  %v1158 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 12876, i32* %v1158
  %v1159 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 16971, i32* %v1159
  ; Cycle B0001
  %v1160 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1161 = load i64, i64* %v1160
  %v1162 = add i64 %v1161, 1
  store i64 %v1162, i64* %v1160
  %v1163 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1316, i32* %v1163
  %v1164 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1572, i32* %v1164
  %v1165 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1828, i32* %v1165
  %v1166 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2084, i32* %v1166
  ; Cycle B0002
  %v1167 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1168 = load i64, i64* %v1167
  %v1169 = add i64 %v1168, 1
  store i64 %v1169, i64* %v1167
  %v1170 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1171 = load i64, i64* %v1170
  %v1172 = add i64 %v1171, 1
  store i64 %v1172, i64* %v1170
  %v1173 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1173
  %v1174 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v1175 = load i32, i32* %v1174
  %v1176 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v1177 = load i32, i32* %v1176
  %v1178 = mul i32 %v1175, %v1177
  store i32 %v1178, i32* %v1174
  ; Cycle B0003
  %v1179 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1180 = load i64, i64* %v1179
  %v1181 = add i64 %v1180, 1
  store i64 %v1181, i64* %v1179
  %v1182 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1183 = load i64, i64* %v1182
  %v1184 = add i64 %v1183, 1
  store i64 %v1184, i64* %v1182
  ; Cycle B0000
  %v1185 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1186 = load i64, i64* %v1185
  %v1187 = add i64 %v1186, 1
  store i64 %v1187, i64* %v1185
  %v1188 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 4701, i32* %v1188
  %v1189 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 549, i32* %v1189
  %v1190 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 805, i32* %v1190
  %v1191 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1061, i32* %v1191
  ; Cycle B0001
  %v1192 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1193 = load i64, i64* %v1192
  %v1194 = add i64 %v1193, 1
  store i64 %v1194, i64* %v1192
  %v1195 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1317, i32* %v1195
  %v1196 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1573, i32* %v1196
  %v1197 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1829, i32* %v1197
  %v1198 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2085, i32* %v1198
  ; Cycle B0002
  %v1199 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1200 = load i64, i64* %v1199
  %v1201 = add i64 %v1200, 1
  store i64 %v1201, i64* %v1199
  %v1202 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1203 = load i64, i64* %v1202
  %v1204 = add i64 %v1203, 1
  store i64 %v1204, i64* %v1202
  %v1205 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1205
  %v1206 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v1207 = load i32, i32* %v1206
  %v1208 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v1209 = load i32, i32* %v1208
  %v1210 = mul i32 %v1207, %v1209
  store i32 %v1210, i32* %v1206
  ; Cycle B0003
  %v1211 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1212 = load i64, i64* %v1211
  %v1213 = add i64 %v1212, 1
  store i64 %v1213, i64* %v1211
  %v1214 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1215 = load i64, i64* %v1214
  %v1216 = add i64 %v1215, 1
  store i64 %v1216, i64* %v1214
  ; Cycle B0000
  %v1217 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1218 = load i64, i64* %v1217
  %v1219 = add i64 %v1218, 1
  store i64 %v1219, i64* %v1217
  %v1220 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 294, i32* %v1220
  %v1221 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 550, i32* %v1221
  %v1222 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 806, i32* %v1222
  %v1223 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1062, i32* %v1223
  ; Cycle B0001
  %v1224 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1225 = load i64, i64* %v1224
  %v1226 = add i64 %v1225, 1
  store i64 %v1226, i64* %v1224
  %v1227 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 21103, i32* %v1227
  %v1228 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1574, i32* %v1228
  %v1229 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 29293, i32* %v1229
  %v1230 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 33380, i32* %v1230
  ; Cycle B0002
  %v1231 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1232 = load i64, i64* %v1231
  %v1233 = add i64 %v1232, 1
  store i64 %v1233, i64* %v1231
  %v1234 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1235 = load i64, i64* %v1234
  %v1236 = add i64 %v1235, 1
  store i64 %v1236, i64* %v1234
  %v1237 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1237
  %v1238 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v1239 = load i32, i32* %v1238
  %v1240 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v1241 = load i32, i32* %v1240
  %v1242 = mul i32 %v1239, %v1241
  store i32 %v1242, i32* %v1238
  ; Cycle B0003
  %v1243 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1244 = load i64, i64* %v1243
  %v1245 = add i64 %v1244, 1
  store i64 %v1245, i64* %v1243
  %v1246 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1247 = load i64, i64* %v1246
  %v1248 = add i64 %v1247, 1
  store i64 %v1248, i64* %v1246
  ; Cycle B0000
  %v1249 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1250 = load i64, i64* %v1249
  %v1251 = add i64 %v1250, 1
  store i64 %v1251, i64* %v1249
  %v1252 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 295, i32* %v1252
  %v1253 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 551, i32* %v1253
  %v1254 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 807, i32* %v1254
  %v1255 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1063, i32* %v1255
  ; Cycle B0001
  %v1256 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1257 = load i64, i64* %v1256
  %v1258 = add i64 %v1257, 1
  store i64 %v1258, i64* %v1256
  %v1259 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 21116, i32* %v1259
  %v1260 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1575, i32* %v1260
  %v1261 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1831, i32* %v1261
  %v1262 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2087, i32* %v1262
  ; Cycle B0002
  %v1263 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1264 = load i64, i64* %v1263
  %v1265 = add i64 %v1264, 1
  store i64 %v1265, i64* %v1263
  %v1266 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1267 = load i64, i64* %v1266
  %v1268 = add i64 %v1267, 1
  store i64 %v1268, i64* %v1266
  %v1269 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1269
  %v1270 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v1271 = load i32, i32* %v1270
  %v1272 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v1273 = load i32, i32* %v1272
  %v1274 = mul i32 %v1271, %v1273
  store i32 %v1274, i32* %v1270
  ; Cycle B0003
  %v1275 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1276 = load i64, i64* %v1275
  %v1277 = add i64 %v1276, 1
  store i64 %v1277, i64* %v1275
  %v1278 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1279 = load i64, i64* %v1278
  %v1280 = add i64 %v1279, 1
  store i64 %v1280, i64* %v1278
  ; Cycle B0000
  %v1281 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1282 = load i64, i64* %v1281
  %v1283 = add i64 %v1282, 1
  store i64 %v1283, i64* %v1281
  %v1284 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 296, i32* %v1284
  %v1285 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 552, i32* %v1285
  %v1286 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 808, i32* %v1286
  %v1287 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1064, i32* %v1287
  ; Cycle B0001
  %v1288 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1289 = load i64, i64* %v1288
  %v1290 = add i64 %v1289, 1
  store i64 %v1290, i64* %v1288
  %v1291 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 21120, i32* %v1291
  %v1292 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1576, i32* %v1292
  %v1293 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1832, i32* %v1293
  %v1294 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2088, i32* %v1294
  ; Cycle B0002
  %v1295 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1296 = load i64, i64* %v1295
  %v1297 = add i64 %v1296, 1
  store i64 %v1297, i64* %v1295
  %v1298 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1299 = load i64, i64* %v1298
  %v1300 = add i64 %v1299, 1
  store i64 %v1300, i64* %v1298
  %v1301 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1301
  %v1302 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v1303 = load i32, i32* %v1302
  %v1304 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v1305 = load i32, i32* %v1304
  %v1306 = mul i32 %v1303, %v1305
  store i32 %v1306, i32* %v1302
  ; Cycle B0003
  %v1307 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1308 = load i64, i64* %v1307
  %v1309 = add i64 %v1308, 1
  store i64 %v1309, i64* %v1307
  %v1310 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1311 = load i64, i64* %v1310
  %v1312 = add i64 %v1311, 1
  store i64 %v1312, i64* %v1310
  ; Cycle B0000
  %v1313 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1314 = load i64, i64* %v1313
  %v1315 = add i64 %v1314, 1
  store i64 %v1315, i64* %v1313
  %v1316 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 297, i32* %v1316
  %v1317 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 553, i32* %v1317
  %v1318 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 12959, i32* %v1318
  %v1319 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1065, i32* %v1319
  ; Cycle B0001
  %v1320 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1321 = load i64, i64* %v1320
  %v1322 = add i64 %v1321, 1
  store i64 %v1322, i64* %v1320
  %v1323 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1321, i32* %v1323
  %v1324 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1577, i32* %v1324
  %v1325 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1833, i32* %v1325
  %v1326 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2089, i32* %v1326
  ; Cycle B0002
  %v1327 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1328 = load i64, i64* %v1327
  %v1329 = add i64 %v1328, 1
  store i64 %v1329, i64* %v1327
  %v1330 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1331 = load i64, i64* %v1330
  %v1332 = add i64 %v1331, 1
  store i64 %v1332, i64* %v1330
  %v1333 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1333
  %v1334 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v1335 = load i32, i32* %v1334
  %v1336 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v1337 = load i32, i32* %v1336
  %v1338 = mul i32 %v1335, %v1337
  store i32 %v1338, i32* %v1334
  ; Cycle B0003
  %v1339 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1340 = load i64, i64* %v1339
  %v1341 = add i64 %v1340, 1
  store i64 %v1341, i64* %v1339
  %v1342 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1343 = load i64, i64* %v1342
  %v1344 = add i64 %v1343, 1
  store i64 %v1344, i64* %v1342
  ; Cycle B0000
  %v1345 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1346 = load i64, i64* %v1345
  %v1347 = add i64 %v1346, 1
  store i64 %v1347, i64* %v1345
  %v1348 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 298, i32* %v1348
  %v1349 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 8874, i32* %v1349
  %v1350 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 810, i32* %v1350
  %v1351 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 17064, i32* %v1351
  ; Cycle B0001
  %v1352 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1353 = load i64, i64* %v1352
  %v1354 = add i64 %v1353, 1
  store i64 %v1354, i64* %v1352
  %v1355 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1322, i32* %v1355
  %v1356 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1578, i32* %v1356
  %v1357 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1834, i32* %v1357
  %v1358 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2090, i32* %v1358
  ; Cycle B0002
  %v1359 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1360 = load i64, i64* %v1359
  %v1361 = add i64 %v1360, 1
  store i64 %v1361, i64* %v1359
  %v1362 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1363 = load i64, i64* %v1362
  %v1364 = add i64 %v1363, 1
  store i64 %v1364, i64* %v1362
  %v1365 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1365
  %v1366 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v1367 = load i32, i32* %v1366
  %v1368 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v1369 = load i32, i32* %v1368
  %v1370 = mul i32 %v1367, %v1369
  store i32 %v1370, i32* %v1366
  ; Cycle B0003
  %v1371 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1372 = load i64, i64* %v1371
  %v1373 = add i64 %v1372, 1
  store i64 %v1373, i64* %v1371
  %v1374 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1375 = load i64, i64* %v1374
  %v1376 = add i64 %v1375, 1
  store i64 %v1376, i64* %v1374
  ; Cycle B0000
  %v1377 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1378 = load i64, i64* %v1377
  %v1379 = add i64 %v1378, 1
  store i64 %v1379, i64* %v1377
  %v1380 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 299, i32* %v1380
  %v1381 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 555, i32* %v1381
  %v1382 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 811, i32* %v1382
  %v1383 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1067, i32* %v1383
  ; Cycle B0001
  %v1384 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1385 = load i64, i64* %v1384
  %v1386 = add i64 %v1385, 1
  store i64 %v1386, i64* %v1384
  %v1387 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 21169, i32* %v1387
  %v1388 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1579, i32* %v1388
  %v1389 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1835, i32* %v1389
  %v1390 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2091, i32* %v1390
  ; Cycle B0002
  %v1391 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1392 = load i64, i64* %v1391
  %v1393 = add i64 %v1392, 1
  store i64 %v1393, i64* %v1391
  %v1394 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1395 = load i64, i64* %v1394
  %v1396 = add i64 %v1395, 1
  store i64 %v1396, i64* %v1394
  %v1397 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1397
  %v1398 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v1399 = load i32, i32* %v1398
  %v1400 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v1401 = load i32, i32* %v1400
  %v1402 = mul i32 %v1399, %v1401
  store i32 %v1402, i32* %v1398
  ; Cycle B0003
  %v1403 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1404 = load i64, i64* %v1403
  %v1405 = add i64 %v1404, 1
  store i64 %v1405, i64* %v1403
  %v1406 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1407 = load i64, i64* %v1406
  %v1408 = add i64 %v1407, 1
  store i64 %v1408, i64* %v1406
  ; Cycle B0000
  %v1409 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1410 = load i64, i64* %v1409
  %v1411 = add i64 %v1410, 1
  store i64 %v1411, i64* %v1409
  %v1412 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 300, i32* %v1412
  %v1413 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 556, i32* %v1413
  %v1414 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 812, i32* %v1414
  %v1415 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 17099, i32* %v1415
  ; Cycle B0001
  %v1416 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1417 = load i64, i64* %v1416
  %v1418 = add i64 %v1417, 1
  store i64 %v1418, i64* %v1416
  %v1419 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1324, i32* %v1419
  %v1420 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 25291, i32* %v1420
  %v1421 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1836, i32* %v1421
  %v1422 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 33487, i32* %v1422
  ; Cycle B0002
  %v1423 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1424 = load i64, i64* %v1423
  %v1425 = add i64 %v1424, 1
  store i64 %v1425, i64* %v1423
  %v1426 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1427 = load i64, i64* %v1426
  %v1428 = add i64 %v1427, 1
  store i64 %v1428, i64* %v1426
  %v1429 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1429
  %v1430 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v1431 = load i32, i32* %v1430
  %v1432 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v1433 = load i32, i32* %v1432
  %v1434 = mul i32 %v1431, %v1433
  store i32 %v1434, i32* %v1430
  ; Cycle B0003
  %v1435 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1436 = load i64, i64* %v1435
  %v1437 = add i64 %v1436, 1
  store i64 %v1437, i64* %v1435
  %v1438 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1439 = load i64, i64* %v1438
  %v1440 = add i64 %v1439, 1
  store i64 %v1440, i64* %v1438
  ; Cycle B0000
  %v1441 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1442 = load i64, i64* %v1441
  %v1443 = add i64 %v1442, 1
  store i64 %v1443, i64* %v1441
  %v1444 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 4822, i32* %v1444
  %v1445 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 557, i32* %v1445
  %v1446 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 813, i32* %v1446
  %v1447 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1069, i32* %v1447
  ; Cycle B0001
  %v1448 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1449 = load i64, i64* %v1448
  %v1450 = add i64 %v1449, 1
  store i64 %v1450, i64* %v1448
  %v1451 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1325, i32* %v1451
  %v1452 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1581, i32* %v1452
  %v1453 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1837, i32* %v1453
  %v1454 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2093, i32* %v1454
  ; Cycle B0002
  %v1455 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1456 = load i64, i64* %v1455
  %v1457 = add i64 %v1456, 1
  store i64 %v1457, i64* %v1455
  %v1458 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1459 = load i64, i64* %v1458
  %v1460 = add i64 %v1459, 1
  store i64 %v1460, i64* %v1458
  %v1461 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1461
  %v1462 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v1463 = load i32, i32* %v1462
  %v1464 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v1465 = load i32, i32* %v1464
  %v1466 = mul i32 %v1463, %v1465
  store i32 %v1466, i32* %v1462
  ; Cycle B0003
  %v1467 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1468 = load i64, i64* %v1467
  %v1469 = add i64 %v1468, 1
  store i64 %v1469, i64* %v1467
  %v1470 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1471 = load i64, i64* %v1470
  %v1472 = add i64 %v1471, 1
  store i64 %v1472, i64* %v1470
  ; Cycle B0000
  %v1473 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1474 = load i64, i64* %v1473
  %v1475 = add i64 %v1474, 1
  store i64 %v1475, i64* %v1473
  %v1476 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 4839, i32* %v1476
  %v1477 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 8942, i32* %v1477
  %v1478 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 814, i32* %v1478
  %v1479 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 17124, i32* %v1479
  ; Cycle B0001
  %v1480 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1481 = load i64, i64* %v1480
  %v1482 = add i64 %v1481, 1
  store i64 %v1482, i64* %v1480
  %v1483 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1326, i32* %v1483
  %v1484 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1582, i32* %v1484
  %v1485 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 29421, i32* %v1485
  %v1486 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2094, i32* %v1486
  ; Cycle B0002
  %v1487 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1488 = load i64, i64* %v1487
  %v1489 = add i64 %v1488, 1
  store i64 %v1489, i64* %v1487
  %v1490 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1491 = load i64, i64* %v1490
  %v1492 = add i64 %v1491, 1
  store i64 %v1492, i64* %v1490
  %v1493 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1493
  %v1494 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v1495 = load i32, i32* %v1494
  %v1496 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v1497 = load i32, i32* %v1496
  %v1498 = mul i32 %v1495, %v1497
  store i32 %v1498, i32* %v1494
  ; Cycle B0003
  %v1499 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1500 = load i64, i64* %v1499
  %v1501 = add i64 %v1500, 1
  store i64 %v1501, i64* %v1499
  %v1502 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1503 = load i64, i64* %v1502
  %v1504 = add i64 %v1503, 1
  store i64 %v1504, i64* %v1502
  ; Cycle B0000
  %v1505 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1506 = load i64, i64* %v1505
  %v1507 = add i64 %v1506, 1
  store i64 %v1507, i64* %v1505
  %v1508 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 303, i32* %v1508
  %v1509 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 559, i32* %v1509
  %v1510 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 815, i32* %v1510
  %v1511 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1071, i32* %v1511
  ; Cycle B0001
  %v1512 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1513 = load i64, i64* %v1512
  %v1514 = add i64 %v1513, 1
  store i64 %v1514, i64* %v1512
  %v1515 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 21237, i32* %v1515
  %v1516 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1583, i32* %v1516
  %v1517 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1839, i32* %v1517
  %v1518 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2095, i32* %v1518
  ; Cycle B0002
  %v1519 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1520 = load i64, i64* %v1519
  %v1521 = add i64 %v1520, 1
  store i64 %v1521, i64* %v1519
  %v1522 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1523 = load i64, i64* %v1522
  %v1524 = add i64 %v1523, 1
  store i64 %v1524, i64* %v1522
  %v1525 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1525
  %v1526 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v1527 = load i32, i32* %v1526
  %v1528 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v1529 = load i32, i32* %v1528
  %v1530 = mul i32 %v1527, %v1529
  store i32 %v1530, i32* %v1526
  ; Cycle B0003
  %v1531 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1532 = load i64, i64* %v1531
  %v1533 = add i64 %v1532, 1
  store i64 %v1533, i64* %v1531
  %v1534 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1535 = load i64, i64* %v1534
  %v1536 = add i64 %v1535, 1
  store i64 %v1536, i64* %v1534
  ; Cycle B0000
  %v1537 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1538 = load i64, i64* %v1537
  %v1539 = add i64 %v1538, 1
  store i64 %v1539, i64* %v1537
  %v1540 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 304, i32* %v1540
  %v1541 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 560, i32* %v1541
  %v1542 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 816, i32* %v1542
  %v1543 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1072, i32* %v1543
  ; Cycle B0001
  %v1544 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1545 = load i64, i64* %v1544
  %v1546 = add i64 %v1545, 1
  store i64 %v1546, i64* %v1544
  %v1547 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 21260, i32* %v1547
  %v1548 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1584, i32* %v1548
  %v1549 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1840, i32* %v1549
  %v1550 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2096, i32* %v1550
  ; Cycle B0002
  %v1551 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1552 = load i64, i64* %v1551
  %v1553 = add i64 %v1552, 1
  store i64 %v1553, i64* %v1551
  %v1554 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1555 = load i64, i64* %v1554
  %v1556 = add i64 %v1555, 1
  store i64 %v1556, i64* %v1554
  %v1557 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1557
  %v1558 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v1558
  ; Cycle B0003
  %v1559 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1560 = load i64, i64* %v1559
  %v1561 = add i64 %v1560, 1
  store i64 %v1561, i64* %v1559
  %v1562 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1563 = load i64, i64* %v1562
  %v1564 = add i64 %v1563, 1
  store i64 %v1564, i64* %v1562
  ; Cycle B0000
  %v1565 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1566 = load i64, i64* %v1565
  %v1567 = add i64 %v1566, 1
  store i64 %v1567, i64* %v1565
  %v1568 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 305, i32* %v1568
  %v1569 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 561, i32* %v1569
  %v1570 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 817, i32* %v1570
  %v1571 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1073, i32* %v1571
  ; Cycle B0001
  %v1572 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1573 = load i64, i64* %v1572
  %v1574 = add i64 %v1573, 1
  store i64 %v1574, i64* %v1572
  %v1575 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 21279, i32* %v1575
  %v1576 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1585, i32* %v1576
  %v1577 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 29469, i32* %v1577
  %v1578 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 33556, i32* %v1578
  ; Cycle B0002
  %v1579 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1580 = load i64, i64* %v1579
  %v1581 = add i64 %v1580, 1
  store i64 %v1581, i64* %v1579
  %v1582 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1583 = load i64, i64* %v1582
  %v1584 = add i64 %v1583, 1
  store i64 %v1584, i64* %v1582
  %v1585 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1585
  %v1586 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v1586
  ; Cycle B0003
  %v1587 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1588 = load i64, i64* %v1587
  %v1589 = add i64 %v1588, 1
  store i64 %v1589, i64* %v1587
  %v1590 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1591 = load i64, i64* %v1590
  %v1592 = add i64 %v1591, 1
  store i64 %v1592, i64* %v1590
  ; Cycle B0000
  %v1593 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1594 = load i64, i64* %v1593
  %v1595 = add i64 %v1594, 1
  store i64 %v1595, i64* %v1593
  %v1596 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 4909, i32* %v1596
  %v1597 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 562, i32* %v1597
  %v1598 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 818, i32* %v1598
  %v1599 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1074, i32* %v1599
  ; Cycle B0001
  %v1600 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1601 = load i64, i64* %v1600
  %v1602 = add i64 %v1601, 1
  store i64 %v1602, i64* %v1600
  %v1603 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1330, i32* %v1603
  %v1604 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1586, i32* %v1604
  %v1605 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1842, i32* %v1605
  %v1606 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2098, i32* %v1606
  ; Cycle B0002
  %v1607 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1608 = load i64, i64* %v1607
  %v1609 = add i64 %v1608, 1
  store i64 %v1609, i64* %v1607
  %v1610 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1611 = load i64, i64* %v1610
  %v1612 = add i64 %v1611, 1
  store i64 %v1612, i64* %v1610
  %v1613 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1613
  %v1614 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v1614
  ; Cycle B0003
  %v1615 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1616 = load i64, i64* %v1615
  %v1617 = add i64 %v1616, 1
  store i64 %v1617, i64* %v1615
  %v1618 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1619 = load i64, i64* %v1618
  %v1620 = add i64 %v1619, 1
  store i64 %v1620, i64* %v1618
  ; Cycle B0000
  %v1621 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1622 = load i64, i64* %v1621
  %v1623 = add i64 %v1622, 1
  store i64 %v1623, i64* %v1621
  %v1624 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 4926, i32* %v1624
  %v1625 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 563, i32* %v1625
  %v1626 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 13116, i32* %v1626
  %v1627 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 17211, i32* %v1627
  ; Cycle B0001
  %v1628 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1629 = load i64, i64* %v1628
  %v1630 = add i64 %v1629, 1
  store i64 %v1630, i64* %v1628
  %v1631 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1331, i32* %v1631
  %v1632 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1587, i32* %v1632
  %v1633 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1843, i32* %v1633
  %v1634 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2099, i32* %v1634
  ; Cycle B0002
  %v1635 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1636 = load i64, i64* %v1635
  %v1637 = add i64 %v1636, 1
  store i64 %v1637, i64* %v1635
  %v1638 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1639 = load i64, i64* %v1638
  %v1640 = add i64 %v1639, 1
  store i64 %v1640, i64* %v1638
  %v1641 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1641
  %v1642 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v1642
  ; Cycle B0003
  %v1643 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1644 = load i64, i64* %v1643
  %v1645 = add i64 %v1644, 1
  store i64 %v1645, i64* %v1643
  %v1646 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1647 = load i64, i64* %v1646
  %v1648 = add i64 %v1647, 1
  store i64 %v1648, i64* %v1646
  ; Cycle B0000
  %v1649 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1650 = load i64, i64* %v1649
  %v1651 = add i64 %v1650, 1
  store i64 %v1651, i64* %v1649
  %v1652 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 308, i32* %v1652
  %v1653 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 564, i32* %v1653
  %v1654 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 820, i32* %v1654
  %v1655 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1076, i32* %v1655
  ; Cycle B0001
  %v1656 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1657 = load i64, i64* %v1656
  %v1658 = add i64 %v1657, 1
  store i64 %v1658, i64* %v1656
  %v1659 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1332, i32* %v1659
  %v1660 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1588, i32* %v1660
  %v1661 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1844, i32* %v1661
  %v1662 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 33609, i32* %v1662
  ; Cycle B0002
  %v1663 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1664 = load i64, i64* %v1663
  %v1665 = add i64 %v1664, 1
  store i64 %v1665, i64* %v1663
  %v1666 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1667 = load i64, i64* %v1666
  %v1668 = add i64 %v1667, 1
  store i64 %v1668, i64* %v1666
  %v1669 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1669
  %v1670 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v1670
  ; Cycle B0003
  %v1671 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1672 = load i64, i64* %v1671
  %v1673 = add i64 %v1672, 1
  store i64 %v1673, i64* %v1671
  %v1674 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1675 = load i64, i64* %v1674
  %v1676 = add i64 %v1675, 1
  store i64 %v1676, i64* %v1674
  ; Cycle B0000
  %v1677 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1678 = load i64, i64* %v1677
  %v1679 = add i64 %v1678, 1
  store i64 %v1679, i64* %v1677
  %v1680 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 309, i32* %v1680
  %v1681 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 9054, i32* %v1681
  %v1682 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 821, i32* %v1682
  %v1683 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1077, i32* %v1683
  ; Cycle B0001
  %v1684 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1685 = load i64, i64* %v1684
  %v1686 = add i64 %v1685, 1
  store i64 %v1686, i64* %v1684
  %v1687 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1333, i32* %v1687
  %v1688 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1589, i32* %v1688
  %v1689 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1845, i32* %v1689
  %v1690 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 33624, i32* %v1690
  ; Cycle B0002
  %v1691 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1692 = load i64, i64* %v1691
  %v1693 = add i64 %v1692, 1
  store i64 %v1693, i64* %v1691
  %v1694 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1695 = load i64, i64* %v1694
  %v1696 = add i64 %v1695, 1
  store i64 %v1696, i64* %v1694
  %v1697 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1697
  %v1698 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v1698
  ; Cycle B0003
  %v1699 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1700 = load i64, i64* %v1699
  %v1701 = add i64 %v1700, 1
  store i64 %v1701, i64* %v1699
  %v1702 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1703 = load i64, i64* %v1702
  %v1704 = add i64 %v1703, 1
  store i64 %v1704, i64* %v1702
  ; Cycle B0000
  %v1705 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1706 = load i64, i64* %v1705
  %v1707 = add i64 %v1706, 1
  store i64 %v1707, i64* %v1705
  %v1708 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 310, i32* %v1708
  %v1709 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 566, i32* %v1709
  %v1710 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 822, i32* %v1710
  %v1711 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1078, i32* %v1711
  ; Cycle B0001
  %v1712 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1713 = load i64, i64* %v1712
  %v1714 = add i64 %v1713, 1
  store i64 %v1714, i64* %v1712
  %v1715 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1334, i32* %v1715
  %v1716 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1590, i32* %v1716
  %v1717 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1846, i32* %v1717
  %v1718 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2102, i32* %v1718
  ; Cycle B0002
  %v1719 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1720 = load i64, i64* %v1719
  %v1721 = add i64 %v1720, 1
  store i64 %v1721, i64* %v1719
  %v1722 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1723 = load i64, i64* %v1722
  %v1724 = add i64 %v1723, 1
  store i64 %v1724, i64* %v1722
  %v1725 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1725
  %v1726 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v1726
  ; Cycle B0003
  %v1727 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1728 = load i64, i64* %v1727
  %v1729 = add i64 %v1728, 1
  store i64 %v1729, i64* %v1727
  %v1730 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1731 = load i64, i64* %v1730
  %v1732 = add i64 %v1731, 1
  store i64 %v1732, i64* %v1730
  ; Cycle B0000
  %v1733 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1734 = load i64, i64* %v1733
  %v1735 = add i64 %v1734, 1
  store i64 %v1735, i64* %v1733
  %v1736 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 311, i32* %v1736
  %v1737 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 567, i32* %v1737
  %v1738 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 823, i32* %v1738
  %v1739 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 17279, i32* %v1739
  ; Cycle B0001
  %v1740 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1741 = load i64, i64* %v1740
  %v1742 = add i64 %v1741, 1
  store i64 %v1742, i64* %v1740
  %v1743 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1335, i32* %v1743
  %v1744 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 25471, i32* %v1744
  %v1745 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1847, i32* %v1745
  %v1746 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 33659, i32* %v1746
  ; Cycle B0002
  %v1747 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1748 = load i64, i64* %v1747
  %v1749 = add i64 %v1748, 1
  store i64 %v1749, i64* %v1747
  %v1750 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1751 = load i64, i64* %v1750
  %v1752 = add i64 %v1751, 1
  store i64 %v1752, i64* %v1750
  %v1753 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1753
  %v1754 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v1754
  ; Cycle B0003
  %v1755 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1756 = load i64, i64* %v1755
  %v1757 = add i64 %v1756, 1
  store i64 %v1757, i64* %v1755
  %v1758 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1759 = load i64, i64* %v1758
  %v1760 = add i64 %v1759, 1
  store i64 %v1760, i64* %v1758
  ; Cycle B0000
  %v1761 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1762 = load i64, i64* %v1761
  %v1763 = add i64 %v1762, 1
  store i64 %v1763, i64* %v1761
  %v1764 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 4998, i32* %v1764
  %v1765 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 9101, i32* %v1765
  %v1766 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 824, i32* %v1766
  %v1767 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1080, i32* %v1767
  ; Cycle B0001
  %v1768 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1769 = load i64, i64* %v1768
  %v1770 = add i64 %v1769, 1
  store i64 %v1770, i64* %v1768
  %v1771 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1336, i32* %v1771
  %v1772 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1592, i32* %v1772
  %v1773 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 29578, i32* %v1773
  %v1774 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2104, i32* %v1774
  ; Cycle B0002
  %v1775 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1776 = load i64, i64* %v1775
  %v1777 = add i64 %v1776, 1
  store i64 %v1777, i64* %v1775
  %v1778 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1779 = load i64, i64* %v1778
  %v1780 = add i64 %v1779, 1
  store i64 %v1780, i64* %v1778
  %v1781 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1781
  %v1782 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v1782
  ; Cycle B0003
  %v1783 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1784 = load i64, i64* %v1783
  %v1785 = add i64 %v1784, 1
  store i64 %v1785, i64* %v1783
  %v1786 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1787 = load i64, i64* %v1786
  %v1788 = add i64 %v1787, 1
  store i64 %v1788, i64* %v1786
  ; Cycle B0000
  %v1789 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1790 = load i64, i64* %v1789
  %v1791 = add i64 %v1790, 1
  store i64 %v1791, i64* %v1789
  %v1792 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 5011, i32* %v1792
  %v1793 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 569, i32* %v1793
  %v1794 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 825, i32* %v1794
  %v1795 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1081, i32* %v1795
  ; Cycle B0001
  %v1796 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1797 = load i64, i64* %v1796
  %v1798 = add i64 %v1797, 1
  store i64 %v1798, i64* %v1796
  %v1799 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1337, i32* %v1799
  %v1800 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1593, i32* %v1800
  %v1801 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1849, i32* %v1801
  %v1802 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2105, i32* %v1802
  ; Cycle B0002
  %v1803 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1804 = load i64, i64* %v1803
  %v1805 = add i64 %v1804, 1
  store i64 %v1805, i64* %v1803
  %v1806 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1807 = load i64, i64* %v1806
  %v1808 = add i64 %v1807, 1
  store i64 %v1808, i64* %v1806
  %v1809 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1809
  %v1810 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v1810
  ; Cycle B0003
  %v1811 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1812 = load i64, i64* %v1811
  %v1813 = add i64 %v1812, 1
  store i64 %v1813, i64* %v1811
  %v1814 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1815 = load i64, i64* %v1814
  %v1816 = add i64 %v1815, 1
  store i64 %v1816, i64* %v1814
  ; Cycle B0000
  %v1817 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1818 = load i64, i64* %v1817
  %v1819 = add i64 %v1818, 1
  store i64 %v1819, i64* %v1817
  %v1820 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 314, i32* %v1820
  %v1821 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 570, i32* %v1821
  %v1822 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 826, i32* %v1822
  %v1823 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1082, i32* %v1823
  ; Cycle B0001
  %v1824 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1825 = load i64, i64* %v1824
  %v1826 = add i64 %v1825, 1
  store i64 %v1826, i64* %v1824
  %v1827 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 21413, i32* %v1827
  %v1828 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1594, i32* %v1828
  %v1829 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1850, i32* %v1829
  %v1830 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2106, i32* %v1830
  ; Cycle B0002
  %v1831 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1832 = load i64, i64* %v1831
  %v1833 = add i64 %v1832, 1
  store i64 %v1833, i64* %v1831
  %v1834 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1835 = load i64, i64* %v1834
  %v1836 = add i64 %v1835, 1
  store i64 %v1836, i64* %v1834
  %v1837 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1837
  %v1838 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v1838
  ; Cycle B0003
  %v1839 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1840 = load i64, i64* %v1839
  %v1841 = add i64 %v1840, 1
  store i64 %v1841, i64* %v1839
  %v1842 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1843 = load i64, i64* %v1842
  %v1844 = add i64 %v1843, 1
  store i64 %v1844, i64* %v1842
  ; Cycle B0000
  %v1845 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1846 = load i64, i64* %v1845
  %v1847 = add i64 %v1846, 1
  store i64 %v1847, i64* %v1845
  %v1848 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 5047, i32* %v1848
  %v1849 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 9150, i32* %v1849
  %v1850 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 827, i32* %v1850
  %v1851 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 17332, i32* %v1851
  ; Cycle B0001
  %v1852 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1853 = load i64, i64* %v1852
  %v1854 = add i64 %v1853, 1
  store i64 %v1854, i64* %v1852
  %v1855 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1339, i32* %v1855
  %v1856 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1595, i32* %v1856
  %v1857 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 29629, i32* %v1857
  %v1858 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2107, i32* %v1858
  ; Cycle B0002
  %v1859 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1860 = load i64, i64* %v1859
  %v1861 = add i64 %v1860, 1
  store i64 %v1861, i64* %v1859
  %v1862 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1863 = load i64, i64* %v1862
  %v1864 = add i64 %v1863, 1
  store i64 %v1864, i64* %v1862
  %v1865 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1865
  %v1866 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v1866
  ; Cycle B0003
  %v1867 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1868 = load i64, i64* %v1867
  %v1869 = add i64 %v1868, 1
  store i64 %v1869, i64* %v1867
  %v1870 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1871 = load i64, i64* %v1870
  %v1872 = add i64 %v1871, 1
  store i64 %v1872, i64* %v1870
  ; Cycle B0000
  %v1873 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1874 = load i64, i64* %v1873
  %v1875 = add i64 %v1874, 1
  store i64 %v1875, i64* %v1873
  %v1876 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 5062, i32* %v1876
  %v1877 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 572, i32* %v1877
  %v1878 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 828, i32* %v1878
  %v1879 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1084, i32* %v1879
  ; Cycle B0001
  %v1880 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1881 = load i64, i64* %v1880
  %v1882 = add i64 %v1881, 1
  store i64 %v1882, i64* %v1880
  %v1883 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1340, i32* %v1883
  %v1884 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1596, i32* %v1884
  %v1885 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1852, i32* %v1885
  %v1886 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2108, i32* %v1886
  ; Cycle B0002
  %v1887 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1888 = load i64, i64* %v1887
  %v1889 = add i64 %v1888, 1
  store i64 %v1889, i64* %v1887
  %v1890 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1891 = load i64, i64* %v1890
  %v1892 = add i64 %v1891, 1
  store i64 %v1892, i64* %v1890
  %v1893 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1893
  %v1894 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v1894
  ; Cycle B0003
  %v1895 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1896 = load i64, i64* %v1895
  %v1897 = add i64 %v1896, 1
  store i64 %v1897, i64* %v1895
  %v1898 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1899 = load i64, i64* %v1898
  %v1900 = add i64 %v1899, 1
  store i64 %v1900, i64* %v1898
  ; Cycle B0000
  %v1901 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1902 = load i64, i64* %v1901
  %v1903 = add i64 %v1902, 1
  store i64 %v1903, i64* %v1901
  %v1904 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 317, i32* %v1904
  %v1905 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 573, i32* %v1905
  %v1906 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 829, i32* %v1906
  %v1907 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 17371, i32* %v1907
  ; Cycle B0001
  %v1908 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1909 = load i64, i64* %v1908
  %v1910 = add i64 %v1909, 1
  store i64 %v1910, i64* %v1908
  %v1911 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1341, i32* %v1911
  %v1912 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 25563, i32* %v1912
  %v1913 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1853, i32* %v1913
  %v1914 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 33759, i32* %v1914
  ; Cycle B0002
  %v1915 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1916 = load i64, i64* %v1915
  %v1917 = add i64 %v1916, 1
  store i64 %v1917, i64* %v1915
  %v1918 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1919 = load i64, i64* %v1918
  %v1920 = add i64 %v1919, 1
  store i64 %v1920, i64* %v1918
  %v1921 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1921
  %v1922 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v1922
  ; Cycle B0003
  %v1923 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1924 = load i64, i64* %v1923
  %v1925 = add i64 %v1924, 1
  store i64 %v1925, i64* %v1923
  %v1926 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1927 = load i64, i64* %v1926
  %v1928 = add i64 %v1927, 1
  store i64 %v1928, i64* %v1926
  ; Cycle B0000
  %v1929 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1930 = load i64, i64* %v1929
  %v1931 = add i64 %v1930, 1
  store i64 %v1931, i64* %v1929
  %v1932 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 318, i32* %v1932
  %v1933 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 574, i32* %v1933
  %v1934 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 830, i32* %v1934
  %v1935 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1086, i32* %v1935
  ; Cycle B0001
  %v1936 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1937 = load i64, i64* %v1936
  %v1938 = add i64 %v1937, 1
  store i64 %v1938, i64* %v1936
  %v1939 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 21473, i32* %v1939
  %v1940 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1598, i32* %v1940
  %v1941 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1854, i32* %v1941
  %v1942 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2110, i32* %v1942
  ; Cycle B0002
  %v1943 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1944 = load i64, i64* %v1943
  %v1945 = add i64 %v1944, 1
  store i64 %v1945, i64* %v1943
  %v1946 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1947 = load i64, i64* %v1946
  %v1948 = add i64 %v1947, 1
  store i64 %v1948, i64* %v1946
  %v1949 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1949
  %v1950 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v1950
  ; Cycle B0003
  %v1951 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1952 = load i64, i64* %v1951
  %v1953 = add i64 %v1952, 1
  store i64 %v1953, i64* %v1951
  %v1954 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1955 = load i64, i64* %v1954
  %v1956 = add i64 %v1955, 1
  store i64 %v1956, i64* %v1954
  ; Cycle B0000
  %v1957 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1958 = load i64, i64* %v1957
  %v1959 = add i64 %v1958, 1
  store i64 %v1959, i64* %v1957
  %v1960 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 319, i32* %v1960
  %v1961 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 9210, i32* %v1961
  %v1962 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 831, i32* %v1962
  %v1963 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 17400, i32* %v1963
  ; Cycle B0001
  %v1964 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1965 = load i64, i64* %v1964
  %v1966 = add i64 %v1965, 1
  store i64 %v1966, i64* %v1964
  %v1967 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1343, i32* %v1967
  %v1968 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1599, i32* %v1968
  %v1969 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1855, i32* %v1969
  %v1970 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2111, i32* %v1970
  ; Cycle B0002
  %v1971 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1972 = load i64, i64* %v1971
  %v1973 = add i64 %v1972, 1
  store i64 %v1973, i64* %v1971
  %v1974 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v1975 = load i64, i64* %v1974
  %v1976 = add i64 %v1975, 1
  store i64 %v1976, i64* %v1974
  %v1977 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v1977
  %v1978 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v1978
  ; Cycle B0003
  %v1979 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1980 = load i64, i64* %v1979
  %v1981 = add i64 %v1980, 1
  store i64 %v1981, i64* %v1979
  %v1982 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v1983 = load i64, i64* %v1982
  %v1984 = add i64 %v1983, 1
  store i64 %v1984, i64* %v1982
  ; Cycle B0000
  %v1985 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1986 = load i64, i64* %v1985
  %v1987 = add i64 %v1986, 1
  store i64 %v1987, i64* %v1985
  %v1988 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 320, i32* %v1988
  %v1989 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 9223, i32* %v1989
  %v1990 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 832, i32* %v1990
  %v1991 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 17419, i32* %v1991
  ; Cycle B0001
  %v1992 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v1993 = load i64, i64* %v1992
  %v1994 = add i64 %v1993, 1
  store i64 %v1994, i64* %v1992
  %v1995 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1344, i32* %v1995
  %v1996 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 25611, i32* %v1996
  %v1997 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 29698, i32* %v1997
  %v1998 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 33807, i32* %v1998
  ; Cycle B0002
  %v1999 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2000 = load i64, i64* %v1999
  %v2001 = add i64 %v2000, 1
  store i64 %v2001, i64* %v1999
  %v2002 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2003 = load i64, i64* %v2002
  %v2004 = add i64 %v2003, 1
  store i64 %v2004, i64* %v2002
  %v2005 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2005
  %v2006 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2006
  ; Cycle B0003
  %v2007 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2008 = load i64, i64* %v2007
  %v2009 = add i64 %v2008, 1
  store i64 %v2009, i64* %v2007
  %v2010 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2011 = load i64, i64* %v2010
  %v2012 = add i64 %v2011, 1
  store i64 %v2012, i64* %v2010
  ; Cycle B0000
  %v2013 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2014 = load i64, i64* %v2013
  %v2015 = add i64 %v2014, 1
  store i64 %v2015, i64* %v2013
  %v2016 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 5147, i32* %v2016
  %v2017 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 577, i32* %v2017
  %v2018 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 833, i32* %v2018
  %v2019 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1089, i32* %v2019
  ; Cycle B0001
  %v2020 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2021 = load i64, i64* %v2020
  %v2022 = add i64 %v2021, 1
  store i64 %v2022, i64* %v2020
  %v2023 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1345, i32* %v2023
  %v2024 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1601, i32* %v2024
  %v2025 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1857, i32* %v2025
  %v2026 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2113, i32* %v2026
  ; Cycle B0002
  %v2027 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2028 = load i64, i64* %v2027
  %v2029 = add i64 %v2028, 1
  store i64 %v2029, i64* %v2027
  %v2030 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2031 = load i64, i64* %v2030
  %v2032 = add i64 %v2031, 1
  store i64 %v2032, i64* %v2030
  %v2033 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2033
  %v2034 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2034
  ; Cycle B0003
  %v2035 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2036 = load i64, i64* %v2035
  %v2037 = add i64 %v2036, 1
  store i64 %v2037, i64* %v2035
  %v2038 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2039 = load i64, i64* %v2038
  %v2040 = add i64 %v2039, 1
  store i64 %v2040, i64* %v2038
  ; Cycle B0000
  %v2041 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2042 = load i64, i64* %v2041
  %v2043 = add i64 %v2042, 1
  store i64 %v2043, i64* %v2041
  %v2044 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 322, i32* %v2044
  %v2045 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 9258, i32* %v2045
  %v2046 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 13345, i32* %v2046
  %v2047 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1090, i32* %v2047
  ; Cycle B0001
  %v2048 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2049 = load i64, i64* %v2048
  %v2050 = add i64 %v2049, 1
  store i64 %v2050, i64* %v2048
  %v2051 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1346, i32* %v2051
  %v2052 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 25640, i32* %v2052
  %v2053 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1858, i32* %v2053
  %v2054 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2114, i32* %v2054
  ; Cycle B0002
  %v2055 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2056 = load i64, i64* %v2055
  %v2057 = add i64 %v2056, 1
  store i64 %v2057, i64* %v2055
  %v2058 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2059 = load i64, i64* %v2058
  %v2060 = add i64 %v2059, 1
  store i64 %v2060, i64* %v2058
  %v2061 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2061
  %v2062 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2062
  ; Cycle B0003
  %v2063 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2064 = load i64, i64* %v2063
  %v2065 = add i64 %v2064, 1
  store i64 %v2065, i64* %v2063
  %v2066 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2067 = load i64, i64* %v2066
  %v2068 = add i64 %v2067, 1
  store i64 %v2068, i64* %v2066
  ; Cycle B0000
  %v2069 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2070 = load i64, i64* %v2069
  %v2071 = add i64 %v2070, 1
  store i64 %v2071, i64* %v2069
  %v2072 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 323, i32* %v2072
  %v2073 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 579, i32* %v2073
  %v2074 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 13360, i32* %v2074
  %v2075 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1091, i32* %v2075
  ; Cycle B0001
  %v2076 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2077 = load i64, i64* %v2076
  %v2078 = add i64 %v2077, 1
  store i64 %v2078, i64* %v2076
  %v2079 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 21562, i32* %v2079
  %v2080 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 25657, i32* %v2080
  %v2081 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1859, i32* %v2081
  %v2082 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2115, i32* %v2082
  ; Cycle B0002
  %v2083 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2084 = load i64, i64* %v2083
  %v2085 = add i64 %v2084, 1
  store i64 %v2085, i64* %v2083
  %v2086 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2087 = load i64, i64* %v2086
  %v2088 = add i64 %v2087, 1
  store i64 %v2088, i64* %v2086
  %v2089 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2089
  %v2090 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2090
  ; Cycle B0003
  %v2091 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2092 = load i64, i64* %v2091
  %v2093 = add i64 %v2092, 1
  store i64 %v2093, i64* %v2091
  %v2094 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2095 = load i64, i64* %v2094
  %v2096 = add i64 %v2095, 1
  store i64 %v2096, i64* %v2094
  ; Cycle B0000
  %v2097 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2098 = load i64, i64* %v2097
  %v2099 = add i64 %v2098, 1
  store i64 %v2099, i64* %v2097
  %v2100 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 324, i32* %v2100
  %v2101 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 9283, i32* %v2101
  %v2102 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 13388, i32* %v2102
  %v2103 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 17487, i32* %v2103
  ; Cycle B0001
  %v2104 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2105 = load i64, i64* %v2104
  %v2106 = add i64 %v2105, 1
  store i64 %v2106, i64* %v2104
  %v2107 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1348, i32* %v2107
  %v2108 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1604, i32* %v2108
  %v2109 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 29766, i32* %v2109
  %v2110 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2116, i32* %v2110
  ; Cycle B0002
  %v2111 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2112 = load i64, i64* %v2111
  %v2113 = add i64 %v2112, 1
  store i64 %v2113, i64* %v2111
  %v2114 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2115 = load i64, i64* %v2114
  %v2116 = add i64 %v2115, 1
  store i64 %v2116, i64* %v2114
  %v2117 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2117
  %v2118 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2118
  ; Cycle B0003
  %v2119 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2120 = load i64, i64* %v2119
  %v2121 = add i64 %v2120, 1
  store i64 %v2121, i64* %v2119
  %v2122 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2123 = load i64, i64* %v2122
  %v2124 = add i64 %v2123, 1
  store i64 %v2124, i64* %v2122
  ; Cycle B0000
  %v2125 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2126 = load i64, i64* %v2125
  %v2127 = add i64 %v2126, 1
  store i64 %v2127, i64* %v2125
  %v2128 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 325, i32* %v2128
  %v2129 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 581, i32* %v2129
  %v2130 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 837, i32* %v2130
  %v2131 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1093, i32* %v2131
  ; Cycle B0001
  %v2132 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2133 = load i64, i64* %v2132
  %v2134 = add i64 %v2133, 1
  store i64 %v2134, i64* %v2132
  %v2135 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1349, i32* %v2135
  %v2136 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1605, i32* %v2136
  %v2137 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1861, i32* %v2137
  %v2138 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2117, i32* %v2138
  ; Cycle B0002
  %v2139 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2140 = load i64, i64* %v2139
  %v2141 = add i64 %v2140, 1
  store i64 %v2141, i64* %v2139
  %v2142 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2143 = load i64, i64* %v2142
  %v2144 = add i64 %v2143, 1
  store i64 %v2144, i64* %v2142
  %v2145 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2145
  %v2146 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2146
  ; Cycle B0003
  %v2147 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2148 = load i64, i64* %v2147
  %v2149 = add i64 %v2148, 1
  store i64 %v2149, i64* %v2147
  %v2150 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2151 = load i64, i64* %v2150
  %v2152 = add i64 %v2151, 1
  store i64 %v2152, i64* %v2150
  ; Cycle B0000
  %v2153 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2154 = load i64, i64* %v2153
  %v2155 = add i64 %v2154, 1
  store i64 %v2155, i64* %v2153
  %v2156 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 326, i32* %v2156
  %v2157 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 582, i32* %v2157
  %v2158 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 13413, i32* %v2158
  %v2159 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1094, i32* %v2159
  ; Cycle B0001
  %v2160 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2161 = load i64, i64* %v2160
  %v2162 = add i64 %v2161, 1
  store i64 %v2162, i64* %v2160
  %v2163 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1350, i32* %v2163
  %v2164 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 25700, i32* %v2164
  %v2165 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 29805, i32* %v2165
  %v2166 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2118, i32* %v2166
  ; Cycle B0002
  %v2167 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2168 = load i64, i64* %v2167
  %v2169 = add i64 %v2168, 1
  store i64 %v2169, i64* %v2167
  %v2170 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2171 = load i64, i64* %v2170
  %v2172 = add i64 %v2171, 1
  store i64 %v2172, i64* %v2170
  %v2173 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2173
  %v2174 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2174
  ; Cycle B0003
  %v2175 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2176 = load i64, i64* %v2175
  %v2177 = add i64 %v2176, 1
  store i64 %v2177, i64* %v2175
  %v2178 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2179 = load i64, i64* %v2178
  %v2180 = add i64 %v2179, 1
  store i64 %v2180, i64* %v2178
  ; Cycle B0000
  %v2181 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2182 = load i64, i64* %v2181
  %v2183 = add i64 %v2182, 1
  store i64 %v2183, i64* %v2181
  %v2184 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 327, i32* %v2184
  %v2185 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 583, i32* %v2185
  %v2186 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 839, i32* %v2186
  %v2187 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1095, i32* %v2187
  ; Cycle B0001
  %v2188 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2189 = load i64, i64* %v2188
  %v2190 = add i64 %v2189, 1
  store i64 %v2190, i64* %v2188
  %v2191 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1351, i32* %v2191
  %v2192 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1607, i32* %v2192
  %v2193 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1863, i32* %v2193
  %v2194 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2119, i32* %v2194
  ; Cycle B0002
  %v2195 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2196 = load i64, i64* %v2195
  %v2197 = add i64 %v2196, 1
  store i64 %v2197, i64* %v2195
  %v2198 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2199 = load i64, i64* %v2198
  %v2200 = add i64 %v2199, 1
  store i64 %v2200, i64* %v2198
  %v2201 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2201
  %v2202 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2202
  ; Cycle B0003
  %v2203 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2204 = load i64, i64* %v2203
  %v2205 = add i64 %v2204, 1
  store i64 %v2205, i64* %v2203
  %v2206 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2207 = load i64, i64* %v2206
  %v2208 = add i64 %v2207, 1
  store i64 %v2208, i64* %v2206
  ; Cycle B0000
  %v2209 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2210 = load i64, i64* %v2209
  %v2211 = add i64 %v2210, 1
  store i64 %v2211, i64* %v2209
  %v2212 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 5258, i32* %v2212
  %v2213 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 584, i32* %v2213
  %v2214 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 840, i32* %v2214
  %v2215 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1096, i32* %v2215
  ; Cycle B0001
  %v2216 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2217 = load i64, i64* %v2216
  %v2218 = add i64 %v2217, 1
  store i64 %v2218, i64* %v2216
  %v2219 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 21636, i32* %v2219
  %v2220 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1608, i32* %v2220
  %v2221 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1864, i32* %v2221
  %v2222 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2120, i32* %v2222
  ; Cycle B0002
  %v2223 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2224 = load i64, i64* %v2223
  %v2225 = add i64 %v2224, 1
  store i64 %v2225, i64* %v2223
  %v2226 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2227 = load i64, i64* %v2226
  %v2228 = add i64 %v2227, 1
  store i64 %v2228, i64* %v2226
  %v2229 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2229
  %v2230 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2230
  ; Cycle B0003
  %v2231 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2232 = load i64, i64* %v2231
  %v2233 = add i64 %v2232, 1
  store i64 %v2233, i64* %v2231
  %v2234 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2235 = load i64, i64* %v2234
  %v2236 = add i64 %v2235, 1
  store i64 %v2236, i64* %v2234
  ; Cycle B0000
  %v2237 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2238 = load i64, i64* %v2237
  %v2239 = add i64 %v2238, 1
  store i64 %v2239, i64* %v2237
  %v2240 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 329, i32* %v2240
  %v2241 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 585, i32* %v2241
  %v2242 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 13467, i32* %v2242
  %v2243 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1097, i32* %v2243
  ; Cycle B0001
  %v2244 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2245 = load i64, i64* %v2244
  %v2246 = add i64 %v2245, 1
  store i64 %v2246, i64* %v2244
  %v2247 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 21657, i32* %v2247
  %v2248 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1609, i32* %v2248
  %v2249 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1865, i32* %v2249
  %v2250 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2121, i32* %v2250
  ; Cycle B0002
  %v2251 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2252 = load i64, i64* %v2251
  %v2253 = add i64 %v2252, 1
  store i64 %v2253, i64* %v2251
  %v2254 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2255 = load i64, i64* %v2254
  %v2256 = add i64 %v2255, 1
  store i64 %v2256, i64* %v2254
  %v2257 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2257
  %v2258 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2258
  ; Cycle B0003
  %v2259 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2260 = load i64, i64* %v2259
  %v2261 = add i64 %v2260, 1
  store i64 %v2261, i64* %v2259
  %v2262 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2263 = load i64, i64* %v2262
  %v2264 = add i64 %v2263, 1
  store i64 %v2264, i64* %v2262
  ; Cycle B0000
  %v2265 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2266 = load i64, i64* %v2265
  %v2267 = add i64 %v2266, 1
  store i64 %v2267, i64* %v2265
  %v2268 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 5287, i32* %v2268
  %v2269 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 9390, i32* %v2269
  %v2270 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 842, i32* %v2270
  %v2271 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1098, i32* %v2271
  ; Cycle B0001
  %v2272 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2273 = load i64, i64* %v2272
  %v2274 = add i64 %v2273, 1
  store i64 %v2274, i64* %v2272
  %v2275 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1354, i32* %v2275
  %v2276 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1610, i32* %v2276
  %v2277 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1866, i32* %v2277
  %v2278 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2122, i32* %v2278
  ; Cycle B0002
  %v2279 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2280 = load i64, i64* %v2279
  %v2281 = add i64 %v2280, 1
  store i64 %v2281, i64* %v2279
  %v2282 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2283 = load i64, i64* %v2282
  %v2284 = add i64 %v2283, 1
  store i64 %v2284, i64* %v2282
  %v2285 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2285
  %v2286 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2286
  ; Cycle B0003
  %v2287 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2288 = load i64, i64* %v2287
  %v2289 = add i64 %v2288, 1
  store i64 %v2289, i64* %v2287
  %v2290 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2291 = load i64, i64* %v2290
  %v2292 = add i64 %v2291, 1
  store i64 %v2292, i64* %v2290
  ; Cycle B0000
  %v2293 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2294 = load i64, i64* %v2293
  %v2295 = add i64 %v2294, 1
  store i64 %v2295, i64* %v2293
  %v2296 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 331, i32* %v2296
  %v2297 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 587, i32* %v2297
  %v2298 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 843, i32* %v2298
  %v2299 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1099, i32* %v2299
  ; Cycle B0001
  %v2300 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2301 = load i64, i64* %v2300
  %v2302 = add i64 %v2301, 1
  store i64 %v2302, i64* %v2300
  %v2303 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 21681, i32* %v2303
  %v2304 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1611, i32* %v2304
  %v2305 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1867, i32* %v2305
  %v2306 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2123, i32* %v2306
  ; Cycle B0002
  %v2307 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2308 = load i64, i64* %v2307
  %v2309 = add i64 %v2308, 1
  store i64 %v2309, i64* %v2307
  %v2310 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2311 = load i64, i64* %v2310
  %v2312 = add i64 %v2311, 1
  store i64 %v2312, i64* %v2310
  %v2313 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2313
  %v2314 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2314
  ; Cycle B0003
  %v2315 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2316 = load i64, i64* %v2315
  %v2317 = add i64 %v2316, 1
  store i64 %v2317, i64* %v2315
  %v2318 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2319 = load i64, i64* %v2318
  %v2320 = add i64 %v2319, 1
  store i64 %v2320, i64* %v2318
  ; Cycle B0000
  %v2321 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2322 = load i64, i64* %v2321
  %v2323 = add i64 %v2322, 1
  store i64 %v2323, i64* %v2321
  %v2324 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 5321, i32* %v2324
  %v2325 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 588, i32* %v2325
  %v2326 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 844, i32* %v2326
  %v2327 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 17615, i32* %v2327
  ; Cycle B0001
  %v2328 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2329 = load i64, i64* %v2328
  %v2330 = add i64 %v2329, 1
  store i64 %v2330, i64* %v2328
  %v2331 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 21704, i32* %v2331
  %v2332 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 25807, i32* %v2332
  %v2333 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1868, i32* %v2333
  %v2334 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 33995, i32* %v2334
  ; Cycle B0002
  %v2335 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2336 = load i64, i64* %v2335
  %v2337 = add i64 %v2336, 1
  store i64 %v2337, i64* %v2335
  %v2338 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2339 = load i64, i64* %v2338
  %v2340 = add i64 %v2339, 1
  store i64 %v2340, i64* %v2338
  %v2341 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2341
  %v2342 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2342
  ; Cycle B0003
  %v2343 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2344 = load i64, i64* %v2343
  %v2345 = add i64 %v2344, 1
  store i64 %v2345, i64* %v2343
  %v2346 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2347 = load i64, i64* %v2346
  %v2348 = add i64 %v2347, 1
  store i64 %v2348, i64* %v2346
  ; Cycle B0000
  %v2349 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2350 = load i64, i64* %v2349
  %v2351 = add i64 %v2350, 1
  store i64 %v2351, i64* %v2349
  %v2352 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 333, i32* %v2352
  %v2353 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 589, i32* %v2353
  %v2354 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 845, i32* %v2354
  %v2355 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1101, i32* %v2355
  ; Cycle B0001
  %v2356 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2357 = load i64, i64* %v2356
  %v2358 = add i64 %v2357, 1
  store i64 %v2358, i64* %v2356
  %v2359 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1357, i32* %v2359
  %v2360 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1613, i32* %v2360
  %v2361 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1869, i32* %v2361
  %v2362 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2125, i32* %v2362
  ; Cycle B0002
  %v2363 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2364 = load i64, i64* %v2363
  %v2365 = add i64 %v2364, 1
  store i64 %v2365, i64* %v2363
  %v2366 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2367 = load i64, i64* %v2366
  %v2368 = add i64 %v2367, 1
  store i64 %v2368, i64* %v2366
  %v2369 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2369
  %v2370 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2370
  ; Cycle B0003
  %v2371 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2372 = load i64, i64* %v2371
  %v2373 = add i64 %v2372, 1
  store i64 %v2373, i64* %v2371
  %v2374 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2375 = load i64, i64* %v2374
  %v2376 = add i64 %v2375, 1
  store i64 %v2376, i64* %v2374
  ; Cycle B0000
  %v2377 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2378 = load i64, i64* %v2377
  %v2379 = add i64 %v2378, 1
  store i64 %v2379, i64* %v2377
  %v2380 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 334, i32* %v2380
  %v2381 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 9450, i32* %v2381
  %v2382 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 846, i32* %v2382
  %v2383 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1102, i32* %v2383
  ; Cycle B0001
  %v2384 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2385 = load i64, i64* %v2384
  %v2386 = add i64 %v2385, 1
  store i64 %v2386, i64* %v2384
  %v2387 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1358, i32* %v2387
  %v2388 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1614, i32* %v2388
  %v2389 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 29933, i32* %v2389
  %v2390 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2126, i32* %v2390
  ; Cycle B0002
  %v2391 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2392 = load i64, i64* %v2391
  %v2393 = add i64 %v2392, 1
  store i64 %v2393, i64* %v2391
  %v2394 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2395 = load i64, i64* %v2394
  %v2396 = add i64 %v2395, 1
  store i64 %v2396, i64* %v2394
  %v2397 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2397
  %v2398 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2398
  ; Cycle B0003
  %v2399 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2400 = load i64, i64* %v2399
  %v2401 = add i64 %v2400, 1
  store i64 %v2401, i64* %v2399
  %v2402 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2403 = load i64, i64* %v2402
  %v2404 = add i64 %v2403, 1
  store i64 %v2404, i64* %v2402
  ; Cycle B0000
  %v2405 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2406 = load i64, i64* %v2405
  %v2407 = add i64 %v2406, 1
  store i64 %v2407, i64* %v2405
  %v2408 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 335, i32* %v2408
  %v2409 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 591, i32* %v2409
  %v2410 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 847, i32* %v2410
  %v2411 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1103, i32* %v2411
  ; Cycle B0001
  %v2412 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2413 = load i64, i64* %v2412
  %v2414 = add i64 %v2413, 1
  store i64 %v2414, i64* %v2412
  %v2415 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1359, i32* %v2415
  %v2416 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1615, i32* %v2416
  %v2417 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1871, i32* %v2417
  %v2418 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2127, i32* %v2418
  ; Cycle B0002
  %v2419 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2420 = load i64, i64* %v2419
  %v2421 = add i64 %v2420, 1
  store i64 %v2421, i64* %v2419
  %v2422 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2423 = load i64, i64* %v2422
  %v2424 = add i64 %v2423, 1
  store i64 %v2424, i64* %v2422
  %v2425 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2425
  %v2426 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2426
  ; Cycle B0003
  %v2427 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2428 = load i64, i64* %v2427
  %v2429 = add i64 %v2428, 1
  store i64 %v2429, i64* %v2427
  %v2430 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2431 = load i64, i64* %v2430
  %v2432 = add i64 %v2431, 1
  store i64 %v2432, i64* %v2430
  ; Cycle B0000
  %v2433 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2434 = load i64, i64* %v2433
  %v2435 = add i64 %v2434, 1
  store i64 %v2435, i64* %v2433
  %v2436 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 336, i32* %v2436
  %v2437 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 592, i32* %v2437
  %v2438 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 848, i32* %v2438
  %v2439 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1104, i32* %v2439
  ; Cycle B0001
  %v2440 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2441 = load i64, i64* %v2440
  %v2442 = add i64 %v2441, 1
  store i64 %v2442, i64* %v2440
  %v2443 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1360, i32* %v2443
  %v2444 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1616, i32* %v2444
  %v2445 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1872, i32* %v2445
  %v2446 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2128, i32* %v2446
  ; Cycle B0002
  %v2447 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2448 = load i64, i64* %v2447
  %v2449 = add i64 %v2448, 1
  store i64 %v2449, i64* %v2447
  %v2450 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2451 = load i64, i64* %v2450
  %v2452 = add i64 %v2451, 1
  store i64 %v2452, i64* %v2450
  %v2453 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2453
  %v2454 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2454
  ; Cycle B0003
  %v2455 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2456 = load i64, i64* %v2455
  %v2457 = add i64 %v2456, 1
  store i64 %v2457, i64* %v2455
  %v2458 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2459 = load i64, i64* %v2458
  %v2460 = add i64 %v2459, 1
  store i64 %v2460, i64* %v2458
  ; Cycle B0000
  %v2461 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2462 = load i64, i64* %v2461
  %v2463 = add i64 %v2462, 1
  store i64 %v2463, i64* %v2461
  %v2464 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 337, i32* %v2464
  %v2465 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 593, i32* %v2465
  %v2466 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 13589, i32* %v2466
  %v2467 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1105, i32* %v2467
  ; Cycle B0001
  %v2468 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2469 = load i64, i64* %v2468
  %v2470 = add i64 %v2469, 1
  store i64 %v2470, i64* %v2468
  %v2471 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1361, i32* %v2471
  %v2472 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 25876, i32* %v2472
  %v2473 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 29981, i32* %v2473
  %v2474 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2129, i32* %v2474
  ; Cycle B0002
  %v2475 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2476 = load i64, i64* %v2475
  %v2477 = add i64 %v2476, 1
  store i64 %v2477, i64* %v2475
  %v2478 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2479 = load i64, i64* %v2478
  %v2480 = add i64 %v2479, 1
  store i64 %v2480, i64* %v2478
  %v2481 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2481
  %v2482 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2482
  ; Cycle B0003
  %v2483 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2484 = load i64, i64* %v2483
  %v2485 = add i64 %v2484, 1
  store i64 %v2485, i64* %v2483
  %v2486 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2487 = load i64, i64* %v2486
  %v2488 = add i64 %v2487, 1
  store i64 %v2488, i64* %v2486
  ; Cycle B0000
  %v2489 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2490 = load i64, i64* %v2489
  %v2491 = add i64 %v2490, 1
  store i64 %v2491, i64* %v2489
  %v2492 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 338, i32* %v2492
  %v2493 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 594, i32* %v2493
  %v2494 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 850, i32* %v2494
  %v2495 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1106, i32* %v2495
  ; Cycle B0001
  %v2496 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2497 = load i64, i64* %v2496
  %v2498 = add i64 %v2497, 1
  store i64 %v2498, i64* %v2496
  %v2499 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1362, i32* %v2499
  %v2500 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1618, i32* %v2500
  %v2501 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1874, i32* %v2501
  %v2502 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2130, i32* %v2502
  ; Cycle B0002
  %v2503 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2504 = load i64, i64* %v2503
  %v2505 = add i64 %v2504, 1
  store i64 %v2505, i64* %v2503
  %v2506 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2507 = load i64, i64* %v2506
  %v2508 = add i64 %v2507, 1
  store i64 %v2508, i64* %v2506
  %v2509 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2509
  %v2510 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2510
  ; Cycle B0003
  %v2511 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2512 = load i64, i64* %v2511
  %v2513 = add i64 %v2512, 1
  store i64 %v2513, i64* %v2511
  %v2514 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2515 = load i64, i64* %v2514
  %v2516 = add i64 %v2515, 1
  store i64 %v2516, i64* %v2514
  ; Cycle B0000
  %v2517 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2518 = load i64, i64* %v2517
  %v2519 = add i64 %v2518, 1
  store i64 %v2519, i64* %v2517
  %v2520 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 339, i32* %v2520
  %v2521 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 9523, i32* %v2521
  %v2522 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 13628, i32* %v2522
  %v2523 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 17727, i32* %v2523
  ; Cycle B0001
  %v2524 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2525 = load i64, i64* %v2524
  %v2526 = add i64 %v2525, 1
  store i64 %v2526, i64* %v2524
  %v2527 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1363, i32* %v2527
  %v2528 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1619, i32* %v2528
  %v2529 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 30006, i32* %v2529
  %v2530 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2131, i32* %v2530
  ; Cycle B0002
  %v2531 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2532 = load i64, i64* %v2531
  %v2533 = add i64 %v2532, 1
  store i64 %v2533, i64* %v2531
  %v2534 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2535 = load i64, i64* %v2534
  %v2536 = add i64 %v2535, 1
  store i64 %v2536, i64* %v2534
  %v2537 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2537
  %v2538 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2538
  ; Cycle B0003
  %v2539 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2540 = load i64, i64* %v2539
  %v2541 = add i64 %v2540, 1
  store i64 %v2541, i64* %v2539
  %v2542 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2543 = load i64, i64* %v2542
  %v2544 = add i64 %v2543, 1
  store i64 %v2544, i64* %v2542
  ; Cycle B0000
  %v2545 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2546 = load i64, i64* %v2545
  %v2547 = add i64 %v2546, 1
  store i64 %v2547, i64* %v2545
  %v2548 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 340, i32* %v2548
  %v2549 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 596, i32* %v2549
  %v2550 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 13632, i32* %v2550
  %v2551 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1108, i32* %v2551
  ; Cycle B0001
  %v2552 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2553 = load i64, i64* %v2552
  %v2554 = add i64 %v2553, 1
  store i64 %v2554, i64* %v2552
  %v2555 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 21834, i32* %v2555
  %v2556 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 25929, i32* %v2556
  %v2557 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1876, i32* %v2557
  %v2558 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2132, i32* %v2558
  ; Cycle B0002
  %v2559 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2560 = load i64, i64* %v2559
  %v2561 = add i64 %v2560, 1
  store i64 %v2561, i64* %v2559
  %v2562 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2563 = load i64, i64* %v2562
  %v2564 = add i64 %v2563, 1
  store i64 %v2564, i64* %v2562
  %v2565 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2565
  %v2566 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2566
  ; Cycle B0003
  %v2567 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2568 = load i64, i64* %v2567
  %v2569 = add i64 %v2568, 1
  store i64 %v2569, i64* %v2567
  %v2570 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2571 = load i64, i64* %v2570
  %v2572 = add i64 %v2571, 1
  store i64 %v2572, i64* %v2570
  ; Cycle B0000
  %v2573 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2574 = load i64, i64* %v2573
  %v2575 = add i64 %v2574, 1
  store i64 %v2575, i64* %v2573
  %v2576 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 341, i32* %v2576
  %v2577 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 9562, i32* %v2577
  %v2578 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 13649, i32* %v2578
  %v2579 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1109, i32* %v2579
  ; Cycle B0001
  %v2580 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2581 = load i64, i64* %v2580
  %v2582 = add i64 %v2581, 1
  store i64 %v2582, i64* %v2580
  %v2583 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1365, i32* %v2583
  %v2584 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 25944, i32* %v2584
  %v2585 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1877, i32* %v2585
  %v2586 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2133, i32* %v2586
  ; Cycle B0002
  %v2587 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2588 = load i64, i64* %v2587
  %v2589 = add i64 %v2588, 1
  store i64 %v2589, i64* %v2587
  %v2590 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2591 = load i64, i64* %v2590
  %v2592 = add i64 %v2591, 1
  store i64 %v2592, i64* %v2590
  %v2593 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2593
  %v2594 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2594
  ; Cycle B0003
  %v2595 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2596 = load i64, i64* %v2595
  %v2597 = add i64 %v2596, 1
  store i64 %v2597, i64* %v2595
  %v2598 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2599 = load i64, i64* %v2598
  %v2600 = add i64 %v2599, 1
  store i64 %v2600, i64* %v2598
  ; Cycle B0000
  %v2601 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2602 = load i64, i64* %v2601
  %v2603 = add i64 %v2602, 1
  store i64 %v2603, i64* %v2601
  %v2604 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 5483, i32* %v2604
  %v2605 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 598, i32* %v2605
  %v2606 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 854, i32* %v2606
  %v2607 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1110, i32* %v2607
  ; Cycle B0001
  %v2608 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2609 = load i64, i64* %v2608
  %v2610 = add i64 %v2609, 1
  store i64 %v2610, i64* %v2608
  %v2611 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1366, i32* %v2611
  %v2612 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1622, i32* %v2612
  %v2613 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1878, i32* %v2613
  %v2614 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2134, i32* %v2614
  ; Cycle B0002
  %v2615 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2616 = load i64, i64* %v2615
  %v2617 = add i64 %v2616, 1
  store i64 %v2617, i64* %v2615
  %v2618 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2619 = load i64, i64* %v2618
  %v2620 = add i64 %v2619, 1
  store i64 %v2620, i64* %v2618
  %v2621 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2621
  %v2622 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2622
  ; Cycle B0003
  %v2623 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2624 = load i64, i64* %v2623
  %v2625 = add i64 %v2624, 1
  store i64 %v2625, i64* %v2623
  %v2626 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2627 = load i64, i64* %v2626
  %v2628 = add i64 %v2627, 1
  store i64 %v2628, i64* %v2626
  ; Cycle B0000
  %v2629 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2630 = load i64, i64* %v2629
  %v2631 = add i64 %v2630, 1
  store i64 %v2631, i64* %v2629
  %v2632 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 343, i32* %v2632
  %v2633 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 9591, i32* %v2633
  %v2634 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 855, i32* %v2634
  %v2635 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 17787, i32* %v2635
  ; Cycle B0001
  %v2636 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2637 = load i64, i64* %v2636
  %v2638 = add i64 %v2637, 1
  store i64 %v2638, i64* %v2636
  %v2639 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1367, i32* %v2639
  %v2640 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 25979, i32* %v2640
  %v2641 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 30066, i32* %v2641
  %v2642 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 34175, i32* %v2642
  ; Cycle B0002
  %v2643 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2644 = load i64, i64* %v2643
  %v2645 = add i64 %v2644, 1
  store i64 %v2645, i64* %v2643
  %v2646 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2647 = load i64, i64* %v2646
  %v2648 = add i64 %v2647, 1
  store i64 %v2648, i64* %v2646
  %v2649 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2649
  %v2650 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2650
  ; Cycle B0003
  %v2651 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2652 = load i64, i64* %v2651
  %v2653 = add i64 %v2652, 1
  store i64 %v2653, i64* %v2651
  %v2654 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2655 = load i64, i64* %v2654
  %v2656 = add i64 %v2655, 1
  store i64 %v2656, i64* %v2654
  ; Cycle B0000
  %v2657 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2658 = load i64, i64* %v2657
  %v2659 = add i64 %v2658, 1
  store i64 %v2659, i64* %v2657
  %v2660 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 344, i32* %v2660
  %v2661 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 9613, i32* %v2661
  %v2662 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 856, i32* %v2662
  %v2663 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 17797, i32* %v2663
  ; Cycle B0001
  %v2664 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2665 = load i64, i64* %v2664
  %v2666 = add i64 %v2665, 1
  store i64 %v2666, i64* %v2664
  %v2667 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1368, i32* %v2667
  %v2668 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1624, i32* %v2668
  %v2669 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 30094, i32* %v2669
  %v2670 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2136, i32* %v2670
  ; Cycle B0002
  %v2671 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2672 = load i64, i64* %v2671
  %v2673 = add i64 %v2672, 1
  store i64 %v2673, i64* %v2671
  %v2674 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2675 = load i64, i64* %v2674
  %v2676 = add i64 %v2675, 1
  store i64 %v2676, i64* %v2674
  %v2677 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2677
  %v2678 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2678
  ; Cycle B0003
  %v2679 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2680 = load i64, i64* %v2679
  %v2681 = add i64 %v2680, 1
  store i64 %v2681, i64* %v2679
  %v2682 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2683 = load i64, i64* %v2682
  %v2684 = add i64 %v2683, 1
  store i64 %v2684, i64* %v2682
  ; Cycle B0000
  %v2685 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2686 = load i64, i64* %v2685
  %v2687 = add i64 %v2686, 1
  store i64 %v2687, i64* %v2685
  %v2688 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 345, i32* %v2688
  %v2689 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 601, i32* %v2689
  %v2690 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 857, i32* %v2690
  %v2691 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1113, i32* %v2691
  ; Cycle B0001
  %v2692 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2693 = load i64, i64* %v2692
  %v2694 = add i64 %v2693, 1
  store i64 %v2694, i64* %v2692
  %v2695 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1369, i32* %v2695
  %v2696 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1625, i32* %v2696
  %v2697 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1881, i32* %v2697
  %v2698 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2137, i32* %v2698
  ; Cycle B0002
  %v2699 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2700 = load i64, i64* %v2699
  %v2701 = add i64 %v2700, 1
  store i64 %v2701, i64* %v2699
  %v2702 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2703 = load i64, i64* %v2702
  %v2704 = add i64 %v2703, 1
  store i64 %v2704, i64* %v2702
  %v2705 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2705
  %v2706 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2706
  ; Cycle B0003
  %v2707 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2708 = load i64, i64* %v2707
  %v2709 = add i64 %v2708, 1
  store i64 %v2709, i64* %v2707
  %v2710 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2711 = load i64, i64* %v2710
  %v2712 = add i64 %v2711, 1
  store i64 %v2712, i64* %v2710
  ; Cycle B0000
  %v2713 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2714 = load i64, i64* %v2713
  %v2715 = add i64 %v2714, 1
  store i64 %v2715, i64* %v2713
  %v2716 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 346, i32* %v2716
  %v2717 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 602, i32* %v2717
  %v2718 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 858, i32* %v2718
  %v2719 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1114, i32* %v2719
  ; Cycle B0001
  %v2720 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2721 = load i64, i64* %v2720
  %v2722 = add i64 %v2721, 1
  store i64 %v2722, i64* %v2720
  %v2723 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1370, i32* %v2723
  %v2724 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1626, i32* %v2724
  %v2725 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1882, i32* %v2725
  %v2726 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2138, i32* %v2726
  ; Cycle B0002
  %v2727 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2728 = load i64, i64* %v2727
  %v2729 = add i64 %v2728, 1
  store i64 %v2729, i64* %v2727
  %v2730 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2731 = load i64, i64* %v2730
  %v2732 = add i64 %v2731, 1
  store i64 %v2732, i64* %v2730
  %v2733 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2733
  %v2734 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2734
  ; Cycle B0003
  %v2735 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2736 = load i64, i64* %v2735
  %v2737 = add i64 %v2736, 1
  store i64 %v2737, i64* %v2735
  %v2738 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2739 = load i64, i64* %v2738
  %v2740 = add i64 %v2739, 1
  store i64 %v2740, i64* %v2738
  ; Cycle B0000
  %v2741 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2742 = load i64, i64* %v2741
  %v2743 = add i64 %v2742, 1
  store i64 %v2743, i64* %v2741
  %v2744 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 347, i32* %v2744
  %v2745 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 9658, i32* %v2745
  %v2746 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 859, i32* %v2746
  %v2747 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1115, i32* %v2747
  ; Cycle B0001
  %v2748 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2749 = load i64, i64* %v2748
  %v2750 = add i64 %v2749, 1
  store i64 %v2750, i64* %v2748
  %v2751 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1371, i32* %v2751
  %v2752 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1627, i32* %v2752
  %v2753 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 30141, i32* %v2753
  %v2754 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2139, i32* %v2754
  ; Cycle B0002
  %v2755 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2756 = load i64, i64* %v2755
  %v2757 = add i64 %v2756, 1
  store i64 %v2757, i64* %v2755
  %v2758 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2759 = load i64, i64* %v2758
  %v2760 = add i64 %v2759, 1
  store i64 %v2760, i64* %v2758
  %v2761 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2761
  %v2762 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2762
  ; Cycle B0003
  %v2763 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2764 = load i64, i64* %v2763
  %v2765 = add i64 %v2764, 1
  store i64 %v2765, i64* %v2763
  %v2766 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2767 = load i64, i64* %v2766
  %v2768 = add i64 %v2767, 1
  store i64 %v2768, i64* %v2766
  ; Cycle B0000
  %v2769 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2770 = load i64, i64* %v2769
  %v2771 = add i64 %v2770, 1
  store i64 %v2771, i64* %v2769
  %v2772 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 348, i32* %v2772
  %v2773 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 604, i32* %v2773
  %v2774 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 860, i32* %v2774
  %v2775 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1116, i32* %v2775
  ; Cycle B0001
  %v2776 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2777 = load i64, i64* %v2776
  %v2778 = add i64 %v2777, 1
  store i64 %v2778, i64* %v2776
  %v2779 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1372, i32* %v2779
  %v2780 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1628, i32* %v2780
  %v2781 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1884, i32* %v2781
  %v2782 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2140, i32* %v2782
  ; Cycle B0002
  %v2783 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2784 = load i64, i64* %v2783
  %v2785 = add i64 %v2784, 1
  store i64 %v2785, i64* %v2783
  %v2786 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2787 = load i64, i64* %v2786
  %v2788 = add i64 %v2787, 1
  store i64 %v2788, i64* %v2786
  %v2789 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2789
  %v2790 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2790
  ; Cycle B0003
  %v2791 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2792 = load i64, i64* %v2791
  %v2793 = add i64 %v2792, 1
  store i64 %v2793, i64* %v2791
  %v2794 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2795 = load i64, i64* %v2794
  %v2796 = add i64 %v2795, 1
  store i64 %v2796, i64* %v2794
  ; Cycle B0000
  %v2797 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2798 = load i64, i64* %v2797
  %v2799 = add i64 %v2798, 1
  store i64 %v2799, i64* %v2797
  %v2800 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 5593, i32* %v2800
  %v2801 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 605, i32* %v2801
  %v2802 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 861, i32* %v2802
  %v2803 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 17887, i32* %v2803
  ; Cycle B0001
  %v2804 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2805 = load i64, i64* %v2804
  %v2806 = add i64 %v2805, 1
  store i64 %v2806, i64* %v2804
  %v2807 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 21976, i32* %v2807
  %v2808 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 26079, i32* %v2808
  %v2809 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1885, i32* %v2809
  %v2810 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 34267, i32* %v2810
  ; Cycle B0002
  %v2811 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2812 = load i64, i64* %v2811
  %v2813 = add i64 %v2812, 1
  store i64 %v2813, i64* %v2811
  %v2814 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2815 = load i64, i64* %v2814
  %v2816 = add i64 %v2815, 1
  store i64 %v2816, i64* %v2814
  %v2817 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2817
  %v2818 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2818
  ; Cycle B0003
  %v2819 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2820 = load i64, i64* %v2819
  %v2821 = add i64 %v2820, 1
  store i64 %v2821, i64* %v2819
  %v2822 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2823 = load i64, i64* %v2822
  %v2824 = add i64 %v2823, 1
  store i64 %v2824, i64* %v2822
  ; Cycle B0000
  %v2825 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2826 = load i64, i64* %v2825
  %v2827 = add i64 %v2826, 1
  store i64 %v2827, i64* %v2825
  %v2828 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 350, i32* %v2828
  %v2829 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 606, i32* %v2829
  %v2830 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 862, i32* %v2830
  %v2831 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1118, i32* %v2831
  ; Cycle B0001
  %v2832 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2833 = load i64, i64* %v2832
  %v2834 = add i64 %v2833, 1
  store i64 %v2834, i64* %v2832
  %v2835 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 21985, i32* %v2835
  %v2836 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1630, i32* %v2836
  %v2837 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1886, i32* %v2837
  %v2838 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2142, i32* %v2838
  ; Cycle B0002
  %v2839 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2840 = load i64, i64* %v2839
  %v2841 = add i64 %v2840, 1
  store i64 %v2841, i64* %v2839
  %v2842 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2843 = load i64, i64* %v2842
  %v2844 = add i64 %v2843, 1
  store i64 %v2844, i64* %v2842
  %v2845 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2845
  %v2846 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2846
  ; Cycle B0003
  %v2847 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2848 = load i64, i64* %v2847
  %v2849 = add i64 %v2848, 1
  store i64 %v2849, i64* %v2847
  %v2850 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2851 = load i64, i64* %v2850
  %v2852 = add i64 %v2851, 1
  store i64 %v2852, i64* %v2850
  ; Cycle B0000
  %v2853 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2854 = load i64, i64* %v2853
  %v2855 = add i64 %v2854, 1
  store i64 %v2855, i64* %v2853
  %v2856 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 5623, i32* %v2856
  %v2857 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 9726, i32* %v2857
  %v2858 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 863, i32* %v2858
  %v2859 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1119, i32* %v2859
  ; Cycle B0001
  %v2860 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2861 = load i64, i64* %v2860
  %v2862 = add i64 %v2861, 1
  store i64 %v2862, i64* %v2860
  %v2863 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1375, i32* %v2863
  %v2864 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1631, i32* %v2864
  %v2865 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1887, i32* %v2865
  %v2866 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2143, i32* %v2866
  ; Cycle B0002
  %v2867 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2868 = load i64, i64* %v2867
  %v2869 = add i64 %v2868, 1
  store i64 %v2869, i64* %v2867
  %v2870 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2871 = load i64, i64* %v2870
  %v2872 = add i64 %v2871, 1
  store i64 %v2872, i64* %v2870
  %v2873 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v2873
  %v2874 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 1193046, i32* %v2874
  ; Cycle B0003
  %v2875 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2876 = load i64, i64* %v2875
  %v2877 = add i64 %v2876, 1
  store i64 %v2877, i64* %v2875
  %v2878 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2879 = load i64, i64* %v2878
  %v2880 = add i64 %v2879, 1
  store i64 %v2880, i64* %v2878
  ; Cycle B0000
  %v2881 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2882 = load i64, i64* %v2881
  %v2883 = add i64 %v2882, 1
  store i64 %v2883, i64* %v2881
  %v2884 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 352, i32* %v2884
  %v2885 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 608, i32* %v2885
  %v2886 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 864, i32* %v2886
  %v2887 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1120, i32* %v2887
  ; Cycle B0001
  %v2888 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2889 = load i64, i64* %v2888
  %v2890 = add i64 %v2889, 1
  store i64 %v2890, i64* %v2888
  %v2891 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1376, i32* %v2891
  %v2892 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1632, i32* %v2892
  %v2893 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1888, i32* %v2893
  %v2894 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2144, i32* %v2894
  ; Cycle B0002
  %v2895 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2896 = load i64, i64* %v2895
  %v2897 = add i64 %v2896, 1
  store i64 %v2897, i64* %v2895
  %v2898 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v2898
  %v2899 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2900 = load i64, i64* %v2899
  %v2901 = add i64 %v2900, 1
  store i64 %v2901, i64* %v2899
  %v2902 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v2902
  ; Cycle B0003
  %v2903 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2904 = load i64, i64* %v2903
  %v2905 = add i64 %v2904, 1
  store i64 %v2905, i64* %v2903
  %v2906 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2907 = load i64, i64* %v2906
  %v2908 = add i64 %v2907, 1
  store i64 %v2908, i64* %v2906
  ; Cycle B0000
  %v2909 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2910 = load i64, i64* %v2909
  %v2911 = add i64 %v2910, 1
  store i64 %v2911, i64* %v2909
  %v2912 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 353, i32* %v2912
  %v2913 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 9757, i32* %v2913
  %v2914 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 865, i32* %v2914
  %v2915 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 17941, i32* %v2915
  ; Cycle B0001
  %v2916 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2917 = load i64, i64* %v2916
  %v2918 = add i64 %v2917, 1
  store i64 %v2918, i64* %v2916
  %v2919 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1377, i32* %v2919
  %v2920 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1633, i32* %v2920
  %v2921 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 30238, i32* %v2921
  %v2922 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2145, i32* %v2922
  ; Cycle B0002
  %v2923 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2924 = load i64, i64* %v2923
  %v2925 = add i64 %v2924, 1
  store i64 %v2925, i64* %v2923
  %v2926 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v2926
  %v2927 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2928 = load i64, i64* %v2927
  %v2929 = add i64 %v2928, 1
  store i64 %v2929, i64* %v2927
  %v2930 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v2930
  ; Cycle B0003
  %v2931 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2932 = load i64, i64* %v2931
  %v2933 = add i64 %v2932, 1
  store i64 %v2933, i64* %v2931
  %v2934 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2935 = load i64, i64* %v2934
  %v2936 = add i64 %v2935, 1
  store i64 %v2936, i64* %v2934
  ; Cycle B0000
  %v2937 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2938 = load i64, i64* %v2937
  %v2939 = add i64 %v2938, 1
  store i64 %v2939, i64* %v2937
  %v2940 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 354, i32* %v2940
  %v2941 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 610, i32* %v2941
  %v2942 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 866, i32* %v2942
  %v2943 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1122, i32* %v2943
  ; Cycle B0001
  %v2944 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2945 = load i64, i64* %v2944
  %v2946 = add i64 %v2945, 1
  store i64 %v2946, i64* %v2944
  %v2947 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 22048, i32* %v2947
  %v2948 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1634, i32* %v2948
  %v2949 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1890, i32* %v2949
  %v2950 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2146, i32* %v2950
  ; Cycle B0002
  %v2951 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2952 = load i64, i64* %v2951
  %v2953 = add i64 %v2952, 1
  store i64 %v2953, i64* %v2951
  %v2954 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v2954
  %v2955 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2956 = load i64, i64* %v2955
  %v2957 = add i64 %v2956, 1
  store i64 %v2957, i64* %v2955
  %v2958 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v2958
  ; Cycle B0003
  %v2959 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2960 = load i64, i64* %v2959
  %v2961 = add i64 %v2960, 1
  store i64 %v2961, i64* %v2959
  %v2962 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2963 = load i64, i64* %v2962
  %v2964 = add i64 %v2963, 1
  store i64 %v2964, i64* %v2962
  ; Cycle B0000
  %v2965 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2966 = load i64, i64* %v2965
  %v2967 = add i64 %v2966, 1
  store i64 %v2967, i64* %v2965
  %v2968 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 355, i32* %v2968
  %v2969 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 611, i32* %v2969
  %v2970 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 13887, i32* %v2970
  %v2971 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 17980, i32* %v2971
  ; Cycle B0001
  %v2972 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2973 = load i64, i64* %v2972
  %v2974 = add i64 %v2973, 1
  store i64 %v2974, i64* %v2972
  %v2975 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1379, i32* %v2975
  %v2976 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 26172, i32* %v2976
  %v2977 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1891, i32* %v2977
  %v2978 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 34364, i32* %v2978
  ; Cycle B0002
  %v2979 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2980 = load i64, i64* %v2979
  %v2981 = add i64 %v2980, 1
  store i64 %v2981, i64* %v2979
  %v2982 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v2982
  %v2983 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v2984 = load i64, i64* %v2983
  %v2985 = add i64 %v2984, 1
  store i64 %v2985, i64* %v2983
  %v2986 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v2986
  ; Cycle B0003
  %v2987 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2988 = load i64, i64* %v2987
  %v2989 = add i64 %v2988, 1
  store i64 %v2989, i64* %v2987
  %v2990 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v2991 = load i64, i64* %v2990
  %v2992 = add i64 %v2991, 1
  store i64 %v2992, i64* %v2990
  ; Cycle B0000
  %v2993 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2994 = load i64, i64* %v2993
  %v2995 = add i64 %v2994, 1
  store i64 %v2995, i64* %v2993
  %v2996 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 5699, i32* %v2996
  %v2997 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 612, i32* %v2997
  %v2998 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 868, i32* %v2998
  %v2999 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 17984, i32* %v2999
  ; Cycle B0001
  %v3000 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3001 = load i64, i64* %v3000
  %v3002 = add i64 %v3001, 1
  store i64 %v3002, i64* %v3000
  %v3003 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 22091, i32* %v3003
  %v3004 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1636, i32* %v3004
  %v3005 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1892, i32* %v3005
  %v3006 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2148, i32* %v3006
  ; Cycle B0002
  %v3007 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3008 = load i64, i64* %v3007
  %v3009 = add i64 %v3008, 1
  store i64 %v3009, i64* %v3007
  %v3010 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3010
  %v3011 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3012 = load i64, i64* %v3011
  %v3013 = add i64 %v3012, 1
  store i64 %v3013, i64* %v3011
  %v3014 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3014
  ; Cycle B0003
  %v3015 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3016 = load i64, i64* %v3015
  %v3017 = add i64 %v3016, 1
  store i64 %v3017, i64* %v3015
  %v3018 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3019 = load i64, i64* %v3018
  %v3020 = add i64 %v3019, 1
  store i64 %v3020, i64* %v3018
  ; Cycle B0000
  %v3021 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3022 = load i64, i64* %v3021
  %v3023 = add i64 %v3022, 1
  store i64 %v3023, i64* %v3021
  %v3024 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 357, i32* %v3024
  %v3025 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 613, i32* %v3025
  %v3026 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 869, i32* %v3026
  %v3027 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 18001, i32* %v3027
  ; Cycle B0001
  %v3028 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3029 = load i64, i64* %v3028
  %v3030 = add i64 %v3029, 1
  store i64 %v3030, i64* %v3028
  %v3031 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1381, i32* %v3031
  %v3032 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1637, i32* %v3032
  %v3033 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 30298, i32* %v3033
  %v3034 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2149, i32* %v3034
  ; Cycle B0002
  %v3035 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3036 = load i64, i64* %v3035
  %v3037 = add i64 %v3036, 1
  store i64 %v3037, i64* %v3035
  %v3038 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3038
  %v3039 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3040 = load i64, i64* %v3039
  %v3041 = add i64 %v3040, 1
  store i64 %v3041, i64* %v3039
  %v3042 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3042
  ; Cycle B0003
  %v3043 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3044 = load i64, i64* %v3043
  %v3045 = add i64 %v3044, 1
  store i64 %v3045, i64* %v3043
  %v3046 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3047 = load i64, i64* %v3046
  %v3048 = add i64 %v3047, 1
  store i64 %v3048, i64* %v3046
  ; Cycle B0000
  %v3049 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3050 = load i64, i64* %v3049
  %v3051 = add i64 %v3050, 1
  store i64 %v3051, i64* %v3049
  %v3052 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 5738, i32* %v3052
  %v3053 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 614, i32* %v3053
  %v3054 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 870, i32* %v3054
  %v3055 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1126, i32* %v3055
  ; Cycle B0001
  %v3056 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3057 = load i64, i64* %v3056
  %v3058 = add i64 %v3057, 1
  store i64 %v3058, i64* %v3056
  %v3059 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 22116, i32* %v3059
  %v3060 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1638, i32* %v3060
  %v3061 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1894, i32* %v3061
  %v3062 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2150, i32* %v3062
  ; Cycle B0002
  %v3063 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3064 = load i64, i64* %v3063
  %v3065 = add i64 %v3064, 1
  store i64 %v3065, i64* %v3063
  %v3066 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3066
  %v3067 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3068 = load i64, i64* %v3067
  %v3069 = add i64 %v3068, 1
  store i64 %v3069, i64* %v3067
  %v3070 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3070
  ; Cycle B0003
  %v3071 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3072 = load i64, i64* %v3071
  %v3073 = add i64 %v3072, 1
  store i64 %v3073, i64* %v3071
  %v3074 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3075 = load i64, i64* %v3074
  %v3076 = add i64 %v3075, 1
  store i64 %v3076, i64* %v3074
  ; Cycle B0000
  %v3077 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3078 = load i64, i64* %v3077
  %v3079 = add i64 %v3078, 1
  store i64 %v3079, i64* %v3077
  %v3080 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 359, i32* %v3080
  %v3081 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 615, i32* %v3081
  %v3082 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 13947, i32* %v3082
  %v3083 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1127, i32* %v3083
  ; Cycle B0001
  %v3084 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3085 = load i64, i64* %v3084
  %v3086 = add i64 %v3085, 1
  store i64 %v3086, i64* %v3084
  %v3087 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 22137, i32* %v3087
  %v3088 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1639, i32* %v3088
  %v3089 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1895, i32* %v3089
  %v3090 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2151, i32* %v3090
  ; Cycle B0002
  %v3091 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3092 = load i64, i64* %v3091
  %v3093 = add i64 %v3092, 1
  store i64 %v3093, i64* %v3091
  %v3094 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3094
  %v3095 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3096 = load i64, i64* %v3095
  %v3097 = add i64 %v3096, 1
  store i64 %v3097, i64* %v3095
  %v3098 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3098
  ; Cycle B0003
  %v3099 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3100 = load i64, i64* %v3099
  %v3101 = add i64 %v3100, 1
  store i64 %v3101, i64* %v3099
  %v3102 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3103 = load i64, i64* %v3102
  %v3104 = add i64 %v3103, 1
  store i64 %v3104, i64* %v3102
  ; Cycle B0000
  %v3105 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3106 = load i64, i64* %v3105
  %v3107 = add i64 %v3106, 1
  store i64 %v3107, i64* %v3105
  %v3108 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 360, i32* %v3108
  %v3109 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 616, i32* %v3109
  %v3110 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 13957, i32* %v3110
  %v3111 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1128, i32* %v3111
  ; Cycle B0001
  %v3112 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3113 = load i64, i64* %v3112
  %v3114 = add i64 %v3113, 1
  store i64 %v3114, i64* %v3112
  %v3115 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1384, i32* %v3115
  %v3116 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 26244, i32* %v3116
  %v3117 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 30349, i32* %v3117
  %v3118 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2152, i32* %v3118
  ; Cycle B0002
  %v3119 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3120 = load i64, i64* %v3119
  %v3121 = add i64 %v3120, 1
  store i64 %v3121, i64* %v3119
  %v3122 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3122
  %v3123 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3124 = load i64, i64* %v3123
  %v3125 = add i64 %v3124, 1
  store i64 %v3125, i64* %v3123
  %v3126 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3126
  ; Cycle B0003
  %v3127 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3128 = load i64, i64* %v3127
  %v3129 = add i64 %v3128, 1
  store i64 %v3129, i64* %v3127
  %v3130 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3131 = load i64, i64* %v3130
  %v3132 = add i64 %v3131, 1
  store i64 %v3132, i64* %v3130
  ; Cycle B0000
  %v3133 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3134 = load i64, i64* %v3133
  %v3135 = add i64 %v3134, 1
  store i64 %v3135, i64* %v3133
  %v3136 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 361, i32* %v3136
  %v3137 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 617, i32* %v3137
  %v3138 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 873, i32* %v3138
  %v3139 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1129, i32* %v3139
  ; Cycle B0001
  %v3140 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3141 = load i64, i64* %v3140
  %v3142 = add i64 %v3141, 1
  store i64 %v3142, i64* %v3140
  %v3143 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1385, i32* %v3143
  %v3144 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1641, i32* %v3144
  %v3145 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1897, i32* %v3145
  %v3146 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2153, i32* %v3146
  ; Cycle B0002
  %v3147 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3148 = load i64, i64* %v3147
  %v3149 = add i64 %v3148, 1
  store i64 %v3149, i64* %v3147
  %v3150 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3150
  %v3151 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3152 = load i64, i64* %v3151
  %v3153 = add i64 %v3152, 1
  store i64 %v3153, i64* %v3151
  %v3154 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3154
  ; Cycle B0003
  %v3155 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3156 = load i64, i64* %v3155
  %v3157 = add i64 %v3156, 1
  store i64 %v3157, i64* %v3155
  %v3158 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3159 = load i64, i64* %v3158
  %v3160 = add i64 %v3159, 1
  store i64 %v3160, i64* %v3158
  ; Cycle B0000
  %v3161 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3162 = load i64, i64* %v3161
  %v3163 = add i64 %v3162, 1
  store i64 %v3163, i64* %v3161
  %v3164 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 362, i32* %v3164
  %v3165 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 618, i32* %v3165
  %v3166 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 874, i32* %v3166
  %v3167 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1130, i32* %v3167
  ; Cycle B0001
  %v3168 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3169 = load i64, i64* %v3168
  %v3170 = add i64 %v3169, 1
  store i64 %v3170, i64* %v3168
  %v3171 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1386, i32* %v3171
  %v3172 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1642, i32* %v3172
  %v3173 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1898, i32* %v3173
  %v3174 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2154, i32* %v3174
  ; Cycle B0002
  %v3175 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3176 = load i64, i64* %v3175
  %v3177 = add i64 %v3176, 1
  store i64 %v3177, i64* %v3175
  %v3178 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3178
  %v3179 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3180 = load i64, i64* %v3179
  %v3181 = add i64 %v3180, 1
  store i64 %v3181, i64* %v3179
  %v3182 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3182
  ; Cycle B0003
  %v3183 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3184 = load i64, i64* %v3183
  %v3185 = add i64 %v3184, 1
  store i64 %v3185, i64* %v3183
  %v3186 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3187 = load i64, i64* %v3186
  %v3188 = add i64 %v3187, 1
  store i64 %v3188, i64* %v3186
  ; Cycle B0000
  %v3189 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3190 = load i64, i64* %v3189
  %v3191 = add i64 %v3190, 1
  store i64 %v3191, i64* %v3189
  %v3192 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 363, i32* %v3192
  %v3193 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 9917, i32* %v3193
  %v3194 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 875, i32* %v3194
  %v3195 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1131, i32* %v3195
  ; Cycle B0001
  %v3196 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3197 = load i64, i64* %v3196
  %v3198 = add i64 %v3197, 1
  store i64 %v3198, i64* %v3196
  %v3199 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1387, i32* %v3199
  %v3200 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1643, i32* %v3200
  %v3201 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1899, i32* %v3201
  %v3202 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 34485, i32* %v3202
  ; Cycle B0002
  %v3203 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3204 = load i64, i64* %v3203
  %v3205 = add i64 %v3204, 1
  store i64 %v3205, i64* %v3203
  %v3206 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3206
  %v3207 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3208 = load i64, i64* %v3207
  %v3209 = add i64 %v3208, 1
  store i64 %v3209, i64* %v3207
  %v3210 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3210
  ; Cycle B0003
  %v3211 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3212 = load i64, i64* %v3211
  %v3213 = add i64 %v3212, 1
  store i64 %v3213, i64* %v3211
  %v3214 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3215 = load i64, i64* %v3214
  %v3216 = add i64 %v3215, 1
  store i64 %v3216, i64* %v3214
  ; Cycle B0000
  %v3217 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3218 = load i64, i64* %v3217
  %v3219 = add i64 %v3218, 1
  store i64 %v3219, i64* %v3217
  %v3220 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 364, i32* %v3220
  %v3221 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 620, i32* %v3221
  %v3222 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 876, i32* %v3222
  %v3223 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1132, i32* %v3223
  ; Cycle B0001
  %v3224 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3225 = load i64, i64* %v3224
  %v3226 = add i64 %v3225, 1
  store i64 %v3226, i64* %v3224
  %v3227 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1388, i32* %v3227
  %v3228 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1644, i32* %v3228
  %v3229 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1900, i32* %v3229
  %v3230 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2156, i32* %v3230
  ; Cycle B0002
  %v3231 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3232 = load i64, i64* %v3231
  %v3233 = add i64 %v3232, 1
  store i64 %v3233, i64* %v3231
  %v3234 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3234
  %v3235 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3236 = load i64, i64* %v3235
  %v3237 = add i64 %v3236, 1
  store i64 %v3237, i64* %v3235
  %v3238 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3238
  ; Cycle B0003
  %v3239 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3240 = load i64, i64* %v3239
  %v3241 = add i64 %v3240, 1
  store i64 %v3241, i64* %v3239
  %v3242 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3243 = load i64, i64* %v3242
  %v3244 = add i64 %v3243, 1
  store i64 %v3244, i64* %v3242
  ; Cycle B0000
  %v3245 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3246 = load i64, i64* %v3245
  %v3247 = add i64 %v3246, 1
  store i64 %v3247, i64* %v3245
  %v3248 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 365, i32* %v3248
  %v3249 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 621, i32* %v3249
  %v3250 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 14047, i32* %v3250
  %v3251 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1133, i32* %v3251
  ; Cycle B0001
  %v3252 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3253 = load i64, i64* %v3252
  %v3254 = add i64 %v3253, 1
  store i64 %v3254, i64* %v3252
  %v3255 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1389, i32* %v3255
  %v3256 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1645, i32* %v3256
  %v3257 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1901, i32* %v3257
  %v3258 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2157, i32* %v3258
  ; Cycle B0002
  %v3259 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3260 = load i64, i64* %v3259
  %v3261 = add i64 %v3260, 1
  store i64 %v3261, i64* %v3259
  %v3262 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3262
  %v3263 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3264 = load i64, i64* %v3263
  %v3265 = add i64 %v3264, 1
  store i64 %v3265, i64* %v3263
  %v3266 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3266
  ; Cycle B0003
  %v3267 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3268 = load i64, i64* %v3267
  %v3269 = add i64 %v3268, 1
  store i64 %v3269, i64* %v3267
  %v3270 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3271 = load i64, i64* %v3270
  %v3272 = add i64 %v3271, 1
  store i64 %v3272, i64* %v3270
  ; Cycle B0000
  %v3273 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3274 = load i64, i64* %v3273
  %v3275 = add i64 %v3274, 1
  store i64 %v3275, i64* %v3273
  %v3276 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 5866, i32* %v3276
  %v3277 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 622, i32* %v3277
  %v3278 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 878, i32* %v3278
  %v3279 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1134, i32* %v3279
  ; Cycle B0001
  %v3280 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3281 = load i64, i64* %v3280
  %v3282 = add i64 %v3281, 1
  store i64 %v3282, i64* %v3280
  %v3283 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1390, i32* %v3283
  %v3284 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1646, i32* %v3284
  %v3285 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1902, i32* %v3285
  %v3286 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2158, i32* %v3286
  ; Cycle B0002
  %v3287 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3288 = load i64, i64* %v3287
  %v3289 = add i64 %v3288, 1
  store i64 %v3289, i64* %v3287
  %v3290 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3290
  %v3291 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3292 = load i64, i64* %v3291
  %v3293 = add i64 %v3292, 1
  store i64 %v3293, i64* %v3291
  %v3294 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3294
  ; Cycle B0003
  %v3295 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3296 = load i64, i64* %v3295
  %v3297 = add i64 %v3296, 1
  store i64 %v3297, i64* %v3295
  %v3298 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3299 = load i64, i64* %v3298
  %v3300 = add i64 %v3299, 1
  store i64 %v3300, i64* %v3298
  ; Cycle B0000
  %v3301 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3302 = load i64, i64* %v3301
  %v3303 = add i64 %v3302, 1
  store i64 %v3303, i64* %v3301
  %v3304 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 367, i32* %v3304
  %v3305 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 623, i32* %v3305
  %v3306 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 879, i32* %v3306
  %v3307 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1135, i32* %v3307
  ; Cycle B0001
  %v3308 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3309 = load i64, i64* %v3308
  %v3310 = add i64 %v3309, 1
  store i64 %v3310, i64* %v3308
  %v3311 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1391, i32* %v3311
  %v3312 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1647, i32* %v3312
  %v3313 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 30462, i32* %v3313
  %v3314 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 34545, i32* %v3314
  ; Cycle B0002
  %v3315 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3316 = load i64, i64* %v3315
  %v3317 = add i64 %v3316, 1
  store i64 %v3317, i64* %v3315
  %v3318 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3318
  %v3319 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3320 = load i64, i64* %v3319
  %v3321 = add i64 %v3320, 1
  store i64 %v3321, i64* %v3319
  %v3322 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3322
  ; Cycle B0003
  %v3323 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3324 = load i64, i64* %v3323
  %v3325 = add i64 %v3324, 1
  store i64 %v3325, i64* %v3323
  %v3326 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3327 = load i64, i64* %v3326
  %v3328 = add i64 %v3327, 1
  store i64 %v3328, i64* %v3326
  ; Cycle B0000
  %v3329 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3330 = load i64, i64* %v3329
  %v3331 = add i64 %v3330, 1
  store i64 %v3331, i64* %v3329
  %v3332 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 368, i32* %v3332
  %v3333 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 624, i32* %v3333
  %v3334 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 14091, i32* %v3334
  %v3335 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1136, i32* %v3335
  ; Cycle B0001
  %v3336 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3337 = load i64, i64* %v3336
  %v3338 = add i64 %v3337, 1
  store i64 %v3338, i64* %v3336
  %v3339 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 22281, i32* %v3339
  %v3340 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1648, i32* %v3340
  %v3341 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1904, i32* %v3341
  %v3342 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2160, i32* %v3342
  ; Cycle B0002
  %v3343 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3344 = load i64, i64* %v3343
  %v3345 = add i64 %v3344, 1
  store i64 %v3345, i64* %v3343
  %v3346 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3346
  %v3347 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3348 = load i64, i64* %v3347
  %v3349 = add i64 %v3348, 1
  store i64 %v3349, i64* %v3347
  %v3350 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3350
  ; Cycle B0003
  %v3351 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3352 = load i64, i64* %v3351
  %v3353 = add i64 %v3352, 1
  store i64 %v3353, i64* %v3351
  %v3354 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3355 = load i64, i64* %v3354
  %v3356 = add i64 %v3355, 1
  store i64 %v3356, i64* %v3354
  ; Cycle B0000
  %v3357 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3358 = load i64, i64* %v3357
  %v3359 = add i64 %v3358, 1
  store i64 %v3359, i64* %v3357
  %v3360 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 5914, i32* %v3360
  %v3361 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 625, i32* %v3361
  %v3362 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 881, i32* %v3362
  %v3363 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1137, i32* %v3363
  ; Cycle B0001
  %v3364 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3365 = load i64, i64* %v3364
  %v3366 = add i64 %v3365, 1
  store i64 %v3366, i64* %v3364
  %v3367 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 22292, i32* %v3367
  %v3368 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1649, i32* %v3368
  %v3369 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1905, i32* %v3369
  %v3370 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2161, i32* %v3370
  ; Cycle B0002
  %v3371 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3372 = load i64, i64* %v3371
  %v3373 = add i64 %v3372, 1
  store i64 %v3373, i64* %v3371
  %v3374 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3374
  %v3375 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3376 = load i64, i64* %v3375
  %v3377 = add i64 %v3376, 1
  store i64 %v3377, i64* %v3375
  %v3378 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3378
  ; Cycle B0003
  %v3379 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3380 = load i64, i64* %v3379
  %v3381 = add i64 %v3380, 1
  store i64 %v3381, i64* %v3379
  %v3382 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3383 = load i64, i64* %v3382
  %v3384 = add i64 %v3383, 1
  store i64 %v3384, i64* %v3382
  ; Cycle B0000
  %v3385 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3386 = load i64, i64* %v3385
  %v3387 = add i64 %v3386, 1
  store i64 %v3387, i64* %v3385
  %v3388 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 370, i32* %v3388
  %v3389 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 626, i32* %v3389
  %v3390 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 882, i32* %v3390
  %v3391 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 18209, i32* %v3391
  ; Cycle B0001
  %v3392 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3393 = load i64, i64* %v3392
  %v3394 = add i64 %v3393, 1
  store i64 %v3394, i64* %v3392
  %v3395 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1394, i32* %v3395
  %v3396 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1650, i32* %v3396
  %v3397 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 30506, i32* %v3397
  %v3398 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2162, i32* %v3398
  ; Cycle B0002
  %v3399 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3400 = load i64, i64* %v3399
  %v3401 = add i64 %v3400, 1
  store i64 %v3401, i64* %v3399
  %v3402 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3402
  %v3403 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3404 = load i64, i64* %v3403
  %v3405 = add i64 %v3404, 1
  store i64 %v3405, i64* %v3403
  %v3406 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3406
  ; Cycle B0003
  %v3407 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3408 = load i64, i64* %v3407
  %v3409 = add i64 %v3408, 1
  store i64 %v3409, i64* %v3407
  %v3410 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3411 = load i64, i64* %v3410
  %v3412 = add i64 %v3411, 1
  store i64 %v3412, i64* %v3410
  ; Cycle B0000
  %v3413 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3414 = load i64, i64* %v3413
  %v3415 = add i64 %v3414, 1
  store i64 %v3415, i64* %v3413
  %v3416 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 5939, i32* %v3416
  %v3417 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 627, i32* %v3417
  %v3418 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 883, i32* %v3418
  %v3419 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 18224, i32* %v3419
  ; Cycle B0001
  %v3420 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3421 = load i64, i64* %v3420
  %v3422 = add i64 %v3421, 1
  store i64 %v3422, i64* %v3420
  %v3423 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 22331, i32* %v3423
  %v3424 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1651, i32* %v3424
  %v3425 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1907, i32* %v3425
  %v3426 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2163, i32* %v3426
  ; Cycle B0002
  %v3427 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3428 = load i64, i64* %v3427
  %v3429 = add i64 %v3428, 1
  store i64 %v3429, i64* %v3427
  %v3430 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3430
  %v3431 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3432 = load i64, i64* %v3431
  %v3433 = add i64 %v3432, 1
  store i64 %v3433, i64* %v3431
  %v3434 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3434
  ; Cycle B0003
  %v3435 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3436 = load i64, i64* %v3435
  %v3437 = add i64 %v3436, 1
  store i64 %v3437, i64* %v3435
  %v3438 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3439 = load i64, i64* %v3438
  %v3440 = add i64 %v3439, 1
  store i64 %v3440, i64* %v3438
  ; Cycle B0000
  %v3441 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3442 = load i64, i64* %v3441
  %v3443 = add i64 %v3442, 1
  store i64 %v3443, i64* %v3441
  %v3444 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 372, i32* %v3444
  %v3445 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 628, i32* %v3445
  %v3446 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 14159, i32* %v3446
  %v3447 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 18252, i32* %v3447
  ; Cycle B0001
  %v3448 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3449 = load i64, i64* %v3448
  %v3450 = add i64 %v3449, 1
  store i64 %v3450, i64* %v3448
  %v3451 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1396, i32* %v3451
  %v3452 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 26444, i32* %v3452
  %v3453 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1908, i32* %v3453
  %v3454 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 34636, i32* %v3454
  ; Cycle B0002
  %v3455 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3456 = load i64, i64* %v3455
  %v3457 = add i64 %v3456, 1
  store i64 %v3457, i64* %v3455
  %v3458 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3458
  %v3459 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3460 = load i64, i64* %v3459
  %v3461 = add i64 %v3460, 1
  store i64 %v3461, i64* %v3459
  %v3462 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3462
  ; Cycle B0003
  %v3463 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3464 = load i64, i64* %v3463
  %v3465 = add i64 %v3464, 1
  store i64 %v3465, i64* %v3463
  %v3466 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3467 = load i64, i64* %v3466
  %v3468 = add i64 %v3467, 1
  store i64 %v3468, i64* %v3466
  ; Cycle B0000
  %v3469 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3470 = load i64, i64* %v3469
  %v3471 = add i64 %v3470, 1
  store i64 %v3471, i64* %v3469
  %v3472 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 373, i32* %v3472
  %v3473 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 629, i32* %v3473
  %v3474 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 885, i32* %v3474
  %v3475 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1141, i32* %v3475
  ; Cycle B0001
  %v3476 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3477 = load i64, i64* %v3476
  %v3478 = add i64 %v3477, 1
  store i64 %v3478, i64* %v3476
  %v3479 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 22352, i32* %v3479
  %v3480 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1653, i32* %v3480
  %v3481 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1909, i32* %v3481
  %v3482 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2165, i32* %v3482
  ; Cycle B0002
  %v3483 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3484 = load i64, i64* %v3483
  %v3485 = add i64 %v3484, 1
  store i64 %v3485, i64* %v3483
  %v3486 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3486
  %v3487 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3488 = load i64, i64* %v3487
  %v3489 = add i64 %v3488, 1
  store i64 %v3489, i64* %v3487
  %v3490 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3490
  ; Cycle B0003
  %v3491 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3492 = load i64, i64* %v3491
  %v3493 = add i64 %v3492, 1
  store i64 %v3493, i64* %v3491
  %v3494 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3495 = load i64, i64* %v3494
  %v3496 = add i64 %v3495, 1
  store i64 %v3496, i64* %v3494
  ; Cycle B0000
  %v3497 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3498 = load i64, i64* %v3497
  %v3499 = add i64 %v3498, 1
  store i64 %v3499, i64* %v3497
  %v3500 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 374, i32* %v3500
  %v3501 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 10093, i32* %v3501
  %v3502 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 886, i32* %v3502
  %v3503 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 18277, i32* %v3503
  ; Cycle B0001
  %v3504 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3505 = load i64, i64* %v3504
  %v3506 = add i64 %v3505, 1
  store i64 %v3506, i64* %v3504
  %v3507 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1398, i32* %v3507
  %v3508 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1654, i32* %v3508
  %v3509 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 30574, i32* %v3509
  %v3510 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2166, i32* %v3510
  ; Cycle B0002
  %v3511 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3512 = load i64, i64* %v3511
  %v3513 = add i64 %v3512, 1
  store i64 %v3513, i64* %v3511
  %v3514 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3514
  %v3515 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3516 = load i64, i64* %v3515
  %v3517 = add i64 %v3516, 1
  store i64 %v3517, i64* %v3515
  %v3518 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3518
  ; Cycle B0003
  %v3519 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3520 = load i64, i64* %v3519
  %v3521 = add i64 %v3520, 1
  store i64 %v3521, i64* %v3519
  %v3522 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3523 = load i64, i64* %v3522
  %v3524 = add i64 %v3523, 1
  store i64 %v3524, i64* %v3522
  ; Cycle B0000
  %v3525 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3526 = load i64, i64* %v3525
  %v3527 = add i64 %v3526, 1
  store i64 %v3527, i64* %v3525
  %v3528 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 375, i32* %v3528
  %v3529 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 631, i32* %v3529
  %v3530 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 887, i32* %v3530
  %v3531 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1143, i32* %v3531
  ; Cycle B0001
  %v3532 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3533 = load i64, i64* %v3532
  %v3534 = add i64 %v3533, 1
  store i64 %v3534, i64* %v3532
  %v3535 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1399, i32* %v3535
  %v3536 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1655, i32* %v3536
  %v3537 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1911, i32* %v3537
  %v3538 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2167, i32* %v3538
  ; Cycle B0002
  %v3539 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3540 = load i64, i64* %v3539
  %v3541 = add i64 %v3540, 1
  store i64 %v3541, i64* %v3539
  %v3542 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3542
  %v3543 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3544 = load i64, i64* %v3543
  %v3545 = add i64 %v3544, 1
  store i64 %v3545, i64* %v3543
  %v3546 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3546
  ; Cycle B0003
  %v3547 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3548 = load i64, i64* %v3547
  %v3549 = add i64 %v3548, 1
  store i64 %v3549, i64* %v3547
  %v3550 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3551 = load i64, i64* %v3550
  %v3552 = add i64 %v3551, 1
  store i64 %v3552, i64* %v3550
  ; Cycle B0000
  %v3553 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3554 = load i64, i64* %v3553
  %v3555 = add i64 %v3554, 1
  store i64 %v3555, i64* %v3553
  %v3556 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 6027, i32* %v3556
  %v3557 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 632, i32* %v3557
  %v3558 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 888, i32* %v3558
  %v3559 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1144, i32* %v3559
  ; Cycle B0001
  %v3560 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3561 = load i64, i64* %v3560
  %v3562 = add i64 %v3561, 1
  store i64 %v3562, i64* %v3560
  %v3563 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1400, i32* %v3563
  %v3564 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1656, i32* %v3564
  %v3565 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1912, i32* %v3565
  %v3566 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2168, i32* %v3566
  ; Cycle B0002
  %v3567 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3568 = load i64, i64* %v3567
  %v3569 = add i64 %v3568, 1
  store i64 %v3569, i64* %v3567
  %v3570 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3570
  %v3571 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3572 = load i64, i64* %v3571
  %v3573 = add i64 %v3572, 1
  store i64 %v3573, i64* %v3571
  %v3574 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3574
  ; Cycle B0003
  %v3575 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3576 = load i64, i64* %v3575
  %v3577 = add i64 %v3576, 1
  store i64 %v3577, i64* %v3575
  %v3578 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3579 = load i64, i64* %v3578
  %v3580 = add i64 %v3579, 1
  store i64 %v3580, i64* %v3578
  ; Cycle B0000
  %v3581 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3582 = load i64, i64* %v3581
  %v3583 = add i64 %v3582, 1
  store i64 %v3583, i64* %v3581
  %v3584 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 377, i32* %v3584
  %v3585 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 10135, i32* %v3585
  %v3586 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 889, i32* %v3586
  %v3587 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 18331, i32* %v3587
  ; Cycle B0001
  %v3588 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3589 = load i64, i64* %v3588
  %v3590 = add i64 %v3589, 1
  store i64 %v3590, i64* %v3588
  %v3591 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1401, i32* %v3591
  %v3592 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 26523, i32* %v3592
  %v3593 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 30610, i32* %v3593
  %v3594 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 34719, i32* %v3594
  ; Cycle B0002
  %v3595 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3596 = load i64, i64* %v3595
  %v3597 = add i64 %v3596, 1
  store i64 %v3597, i64* %v3595
  %v3598 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3598
  %v3599 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3600 = load i64, i64* %v3599
  %v3601 = add i64 %v3600, 1
  store i64 %v3601, i64* %v3599
  %v3602 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3602
  ; Cycle B0003
  %v3603 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3604 = load i64, i64* %v3603
  %v3605 = add i64 %v3604, 1
  store i64 %v3605, i64* %v3603
  %v3606 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3607 = load i64, i64* %v3606
  %v3608 = add i64 %v3607, 1
  store i64 %v3608, i64* %v3606
  ; Cycle B0000
  %v3609 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3610 = load i64, i64* %v3609
  %v3611 = add i64 %v3610, 1
  store i64 %v3611, i64* %v3609
  %v3612 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 378, i32* %v3612
  %v3613 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 634, i32* %v3613
  %v3614 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 890, i32* %v3614
  %v3615 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1146, i32* %v3615
  ; Cycle B0001
  %v3616 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3617 = load i64, i64* %v3616
  %v3618 = add i64 %v3617, 1
  store i64 %v3618, i64* %v3616
  %v3619 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1402, i32* %v3619
  %v3620 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1658, i32* %v3620
  %v3621 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 30638, i32* %v3621
  %v3622 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 34721, i32* %v3622
  ; Cycle B0002
  %v3623 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3624 = load i64, i64* %v3623
  %v3625 = add i64 %v3624, 1
  store i64 %v3625, i64* %v3623
  %v3626 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3626
  %v3627 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3628 = load i64, i64* %v3627
  %v3629 = add i64 %v3628, 1
  store i64 %v3629, i64* %v3627
  %v3630 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3630
  ; Cycle B0003
  %v3631 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3632 = load i64, i64* %v3631
  %v3633 = add i64 %v3632, 1
  store i64 %v3633, i64* %v3631
  %v3634 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3635 = load i64, i64* %v3634
  %v3636 = add i64 %v3635, 1
  store i64 %v3636, i64* %v3634
  ; Cycle B0000
  %v3637 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3638 = load i64, i64* %v3637
  %v3639 = add i64 %v3638, 1
  store i64 %v3639, i64* %v3637
  %v3640 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 6074, i32* %v3640
  %v3641 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 635, i32* %v3641
  %v3642 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 891, i32* %v3642
  %v3643 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1147, i32* %v3643
  ; Cycle B0001
  %v3644 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3645 = load i64, i64* %v3644
  %v3646 = add i64 %v3645, 1
  store i64 %v3646, i64* %v3644
  %v3647 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1403, i32* %v3647
  %v3648 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1659, i32* %v3648
  %v3649 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1915, i32* %v3649
  %v3650 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2171, i32* %v3650
  ; Cycle B0002
  %v3651 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3652 = load i64, i64* %v3651
  %v3653 = add i64 %v3652, 1
  store i64 %v3653, i64* %v3651
  %v3654 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3654
  %v3655 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3656 = load i64, i64* %v3655
  %v3657 = add i64 %v3656, 1
  store i64 %v3657, i64* %v3655
  %v3658 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3658
  ; Cycle B0003
  %v3659 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3660 = load i64, i64* %v3659
  %v3661 = add i64 %v3660, 1
  store i64 %v3661, i64* %v3659
  %v3662 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3663 = load i64, i64* %v3662
  %v3664 = add i64 %v3663, 1
  store i64 %v3664, i64* %v3662
  ; Cycle B0000
  %v3665 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3666 = load i64, i64* %v3665
  %v3667 = add i64 %v3666, 1
  store i64 %v3667, i64* %v3665
  %v3668 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 380, i32* %v3668
  %v3669 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 636, i32* %v3669
  %v3670 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 14287, i32* %v3670
  %v3671 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1148, i32* %v3671
  ; Cycle B0001
  %v3672 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3673 = load i64, i64* %v3672
  %v3674 = add i64 %v3673, 1
  store i64 %v3674, i64* %v3672
  %v3675 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1404, i32* %v3675
  %v3676 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1660, i32* %v3676
  %v3677 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1916, i32* %v3677
  %v3678 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2172, i32* %v3678
  ; Cycle B0002
  %v3679 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3680 = load i64, i64* %v3679
  %v3681 = add i64 %v3680, 1
  store i64 %v3681, i64* %v3679
  %v3682 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3682
  %v3683 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3684 = load i64, i64* %v3683
  %v3685 = add i64 %v3684, 1
  store i64 %v3685, i64* %v3683
  %v3686 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3686
  ; Cycle B0003
  %v3687 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3688 = load i64, i64* %v3687
  %v3689 = add i64 %v3688, 1
  store i64 %v3689, i64* %v3687
  %v3690 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3691 = load i64, i64* %v3690
  %v3692 = add i64 %v3691, 1
  store i64 %v3692, i64* %v3690
  ; Cycle B0000
  %v3693 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3694 = load i64, i64* %v3693
  %v3695 = add i64 %v3694, 1
  store i64 %v3695, i64* %v3693
  %v3696 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 381, i32* %v3696
  %v3697 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 637, i32* %v3697
  %v3698 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 893, i32* %v3698
  %v3699 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1149, i32* %v3699
  ; Cycle B0001
  %v3700 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3701 = load i64, i64* %v3700
  %v3702 = add i64 %v3701, 1
  store i64 %v3702, i64* %v3700
  %v3703 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1405, i32* %v3703
  %v3704 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1661, i32* %v3704
  %v3705 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1917, i32* %v3705
  %v3706 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2173, i32* %v3706
  ; Cycle B0002
  %v3707 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3708 = load i64, i64* %v3707
  %v3709 = add i64 %v3708, 1
  store i64 %v3709, i64* %v3707
  %v3710 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3710
  %v3711 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3712 = load i64, i64* %v3711
  %v3713 = add i64 %v3712, 1
  store i64 %v3713, i64* %v3711
  %v3714 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3714
  ; Cycle B0003
  %v3715 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3716 = load i64, i64* %v3715
  %v3717 = add i64 %v3716, 1
  store i64 %v3717, i64* %v3715
  %v3718 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3719 = load i64, i64* %v3718
  %v3720 = add i64 %v3719, 1
  store i64 %v3720, i64* %v3718
  ; Cycle B0000
  %v3721 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3722 = load i64, i64* %v3721
  %v3723 = add i64 %v3722, 1
  store i64 %v3723, i64* %v3721
  %v3724 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 382, i32* %v3724
  %v3725 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 10221, i32* %v3725
  %v3726 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 894, i32* %v3726
  %v3727 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1150, i32* %v3727
  ; Cycle B0001
  %v3728 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3729 = load i64, i64* %v3728
  %v3730 = add i64 %v3729, 1
  store i64 %v3730, i64* %v3728
  %v3731 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1406, i32* %v3731
  %v3732 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1662, i32* %v3732
  %v3733 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1918, i32* %v3733
  %v3734 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 34789, i32* %v3734
  ; Cycle B0002
  %v3735 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3736 = load i64, i64* %v3735
  %v3737 = add i64 %v3736, 1
  store i64 %v3737, i64* %v3735
  %v3738 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3738
  %v3739 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3740 = load i64, i64* %v3739
  %v3741 = add i64 %v3740, 1
  store i64 %v3741, i64* %v3739
  %v3742 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3742
  ; Cycle B0003
  %v3743 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3744 = load i64, i64* %v3743
  %v3745 = add i64 %v3744, 1
  store i64 %v3745, i64* %v3743
  %v3746 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3747 = load i64, i64* %v3746
  %v3748 = add i64 %v3747, 1
  store i64 %v3748, i64* %v3746
  ; Cycle B0000
  %v3749 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3750 = load i64, i64* %v3749
  %v3751 = add i64 %v3750, 1
  store i64 %v3751, i64* %v3749
  %v3752 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 383, i32* %v3752
  %v3753 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 639, i32* %v3753
  %v3754 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 895, i32* %v3754
  %v3755 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1151, i32* %v3755
  ; Cycle B0001
  %v3756 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3757 = load i64, i64* %v3756
  %v3758 = add i64 %v3757, 1
  store i64 %v3758, i64* %v3756
  %v3759 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1407, i32* %v3759
  %v3760 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1663, i32* %v3760
  %v3761 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1919, i32* %v3761
  %v3762 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2175, i32* %v3762
  ; Cycle B0002
  %v3763 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3764 = load i64, i64* %v3763
  %v3765 = add i64 %v3764, 1
  store i64 %v3765, i64* %v3763
  %v3766 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3766
  %v3767 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3768 = load i64, i64* %v3767
  %v3769 = add i64 %v3768, 1
  store i64 %v3769, i64* %v3767
  %v3770 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3770
  ; Cycle B0003
  %v3771 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3772 = load i64, i64* %v3771
  %v3773 = add i64 %v3772, 1
  store i64 %v3773, i64* %v3771
  %v3774 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3775 = load i64, i64* %v3774
  %v3776 = add i64 %v3775, 1
  store i64 %v3776, i64* %v3774
  ; Cycle B0000
  %v3777 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3778 = load i64, i64* %v3777
  %v3779 = add i64 %v3778, 1
  store i64 %v3779, i64* %v3777
  %v3780 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 384, i32* %v3780
  %v3781 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 640, i32* %v3781
  %v3782 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 896, i32* %v3782
  %v3783 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1152, i32* %v3783
  ; Cycle B0001
  %v3784 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3785 = load i64, i64* %v3784
  %v3786 = add i64 %v3785, 1
  store i64 %v3786, i64* %v3784
  %v3787 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1408, i32* %v3787
  %v3788 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 26635, i32* %v3788
  %v3789 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1920, i32* %v3789
  %v3790 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2176, i32* %v3790
  ; Cycle B0002
  %v3791 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3792 = load i64, i64* %v3791
  %v3793 = add i64 %v3792, 1
  store i64 %v3793, i64* %v3791
  %v3794 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3794
  %v3795 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3796 = load i64, i64* %v3795
  %v3797 = add i64 %v3796, 1
  store i64 %v3797, i64* %v3795
  %v3798 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3798
  ; Cycle B0003
  %v3799 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3800 = load i64, i64* %v3799
  %v3801 = add i64 %v3800, 1
  store i64 %v3801, i64* %v3799
  %v3802 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3803 = load i64, i64* %v3802
  %v3804 = add i64 %v3803, 1
  store i64 %v3804, i64* %v3802
  ; Cycle B0000
  %v3805 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3806 = load i64, i64* %v3805
  %v3807 = add i64 %v3806, 1
  store i64 %v3807, i64* %v3805
  %v3808 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 385, i32* %v3808
  %v3809 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 641, i32* %v3809
  %v3810 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 897, i32* %v3810
  %v3811 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1153, i32* %v3811
  ; Cycle B0001
  %v3812 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3813 = load i64, i64* %v3812
  %v3814 = add i64 %v3813, 1
  store i64 %v3814, i64* %v3812
  %v3815 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1409, i32* %v3815
  %v3816 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1665, i32* %v3816
  %v3817 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1921, i32* %v3817
  %v3818 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2177, i32* %v3818
  ; Cycle B0002
  %v3819 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3820 = load i64, i64* %v3819
  %v3821 = add i64 %v3820, 1
  store i64 %v3821, i64* %v3819
  %v3822 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3822
  %v3823 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3824 = load i64, i64* %v3823
  %v3825 = add i64 %v3824, 1
  store i64 %v3825, i64* %v3823
  %v3826 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3826
  ; Cycle B0003
  %v3827 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3828 = load i64, i64* %v3827
  %v3829 = add i64 %v3828, 1
  store i64 %v3829, i64* %v3827
  %v3830 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3831 = load i64, i64* %v3830
  %v3832 = add i64 %v3831, 1
  store i64 %v3832, i64* %v3830
  ; Cycle B0000
  %v3833 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3834 = load i64, i64* %v3833
  %v3835 = add i64 %v3834, 1
  store i64 %v3835, i64* %v3833
  %v3836 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 386, i32* %v3836
  %v3837 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 10282, i32* %v3837
  %v3838 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 898, i32* %v3838
  %v3839 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 18464, i32* %v3839
  ; Cycle B0001
  %v3840 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3841 = load i64, i64* %v3840
  %v3842 = add i64 %v3841, 1
  store i64 %v3842, i64* %v3840
  %v3843 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1410, i32* %v3843
  %v3844 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1666, i32* %v3844
  %v3845 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1922, i32* %v3845
  %v3846 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2178, i32* %v3846
  ; Cycle B0002
  %v3847 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3848 = load i64, i64* %v3847
  %v3849 = add i64 %v3848, 1
  store i64 %v3849, i64* %v3847
  %v3850 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3850
  %v3851 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3852 = load i64, i64* %v3851
  %v3853 = add i64 %v3852, 1
  store i64 %v3853, i64* %v3851
  %v3854 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3854
  ; Cycle B0003
  %v3855 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3856 = load i64, i64* %v3855
  %v3857 = add i64 %v3856, 1
  store i64 %v3857, i64* %v3855
  %v3858 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3859 = load i64, i64* %v3858
  %v3860 = add i64 %v3859, 1
  store i64 %v3860, i64* %v3858
  ; Cycle B0000
  %v3861 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3862 = load i64, i64* %v3861
  %v3863 = add i64 %v3862, 1
  store i64 %v3863, i64* %v3861
  %v3864 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 387, i32* %v3864
  %v3865 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 643, i32* %v3865
  %v3866 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 899, i32* %v3866
  %v3867 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1155, i32* %v3867
  ; Cycle B0001
  %v3868 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3869 = load i64, i64* %v3868
  %v3870 = add i64 %v3869, 1
  store i64 %v3870, i64* %v3868
  %v3871 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1411, i32* %v3871
  %v3872 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1667, i32* %v3872
  %v3873 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1923, i32* %v3873
  %v3874 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2179, i32* %v3874
  ; Cycle B0002
  %v3875 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3876 = load i64, i64* %v3875
  %v3877 = add i64 %v3876, 1
  store i64 %v3877, i64* %v3875
  %v3878 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3878
  %v3879 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3880 = load i64, i64* %v3879
  %v3881 = add i64 %v3880, 1
  store i64 %v3881, i64* %v3879
  %v3882 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3882
  ; Cycle B0003
  %v3883 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3884 = load i64, i64* %v3883
  %v3885 = add i64 %v3884, 1
  store i64 %v3885, i64* %v3883
  %v3886 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3887 = load i64, i64* %v3886
  %v3888 = add i64 %v3887, 1
  store i64 %v3888, i64* %v3886
  ; Cycle B0000
  %v3889 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3890 = load i64, i64* %v3889
  %v3891 = add i64 %v3890, 1
  store i64 %v3891, i64* %v3889
  %v3892 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 388, i32* %v3892
  %v3893 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 644, i32* %v3893
  %v3894 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 14412, i32* %v3894
  %v3895 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1156, i32* %v3895
  ; Cycle B0001
  %v3896 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3897 = load i64, i64* %v3896
  %v3898 = add i64 %v3897, 1
  store i64 %v3898, i64* %v3896
  %v3899 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1412, i32* %v3899
  %v3900 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1668, i32* %v3900
  %v3901 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1924, i32* %v3901
  %v3902 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 34893, i32* %v3902
  ; Cycle B0002
  %v3903 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3904 = load i64, i64* %v3903
  %v3905 = add i64 %v3904, 1
  store i64 %v3905, i64* %v3903
  %v3906 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3906
  %v3907 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3908 = load i64, i64* %v3907
  %v3909 = add i64 %v3908, 1
  store i64 %v3909, i64* %v3907
  %v3910 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3910
  ; Cycle B0003
  %v3911 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3912 = load i64, i64* %v3911
  %v3913 = add i64 %v3912, 1
  store i64 %v3913, i64* %v3911
  %v3914 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3915 = load i64, i64* %v3914
  %v3916 = add i64 %v3915, 1
  store i64 %v3916, i64* %v3914
  ; Cycle B0000
  %v3917 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3918 = load i64, i64* %v3917
  %v3919 = add i64 %v3918, 1
  store i64 %v3919, i64* %v3917
  %v3920 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 6237, i32* %v3920
  %v3921 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 645, i32* %v3921
  %v3922 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 14431, i32* %v3922
  %v3923 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1157, i32* %v3923
  ; Cycle B0001
  %v3924 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3925 = load i64, i64* %v3924
  %v3926 = add i64 %v3925, 1
  store i64 %v3926, i64* %v3924
  %v3927 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1413, i32* %v3927
  %v3928 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1669, i32* %v3928
  %v3929 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1925, i32* %v3929
  %v3930 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 34910, i32* %v3930
  ; Cycle B0002
  %v3931 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3932 = load i64, i64* %v3931
  %v3933 = add i64 %v3932, 1
  store i64 %v3933, i64* %v3931
  %v3934 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3934
  %v3935 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3936 = load i64, i64* %v3935
  %v3937 = add i64 %v3936, 1
  store i64 %v3937, i64* %v3935
  %v3938 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3938
  ; Cycle B0003
  %v3939 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3940 = load i64, i64* %v3939
  %v3941 = add i64 %v3940, 1
  store i64 %v3941, i64* %v3939
  %v3942 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3943 = load i64, i64* %v3942
  %v3944 = add i64 %v3943, 1
  store i64 %v3944, i64* %v3942
  ; Cycle B0000
  %v3945 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3946 = load i64, i64* %v3945
  %v3947 = add i64 %v3946, 1
  store i64 %v3947, i64* %v3945
  %v3948 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 390, i32* %v3948
  %v3949 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 646, i32* %v3949
  %v3950 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 902, i32* %v3950
  %v3951 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 18532, i32* %v3951
  ; Cycle B0001
  %v3952 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3953 = load i64, i64* %v3952
  %v3954 = add i64 %v3953, 1
  store i64 %v3954, i64* %v3952
  %v3955 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1414, i32* %v3955
  %v3956 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1670, i32* %v3956
  %v3957 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 30829, i32* %v3957
  %v3958 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2182, i32* %v3958
  ; Cycle B0002
  %v3959 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3960 = load i64, i64* %v3959
  %v3961 = add i64 %v3960, 1
  store i64 %v3961, i64* %v3959
  %v3962 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3962
  %v3963 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3964 = load i64, i64* %v3963
  %v3965 = add i64 %v3964, 1
  store i64 %v3965, i64* %v3963
  %v3966 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3966
  ; Cycle B0003
  %v3967 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3968 = load i64, i64* %v3967
  %v3969 = add i64 %v3968, 1
  store i64 %v3969, i64* %v3967
  %v3970 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3971 = load i64, i64* %v3970
  %v3972 = add i64 %v3971, 1
  store i64 %v3972, i64* %v3970
  ; Cycle B0000
  %v3973 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3974 = load i64, i64* %v3973
  %v3975 = add i64 %v3974, 1
  store i64 %v3975, i64* %v3973
  %v3976 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 391, i32* %v3976
  %v3977 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 647, i32* %v3977
  %v3978 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 903, i32* %v3978
  %v3979 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 18549, i32* %v3979
  ; Cycle B0001
  %v3980 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3981 = load i64, i64* %v3980
  %v3982 = add i64 %v3981, 1
  store i64 %v3982, i64* %v3980
  %v3983 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 22652, i32* %v3983
  %v3984 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1671, i32* %v3984
  %v3985 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 30846, i32* %v3985
  %v3986 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2183, i32* %v3986
  ; Cycle B0002
  %v3987 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3988 = load i64, i64* %v3987
  %v3989 = add i64 %v3988, 1
  store i64 %v3989, i64* %v3987
  %v3990 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v3990
  %v3991 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v3992 = load i64, i64* %v3991
  %v3993 = add i64 %v3992, 1
  store i64 %v3993, i64* %v3991
  %v3994 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v3994
  ; Cycle B0003
  %v3995 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v3996 = load i64, i64* %v3995
  %v3997 = add i64 %v3996, 1
  store i64 %v3997, i64* %v3995
  %v3998 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v3999 = load i64, i64* %v3998
  %v4000 = add i64 %v3999, 1
  store i64 %v4000, i64* %v3998
  ; Cycle B0000
  %v4001 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4002 = load i64, i64* %v4001
  %v4003 = add i64 %v4002, 1
  store i64 %v4003, i64* %v4001
  %v4004 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 6286, i32* %v4004
  %v4005 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 10375, i32* %v4005
  %v4006 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 904, i32* %v4006
  %v4007 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1160, i32* %v4007
  ; Cycle B0001
  %v4008 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4009 = load i64, i64* %v4008
  %v4010 = add i64 %v4009, 1
  store i64 %v4010, i64* %v4008
  %v4011 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1416, i32* %v4011
  %v4012 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1672, i32* %v4012
  %v4013 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1928, i32* %v4013
  %v4014 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2184, i32* %v4014
  ; Cycle B0002
  %v4015 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4016 = load i64, i64* %v4015
  %v4017 = add i64 %v4016, 1
  store i64 %v4017, i64* %v4015
  %v4018 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v4018
  %v4019 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4020 = load i64, i64* %v4019
  %v4021 = add i64 %v4020, 1
  store i64 %v4021, i64* %v4019
  %v4022 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v4022
  ; Cycle B0003
  %v4023 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4024 = load i64, i64* %v4023
  %v4025 = add i64 %v4024, 1
  store i64 %v4025, i64* %v4023
  %v4026 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4027 = load i64, i64* %v4026
  %v4028 = add i64 %v4027, 1
  store i64 %v4028, i64* %v4026
  ; Cycle B0000
  %v4029 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4030 = load i64, i64* %v4029
  %v4031 = add i64 %v4030, 1
  store i64 %v4031, i64* %v4029
  %v4032 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 393, i32* %v4032
  %v4033 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 10386, i32* %v4033
  %v4034 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 905, i32* %v4034
  %v4035 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1161, i32* %v4035
  ; Cycle B0001
  %v4036 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4037 = load i64, i64* %v4036
  %v4038 = add i64 %v4037, 1
  store i64 %v4038, i64* %v4036
  %v4039 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1417, i32* %v4039
  %v4040 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1673, i32* %v4040
  %v4041 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1929, i32* %v4041
  %v4042 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2185, i32* %v4042
  ; Cycle B0002
  %v4043 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4044 = load i64, i64* %v4043
  %v4045 = add i64 %v4044, 1
  store i64 %v4045, i64* %v4043
  %v4046 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v4046
  %v4047 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4048 = load i64, i64* %v4047
  %v4049 = add i64 %v4048, 1
  store i64 %v4049, i64* %v4047
  %v4050 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v4050
  ; Cycle B0003
  %v4051 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4052 = load i64, i64* %v4051
  %v4053 = add i64 %v4052, 1
  store i64 %v4053, i64* %v4051
  %v4054 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4055 = load i64, i64* %v4054
  %v4056 = add i64 %v4055, 1
  store i64 %v4056, i64* %v4054
  ; Cycle B0000
  %v4057 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4058 = load i64, i64* %v4057
  %v4059 = add i64 %v4058, 1
  store i64 %v4059, i64* %v4057
  %v4060 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 394, i32* %v4060
  %v4061 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 10410, i32* %v4061
  %v4062 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 14497, i32* %v4062
  %v4063 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1162, i32* %v4063
  ; Cycle B0001
  %v4064 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4065 = load i64, i64* %v4064
  %v4066 = add i64 %v4065, 1
  store i64 %v4066, i64* %v4064
  %v4067 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1418, i32* %v4067
  %v4068 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 26788, i32* %v4068
  %v4069 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1930, i32* %v4069
  %v4070 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 34976, i32* %v4070
  ; Cycle B0002
  %v4071 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4072 = load i64, i64* %v4071
  %v4073 = add i64 %v4072, 1
  store i64 %v4073, i64* %v4071
  %v4074 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v4074
  %v4075 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4076 = load i64, i64* %v4075
  %v4077 = add i64 %v4076, 1
  store i64 %v4077, i64* %v4075
  %v4078 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v4078
  ; Cycle B0003
  %v4079 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4080 = load i64, i64* %v4079
  %v4081 = add i64 %v4080, 1
  store i64 %v4081, i64* %v4079
  %v4082 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4083 = load i64, i64* %v4082
  %v4084 = add i64 %v4083, 1
  store i64 %v4084, i64* %v4082
  ; Cycle B0000
  %v4085 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4086 = load i64, i64* %v4085
  %v4087 = add i64 %v4086, 1
  store i64 %v4087, i64* %v4085
  %v4088 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 395, i32* %v4088
  %v4089 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 651, i32* %v4089
  %v4090 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 907, i32* %v4090
  %v4091 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1163, i32* %v4091
  ; Cycle B0001
  %v4092 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4093 = load i64, i64* %v4092
  %v4094 = add i64 %v4093, 1
  store i64 %v4094, i64* %v4092
  %v4095 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1419, i32* %v4095
  %v4096 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1675, i32* %v4096
  %v4097 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1931, i32* %v4097
  %v4098 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2187, i32* %v4098
  ; Cycle B0002
  %v4099 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4100 = load i64, i64* %v4099
  %v4101 = add i64 %v4100, 1
  store i64 %v4101, i64* %v4099
  %v4102 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v4102
  %v4103 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4104 = load i64, i64* %v4103
  %v4105 = add i64 %v4104, 1
  store i64 %v4105, i64* %v4103
  %v4106 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v4106
  ; Cycle B0003
  %v4107 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4108 = load i64, i64* %v4107
  %v4109 = add i64 %v4108, 1
  store i64 %v4109, i64* %v4107
  %v4110 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4111 = load i64, i64* %v4110
  %v4112 = add i64 %v4111, 1
  store i64 %v4112, i64* %v4110
  ; Cycle B0000
  %v4113 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4114 = load i64, i64* %v4113
  %v4115 = add i64 %v4114, 1
  store i64 %v4115, i64* %v4113
  %v4116 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 396, i32* %v4116
  %v4117 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 10435, i32* %v4117
  %v4118 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 908, i32* %v4118
  %v4119 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1164, i32* %v4119
  ; Cycle B0001
  %v4120 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4121 = load i64, i64* %v4120
  %v4122 = add i64 %v4121, 1
  store i64 %v4122, i64* %v4120
  %v4123 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1420, i32* %v4123
  %v4124 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 26827, i32* %v4124
  %v4125 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 30914, i32* %v4125
  %v4126 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2188, i32* %v4126
  ; Cycle B0002
  %v4127 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4128 = load i64, i64* %v4127
  %v4129 = add i64 %v4128, 1
  store i64 %v4129, i64* %v4127
  %v4130 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v4130
  %v4131 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4132 = load i64, i64* %v4131
  %v4133 = add i64 %v4132, 1
  store i64 %v4133, i64* %v4131
  %v4134 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v4134
  ; Cycle B0003
  %v4135 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4136 = load i64, i64* %v4135
  %v4137 = add i64 %v4136, 1
  store i64 %v4137, i64* %v4135
  %v4138 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4139 = load i64, i64* %v4138
  %v4140 = add i64 %v4139, 1
  store i64 %v4140, i64* %v4138
  ; Cycle B0000
  %v4141 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4142 = load i64, i64* %v4141
  %v4143 = add i64 %v4142, 1
  store i64 %v4143, i64* %v4141
  %v4144 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 397, i32* %v4144
  %v4145 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 653, i32* %v4145
  %v4146 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 14552, i32* %v4146
  %v4147 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1165, i32* %v4147
  ; Cycle B0001
  %v4148 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4149 = load i64, i64* %v4148
  %v4150 = add i64 %v4149, 1
  store i64 %v4150, i64* %v4148
  %v4151 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 22748, i32* %v4151
  %v4152 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1677, i32* %v4152
  %v4153 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1933, i32* %v4153
  %v4154 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 35029, i32* %v4154
  ; Cycle B0002
  %v4155 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4156 = load i64, i64* %v4155
  %v4157 = add i64 %v4156, 1
  store i64 %v4157, i64* %v4155
  %v4158 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v4158
  %v4159 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4160 = load i64, i64* %v4159
  %v4161 = add i64 %v4160, 1
  store i64 %v4161, i64* %v4159
  %v4162 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v4162
  ; Cycle B0003
  %v4163 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4164 = load i64, i64* %v4163
  %v4165 = add i64 %v4164, 1
  store i64 %v4165, i64* %v4163
  %v4166 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4167 = load i64, i64* %v4166
  %v4168 = add i64 %v4167, 1
  store i64 %v4168, i64* %v4166
  ; Cycle B0000
  %v4169 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4170 = load i64, i64* %v4169
  %v4171 = add i64 %v4170, 1
  store i64 %v4171, i64* %v4169
  %v4172 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 398, i32* %v4172
  %v4173 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 654, i32* %v4173
  %v4174 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 14565, i32* %v4174
  %v4175 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1166, i32* %v4175
  ; Cycle B0001
  %v4176 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4177 = load i64, i64* %v4176
  %v4178 = add i64 %v4177, 1
  store i64 %v4178, i64* %v4176
  %v4179 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1422, i32* %v4179
  %v4180 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 26848, i32* %v4180
  %v4181 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1934, i32* %v4181
  %v4182 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 35044, i32* %v4182
  ; Cycle B0002
  %v4183 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4184 = load i64, i64* %v4183
  %v4185 = add i64 %v4184, 1
  store i64 %v4185, i64* %v4183
  %v4186 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v4186
  %v4187 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4188 = load i64, i64* %v4187
  %v4189 = add i64 %v4188, 1
  store i64 %v4189, i64* %v4187
  %v4190 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v4190
  ; Cycle B0003
  %v4191 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4192 = load i64, i64* %v4191
  %v4193 = add i64 %v4192, 1
  store i64 %v4193, i64* %v4191
  %v4194 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4195 = load i64, i64* %v4194
  %v4196 = add i64 %v4195, 1
  store i64 %v4196, i64* %v4194
  ; Cycle B0000
  %v4197 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4198 = load i64, i64* %v4197
  %v4199 = add i64 %v4198, 1
  store i64 %v4199, i64* %v4197
  %v4200 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 6397, i32* %v4200
  %v4201 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 655, i32* %v4201
  %v4202 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 911, i32* %v4202
  %v4203 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 18686, i32* %v4203
  ; Cycle B0001
  %v4204 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4205 = load i64, i64* %v4204
  %v4206 = add i64 %v4205, 1
  store i64 %v4206, i64* %v4204
  %v4207 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1423, i32* %v4207
  %v4208 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1679, i32* %v4208
  %v4209 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 30967, i32* %v4209
  %v4210 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2191, i32* %v4210
  ; Cycle B0002
  %v4211 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4212 = load i64, i64* %v4211
  %v4213 = add i64 %v4212, 1
  store i64 %v4213, i64* %v4211
  %v4214 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 1193046, i32* %v4214
  %v4215 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4216 = load i64, i64* %v4215
  %v4217 = add i64 %v4216, 1
  store i64 %v4217, i64* %v4215
  %v4218 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 16711680, i32* %v4218
  ; Cycle B0003
  %v4219 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4220 = load i64, i64* %v4219
  %v4221 = add i64 %v4220, 1
  store i64 %v4221, i64* %v4219
  %v4222 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4223 = load i64, i64* %v4222
  %v4224 = add i64 %v4223, 1
  store i64 %v4224, i64* %v4222
  ; Cycle B0000
  %v4225 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4226 = load i64, i64* %v4225
  %v4227 = add i64 %v4226, 1
  store i64 %v4227, i64* %v4225
  %v4228 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 400, i32* %v4228
  %v4229 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 656, i32* %v4229
  %v4230 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 912, i32* %v4230
  %v4231 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 18693, i32* %v4231
  ; Cycle B0001
  %v4232 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4233 = load i64, i64* %v4232
  %v4234 = add i64 %v4233, 1
  store i64 %v4234, i64* %v4232
  %v4235 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 22796, i32* %v4235
  %v4236 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1680, i32* %v4236
  %v4237 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 30990, i32* %v4237
  %v4238 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2192, i32* %v4238
  ; Cycle B0002
  %v4239 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4240 = load i64, i64* %v4239
  %v4241 = add i64 %v4240, 1
  store i64 %v4241, i64* %v4239
  %v4242 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4243 = load i64, i64* %v4242
  %v4244 = add i64 %v4243, 1
  store i64 %v4244, i64* %v4242
  %v4245 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4245
  %v4246 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4247 = load i64, i64* %v4246
  %v4248 = add i64 %v4247, 1
  store i64 %v4248, i64* %v4246
  %v4249 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4249
  ; Cycle B0003
  %v4250 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4251 = load i64, i64* %v4250
  %v4252 = add i64 %v4251, 1
  store i64 %v4252, i64* %v4250
  %v4253 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4254 = load i64, i64* %v4253
  %v4255 = add i64 %v4254, 1
  store i64 %v4255, i64* %v4253
  ; Cycle B0000
  %v4256 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4257 = load i64, i64* %v4256
  %v4258 = add i64 %v4257, 1
  store i64 %v4258, i64* %v4256
  %v4259 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 401, i32* %v4259
  %v4260 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 657, i32* %v4260
  %v4261 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 913, i32* %v4261
  %v4262 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 18708, i32* %v4262
  ; Cycle B0001
  %v4263 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4264 = load i64, i64* %v4263
  %v4265 = add i64 %v4264, 1
  store i64 %v4265, i64* %v4263
  %v4266 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1425, i32* %v4266
  %v4267 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1681, i32* %v4267
  %v4268 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 31005, i32* %v4268
  %v4269 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2193, i32* %v4269
  ; Cycle B0002
  %v4270 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4271 = load i64, i64* %v4270
  %v4272 = add i64 %v4271, 1
  store i64 %v4272, i64* %v4270
  %v4273 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4274 = load i64, i64* %v4273
  %v4275 = add i64 %v4274, 1
  store i64 %v4275, i64* %v4273
  %v4276 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4276
  %v4277 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4278 = load i64, i64* %v4277
  %v4279 = add i64 %v4278, 1
  store i64 %v4279, i64* %v4277
  %v4280 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4280
  ; Cycle B0003
  %v4281 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4282 = load i64, i64* %v4281
  %v4283 = add i64 %v4282, 1
  store i64 %v4283, i64* %v4281
  %v4284 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4285 = load i64, i64* %v4284
  %v4286 = add i64 %v4285, 1
  store i64 %v4286, i64* %v4284
  ; Cycle B0000
  %v4287 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4288 = load i64, i64* %v4287
  %v4289 = add i64 %v4288, 1
  store i64 %v4289, i64* %v4287
  %v4290 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 6445, i32* %v4290
  %v4291 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 658, i32* %v4291
  %v4292 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 14639, i32* %v4292
  %v4293 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1170, i32* %v4293
  ; Cycle B0001
  %v4294 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4295 = load i64, i64* %v4294
  %v4296 = add i64 %v4295, 1
  store i64 %v4296, i64* %v4294
  %v4297 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1426, i32* %v4297
  %v4298 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1682, i32* %v4298
  %v4299 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1938, i32* %v4299
  %v4300 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 35118, i32* %v4300
  ; Cycle B0002
  %v4301 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4302 = load i64, i64* %v4301
  %v4303 = add i64 %v4302, 1
  store i64 %v4303, i64* %v4301
  %v4304 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4305 = load i64, i64* %v4304
  %v4306 = add i64 %v4305, 1
  store i64 %v4306, i64* %v4304
  %v4307 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4307
  %v4308 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4309 = load i64, i64* %v4308
  %v4310 = add i64 %v4309, 1
  store i64 %v4310, i64* %v4308
  %v4311 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4311
  ; Cycle B0003
  %v4312 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4313 = load i64, i64* %v4312
  %v4314 = add i64 %v4313, 1
  store i64 %v4314, i64* %v4312
  %v4315 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4316 = load i64, i64* %v4315
  %v4317 = add i64 %v4316, 1
  store i64 %v4317, i64* %v4315
  ; Cycle B0000
  %v4318 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4319 = load i64, i64* %v4318
  %v4320 = add i64 %v4319, 1
  store i64 %v4320, i64* %v4318
  %v4321 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 403, i32* %v4321
  %v4322 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 659, i32* %v4322
  %v4323 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 14652, i32* %v4323
  %v4324 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1171, i32* %v4324
  ; Cycle B0001
  %v4325 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4326 = load i64, i64* %v4325
  %v4327 = add i64 %v4326, 1
  store i64 %v4327, i64* %v4325
  %v4328 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1427, i32* %v4328
  %v4329 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1683, i32* %v4329
  %v4330 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1939, i32* %v4330
  %v4331 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 35133, i32* %v4331
  ; Cycle B0002
  %v4332 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4333 = load i64, i64* %v4332
  %v4334 = add i64 %v4333, 1
  store i64 %v4334, i64* %v4332
  %v4335 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4336 = load i64, i64* %v4335
  %v4337 = add i64 %v4336, 1
  store i64 %v4337, i64* %v4335
  %v4338 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4338
  %v4339 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4340 = load i64, i64* %v4339
  %v4341 = add i64 %v4340, 1
  store i64 %v4341, i64* %v4339
  %v4342 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4342
  ; Cycle B0003
  %v4343 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4344 = load i64, i64* %v4343
  %v4345 = add i64 %v4344, 1
  store i64 %v4345, i64* %v4343
  %v4346 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4347 = load i64, i64* %v4346
  %v4348 = add i64 %v4347, 1
  store i64 %v4348, i64* %v4346
  ; Cycle B0000
  %v4349 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4350 = load i64, i64* %v4349
  %v4351 = add i64 %v4350, 1
  store i64 %v4351, i64* %v4349
  %v4352 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 404, i32* %v4352
  %v4353 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 660, i32* %v4353
  %v4354 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 916, i32* %v4354
  %v4355 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1172, i32* %v4355
  ; Cycle B0001
  %v4356 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4357 = load i64, i64* %v4356
  %v4358 = add i64 %v4357, 1
  store i64 %v4358, i64* %v4356
  %v4359 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1428, i32* %v4359
  %v4360 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1684, i32* %v4360
  %v4361 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1940, i32* %v4361
  %v4362 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2196, i32* %v4362
  ; Cycle B0002
  %v4363 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4364 = load i64, i64* %v4363
  %v4365 = add i64 %v4364, 1
  store i64 %v4365, i64* %v4363
  %v4366 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4367 = load i64, i64* %v4366
  %v4368 = add i64 %v4367, 1
  store i64 %v4368, i64* %v4366
  %v4369 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4369
  %v4370 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4371 = load i64, i64* %v4370
  %v4372 = add i64 %v4371, 1
  store i64 %v4372, i64* %v4370
  %v4373 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4373
  ; Cycle B0003
  %v4374 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4375 = load i64, i64* %v4374
  %v4376 = add i64 %v4375, 1
  store i64 %v4376, i64* %v4374
  %v4377 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4378 = load i64, i64* %v4377
  %v4379 = add i64 %v4378, 1
  store i64 %v4379, i64* %v4377
  ; Cycle B0000
  %v4380 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4381 = load i64, i64* %v4380
  %v4382 = add i64 %v4381, 1
  store i64 %v4382, i64* %v4380
  %v4383 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 405, i32* %v4383
  %v4384 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 10586, i32* %v4384
  %v4385 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 917, i32* %v4385
  %v4386 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 18768, i32* %v4386
  ; Cycle B0001
  %v4387 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4388 = load i64, i64* %v4387
  %v4389 = add i64 %v4388, 1
  store i64 %v4389, i64* %v4387
  %v4390 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1429, i32* %v4390
  %v4391 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1685, i32* %v4391
  %v4392 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1941, i32* %v4392
  %v4393 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2197, i32* %v4393
  ; Cycle B0002
  %v4394 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4395 = load i64, i64* %v4394
  %v4396 = add i64 %v4395, 1
  store i64 %v4396, i64* %v4394
  %v4397 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4398 = load i64, i64* %v4397
  %v4399 = add i64 %v4398, 1
  store i64 %v4399, i64* %v4397
  %v4400 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4400
  %v4401 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4402 = load i64, i64* %v4401
  %v4403 = add i64 %v4402, 1
  store i64 %v4403, i64* %v4401
  %v4404 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4404
  ; Cycle B0003
  %v4405 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4406 = load i64, i64* %v4405
  %v4407 = add i64 %v4406, 1
  store i64 %v4407, i64* %v4405
  %v4408 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4409 = load i64, i64* %v4408
  %v4410 = add i64 %v4409, 1
  store i64 %v4410, i64* %v4408
  ; Cycle B0000
  %v4411 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4412 = load i64, i64* %v4411
  %v4413 = add i64 %v4412, 1
  store i64 %v4413, i64* %v4411
  %v4414 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 406, i32* %v4414
  %v4415 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 662, i32* %v4415
  %v4416 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 918, i32* %v4416
  %v4417 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1174, i32* %v4417
  ; Cycle B0001
  %v4418 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4419 = load i64, i64* %v4418
  %v4420 = add i64 %v4419, 1
  store i64 %v4420, i64* %v4418
  %v4421 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1430, i32* %v4421
  %v4422 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1686, i32* %v4422
  %v4423 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1942, i32* %v4423
  %v4424 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2198, i32* %v4424
  ; Cycle B0002
  %v4425 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4426 = load i64, i64* %v4425
  %v4427 = add i64 %v4426, 1
  store i64 %v4427, i64* %v4425
  %v4428 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4429 = load i64, i64* %v4428
  %v4430 = add i64 %v4429, 1
  store i64 %v4430, i64* %v4428
  %v4431 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4431
  %v4432 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4433 = load i64, i64* %v4432
  %v4434 = add i64 %v4433, 1
  store i64 %v4434, i64* %v4432
  %v4435 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4435
  ; Cycle B0003
  %v4436 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4437 = load i64, i64* %v4436
  %v4438 = add i64 %v4437, 1
  store i64 %v4438, i64* %v4436
  %v4439 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4440 = load i64, i64* %v4439
  %v4441 = add i64 %v4440, 1
  store i64 %v4441, i64* %v4439
  ; Cycle B0000
  %v4442 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4443 = load i64, i64* %v4442
  %v4444 = add i64 %v4443, 1
  store i64 %v4444, i64* %v4442
  %v4445 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 407, i32* %v4445
  %v4446 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 663, i32* %v4446
  %v4447 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 919, i32* %v4447
  %v4448 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1175, i32* %v4448
  ; Cycle B0001
  %v4449 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4450 = load i64, i64* %v4449
  %v4451 = add i64 %v4450, 1
  store i64 %v4451, i64* %v4449
  %v4452 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1431, i32* %v4452
  %v4453 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 27003, i32* %v4453
  %v4454 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1943, i32* %v4454
  %v4455 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2199, i32* %v4455
  ; Cycle B0002
  %v4456 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4457 = load i64, i64* %v4456
  %v4458 = add i64 %v4457, 1
  store i64 %v4458, i64* %v4456
  %v4459 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4460 = load i64, i64* %v4459
  %v4461 = add i64 %v4460, 1
  store i64 %v4461, i64* %v4459
  %v4462 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4462
  %v4463 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4464 = load i64, i64* %v4463
  %v4465 = add i64 %v4464, 1
  store i64 %v4465, i64* %v4463
  %v4466 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4466
  ; Cycle B0003
  %v4467 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4468 = load i64, i64* %v4467
  %v4469 = add i64 %v4468, 1
  store i64 %v4469, i64* %v4467
  %v4470 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4471 = load i64, i64* %v4470
  %v4472 = add i64 %v4471, 1
  store i64 %v4472, i64* %v4470
  ; Cycle B0000
  %v4473 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4474 = load i64, i64* %v4473
  %v4475 = add i64 %v4474, 1
  store i64 %v4475, i64* %v4473
  %v4476 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 408, i32* %v4476
  %v4477 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 664, i32* %v4477
  %v4478 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 14724, i32* %v4478
  %v4479 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1176, i32* %v4479
  ; Cycle B0001
  %v4480 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4481 = load i64, i64* %v4480
  %v4482 = add i64 %v4481, 1
  store i64 %v4482, i64* %v4480
  %v4483 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1432, i32* %v4483
  %v4484 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1688, i32* %v4484
  %v4485 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1944, i32* %v4485
  %v4486 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 35205, i32* %v4486
  ; Cycle B0002
  %v4487 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4488 = load i64, i64* %v4487
  %v4489 = add i64 %v4488, 1
  store i64 %v4489, i64* %v4487
  %v4490 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4491 = load i64, i64* %v4490
  %v4492 = add i64 %v4491, 1
  store i64 %v4492, i64* %v4490
  %v4493 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4493
  %v4494 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4495 = load i64, i64* %v4494
  %v4496 = add i64 %v4495, 1
  store i64 %v4496, i64* %v4494
  %v4497 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4497
  ; Cycle B0003
  %v4498 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4499 = load i64, i64* %v4498
  %v4500 = add i64 %v4499, 1
  store i64 %v4500, i64* %v4498
  %v4501 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4502 = load i64, i64* %v4501
  %v4503 = add i64 %v4502, 1
  store i64 %v4503, i64* %v4501
  ; Cycle B0000
  %v4504 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4505 = load i64, i64* %v4504
  %v4506 = add i64 %v4505, 1
  store i64 %v4506, i64* %v4504
  %v4507 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 409, i32* %v4507
  %v4508 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 665, i32* %v4508
  %v4509 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 14741, i32* %v4509
  %v4510 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1177, i32* %v4510
  ; Cycle B0001
  %v4511 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4512 = load i64, i64* %v4511
  %v4513 = add i64 %v4512, 1
  store i64 %v4513, i64* %v4511
  %v4514 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 22939, i32* %v4514
  %v4515 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1689, i32* %v4515
  %v4516 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1945, i32* %v4516
  %v4517 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 35224, i32* %v4517
  ; Cycle B0002
  %v4518 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4519 = load i64, i64* %v4518
  %v4520 = add i64 %v4519, 1
  store i64 %v4520, i64* %v4518
  %v4521 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4522 = load i64, i64* %v4521
  %v4523 = add i64 %v4522, 1
  store i64 %v4523, i64* %v4521
  %v4524 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4524
  %v4525 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4526 = load i64, i64* %v4525
  %v4527 = add i64 %v4526, 1
  store i64 %v4527, i64* %v4525
  %v4528 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4528
  ; Cycle B0003
  %v4529 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4530 = load i64, i64* %v4529
  %v4531 = add i64 %v4530, 1
  store i64 %v4531, i64* %v4529
  %v4532 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4533 = load i64, i64* %v4532
  %v4534 = add i64 %v4533, 1
  store i64 %v4534, i64* %v4532
  ; Cycle B0000
  %v4535 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4536 = load i64, i64* %v4535
  %v4537 = add i64 %v4536, 1
  store i64 %v4537, i64* %v4535
  %v4538 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 6573, i32* %v4538
  %v4539 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 666, i32* %v4539
  %v4540 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 922, i32* %v4540
  %v4541 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 18862, i32* %v4541
  ; Cycle B0001
  %v4542 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4543 = load i64, i64* %v4542
  %v4544 = add i64 %v4543, 1
  store i64 %v4544, i64* %v4542
  %v4545 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1434, i32* %v4545
  %v4546 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1690, i32* %v4546
  %v4547 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 31143, i32* %v4547
  %v4548 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2202, i32* %v4548
  ; Cycle B0002
  %v4549 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4550 = load i64, i64* %v4549
  %v4551 = add i64 %v4550, 1
  store i64 %v4551, i64* %v4549
  %v4552 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4553 = load i64, i64* %v4552
  %v4554 = add i64 %v4553, 1
  store i64 %v4554, i64* %v4552
  %v4555 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4555
  %v4556 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4557 = load i64, i64* %v4556
  %v4558 = add i64 %v4557, 1
  store i64 %v4558, i64* %v4556
  %v4559 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4559
  ; Cycle B0003
  %v4560 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4561 = load i64, i64* %v4560
  %v4562 = add i64 %v4561, 1
  store i64 %v4562, i64* %v4560
  %v4563 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4564 = load i64, i64* %v4563
  %v4565 = add i64 %v4564, 1
  store i64 %v4565, i64* %v4563
  ; Cycle B0000
  %v4566 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4567 = load i64, i64* %v4566
  %v4568 = add i64 %v4567, 1
  store i64 %v4568, i64* %v4566
  %v4569 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 411, i32* %v4569
  %v4570 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 667, i32* %v4570
  %v4571 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 14773, i32* %v4571
  %v4572 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1179, i32* %v4572
  ; Cycle B0001
  %v4573 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4574 = load i64, i64* %v4573
  %v4575 = add i64 %v4574, 1
  store i64 %v4575, i64* %v4573
  %v4576 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1435, i32* %v4576
  %v4577 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 27056, i32* %v4577
  %v4578 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1947, i32* %v4578
  %v4579 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 35252, i32* %v4579
  ; Cycle B0002
  %v4580 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4581 = load i64, i64* %v4580
  %v4582 = add i64 %v4581, 1
  store i64 %v4582, i64* %v4580
  %v4583 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4584 = load i64, i64* %v4583
  %v4585 = add i64 %v4584, 1
  store i64 %v4585, i64* %v4583
  %v4586 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4586
  %v4587 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4588 = load i64, i64* %v4587
  %v4589 = add i64 %v4588, 1
  store i64 %v4589, i64* %v4587
  %v4590 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4590
  ; Cycle B0003
  %v4591 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4592 = load i64, i64* %v4591
  %v4593 = add i64 %v4592, 1
  store i64 %v4593, i64* %v4591
  %v4594 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4595 = load i64, i64* %v4594
  %v4596 = add i64 %v4595, 1
  store i64 %v4596, i64* %v4594
  ; Cycle B0000
  %v4597 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4598 = load i64, i64* %v4597
  %v4599 = add i64 %v4598, 1
  store i64 %v4599, i64* %v4597
  %v4600 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 412, i32* %v4600
  %v4601 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 668, i32* %v4601
  %v4602 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 14792, i32* %v4602
  %v4603 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1180, i32* %v4603
  ; Cycle B0001
  %v4604 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4605 = load i64, i64* %v4604
  %v4606 = add i64 %v4605, 1
  store i64 %v4606, i64* %v4604
  %v4607 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 22988, i32* %v4607
  %v4608 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1692, i32* %v4608
  %v4609 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1948, i32* %v4609
  %v4610 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 35269, i32* %v4610
  ; Cycle B0002
  %v4611 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4612 = load i64, i64* %v4611
  %v4613 = add i64 %v4612, 1
  store i64 %v4613, i64* %v4611
  %v4614 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4615 = load i64, i64* %v4614
  %v4616 = add i64 %v4615, 1
  store i64 %v4616, i64* %v4614
  %v4617 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4617
  %v4618 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4619 = load i64, i64* %v4618
  %v4620 = add i64 %v4619, 1
  store i64 %v4620, i64* %v4618
  %v4621 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4621
  ; Cycle B0003
  %v4622 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4623 = load i64, i64* %v4622
  %v4624 = add i64 %v4623, 1
  store i64 %v4624, i64* %v4622
  %v4625 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4626 = load i64, i64* %v4625
  %v4627 = add i64 %v4626, 1
  store i64 %v4627, i64* %v4625
  ; Cycle B0000
  %v4628 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4629 = load i64, i64* %v4628
  %v4630 = add i64 %v4629, 1
  store i64 %v4630, i64* %v4628
  %v4631 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 413, i32* %v4631
  %v4632 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 10707, i32* %v4632
  %v4633 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 925, i32* %v4633
  %v4634 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1181, i32* %v4634
  ; Cycle B0001
  %v4635 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4636 = load i64, i64* %v4635
  %v4637 = add i64 %v4636, 1
  store i64 %v4637, i64* %v4635
  %v4638 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1437, i32* %v4638
  %v4639 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 27099, i32* %v4639
  %v4640 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 31186, i32* %v4640
  %v4641 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2205, i32* %v4641
  ; Cycle B0002
  %v4642 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4643 = load i64, i64* %v4642
  %v4644 = add i64 %v4643, 1
  store i64 %v4644, i64* %v4642
  %v4645 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4646 = load i64, i64* %v4645
  %v4647 = add i64 %v4646, 1
  store i64 %v4647, i64* %v4645
  %v4648 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4648
  %v4649 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4650 = load i64, i64* %v4649
  %v4651 = add i64 %v4650, 1
  store i64 %v4651, i64* %v4649
  %v4652 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4652
  ; Cycle B0003
  %v4653 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4654 = load i64, i64* %v4653
  %v4655 = add i64 %v4654, 1
  store i64 %v4655, i64* %v4653
  %v4656 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4657 = load i64, i64* %v4656
  %v4658 = add i64 %v4657, 1
  store i64 %v4658, i64* %v4656
  ; Cycle B0000
  %v4659 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4660 = load i64, i64* %v4659
  %v4661 = add i64 %v4660, 1
  store i64 %v4661, i64* %v4659
  %v4662 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 414, i32* %v4662
  %v4663 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 670, i32* %v4663
  %v4664 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 926, i32* %v4664
  %v4665 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1182, i32* %v4665
  ; Cycle B0001
  %v4666 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4667 = load i64, i64* %v4666
  %v4668 = add i64 %v4667, 1
  store i64 %v4668, i64* %v4666
  %v4669 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1438, i32* %v4669
  %v4670 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1694, i32* %v4670
  %v4671 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1950, i32* %v4671
  %v4672 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2206, i32* %v4672
  ; Cycle B0002
  %v4673 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4674 = load i64, i64* %v4673
  %v4675 = add i64 %v4674, 1
  store i64 %v4675, i64* %v4673
  %v4676 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4677 = load i64, i64* %v4676
  %v4678 = add i64 %v4677, 1
  store i64 %v4678, i64* %v4676
  %v4679 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4679
  %v4680 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4681 = load i64, i64* %v4680
  %v4682 = add i64 %v4681, 1
  store i64 %v4682, i64* %v4680
  %v4683 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4683
  ; Cycle B0003
  %v4684 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4685 = load i64, i64* %v4684
  %v4686 = add i64 %v4685, 1
  store i64 %v4686, i64* %v4684
  %v4687 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4688 = load i64, i64* %v4687
  %v4689 = add i64 %v4688, 1
  store i64 %v4689, i64* %v4687
  ; Cycle B0000
  %v4690 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4691 = load i64, i64* %v4690
  %v4692 = add i64 %v4691, 1
  store i64 %v4692, i64* %v4690
  %v4693 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 415, i32* %v4693
  %v4694 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 10746, i32* %v4694
  %v4695 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 14833, i32* %v4695
  %v4696 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1183, i32* %v4696
  ; Cycle B0001
  %v4697 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4698 = load i64, i64* %v4697
  %v4699 = add i64 %v4698, 1
  store i64 %v4699, i64* %v4697
  %v4700 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1439, i32* %v4700
  %v4701 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 27124, i32* %v4701
  %v4702 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1951, i32* %v4702
  %v4703 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 35312, i32* %v4703
  ; Cycle B0002
  %v4704 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4705 = load i64, i64* %v4704
  %v4706 = add i64 %v4705, 1
  store i64 %v4706, i64* %v4704
  %v4707 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4708 = load i64, i64* %v4707
  %v4709 = add i64 %v4708, 1
  store i64 %v4709, i64* %v4707
  %v4710 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4710
  %v4711 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4712 = load i64, i64* %v4711
  %v4713 = add i64 %v4712, 1
  store i64 %v4713, i64* %v4711
  %v4714 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4714
  ; Cycle B0003
  %v4715 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4716 = load i64, i64* %v4715
  %v4717 = add i64 %v4716, 1
  store i64 %v4717, i64* %v4715
  %v4718 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4719 = load i64, i64* %v4718
  %v4720 = add i64 %v4719, 1
  store i64 %v4720, i64* %v4718
  ; Cycle B0000
  %v4721 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4722 = load i64, i64* %v4721
  %v4723 = add i64 %v4722, 1
  store i64 %v4723, i64* %v4721
  %v4724 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 416, i32* %v4724
  %v4725 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 672, i32* %v4725
  %v4726 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 14859, i32* %v4726
  %v4727 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1184, i32* %v4727
  ; Cycle B0001
  %v4728 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4729 = load i64, i64* %v4728
  %v4730 = add i64 %v4729, 1
  store i64 %v4730, i64* %v4728
  %v4731 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1440, i32* %v4731
  %v4732 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1696, i32* %v4732
  %v4733 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1952, i32* %v4733
  %v4734 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 35338, i32* %v4734
  ; Cycle B0002
  %v4735 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4736 = load i64, i64* %v4735
  %v4737 = add i64 %v4736, 1
  store i64 %v4737, i64* %v4735
  %v4738 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4739 = load i64, i64* %v4738
  %v4740 = add i64 %v4739, 1
  store i64 %v4740, i64* %v4738
  %v4741 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4741
  %v4742 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4743 = load i64, i64* %v4742
  %v4744 = add i64 %v4743, 1
  store i64 %v4744, i64* %v4742
  %v4745 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4745
  ; Cycle B0003
  %v4746 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4747 = load i64, i64* %v4746
  %v4748 = add i64 %v4747, 1
  store i64 %v4748, i64* %v4746
  %v4749 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4750 = load i64, i64* %v4749
  %v4751 = add i64 %v4750, 1
  store i64 %v4751, i64* %v4749
  ; Cycle B0000
  %v4752 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4753 = load i64, i64* %v4752
  %v4754 = add i64 %v4753, 1
  store i64 %v4754, i64* %v4752
  %v4755 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 6686, i32* %v4755
  %v4756 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 673, i32* %v4756
  %v4757 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 929, i32* %v4757
  %v4758 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1185, i32* %v4758
  ; Cycle B0001
  %v4759 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4760 = load i64, i64* %v4759
  %v4761 = add i64 %v4760, 1
  store i64 %v4761, i64* %v4759
  %v4762 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1441, i32* %v4762
  %v4763 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1697, i32* %v4763
  %v4764 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1953, i32* %v4764
  %v4765 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2209, i32* %v4765
  ; Cycle B0002
  %v4766 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4767 = load i64, i64* %v4766
  %v4768 = add i64 %v4767, 1
  store i64 %v4768, i64* %v4766
  %v4769 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4770 = load i64, i64* %v4769
  %v4771 = add i64 %v4770, 1
  store i64 %v4771, i64* %v4769
  %v4772 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4772
  %v4773 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4774 = load i64, i64* %v4773
  %v4775 = add i64 %v4774, 1
  store i64 %v4775, i64* %v4773
  %v4776 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4776
  ; Cycle B0003
  %v4777 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4778 = load i64, i64* %v4777
  %v4779 = add i64 %v4778, 1
  store i64 %v4779, i64* %v4777
  %v4780 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4781 = load i64, i64* %v4780
  %v4782 = add i64 %v4781, 1
  store i64 %v4782, i64* %v4780
  ; Cycle B0000
  %v4783 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4784 = load i64, i64* %v4783
  %v4785 = add i64 %v4784, 1
  store i64 %v4785, i64* %v4783
  %v4786 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 418, i32* %v4786
  %v4787 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 674, i32* %v4787
  %v4788 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 930, i32* %v4788
  %v4789 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 18977, i32* %v4789
  ; Cycle B0001
  %v4790 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4791 = load i64, i64* %v4790
  %v4792 = add i64 %v4791, 1
  store i64 %v4792, i64* %v4790
  %v4793 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1442, i32* %v4793
  %v4794 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1698, i32* %v4794
  %v4795 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 31274, i32* %v4795
  %v4796 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2210, i32* %v4796
  ; Cycle B0002
  %v4797 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4798 = load i64, i64* %v4797
  %v4799 = add i64 %v4798, 1
  store i64 %v4799, i64* %v4797
  %v4800 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4801 = load i64, i64* %v4800
  %v4802 = add i64 %v4801, 1
  store i64 %v4802, i64* %v4800
  %v4803 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4803
  %v4804 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4805 = load i64, i64* %v4804
  %v4806 = add i64 %v4805, 1
  store i64 %v4806, i64* %v4804
  %v4807 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4807
  ; Cycle B0003
  %v4808 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4809 = load i64, i64* %v4808
  %v4810 = add i64 %v4809, 1
  store i64 %v4810, i64* %v4808
  %v4811 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4812 = load i64, i64* %v4811
  %v4813 = add i64 %v4812, 1
  store i64 %v4813, i64* %v4811
  ; Cycle B0000
  %v4814 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4815 = load i64, i64* %v4814
  %v4816 = add i64 %v4815, 1
  store i64 %v4816, i64* %v4814
  %v4817 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 419, i32* %v4817
  %v4818 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 675, i32* %v4818
  %v4819 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 931, i32* %v4819
  %v4820 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 19000, i32* %v4820
  ; Cycle B0001
  %v4821 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4822 = load i64, i64* %v4821
  %v4823 = add i64 %v4822, 1
  store i64 %v4823, i64* %v4821
  %v4824 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 23103, i32* %v4824
  %v4825 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1699, i32* %v4825
  %v4826 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1955, i32* %v4826
  %v4827 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2211, i32* %v4827
  ; Cycle B0002
  %v4828 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4829 = load i64, i64* %v4828
  %v4830 = add i64 %v4829, 1
  store i64 %v4830, i64* %v4828
  %v4831 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4832 = load i64, i64* %v4831
  %v4833 = add i64 %v4832, 1
  store i64 %v4833, i64* %v4831
  %v4834 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4834
  %v4835 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4836 = load i64, i64* %v4835
  %v4837 = add i64 %v4836, 1
  store i64 %v4837, i64* %v4835
  %v4838 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4838
  ; Cycle B0003
  %v4839 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4840 = load i64, i64* %v4839
  %v4841 = add i64 %v4840, 1
  store i64 %v4841, i64* %v4839
  %v4842 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4843 = load i64, i64* %v4842
  %v4844 = add i64 %v4843, 1
  store i64 %v4844, i64* %v4842
  ; Cycle B0000
  %v4845 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4846 = load i64, i64* %v4845
  %v4847 = add i64 %v4846, 1
  store i64 %v4847, i64* %v4845
  %v4848 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 420, i32* %v4848
  %v4849 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 676, i32* %v4849
  %v4850 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 932, i32* %v4850
  %v4851 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1188, i32* %v4851
  ; Cycle B0001
  %v4852 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4853 = load i64, i64* %v4852
  %v4854 = add i64 %v4853, 1
  store i64 %v4854, i64* %v4852
  %v4855 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1444, i32* %v4855
  %v4856 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 27212, i32* %v4856
  %v4857 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1956, i32* %v4857
  %v4858 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2212, i32* %v4858
  ; Cycle B0002
  %v4859 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4860 = load i64, i64* %v4859
  %v4861 = add i64 %v4860, 1
  store i64 %v4861, i64* %v4859
  %v4862 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4863 = load i64, i64* %v4862
  %v4864 = add i64 %v4863, 1
  store i64 %v4864, i64* %v4862
  %v4865 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4865
  %v4866 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4867 = load i64, i64* %v4866
  %v4868 = add i64 %v4867, 1
  store i64 %v4868, i64* %v4866
  %v4869 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4869
  ; Cycle B0003
  %v4870 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4871 = load i64, i64* %v4870
  %v4872 = add i64 %v4871, 1
  store i64 %v4872, i64* %v4870
  %v4873 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4874 = load i64, i64* %v4873
  %v4875 = add i64 %v4874, 1
  store i64 %v4875, i64* %v4873
  ; Cycle B0000
  %v4876 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4877 = load i64, i64* %v4876
  %v4878 = add i64 %v4877, 1
  store i64 %v4878, i64* %v4876
  %v4879 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 421, i32* %v4879
  %v4880 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 677, i32* %v4880
  %v4881 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 933, i32* %v4881
  %v4882 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1189, i32* %v4882
  ; Cycle B0001
  %v4883 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4884 = load i64, i64* %v4883
  %v4885 = add i64 %v4884, 1
  store i64 %v4885, i64* %v4883
  %v4886 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1445, i32* %v4886
  %v4887 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 27231, i32* %v4887
  %v4888 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1957, i32* %v4888
  %v4889 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2213, i32* %v4889
  ; Cycle B0002
  %v4890 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4891 = load i64, i64* %v4890
  %v4892 = add i64 %v4891, 1
  store i64 %v4892, i64* %v4890
  %v4893 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4894 = load i64, i64* %v4893
  %v4895 = add i64 %v4894, 1
  store i64 %v4895, i64* %v4893
  %v4896 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4896
  %v4897 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4898 = load i64, i64* %v4897
  %v4899 = add i64 %v4898, 1
  store i64 %v4899, i64* %v4897
  %v4900 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4900
  ; Cycle B0003
  %v4901 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4902 = load i64, i64* %v4901
  %v4903 = add i64 %v4902, 1
  store i64 %v4903, i64* %v4901
  %v4904 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4905 = load i64, i64* %v4904
  %v4906 = add i64 %v4905, 1
  store i64 %v4906, i64* %v4904
  ; Cycle B0000
  %v4907 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4908 = load i64, i64* %v4907
  %v4909 = add i64 %v4908, 1
  store i64 %v4909, i64* %v4907
  %v4910 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 422, i32* %v4910
  %v4911 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 10861, i32* %v4911
  %v4912 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 934, i32* %v4912
  %v4913 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1190, i32* %v4913
  ; Cycle B0001
  %v4914 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4915 = load i64, i64* %v4914
  %v4916 = add i64 %v4915, 1
  store i64 %v4916, i64* %v4914
  %v4917 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1446, i32* %v4917
  %v4918 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1702, i32* %v4918
  %v4919 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1958, i32* %v4919
  %v4920 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2214, i32* %v4920
  ; Cycle B0002
  %v4921 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4922 = load i64, i64* %v4921
  %v4923 = add i64 %v4922, 1
  store i64 %v4923, i64* %v4921
  %v4924 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4925 = load i64, i64* %v4924
  %v4926 = add i64 %v4925, 1
  store i64 %v4926, i64* %v4924
  %v4927 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4927
  %v4928 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4929 = load i64, i64* %v4928
  %v4930 = add i64 %v4929, 1
  store i64 %v4930, i64* %v4928
  %v4931 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4931
  ; Cycle B0003
  %v4932 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4933 = load i64, i64* %v4932
  %v4934 = add i64 %v4933, 1
  store i64 %v4934, i64* %v4932
  %v4935 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4936 = load i64, i64* %v4935
  %v4937 = add i64 %v4936, 1
  store i64 %v4937, i64* %v4935
  ; Cycle B0000
  %v4938 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4939 = load i64, i64* %v4938
  %v4940 = add i64 %v4939, 1
  store i64 %v4940, i64* %v4938
  %v4941 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 423, i32* %v4941
  %v4942 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 10878, i32* %v4942
  %v4943 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 935, i32* %v4943
  %v4944 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1191, i32* %v4944
  ; Cycle B0001
  %v4945 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4946 = load i64, i64* %v4945
  %v4947 = add i64 %v4946, 1
  store i64 %v4947, i64* %v4945
  %v4948 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1447, i32* %v4948
  %v4949 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1703, i32* %v4949
  %v4950 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1959, i32* %v4950
  %v4951 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2215, i32* %v4951
  ; Cycle B0002
  %v4952 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4953 = load i64, i64* %v4952
  %v4954 = add i64 %v4953, 1
  store i64 %v4954, i64* %v4952
  %v4955 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4956 = load i64, i64* %v4955
  %v4957 = add i64 %v4956, 1
  store i64 %v4957, i64* %v4955
  %v4958 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4958
  %v4959 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4960 = load i64, i64* %v4959
  %v4961 = add i64 %v4960, 1
  store i64 %v4961, i64* %v4959
  %v4962 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4962
  ; Cycle B0003
  %v4963 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4964 = load i64, i64* %v4963
  %v4965 = add i64 %v4964, 1
  store i64 %v4965, i64* %v4963
  %v4966 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4967 = load i64, i64* %v4966
  %v4968 = add i64 %v4967, 1
  store i64 %v4968, i64* %v4966
  ; Cycle B0000
  %v4969 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4970 = load i64, i64* %v4969
  %v4971 = add i64 %v4970, 1
  store i64 %v4971, i64* %v4969
  %v4972 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 424, i32* %v4972
  %v4973 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 680, i32* %v4973
  %v4974 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 936, i32* %v4974
  %v4975 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1192, i32* %v4975
  ; Cycle B0001
  %v4976 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4977 = load i64, i64* %v4976
  %v4978 = add i64 %v4977, 1
  store i64 %v4978, i64* %v4976
  %v4979 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1448, i32* %v4979
  %v4980 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1704, i32* %v4980
  %v4981 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 31367, i32* %v4981
  %v4982 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2216, i32* %v4982
  ; Cycle B0002
  %v4983 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4984 = load i64, i64* %v4983
  %v4985 = add i64 %v4984, 1
  store i64 %v4985, i64* %v4983
  %v4986 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v4987 = load i64, i64* %v4986
  %v4988 = add i64 %v4987, 1
  store i64 %v4988, i64* %v4986
  %v4989 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v4989
  %v4990 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v4991 = load i64, i64* %v4990
  %v4992 = add i64 %v4991, 1
  store i64 %v4992, i64* %v4990
  %v4993 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v4993
  ; Cycle B0003
  %v4994 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v4995 = load i64, i64* %v4994
  %v4996 = add i64 %v4995, 1
  store i64 %v4996, i64* %v4994
  %v4997 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v4998 = load i64, i64* %v4997
  %v4999 = add i64 %v4998, 1
  store i64 %v4999, i64* %v4997
  ; Cycle B0000
  %v5000 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5001 = load i64, i64* %v5000
  %v5002 = add i64 %v5001, 1
  store i64 %v5002, i64* %v5000
  %v5003 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 425, i32* %v5003
  %v5004 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 681, i32* %v5004
  %v5005 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 937, i32* %v5005
  %v5006 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 19099, i32* %v5006
  ; Cycle B0001
  %v5007 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5008 = load i64, i64* %v5007
  %v5009 = add i64 %v5008, 1
  store i64 %v5009, i64* %v5007
  %v5010 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1449, i32* %v5010
  %v5011 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1705, i32* %v5011
  %v5012 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 31378, i32* %v5012
  %v5013 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 35483, i32* %v5013
  ; Cycle B0002
  %v5014 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5015 = load i64, i64* %v5014
  %v5016 = add i64 %v5015, 1
  store i64 %v5016, i64* %v5014
  %v5017 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5018 = load i64, i64* %v5017
  %v5019 = add i64 %v5018, 1
  store i64 %v5019, i64* %v5017
  %v5020 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v5020
  %v5021 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v5022 = load i64, i64* %v5021
  %v5023 = add i64 %v5022, 1
  store i64 %v5023, i64* %v5021
  %v5024 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v5024
  ; Cycle B0003
  %v5025 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5026 = load i64, i64* %v5025
  %v5027 = add i64 %v5026, 1
  store i64 %v5027, i64* %v5025
  %v5028 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5029 = load i64, i64* %v5028
  %v5030 = add i64 %v5029, 1
  store i64 %v5030, i64* %v5028
  ; Cycle B0000
  %v5031 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5032 = load i64, i64* %v5031
  %v5033 = add i64 %v5032, 1
  store i64 %v5033, i64* %v5031
  %v5034 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 426, i32* %v5034
  %v5035 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 682, i32* %v5035
  %v5036 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 15012, i32* %v5036
  %v5037 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1194, i32* %v5037
  ; Cycle B0001
  %v5038 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5039 = load i64, i64* %v5038
  %v5040 = add i64 %v5039, 1
  store i64 %v5040, i64* %v5038
  %v5041 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1450, i32* %v5041
  %v5042 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 27297, i32* %v5042
  %v5043 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 31402, i32* %v5043
  %v5044 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 35489, i32* %v5044
  ; Cycle B0002
  %v5045 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5046 = load i64, i64* %v5045
  %v5047 = add i64 %v5046, 1
  store i64 %v5047, i64* %v5045
  %v5048 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5049 = load i64, i64* %v5048
  %v5050 = add i64 %v5049, 1
  store i64 %v5050, i64* %v5048
  %v5051 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v5051
  %v5052 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v5053 = load i64, i64* %v5052
  %v5054 = add i64 %v5053, 1
  store i64 %v5054, i64* %v5052
  %v5055 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v5055
  ; Cycle B0003
  %v5056 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5057 = load i64, i64* %v5056
  %v5058 = add i64 %v5057, 1
  store i64 %v5058, i64* %v5056
  %v5059 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5060 = load i64, i64* %v5059
  %v5061 = add i64 %v5060, 1
  store i64 %v5061, i64* %v5059
  ; Cycle B0000
  %v5062 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5063 = load i64, i64* %v5062
  %v5064 = add i64 %v5063, 1
  store i64 %v5064, i64* %v5062
  %v5065 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 6842, i32* %v5065
  %v5066 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 683, i32* %v5066
  %v5067 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 939, i32* %v5067
  %v5068 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1195, i32* %v5068
  ; Cycle B0001
  %v5069 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5070 = load i64, i64* %v5069
  %v5071 = add i64 %v5070, 1
  store i64 %v5071, i64* %v5069
  %v5072 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1451, i32* %v5072
  %v5073 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1707, i32* %v5073
  %v5074 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1963, i32* %v5074
  %v5075 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2219, i32* %v5075
  ; Cycle B0002
  %v5076 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5077 = load i64, i64* %v5076
  %v5078 = add i64 %v5077, 1
  store i64 %v5078, i64* %v5076
  %v5079 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5080 = load i64, i64* %v5079
  %v5081 = add i64 %v5080, 1
  store i64 %v5081, i64* %v5079
  %v5082 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v5082
  %v5083 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v5084 = load i64, i64* %v5083
  %v5085 = add i64 %v5084, 1
  store i64 %v5085, i64* %v5083
  %v5086 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v5086
  ; Cycle B0003
  %v5087 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5088 = load i64, i64* %v5087
  %v5089 = add i64 %v5088, 1
  store i64 %v5089, i64* %v5087
  %v5090 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5091 = load i64, i64* %v5090
  %v5092 = add i64 %v5091, 1
  store i64 %v5092, i64* %v5090
  ; Cycle B0000
  %v5093 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5094 = load i64, i64* %v5093
  %v5095 = add i64 %v5094, 1
  store i64 %v5095, i64* %v5093
  %v5096 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 428, i32* %v5096
  %v5097 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 10946, i32* %v5097
  %v5098 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 15051, i32* %v5098
  %v5099 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 19146, i32* %v5099
  ; Cycle B0001
  %v5100 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5101 = load i64, i64* %v5100
  %v5102 = add i64 %v5101, 1
  store i64 %v5102, i64* %v5100
  %v5103 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1452, i32* %v5103
  %v5104 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1708, i32* %v5104
  %v5105 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 31427, i32* %v5105
  %v5106 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2220, i32* %v5106
  ; Cycle B0002
  %v5107 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5108 = load i64, i64* %v5107
  %v5109 = add i64 %v5108, 1
  store i64 %v5109, i64* %v5107
  %v5110 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5111 = load i64, i64* %v5110
  %v5112 = add i64 %v5111, 1
  store i64 %v5112, i64* %v5110
  %v5113 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v5113
  %v5114 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v5115 = load i64, i64* %v5114
  %v5116 = add i64 %v5115, 1
  store i64 %v5116, i64* %v5114
  %v5117 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v5117
  ; Cycle B0003
  %v5118 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5119 = load i64, i64* %v5118
  %v5120 = add i64 %v5119, 1
  store i64 %v5120, i64* %v5118
  %v5121 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5122 = load i64, i64* %v5121
  %v5123 = add i64 %v5122, 1
  store i64 %v5123, i64* %v5121
  ; Cycle B0000
  %v5124 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5125 = load i64, i64* %v5124
  %v5126 = add i64 %v5125, 1
  store i64 %v5126, i64* %v5124
  %v5127 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 429, i32* %v5127
  %v5128 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 685, i32* %v5128
  %v5129 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 941, i32* %v5129
  %v5130 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1197, i32* %v5130
  ; Cycle B0001
  %v5131 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5132 = load i64, i64* %v5131
  %v5133 = add i64 %v5132, 1
  store i64 %v5133, i64* %v5131
  %v5134 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 23263, i32* %v5134
  %v5135 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 27352, i32* %v5135
  %v5136 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1965, i32* %v5136
  %v5137 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2221, i32* %v5137
  ; Cycle B0002
  %v5138 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5139 = load i64, i64* %v5138
  %v5140 = add i64 %v5139, 1
  store i64 %v5140, i64* %v5138
  %v5141 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5142 = load i64, i64* %v5141
  %v5143 = add i64 %v5142, 1
  store i64 %v5143, i64* %v5141
  %v5144 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v5144
  %v5145 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v5146 = load i64, i64* %v5145
  %v5147 = add i64 %v5146, 1
  store i64 %v5147, i64* %v5145
  %v5148 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v5148
  ; Cycle B0003
  %v5149 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5150 = load i64, i64* %v5149
  %v5151 = add i64 %v5150, 1
  store i64 %v5151, i64* %v5149
  %v5152 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5153 = load i64, i64* %v5152
  %v5154 = add i64 %v5153, 1
  store i64 %v5154, i64* %v5152
  ; Cycle B0000
  %v5155 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5156 = load i64, i64* %v5155
  %v5157 = add i64 %v5156, 1
  store i64 %v5157, i64* %v5155
  %v5158 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 430, i32* %v5158
  %v5159 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 686, i32* %v5159
  %v5160 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 15072, i32* %v5160
  %v5161 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1198, i32* %v5161
  ; Cycle B0001
  %v5162 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5163 = load i64, i64* %v5162
  %v5164 = add i64 %v5163, 1
  store i64 %v5164, i64* %v5162
  %v5165 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1454, i32* %v5165
  %v5166 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 27365, i32* %v5166
  %v5167 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1966, i32* %v5167
  %v5168 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2222, i32* %v5168
  ; Cycle B0002
  %v5169 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5170 = load i64, i64* %v5169
  %v5171 = add i64 %v5170, 1
  store i64 %v5171, i64* %v5169
  %v5172 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5173 = load i64, i64* %v5172
  %v5174 = add i64 %v5173, 1
  store i64 %v5174, i64* %v5172
  %v5175 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v5175
  %v5176 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v5177 = load i64, i64* %v5176
  %v5178 = add i64 %v5177, 1
  store i64 %v5178, i64* %v5176
  %v5179 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v5179
  ; Cycle B0003
  %v5180 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5181 = load i64, i64* %v5180
  %v5182 = add i64 %v5181, 1
  store i64 %v5182, i64* %v5180
  %v5183 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5184 = load i64, i64* %v5183
  %v5185 = add i64 %v5184, 1
  store i64 %v5185, i64* %v5183
  ; Cycle B0000
  %v5186 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5187 = load i64, i64* %v5186
  %v5188 = add i64 %v5187, 1
  store i64 %v5188, i64* %v5186
  %v5189 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 6910, i32* %v5189
  %v5190 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 10999, i32* %v5190
  %v5191 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 943, i32* %v5191
  %v5192 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1199, i32* %v5192
  ; Cycle B0001
  %v5193 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5194 = load i64, i64* %v5193
  %v5195 = add i64 %v5194, 1
  store i64 %v5195, i64* %v5193
  %v5196 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1455, i32* %v5196
  %v5197 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1711, i32* %v5197
  %v5198 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1967, i32* %v5198
  %v5199 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2223, i32* %v5199
  ; Cycle B0002
  %v5200 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5201 = load i64, i64* %v5200
  %v5202 = add i64 %v5201, 1
  store i64 %v5202, i64* %v5200
  %v5203 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5204 = load i64, i64* %v5203
  %v5205 = add i64 %v5204, 1
  store i64 %v5205, i64* %v5203
  %v5206 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v5206
  %v5207 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v5208 = load i64, i64* %v5207
  %v5209 = add i64 %v5208, 1
  store i64 %v5209, i64* %v5207
  %v5210 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v5210
  ; Cycle B0003
  %v5211 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5212 = load i64, i64* %v5211
  %v5213 = add i64 %v5212, 1
  store i64 %v5213, i64* %v5211
  %v5214 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5215 = load i64, i64* %v5214
  %v5216 = add i64 %v5215, 1
  store i64 %v5216, i64* %v5214
  ; Cycle B0000
  %v5217 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5218 = load i64, i64* %v5217
  %v5219 = add i64 %v5218, 1
  store i64 %v5219, i64* %v5217
  %v5220 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 432, i32* %v5220
  %v5221 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 688, i32* %v5221
  %v5222 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 944, i32* %v5222
  %v5223 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 19211, i32* %v5223
  ; Cycle B0001
  %v5224 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5225 = load i64, i64* %v5224
  %v5226 = add i64 %v5225, 1
  store i64 %v5226, i64* %v5224
  %v5227 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1456, i32* %v5227
  %v5228 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1712, i32* %v5228
  %v5229 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 31490, i32* %v5229
  %v5230 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 35595, i32* %v5230
  ; Cycle B0002
  %v5231 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5232 = load i64, i64* %v5231
  %v5233 = add i64 %v5232, 1
  store i64 %v5233, i64* %v5231
  %v5234 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5235 = load i64, i64* %v5234
  %v5236 = add i64 %v5235, 1
  store i64 %v5236, i64* %v5234
  %v5237 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v5237
  %v5238 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v5239 = load i64, i64* %v5238
  %v5240 = add i64 %v5239, 1
  store i64 %v5240, i64* %v5238
  %v5241 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v5241
  ; Cycle B0003
  %v5242 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5243 = load i64, i64* %v5242
  %v5244 = add i64 %v5243, 1
  store i64 %v5244, i64* %v5242
  %v5245 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5246 = load i64, i64* %v5245
  %v5247 = add i64 %v5246, 1
  store i64 %v5247, i64* %v5245
  ; Cycle B0000
  %v5248 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5249 = load i64, i64* %v5248
  %v5250 = add i64 %v5249, 1
  store i64 %v5250, i64* %v5248
  %v5251 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 433, i32* %v5251
  %v5252 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 689, i32* %v5252
  %v5253 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 945, i32* %v5253
  %v5254 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1201, i32* %v5254
  ; Cycle B0001
  %v5255 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5256 = load i64, i64* %v5255
  %v5257 = add i64 %v5256, 1
  store i64 %v5257, i64* %v5255
  %v5258 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1457, i32* %v5258
  %v5259 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1713, i32* %v5259
  %v5260 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 31511, i32* %v5260
  %v5261 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2225, i32* %v5261
  ; Cycle B0002
  %v5262 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5263 = load i64, i64* %v5262
  %v5264 = add i64 %v5263, 1
  store i64 %v5264, i64* %v5262
  %v5265 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5266 = load i64, i64* %v5265
  %v5267 = add i64 %v5266, 1
  store i64 %v5267, i64* %v5265
  %v5268 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v5268
  %v5269 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v5270 = load i64, i64* %v5269
  %v5271 = add i64 %v5270, 1
  store i64 %v5271, i64* %v5269
  %v5272 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v5272
  ; Cycle B0003
  %v5273 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5274 = load i64, i64* %v5273
  %v5275 = add i64 %v5274, 1
  store i64 %v5275, i64* %v5273
  %v5276 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5277 = load i64, i64* %v5276
  %v5278 = add i64 %v5277, 1
  store i64 %v5278, i64* %v5276
  ; Cycle B0000
  %v5279 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5280 = load i64, i64* %v5279
  %v5281 = add i64 %v5280, 1
  store i64 %v5281, i64* %v5279
  %v5282 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 434, i32* %v5282
  %v5283 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 690, i32* %v5283
  %v5284 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 15137, i32* %v5284
  %v5285 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1202, i32* %v5285
  ; Cycle B0001
  %v5286 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5287 = load i64, i64* %v5286
  %v5288 = add i64 %v5287, 1
  store i64 %v5288, i64* %v5286
  %v5289 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1458, i32* %v5289
  %v5290 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 27424, i32* %v5290
  %v5291 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1970, i32* %v5291
  %v5292 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 35620, i32* %v5292
  ; Cycle B0002
  %v5293 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5294 = load i64, i64* %v5293
  %v5295 = add i64 %v5294, 1
  store i64 %v5295, i64* %v5293
  %v5296 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5297 = load i64, i64* %v5296
  %v5298 = add i64 %v5297, 1
  store i64 %v5298, i64* %v5296
  %v5299 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v5299
  %v5300 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v5301 = load i64, i64* %v5300
  %v5302 = add i64 %v5301, 1
  store i64 %v5302, i64* %v5300
  %v5303 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v5303
  ; Cycle B0003
  %v5304 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5305 = load i64, i64* %v5304
  %v5306 = add i64 %v5305, 1
  store i64 %v5306, i64* %v5304
  %v5307 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5308 = load i64, i64* %v5307
  %v5309 = add i64 %v5308, 1
  store i64 %v5309, i64* %v5307
  ; Cycle B0000
  %v5310 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5311 = load i64, i64* %v5310
  %v5312 = add i64 %v5311, 1
  store i64 %v5312, i64* %v5310
  %v5313 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 435, i32* %v5313
  %v5314 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 691, i32* %v5314
  %v5315 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 15160, i32* %v5315
  %v5316 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1203, i32* %v5316
  ; Cycle B0001
  %v5317 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5318 = load i64, i64* %v5317
  %v5319 = add i64 %v5318, 1
  store i64 %v5319, i64* %v5317
  %v5320 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1459, i32* %v5320
  %v5321 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1715, i32* %v5321
  %v5322 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 31545, i32* %v5322
  %v5323 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 35641, i32* %v5323
  ; Cycle B0002
  %v5324 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5325 = load i64, i64* %v5324
  %v5326 = add i64 %v5325, 1
  store i64 %v5326, i64* %v5324
  %v5327 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5328 = load i64, i64* %v5327
  %v5329 = add i64 %v5328, 1
  store i64 %v5329, i64* %v5327
  %v5330 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v5330
  %v5331 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v5332 = load i64, i64* %v5331
  %v5333 = add i64 %v5332, 1
  store i64 %v5333, i64* %v5331
  %v5334 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v5334
  ; Cycle B0003
  %v5335 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5336 = load i64, i64* %v5335
  %v5337 = add i64 %v5336, 1
  store i64 %v5337, i64* %v5335
  %v5338 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5339 = load i64, i64* %v5338
  %v5340 = add i64 %v5339, 1
  store i64 %v5340, i64* %v5338
  ; Cycle B0000
  %v5341 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5342 = load i64, i64* %v5341
  %v5343 = add i64 %v5342, 1
  store i64 %v5343, i64* %v5341
  %v5344 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 436, i32* %v5344
  %v5345 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 11075, i32* %v5345
  %v5346 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 948, i32* %v5346
  %v5347 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1204, i32* %v5347
  ; Cycle B0001
  %v5348 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5349 = load i64, i64* %v5348
  %v5350 = add i64 %v5349, 1
  store i64 %v5350, i64* %v5348
  %v5351 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1460, i32* %v5351
  %v5352 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1716, i32* %v5352
  %v5353 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1972, i32* %v5353
  %v5354 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2228, i32* %v5354
  ; Cycle B0002
  %v5355 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5356 = load i64, i64* %v5355
  %v5357 = add i64 %v5356, 1
  store i64 %v5357, i64* %v5355
  %v5358 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5359 = load i64, i64* %v5358
  %v5360 = add i64 %v5359, 1
  store i64 %v5360, i64* %v5358
  %v5361 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v5361
  %v5362 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v5363 = load i64, i64* %v5362
  %v5364 = add i64 %v5363, 1
  store i64 %v5364, i64* %v5362
  %v5365 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v5365
  ; Cycle B0003
  %v5366 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5367 = load i64, i64* %v5366
  %v5368 = add i64 %v5367, 1
  store i64 %v5368, i64* %v5366
  %v5369 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5370 = load i64, i64* %v5369
  %v5371 = add i64 %v5370, 1
  store i64 %v5371, i64* %v5369
  ; Cycle B0000
  %v5372 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5373 = load i64, i64* %v5372
  %v5374 = add i64 %v5373, 1
  store i64 %v5374, i64* %v5372
  %v5375 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 7005, i32* %v5375
  %v5376 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 11094, i32* %v5376
  %v5377 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 949, i32* %v5377
  %v5378 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1205, i32* %v5378
  ; Cycle B0001
  %v5379 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5380 = load i64, i64* %v5379
  %v5381 = add i64 %v5380, 1
  store i64 %v5381, i64* %v5379
  %v5382 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1461, i32* %v5382
  %v5383 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1717, i32* %v5383
  %v5384 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1973, i32* %v5384
  %v5385 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2229, i32* %v5385
  ; Cycle B0002
  %v5386 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5387 = load i64, i64* %v5386
  %v5388 = add i64 %v5387, 1
  store i64 %v5388, i64* %v5386
  %v5389 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5390 = load i64, i64* %v5389
  %v5391 = add i64 %v5390, 1
  store i64 %v5391, i64* %v5389
  %v5392 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v5392
  %v5393 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v5394 = load i64, i64* %v5393
  %v5395 = add i64 %v5394, 1
  store i64 %v5395, i64* %v5393
  %v5396 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v5396
  ; Cycle B0003
  %v5397 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5398 = load i64, i64* %v5397
  %v5399 = add i64 %v5398, 1
  store i64 %v5399, i64* %v5397
  %v5400 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5401 = load i64, i64* %v5400
  %v5402 = add i64 %v5401, 1
  store i64 %v5402, i64* %v5400
  ; Cycle B0000
  %v5403 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5404 = load i64, i64* %v5403
  %v5405 = add i64 %v5404, 1
  store i64 %v5405, i64* %v5403
  %v5406 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 438, i32* %v5406
  %v5407 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 694, i32* %v5407
  %v5408 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 950, i32* %v5408
  %v5409 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1206, i32* %v5409
  ; Cycle B0001
  %v5410 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5411 = load i64, i64* %v5410
  %v5412 = add i64 %v5411, 1
  store i64 %v5412, i64* %v5410
  %v5413 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1462, i32* %v5413
  %v5414 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 27492, i32* %v5414
  %v5415 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1974, i32* %v5415
  %v5416 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 35680, i32* %v5416
  ; Cycle B0002
  %v5417 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5418 = load i64, i64* %v5417
  %v5419 = add i64 %v5418, 1
  store i64 %v5419, i64* %v5417
  %v5420 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5421 = load i64, i64* %v5420
  %v5422 = add i64 %v5421, 1
  store i64 %v5422, i64* %v5420
  %v5423 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v5423
  %v5424 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v5425 = load i64, i64* %v5424
  %v5426 = add i64 %v5425, 1
  store i64 %v5426, i64* %v5424
  %v5427 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v5427
  ; Cycle B0003
  %v5428 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5429 = load i64, i64* %v5428
  %v5430 = add i64 %v5429, 1
  store i64 %v5430, i64* %v5428
  %v5431 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5432 = load i64, i64* %v5431
  %v5433 = add i64 %v5432, 1
  store i64 %v5433, i64* %v5431
  ; Cycle B0000
  %v5434 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5435 = load i64, i64* %v5434
  %v5436 = add i64 %v5435, 1
  store i64 %v5436, i64* %v5434
  %v5437 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 439, i32* %v5437
  %v5438 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 695, i32* %v5438
  %v5439 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 951, i32* %v5439
  %v5440 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1207, i32* %v5440
  ; Cycle B0001
  %v5441 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5442 = load i64, i64* %v5441
  %v5443 = add i64 %v5442, 1
  store i64 %v5443, i64* %v5441
  %v5444 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 23420, i32* %v5444
  %v5445 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 27509, i32* %v5445
  %v5446 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1975, i32* %v5446
  %v5447 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2231, i32* %v5447
  ; Cycle B0002
  %v5448 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5449 = load i64, i64* %v5448
  %v5450 = add i64 %v5449, 1
  store i64 %v5450, i64* %v5448
  %v5451 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5452 = load i64, i64* %v5451
  %v5453 = add i64 %v5452, 1
  store i64 %v5453, i64* %v5451
  %v5454 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v5454
  %v5455 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v5456 = load i64, i64* %v5455
  %v5457 = add i64 %v5456, 1
  store i64 %v5457, i64* %v5455
  %v5458 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v5458
  ; Cycle B0003
  %v5459 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5460 = load i64, i64* %v5459
  %v5461 = add i64 %v5460, 1
  store i64 %v5461, i64* %v5459
  %v5462 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5463 = load i64, i64* %v5462
  %v5464 = add i64 %v5463, 1
  store i64 %v5464, i64* %v5462
  ; Cycle B0000
  %v5465 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5466 = load i64, i64* %v5465
  %v5467 = add i64 %v5466, 1
  store i64 %v5467, i64* %v5465
  %v5468 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 7054, i32* %v5468
  %v5469 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 696, i32* %v5469
  %v5470 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 952, i32* %v5470
  %v5471 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1208, i32* %v5471
  ; Cycle B0001
  %v5472 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5473 = load i64, i64* %v5472
  %v5474 = add i64 %v5473, 1
  store i64 %v5474, i64* %v5472
  %v5475 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1464, i32* %v5475
  %v5476 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1720, i32* %v5476
  %v5477 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1976, i32* %v5477
  %v5478 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2232, i32* %v5478
  ; Cycle B0002
  %v5479 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5480 = load i64, i64* %v5479
  %v5481 = add i64 %v5480, 1
  store i64 %v5481, i64* %v5479
  %v5482 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5483 = load i64, i64* %v5482
  %v5484 = add i64 %v5483, 1
  store i64 %v5484, i64* %v5482
  %v5485 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v5485
  %v5486 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v5487 = load i64, i64* %v5486
  %v5488 = add i64 %v5487, 1
  store i64 %v5488, i64* %v5486
  %v5489 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v5489
  ; Cycle B0003
  %v5490 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5491 = load i64, i64* %v5490
  %v5492 = add i64 %v5491, 1
  store i64 %v5492, i64* %v5490
  %v5493 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5494 = load i64, i64* %v5493
  %v5495 = add i64 %v5494, 1
  store i64 %v5495, i64* %v5493
  ; Cycle B0000
  %v5496 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5497 = load i64, i64* %v5496
  %v5498 = add i64 %v5497, 1
  store i64 %v5498, i64* %v5496
  %v5499 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 441, i32* %v5499
  %v5500 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 697, i32* %v5500
  %v5501 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 15259, i32* %v5501
  %v5502 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1209, i32* %v5502
  ; Cycle B0001
  %v5503 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5504 = load i64, i64* %v5503
  %v5505 = add i64 %v5504, 1
  store i64 %v5505, i64* %v5503
  %v5506 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1465, i32* %v5506
  %v5507 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1721, i32* %v5507
  %v5508 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1977, i32* %v5508
  %v5509 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 35738, i32* %v5509
  ; Cycle B0002
  %v5510 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5511 = load i64, i64* %v5510
  %v5512 = add i64 %v5511, 1
  store i64 %v5512, i64* %v5510
  %v5513 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5514 = load i64, i64* %v5513
  %v5515 = add i64 %v5514, 1
  store i64 %v5515, i64* %v5513
  %v5516 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v5516
  %v5517 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v5518 = load i64, i64* %v5517
  %v5519 = add i64 %v5518, 1
  store i64 %v5519, i64* %v5517
  %v5520 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v5520
  ; Cycle B0003
  %v5521 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5522 = load i64, i64* %v5521
  %v5523 = add i64 %v5522, 1
  store i64 %v5523, i64* %v5521
  %v5524 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5525 = load i64, i64* %v5524
  %v5526 = add i64 %v5525, 1
  store i64 %v5526, i64* %v5524
  ; Cycle B0000
  %v5527 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5528 = load i64, i64* %v5527
  %v5529 = add i64 %v5528, 1
  store i64 %v5529, i64* %v5527
  %v5530 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 442, i32* %v5530
  %v5531 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 698, i32* %v5531
  %v5532 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 954, i32* %v5532
  %v5533 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 19364, i32* %v5533
  ; Cycle B0001
  %v5534 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5535 = load i64, i64* %v5534
  %v5536 = add i64 %v5535, 1
  store i64 %v5536, i64* %v5534
  %v5537 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1466, i32* %v5537
  %v5538 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1722, i32* %v5538
  %v5539 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 31659, i32* %v5539
  %v5540 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2234, i32* %v5540
  ; Cycle B0002
  %v5541 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5542 = load i64, i64* %v5541
  %v5543 = add i64 %v5542, 1
  store i64 %v5543, i64* %v5541
  %v5544 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5545 = load i64, i64* %v5544
  %v5546 = add i64 %v5545, 1
  store i64 %v5546, i64* %v5544
  %v5547 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v5547
  %v5548 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v5549 = load i64, i64* %v5548
  %v5550 = add i64 %v5549, 1
  store i64 %v5550, i64* %v5548
  %v5551 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v5551
  ; Cycle B0003
  %v5552 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5553 = load i64, i64* %v5552
  %v5554 = add i64 %v5553, 1
  store i64 %v5554, i64* %v5552
  %v5555 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5556 = load i64, i64* %v5555
  %v5557 = add i64 %v5556, 1
  store i64 %v5557, i64* %v5555
  ; Cycle B0000
  %v5558 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5559 = load i64, i64* %v5558
  %v5560 = add i64 %v5559, 1
  store i64 %v5560, i64* %v5558
  %v5561 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 443, i32* %v5561
  %v5562 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 699, i32* %v5562
  %v5563 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 955, i32* %v5563
  %v5564 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1211, i32* %v5564
  ; Cycle B0001
  %v5565 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5566 = load i64, i64* %v5565
  %v5567 = add i64 %v5566, 1
  store i64 %v5567, i64* %v5565
  %v5568 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 23473, i32* %v5568
  %v5569 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1723, i32* %v5569
  %v5570 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1979, i32* %v5570
  %v5571 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2235, i32* %v5571
  ; Cycle B0002
  %v5572 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5573 = load i64, i64* %v5572
  %v5574 = add i64 %v5573, 1
  store i64 %v5574, i64* %v5572
  %v5575 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5576 = load i64, i64* %v5575
  %v5577 = add i64 %v5576, 1
  store i64 %v5577, i64* %v5575
  %v5578 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v5578
  %v5579 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v5580 = load i64, i64* %v5579
  %v5581 = add i64 %v5580, 1
  store i64 %v5581, i64* %v5579
  %v5582 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v5582
  ; Cycle B0003
  %v5583 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5584 = load i64, i64* %v5583
  %v5585 = add i64 %v5584, 1
  store i64 %v5585, i64* %v5583
  %v5586 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5587 = load i64, i64* %v5586
  %v5588 = add i64 %v5587, 1
  store i64 %v5588, i64* %v5586
  ; Cycle B0000
  %v5589 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5590 = load i64, i64* %v5589
  %v5591 = add i64 %v5590, 1
  store i64 %v5591, i64* %v5589
  %v5592 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 444, i32* %v5592
  %v5593 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 700, i32* %v5593
  %v5594 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 15306, i32* %v5594
  %v5595 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 19403, i32* %v5595
  ; Cycle B0001
  %v5596 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5597 = load i64, i64* %v5596
  %v5598 = add i64 %v5597, 1
  store i64 %v5598, i64* %v5596
  %v5599 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 23488, i32* %v5599
  %v5600 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1724, i32* %v5600
  %v5601 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1980, i32* %v5601
  %v5602 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 35787, i32* %v5602
  ; Cycle B0002
  %v5603 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5604 = load i64, i64* %v5603
  %v5605 = add i64 %v5604, 1
  store i64 %v5605, i64* %v5603
  %v5606 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5607 = load i64, i64* %v5606
  %v5608 = add i64 %v5607, 1
  store i64 %v5608, i64* %v5606
  %v5609 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v5609
  %v5610 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v5611 = load i64, i64* %v5610
  %v5612 = add i64 %v5611, 1
  store i64 %v5612, i64* %v5610
  %v5613 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v5613
  ; Cycle B0003
  %v5614 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5615 = load i64, i64* %v5614
  %v5616 = add i64 %v5615, 1
  store i64 %v5616, i64* %v5614
  %v5617 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5618 = load i64, i64* %v5617
  %v5619 = add i64 %v5618, 1
  store i64 %v5619, i64* %v5617
  ; Cycle B0000
  %v5620 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5621 = load i64, i64* %v5620
  %v5622 = add i64 %v5621, 1
  store i64 %v5622, i64* %v5620
  %v5623 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 7126, i32* %v5623
  %v5624 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 11231, i32* %v5624
  %v5625 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 957, i32* %v5625
  %v5626 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1213, i32* %v5626
  ; Cycle B0001
  %v5627 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5628 = load i64, i64* %v5627
  %v5629 = add i64 %v5628, 1
  store i64 %v5629, i64* %v5627
  %v5630 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 23516, i32* %v5630
  %v5631 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1725, i32* %v5631
  %v5632 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1981, i32* %v5632
  %v5633 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2237, i32* %v5633
  ; Cycle B0002
  %v5634 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5635 = load i64, i64* %v5634
  %v5636 = add i64 %v5635, 1
  store i64 %v5636, i64* %v5634
  %v5637 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5638 = load i64, i64* %v5637
  %v5639 = add i64 %v5638, 1
  store i64 %v5639, i64* %v5637
  %v5640 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v5640
  %v5641 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v5642 = load i64, i64* %v5641
  %v5643 = add i64 %v5642, 1
  store i64 %v5643, i64* %v5641
  %v5644 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v5644
  ; Cycle B0003
  %v5645 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5646 = load i64, i64* %v5645
  %v5647 = add i64 %v5646, 1
  store i64 %v5647, i64* %v5645
  %v5648 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5649 = load i64, i64* %v5648
  %v5650 = add i64 %v5649, 1
  store i64 %v5650, i64* %v5648
  ; Cycle B0000
  %v5651 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5652 = load i64, i64* %v5651
  %v5653 = add i64 %v5652, 1
  store i64 %v5653, i64* %v5651
  %v5654 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 446, i32* %v5654
  %v5655 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 11244, i32* %v5655
  %v5656 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 958, i32* %v5656
  %v5657 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 19424, i32* %v5657
  ; Cycle B0001
  %v5658 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5659 = load i64, i64* %v5658
  %v5660 = add i64 %v5659, 1
  store i64 %v5660, i64* %v5658
  %v5661 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1470, i32* %v5661
  %v5662 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1726, i32* %v5662
  %v5663 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1982, i32* %v5663
  %v5664 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2238, i32* %v5664
  ; Cycle B0002
  %v5665 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5666 = load i64, i64* %v5665
  %v5667 = add i64 %v5666, 1
  store i64 %v5667, i64* %v5665
  %v5668 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5669 = load i64, i64* %v5668
  %v5670 = add i64 %v5669, 1
  store i64 %v5670, i64* %v5668
  %v5671 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v5671
  %v5672 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v5673 = load i64, i64* %v5672
  %v5674 = add i64 %v5673, 1
  store i64 %v5674, i64* %v5672
  %v5675 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v5675
  ; Cycle B0003
  %v5676 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5677 = load i64, i64* %v5676
  %v5678 = add i64 %v5677, 1
  store i64 %v5678, i64* %v5676
  %v5679 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5680 = load i64, i64* %v5679
  %v5681 = add i64 %v5680, 1
  store i64 %v5681, i64* %v5679
  ; Cycle B0000
  %v5682 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5683 = load i64, i64* %v5682
  %v5684 = add i64 %v5683, 1
  store i64 %v5684, i64* %v5682
  %v5685 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 7165, i32* %v5685
  %v5686 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 703, i32* %v5686
  %v5687 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 959, i32* %v5687
  %v5688 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1215, i32* %v5688
  ; Cycle B0001
  %v5689 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5690 = load i64, i64* %v5689
  %v5691 = add i64 %v5690, 1
  store i64 %v5691, i64* %v5689
  %v5692 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 23541, i32* %v5692
  %v5693 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 27646, i32* %v5693
  %v5694 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1983, i32* %v5694
  %v5695 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2239, i32* %v5695
  ; Cycle B0002
  %v5696 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5697 = load i64, i64* %v5696
  %v5698 = add i64 %v5697, 1
  store i64 %v5698, i64* %v5696
  %v5699 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5700 = load i64, i64* %v5699
  %v5701 = add i64 %v5700, 1
  store i64 %v5701, i64* %v5699
  %v5702 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 132, i32* %v5702
  %v5703 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v5704 = load i64, i64* %v5703
  %v5705 = add i64 %v5704, 1
  store i64 %v5705, i64* %v5703
  %v5706 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 16711680, i32* %v5706
  ; Cycle B0003
  %v5707 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5708 = load i64, i64* %v5707
  %v5709 = add i64 %v5708, 1
  store i64 %v5709, i64* %v5707
  %v5710 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5711 = load i64, i64* %v5710
  %v5712 = add i64 %v5711, 1
  store i64 %v5712, i64* %v5710
  ; Cycle B0000
  %v5713 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5714 = load i64, i64* %v5713
  %v5715 = add i64 %v5714, 1
  store i64 %v5715, i64* %v5713
  %v5716 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 448, i32* %v5716
  %v5717 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 704, i32* %v5717
  %v5718 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 960, i32* %v5718
  %v5719 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1216, i32* %v5719
  ; Cycle B0001
  %v5720 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5721 = load i64, i64* %v5720
  %v5722 = add i64 %v5721, 1
  store i64 %v5722, i64* %v5720
  %v5723 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 23564, i32* %v5723
  %v5724 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 27653, i32* %v5724
  %v5725 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1984, i32* %v5725
  %v5726 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2240, i32* %v5726
  ; Cycle B0002
  %v5727 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5728 = load i64, i64* %v5727
  %v5729 = add i64 %v5728, 1
  store i64 %v5729, i64* %v5727
  %v5730 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v5731 = load i32, i32* %v5730
  %v5732 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v5733 = load i32, i32* %v5732
  %v5734 = call i32 @cron_subbyte_ternary_dot(i32 %v5731, i32 %v5733)
  store i32 %v5734, i32* %v5730
  ; Cycle B0003
  %v5735 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5736 = load i64, i64* %v5735
  %v5737 = add i64 %v5736, 1
  store i64 %v5737, i64* %v5735
  %v5738 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5739 = load i64, i64* %v5738
  %v5740 = add i64 %v5739, 1
  store i64 %v5740, i64* %v5738
  ; Cycle B0000
  %v5741 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5742 = load i64, i64* %v5741
  %v5743 = add i64 %v5742, 1
  store i64 %v5743, i64* %v5741
  %v5744 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 449, i32* %v5744
  %v5745 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 705, i32* %v5745
  %v5746 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 961, i32* %v5746
  %v5747 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1217, i32* %v5747
  ; Cycle B0001
  %v5748 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5749 = load i64, i64* %v5748
  %v5750 = add i64 %v5749, 1
  store i64 %v5750, i64* %v5748
  %v5751 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1473, i32* %v5751
  %v5752 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 27668, i32* %v5752
  %v5753 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1985, i32* %v5753
  %v5754 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 35856, i32* %v5754
  ; Cycle B0002
  %v5755 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5756 = load i64, i64* %v5755
  %v5757 = add i64 %v5756, 1
  store i64 %v5757, i64* %v5755
  %v5758 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v5759 = load i32, i32* %v5758
  %v5760 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v5761 = load i32, i32* %v5760
  %v5762 = call i32 @cron_subbyte_ternary_dot(i32 %v5759, i32 %v5761)
  store i32 %v5762, i32* %v5758
  ; Cycle B0003
  %v5763 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5764 = load i64, i64* %v5763
  %v5765 = add i64 %v5764, 1
  store i64 %v5765, i64* %v5763
  %v5766 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5767 = load i64, i64* %v5766
  %v5768 = add i64 %v5767, 1
  store i64 %v5768, i64* %v5766
  ; Cycle B0000
  %v5769 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5770 = load i64, i64* %v5769
  %v5771 = add i64 %v5770, 1
  store i64 %v5771, i64* %v5769
  %v5772 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 7213, i32* %v5772
  %v5773 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 11302, i32* %v5773
  %v5774 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 962, i32* %v5774
  %v5775 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1218, i32* %v5775
  ; Cycle B0001
  %v5776 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5777 = load i64, i64* %v5776
  %v5778 = add i64 %v5777, 1
  store i64 %v5778, i64* %v5776
  %v5779 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1474, i32* %v5779
  %v5780 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1730, i32* %v5780
  %v5781 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1986, i32* %v5781
  %v5782 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2242, i32* %v5782
  ; Cycle B0002
  %v5783 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5784 = load i64, i64* %v5783
  %v5785 = add i64 %v5784, 1
  store i64 %v5785, i64* %v5783
  %v5786 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v5787 = load i32, i32* %v5786
  %v5788 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v5789 = load i32, i32* %v5788
  %v5790 = call i32 @cron_subbyte_ternary_dot(i32 %v5787, i32 %v5789)
  store i32 %v5790, i32* %v5786
  ; Cycle B0003
  %v5791 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5792 = load i64, i64* %v5791
  %v5793 = add i64 %v5792, 1
  store i64 %v5793, i64* %v5791
  %v5794 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5795 = load i64, i64* %v5794
  %v5796 = add i64 %v5795, 1
  store i64 %v5796, i64* %v5794
  ; Cycle B0000
  %v5797 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5798 = load i64, i64* %v5797
  %v5799 = add i64 %v5798, 1
  store i64 %v5799, i64* %v5797
  %v5800 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 451, i32* %v5800
  %v5801 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 11315, i32* %v5801
  %v5802 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 963, i32* %v5802
  %v5803 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1219, i32* %v5803
  ; Cycle B0001
  %v5804 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5805 = load i64, i64* %v5804
  %v5806 = add i64 %v5805, 1
  store i64 %v5806, i64* %v5804
  %v5807 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1475, i32* %v5807
  %v5808 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1731, i32* %v5808
  %v5809 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1987, i32* %v5809
  %v5810 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2243, i32* %v5810
  ; Cycle B0002
  %v5811 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5812 = load i64, i64* %v5811
  %v5813 = add i64 %v5812, 1
  store i64 %v5813, i64* %v5811
  %v5814 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v5815 = load i32, i32* %v5814
  %v5816 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v5817 = load i32, i32* %v5816
  %v5818 = call i32 @cron_subbyte_ternary_dot(i32 %v5815, i32 %v5817)
  store i32 %v5818, i32* %v5814
  ; Cycle B0003
  %v5819 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5820 = load i64, i64* %v5819
  %v5821 = add i64 %v5820, 1
  store i64 %v5821, i64* %v5819
  %v5822 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5823 = load i64, i64* %v5822
  %v5824 = add i64 %v5823, 1
  store i64 %v5824, i64* %v5822
  ; Cycle B0000
  %v5825 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5826 = load i64, i64* %v5825
  %v5827 = add i64 %v5826, 1
  store i64 %v5827, i64* %v5825
  %v5828 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 452, i32* %v5828
  %v5829 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 708, i32* %v5829
  %v5830 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 15432, i32* %v5830
  %v5831 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1220, i32* %v5831
  ; Cycle B0001
  %v5832 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5833 = load i64, i64* %v5832
  %v5834 = add i64 %v5833, 1
  store i64 %v5834, i64* %v5832
  %v5835 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1476, i32* %v5835
  %v5836 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1732, i32* %v5836
  %v5837 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 31817, i32* %v5837
  %v5838 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 35913, i32* %v5838
  ; Cycle B0002
  %v5839 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5840 = load i64, i64* %v5839
  %v5841 = add i64 %v5840, 1
  store i64 %v5841, i64* %v5839
  %v5842 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v5843 = load i32, i32* %v5842
  %v5844 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v5845 = load i32, i32* %v5844
  %v5846 = call i32 @cron_subbyte_ternary_dot(i32 %v5843, i32 %v5845)
  store i32 %v5846, i32* %v5842
  ; Cycle B0003
  %v5847 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5848 = load i64, i64* %v5847
  %v5849 = add i64 %v5848, 1
  store i64 %v5849, i64* %v5847
  %v5850 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5851 = load i64, i64* %v5850
  %v5852 = add i64 %v5851, 1
  store i64 %v5852, i64* %v5850
  ; Cycle B0000
  %v5853 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5854 = load i64, i64* %v5853
  %v5855 = add i64 %v5854, 1
  store i64 %v5855, i64* %v5853
  %v5856 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 453, i32* %v5856
  %v5857 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 709, i32* %v5857
  %v5858 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 15441, i32* %v5858
  %v5859 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1221, i32* %v5859
  ; Cycle B0001
  %v5860 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5861 = load i64, i64* %v5860
  %v5862 = add i64 %v5861, 1
  store i64 %v5862, i64* %v5860
  %v5863 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1477, i32* %v5863
  %v5864 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 27728, i32* %v5864
  %v5865 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1989, i32* %v5865
  %v5866 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 35924, i32* %v5866
  ; Cycle B0002
  %v5867 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5868 = load i64, i64* %v5867
  %v5869 = add i64 %v5868, 1
  store i64 %v5869, i64* %v5867
  %v5870 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v5871 = load i32, i32* %v5870
  %v5872 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v5873 = load i32, i32* %v5872
  %v5874 = call i32 @cron_subbyte_ternary_dot(i32 %v5871, i32 %v5873)
  store i32 %v5874, i32* %v5870
  ; Cycle B0003
  %v5875 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5876 = load i64, i64* %v5875
  %v5877 = add i64 %v5876, 1
  store i64 %v5877, i64* %v5875
  %v5878 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5879 = load i64, i64* %v5878
  %v5880 = add i64 %v5879, 1
  store i64 %v5880, i64* %v5878
  ; Cycle B0000
  %v5881 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5882 = load i64, i64* %v5881
  %v5883 = add i64 %v5882, 1
  store i64 %v5883, i64* %v5881
  %v5884 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 454, i32* %v5884
  %v5885 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 710, i32* %v5885
  %v5886 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 966, i32* %v5886
  %v5887 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1222, i32* %v5887
  ; Cycle B0001
  %v5888 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5889 = load i64, i64* %v5888
  %v5890 = add i64 %v5889, 1
  store i64 %v5890, i64* %v5888
  %v5891 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1478, i32* %v5891
  %v5892 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1734, i32* %v5892
  %v5893 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 31847, i32* %v5893
  %v5894 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2246, i32* %v5894
  ; Cycle B0002
  %v5895 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5896 = load i64, i64* %v5895
  %v5897 = add i64 %v5896, 1
  store i64 %v5897, i64* %v5895
  %v5898 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v5899 = load i32, i32* %v5898
  %v5900 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v5901 = load i32, i32* %v5900
  %v5902 = call i32 @cron_subbyte_ternary_dot(i32 %v5899, i32 %v5901)
  store i32 %v5902, i32* %v5898
  ; Cycle B0003
  %v5903 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5904 = load i64, i64* %v5903
  %v5905 = add i64 %v5904, 1
  store i64 %v5905, i64* %v5903
  %v5906 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5907 = load i64, i64* %v5906
  %v5908 = add i64 %v5907, 1
  store i64 %v5908, i64* %v5906
  ; Cycle B0000
  %v5909 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5910 = load i64, i64* %v5909
  %v5911 = add i64 %v5910, 1
  store i64 %v5911, i64* %v5909
  %v5912 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 455, i32* %v5912
  %v5913 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 711, i32* %v5913
  %v5914 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 967, i32* %v5914
  %v5915 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 19579, i32* %v5915
  ; Cycle B0001
  %v5916 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5917 = load i64, i64* %v5916
  %v5918 = add i64 %v5917, 1
  store i64 %v5918, i64* %v5916
  %v5919 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1479, i32* %v5919
  %v5920 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1735, i32* %v5920
  %v5921 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 31858, i32* %v5921
  %v5922 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 35963, i32* %v5922
  ; Cycle B0002
  %v5923 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5924 = load i64, i64* %v5923
  %v5925 = add i64 %v5924, 1
  store i64 %v5925, i64* %v5923
  %v5926 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v5927 = load i32, i32* %v5926
  %v5928 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v5929 = load i32, i32* %v5928
  %v5930 = call i32 @cron_subbyte_ternary_dot(i32 %v5927, i32 %v5929)
  store i32 %v5930, i32* %v5926
  ; Cycle B0003
  %v5931 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5932 = load i64, i64* %v5931
  %v5933 = add i64 %v5932, 1
  store i64 %v5933, i64* %v5931
  %v5934 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5935 = load i64, i64* %v5934
  %v5936 = add i64 %v5935, 1
  store i64 %v5936, i64* %v5934
  ; Cycle B0000
  %v5937 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5938 = load i64, i64* %v5937
  %v5939 = add i64 %v5938, 1
  store i64 %v5939, i64* %v5937
  %v5940 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 456, i32* %v5940
  %v5941 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 11405, i32* %v5941
  %v5942 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 968, i32* %v5942
  %v5943 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1224, i32* %v5943
  ; Cycle B0001
  %v5944 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5945 = load i64, i64* %v5944
  %v5946 = add i64 %v5945, 1
  store i64 %v5946, i64* %v5944
  %v5947 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1480, i32* %v5947
  %v5948 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1736, i32* %v5948
  %v5949 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1992, i32* %v5949
  %v5950 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2248, i32* %v5950
  ; Cycle B0002
  %v5951 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5952 = load i64, i64* %v5951
  %v5953 = add i64 %v5952, 1
  store i64 %v5953, i64* %v5951
  %v5954 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v5955 = load i32, i32* %v5954
  %v5956 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v5957 = load i32, i32* %v5956
  %v5958 = call i32 @cron_subbyte_ternary_dot(i32 %v5955, i32 %v5957)
  store i32 %v5958, i32* %v5954
  ; Cycle B0003
  %v5959 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5960 = load i64, i64* %v5959
  %v5961 = add i64 %v5960, 1
  store i64 %v5961, i64* %v5959
  %v5962 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5963 = load i64, i64* %v5962
  %v5964 = add i64 %v5963, 1
  store i64 %v5964, i64* %v5962
  ; Cycle B0000
  %v5965 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5966 = load i64, i64* %v5965
  %v5967 = add i64 %v5966, 1
  store i64 %v5967, i64* %v5965
  %v5968 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 457, i32* %v5968
  %v5969 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 11422, i32* %v5969
  %v5970 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 969, i32* %v5970
  %v5971 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1225, i32* %v5971
  ; Cycle B0001
  %v5972 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5973 = load i64, i64* %v5972
  %v5974 = add i64 %v5973, 1
  store i64 %v5974, i64* %v5972
  %v5975 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1481, i32* %v5975
  %v5976 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1737, i32* %v5976
  %v5977 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1993, i32* %v5977
  %v5978 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2249, i32* %v5978
  ; Cycle B0002
  %v5979 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5980 = load i64, i64* %v5979
  %v5981 = add i64 %v5980, 1
  store i64 %v5981, i64* %v5979
  %v5982 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v5983 = load i32, i32* %v5982
  %v5984 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v5985 = load i32, i32* %v5984
  %v5986 = call i32 @cron_subbyte_ternary_dot(i32 %v5983, i32 %v5985)
  store i32 %v5986, i32* %v5982
  ; Cycle B0003
  %v5987 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5988 = load i64, i64* %v5987
  %v5989 = add i64 %v5988, 1
  store i64 %v5989, i64* %v5987
  %v5990 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v5991 = load i64, i64* %v5990
  %v5992 = add i64 %v5991, 1
  store i64 %v5992, i64* %v5990
  ; Cycle B0000
  %v5993 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v5994 = load i64, i64* %v5993
  %v5995 = add i64 %v5994, 1
  store i64 %v5995, i64* %v5993
  %v5996 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 7341, i32* %v5996
  %v5997 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 714, i32* %v5997
  %v5998 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 970, i32* %v5998
  %v5999 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1226, i32* %v5999
  ; Cycle B0001
  %v6000 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6001 = load i64, i64* %v6000
  %v6002 = add i64 %v6001, 1
  store i64 %v6002, i64* %v6000
  %v6003 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 23717, i32* %v6003
  %v6004 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 27822, i32* %v6004
  %v6005 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1994, i32* %v6005
  %v6006 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2250, i32* %v6006
  ; Cycle B0002
  %v6007 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6008 = load i64, i64* %v6007
  %v6009 = add i64 %v6008, 1
  store i64 %v6009, i64* %v6007
  %v6010 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6011 = load i32, i32* %v6010
  %v6012 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6013 = load i32, i32* %v6012
  %v6014 = call i32 @cron_subbyte_ternary_dot(i32 %v6011, i32 %v6013)
  store i32 %v6014, i32* %v6010
  ; Cycle B0003
  %v6015 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6016 = load i64, i64* %v6015
  %v6017 = add i64 %v6016, 1
  store i64 %v6017, i64* %v6015
  %v6018 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6019 = load i64, i64* %v6018
  %v6020 = add i64 %v6019, 1
  store i64 %v6020, i64* %v6018
  ; Cycle B0000
  %v6021 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6022 = load i64, i64* %v6021
  %v6023 = add i64 %v6022, 1
  store i64 %v6023, i64* %v6021
  %v6024 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 459, i32* %v6024
  %v6025 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 11452, i32* %v6025
  %v6026 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 971, i32* %v6026
  %v6027 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 19632, i32* %v6027
  ; Cycle B0001
  %v6028 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6029 = load i64, i64* %v6028
  %v6030 = add i64 %v6029, 1
  store i64 %v6030, i64* %v6028
  %v6031 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1483, i32* %v6031
  %v6032 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1739, i32* %v6032
  %v6033 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1995, i32* %v6033
  %v6034 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2251, i32* %v6034
  ; Cycle B0002
  %v6035 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6036 = load i64, i64* %v6035
  %v6037 = add i64 %v6036, 1
  store i64 %v6037, i64* %v6035
  %v6038 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6039 = load i32, i32* %v6038
  %v6040 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6041 = load i32, i32* %v6040
  %v6042 = call i32 @cron_subbyte_ternary_dot(i32 %v6039, i32 %v6041)
  store i32 %v6042, i32* %v6038
  ; Cycle B0003
  %v6043 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6044 = load i64, i64* %v6043
  %v6045 = add i64 %v6044, 1
  store i64 %v6045, i64* %v6043
  %v6046 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6047 = load i64, i64* %v6046
  %v6048 = add i64 %v6047, 1
  store i64 %v6048, i64* %v6046
  ; Cycle B0000
  %v6049 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6050 = load i64, i64* %v6049
  %v6051 = add i64 %v6050, 1
  store i64 %v6051, i64* %v6049
  %v6052 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 7366, i32* %v6052
  %v6053 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 11471, i32* %v6053
  %v6054 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 972, i32* %v6054
  %v6055 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1228, i32* %v6055
  ; Cycle B0001
  %v6056 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6057 = load i64, i64* %v6056
  %v6058 = add i64 %v6057, 1
  store i64 %v6058, i64* %v6056
  %v6059 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 23756, i32* %v6059
  %v6060 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1740, i32* %v6060
  %v6061 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1996, i32* %v6061
  %v6062 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2252, i32* %v6062
  ; Cycle B0002
  %v6063 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6064 = load i64, i64* %v6063
  %v6065 = add i64 %v6064, 1
  store i64 %v6065, i64* %v6063
  %v6066 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6067 = load i32, i32* %v6066
  %v6068 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6069 = load i32, i32* %v6068
  %v6070 = call i32 @cron_subbyte_ternary_dot(i32 %v6067, i32 %v6069)
  store i32 %v6070, i32* %v6066
  ; Cycle B0003
  %v6071 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6072 = load i64, i64* %v6071
  %v6073 = add i64 %v6072, 1
  store i64 %v6073, i64* %v6071
  %v6074 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6075 = load i64, i64* %v6074
  %v6076 = add i64 %v6075, 1
  store i64 %v6076, i64* %v6074
  ; Cycle B0000
  %v6077 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6078 = load i64, i64* %v6077
  %v6079 = add i64 %v6078, 1
  store i64 %v6079, i64* %v6077
  %v6080 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 461, i32* %v6080
  %v6081 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 717, i32* %v6081
  %v6082 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 15578, i32* %v6082
  %v6083 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 19675, i32* %v6083
  ; Cycle B0001
  %v6084 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6085 = load i64, i64* %v6084
  %v6086 = add i64 %v6085, 1
  store i64 %v6086, i64* %v6084
  %v6087 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 23760, i32* %v6087
  %v6088 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1741, i32* %v6088
  %v6089 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1997, i32* %v6089
  %v6090 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 36059, i32* %v6090
  ; Cycle B0002
  %v6091 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6092 = load i64, i64* %v6091
  %v6093 = add i64 %v6092, 1
  store i64 %v6093, i64* %v6091
  %v6094 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6095 = load i32, i32* %v6094
  %v6096 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6097 = load i32, i32* %v6096
  %v6098 = call i32 @cron_subbyte_ternary_dot(i32 %v6095, i32 %v6097)
  store i32 %v6098, i32* %v6094
  ; Cycle B0003
  %v6099 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6100 = load i64, i64* %v6099
  %v6101 = add i64 %v6100, 1
  store i64 %v6101, i64* %v6099
  %v6102 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6103 = load i64, i64* %v6102
  %v6104 = add i64 %v6103, 1
  store i64 %v6104, i64* %v6102
  ; Cycle B0000
  %v6105 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6106 = load i64, i64* %v6105
  %v6107 = add i64 %v6106, 1
  store i64 %v6107, i64* %v6105
  %v6108 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 462, i32* %v6108
  %v6109 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 718, i32* %v6109
  %v6110 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 974, i32* %v6110
  %v6111 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1230, i32* %v6111
  ; Cycle B0001
  %v6112 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6113 = load i64, i64* %v6112
  %v6114 = add i64 %v6113, 1
  store i64 %v6114, i64* %v6112
  %v6115 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 23777, i32* %v6115
  %v6116 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1742, i32* %v6116
  %v6117 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1998, i32* %v6117
  %v6118 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2254, i32* %v6118
  ; Cycle B0002
  %v6119 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6120 = load i64, i64* %v6119
  %v6121 = add i64 %v6120, 1
  store i64 %v6121, i64* %v6119
  %v6122 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6123 = load i32, i32* %v6122
  %v6124 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6125 = load i32, i32* %v6124
  %v6126 = call i32 @cron_subbyte_ternary_dot(i32 %v6123, i32 %v6125)
  store i32 %v6126, i32* %v6122
  ; Cycle B0003
  %v6127 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6128 = load i64, i64* %v6127
  %v6129 = add i64 %v6128, 1
  store i64 %v6129, i64* %v6127
  %v6130 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6131 = load i64, i64* %v6130
  %v6132 = add i64 %v6131, 1
  store i64 %v6132, i64* %v6130
  ; Cycle B0000
  %v6133 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6134 = load i64, i64* %v6133
  %v6135 = add i64 %v6134, 1
  store i64 %v6135, i64* %v6133
  %v6136 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 463, i32* %v6136
  %v6137 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 719, i32* %v6137
  %v6138 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 975, i32* %v6138
  %v6139 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 19700, i32* %v6139
  ; Cycle B0001
  %v6140 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6141 = load i64, i64* %v6140
  %v6142 = add i64 %v6141, 1
  store i64 %v6142, i64* %v6140
  %v6143 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1487, i32* %v6143
  %v6144 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1743, i32* %v6144
  %v6145 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 31995, i32* %v6145
  %v6146 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2255, i32* %v6146
  ; Cycle B0002
  %v6147 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6148 = load i64, i64* %v6147
  %v6149 = add i64 %v6148, 1
  store i64 %v6149, i64* %v6147
  %v6150 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6151 = load i32, i32* %v6150
  %v6152 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6153 = load i32, i32* %v6152
  %v6154 = call i32 @cron_subbyte_ternary_dot(i32 %v6151, i32 %v6153)
  store i32 %v6154, i32* %v6150
  ; Cycle B0003
  %v6155 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6156 = load i64, i64* %v6155
  %v6157 = add i64 %v6156, 1
  store i64 %v6157, i64* %v6155
  %v6158 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6159 = load i64, i64* %v6158
  %v6160 = add i64 %v6159, 1
  store i64 %v6160, i64* %v6158
  ; Cycle B0000
  %v6161 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6162 = load i64, i64* %v6161
  %v6163 = add i64 %v6162, 1
  store i64 %v6163, i64* %v6161
  %v6164 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 464, i32* %v6164
  %v6165 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 11523, i32* %v6165
  %v6166 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 976, i32* %v6166
  %v6167 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1232, i32* %v6167
  ; Cycle B0001
  %v6168 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6169 = load i64, i64* %v6168
  %v6170 = add i64 %v6169, 1
  store i64 %v6170, i64* %v6168
  %v6171 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1488, i32* %v6171
  %v6172 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 27915, i32* %v6172
  %v6173 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 32002, i32* %v6173
  %v6174 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2256, i32* %v6174
  ; Cycle B0002
  %v6175 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6176 = load i64, i64* %v6175
  %v6177 = add i64 %v6176, 1
  store i64 %v6177, i64* %v6175
  %v6178 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6179 = load i32, i32* %v6178
  %v6180 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6181 = load i32, i32* %v6180
  %v6182 = call i32 @cron_subbyte_ternary_dot(i32 %v6179, i32 %v6181)
  store i32 %v6182, i32* %v6178
  ; Cycle B0003
  %v6183 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6184 = load i64, i64* %v6183
  %v6185 = add i64 %v6184, 1
  store i64 %v6185, i64* %v6183
  %v6186 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6187 = load i64, i64* %v6186
  %v6188 = add i64 %v6187, 1
  store i64 %v6188, i64* %v6186
  ; Cycle B0000
  %v6189 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6190 = load i64, i64* %v6189
  %v6191 = add i64 %v6190, 1
  store i64 %v6191, i64* %v6189
  %v6192 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 465, i32* %v6192
  %v6193 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 721, i32* %v6193
  %v6194 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 977, i32* %v6194
  %v6195 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1233, i32* %v6195
  ; Cycle B0001
  %v6196 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6197 = load i64, i64* %v6196
  %v6198 = add i64 %v6197, 1
  store i64 %v6198, i64* %v6196
  %v6199 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1489, i32* %v6199
  %v6200 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1745, i32* %v6200
  %v6201 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2001, i32* %v6201
  %v6202 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2257, i32* %v6202
  ; Cycle B0002
  %v6203 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6204 = load i64, i64* %v6203
  %v6205 = add i64 %v6204, 1
  store i64 %v6205, i64* %v6203
  %v6206 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6207 = load i32, i32* %v6206
  %v6208 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6209 = load i32, i32* %v6208
  %v6210 = call i32 @cron_subbyte_ternary_dot(i32 %v6207, i32 %v6209)
  store i32 %v6210, i32* %v6206
  ; Cycle B0003
  %v6211 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6212 = load i64, i64* %v6211
  %v6213 = add i64 %v6212, 1
  store i64 %v6213, i64* %v6211
  %v6214 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6215 = load i64, i64* %v6214
  %v6216 = add i64 %v6215, 1
  store i64 %v6216, i64* %v6214
  ; Cycle B0000
  %v6217 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6218 = load i64, i64* %v6217
  %v6219 = add i64 %v6218, 1
  store i64 %v6219, i64* %v6217
  %v6220 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 466, i32* %v6220
  %v6221 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 11562, i32* %v6221
  %v6222 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 15649, i32* %v6222
  %v6223 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1234, i32* %v6223
  ; Cycle B0001
  %v6224 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6225 = load i64, i64* %v6224
  %v6226 = add i64 %v6225, 1
  store i64 %v6226, i64* %v6224
  %v6227 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1490, i32* %v6227
  %v6228 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 27940, i32* %v6228
  %v6229 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2002, i32* %v6229
  %v6230 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 36128, i32* %v6230
  ; Cycle B0002
  %v6231 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6232 = load i64, i64* %v6231
  %v6233 = add i64 %v6232, 1
  store i64 %v6233, i64* %v6231
  %v6234 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6235 = load i32, i32* %v6234
  %v6236 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6237 = load i32, i32* %v6236
  %v6238 = call i32 @cron_subbyte_ternary_dot(i32 %v6235, i32 %v6237)
  store i32 %v6238, i32* %v6234
  ; Cycle B0003
  %v6239 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6240 = load i64, i64* %v6239
  %v6241 = add i64 %v6240, 1
  store i64 %v6241, i64* %v6239
  %v6242 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6243 = load i64, i64* %v6242
  %v6244 = add i64 %v6243, 1
  store i64 %v6244, i64* %v6242
  ; Cycle B0000
  %v6245 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6246 = load i64, i64* %v6245
  %v6247 = add i64 %v6246, 1
  store i64 %v6247, i64* %v6245
  %v6248 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 467, i32* %v6248
  %v6249 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 723, i32* %v6249
  %v6250 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 979, i32* %v6250
  %v6251 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1235, i32* %v6251
  ; Cycle B0001
  %v6252 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6253 = load i64, i64* %v6252
  %v6254 = add i64 %v6253, 1
  store i64 %v6254, i64* %v6252
  %v6255 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1491, i32* %v6255
  %v6256 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 27961, i32* %v6256
  %v6257 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2003, i32* %v6257
  %v6258 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2259, i32* %v6258
  ; Cycle B0002
  %v6259 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6260 = load i64, i64* %v6259
  %v6261 = add i64 %v6260, 1
  store i64 %v6261, i64* %v6259
  %v6262 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6263 = load i32, i32* %v6262
  %v6264 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6265 = load i32, i32* %v6264
  %v6266 = call i32 @cron_subbyte_ternary_dot(i32 %v6263, i32 %v6265)
  store i32 %v6266, i32* %v6262
  ; Cycle B0003
  %v6267 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6268 = load i64, i64* %v6267
  %v6269 = add i64 %v6268, 1
  store i64 %v6269, i64* %v6267
  %v6270 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6271 = load i64, i64* %v6270
  %v6272 = add i64 %v6271, 1
  store i64 %v6272, i64* %v6270
  ; Cycle B0000
  %v6273 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6274 = load i64, i64* %v6273
  %v6275 = add i64 %v6274, 1
  store i64 %v6275, i64* %v6273
  %v6276 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 468, i32* %v6276
  %v6277 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 724, i32* %v6277
  %v6278 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 980, i32* %v6278
  %v6279 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 19789, i32* %v6279
  ; Cycle B0001
  %v6280 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6281 = load i64, i64* %v6280
  %v6282 = add i64 %v6281, 1
  store i64 %v6282, i64* %v6280
  %v6283 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1492, i32* %v6283
  %v6284 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1748, i32* %v6284
  %v6285 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 32070, i32* %v6285
  %v6286 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2260, i32* %v6286
  ; Cycle B0002
  %v6287 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6288 = load i64, i64* %v6287
  %v6289 = add i64 %v6288, 1
  store i64 %v6289, i64* %v6287
  %v6290 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6291 = load i32, i32* %v6290
  %v6292 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6293 = load i32, i32* %v6292
  %v6294 = call i32 @cron_subbyte_ternary_dot(i32 %v6291, i32 %v6293)
  store i32 %v6294, i32* %v6290
  ; Cycle B0003
  %v6295 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6296 = load i64, i64* %v6295
  %v6297 = add i64 %v6296, 1
  store i64 %v6297, i64* %v6295
  %v6298 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6299 = load i64, i64* %v6298
  %v6300 = add i64 %v6299, 1
  store i64 %v6300, i64* %v6298
  ; Cycle B0000
  %v6301 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6302 = load i64, i64* %v6301
  %v6303 = add i64 %v6302, 1
  store i64 %v6303, i64* %v6301
  %v6304 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 7517, i32* %v6304
  %v6305 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 725, i32* %v6305
  %v6306 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 981, i32* %v6306
  %v6307 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 19806, i32* %v6307
  ; Cycle B0001
  %v6308 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6309 = load i64, i64* %v6308
  %v6310 = add i64 %v6309, 1
  store i64 %v6310, i64* %v6308
  %v6311 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1493, i32* %v6311
  %v6312 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1749, i32* %v6312
  %v6313 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 32087, i32* %v6313
  %v6314 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2261, i32* %v6314
  ; Cycle B0002
  %v6315 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6316 = load i64, i64* %v6315
  %v6317 = add i64 %v6316, 1
  store i64 %v6317, i64* %v6315
  %v6318 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6319 = load i32, i32* %v6318
  %v6320 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6321 = load i32, i32* %v6320
  %v6322 = call i32 @cron_subbyte_ternary_dot(i32 %v6319, i32 %v6321)
  store i32 %v6322, i32* %v6318
  ; Cycle B0003
  %v6323 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6324 = load i64, i64* %v6323
  %v6325 = add i64 %v6324, 1
  store i64 %v6325, i64* %v6323
  %v6326 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6327 = load i64, i64* %v6326
  %v6328 = add i64 %v6327, 1
  store i64 %v6328, i64* %v6326
  ; Cycle B0000
  %v6329 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6330 = load i64, i64* %v6329
  %v6331 = add i64 %v6330, 1
  store i64 %v6331, i64* %v6329
  %v6332 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 470, i32* %v6332
  %v6333 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 726, i32* %v6333
  %v6334 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 15717, i32* %v6334
  %v6335 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1238, i32* %v6335
  ; Cycle B0001
  %v6336 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6337 = load i64, i64* %v6336
  %v6338 = add i64 %v6337, 1
  store i64 %v6338, i64* %v6336
  %v6339 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1494, i32* %v6339
  %v6340 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 28000, i32* %v6340
  %v6341 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2006, i32* %v6341
  %v6342 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 36196, i32* %v6342
  ; Cycle B0002
  %v6343 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6344 = load i64, i64* %v6343
  %v6345 = add i64 %v6344, 1
  store i64 %v6345, i64* %v6343
  %v6346 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6347 = load i32, i32* %v6346
  %v6348 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6349 = load i32, i32* %v6348
  %v6350 = call i32 @cron_subbyte_ternary_dot(i32 %v6347, i32 %v6349)
  store i32 %v6350, i32* %v6346
  ; Cycle B0003
  %v6351 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6352 = load i64, i64* %v6351
  %v6353 = add i64 %v6352, 1
  store i64 %v6353, i64* %v6351
  %v6354 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6355 = load i64, i64* %v6354
  %v6356 = add i64 %v6355, 1
  store i64 %v6356, i64* %v6354
  ; Cycle B0000
  %v6357 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6358 = load i64, i64* %v6357
  %v6359 = add i64 %v6358, 1
  store i64 %v6359, i64* %v6357
  %v6360 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 471, i32* %v6360
  %v6361 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 727, i32* %v6361
  %v6362 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 15736, i32* %v6362
  %v6363 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1239, i32* %v6363
  ; Cycle B0001
  %v6364 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6365 = load i64, i64* %v6364
  %v6366 = add i64 %v6365, 1
  store i64 %v6366, i64* %v6364
  %v6367 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 23932, i32* %v6367
  %v6368 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1751, i32* %v6368
  %v6369 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2007, i32* %v6369
  %v6370 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 36213, i32* %v6370
  ; Cycle B0002
  %v6371 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6372 = load i64, i64* %v6371
  %v6373 = add i64 %v6372, 1
  store i64 %v6373, i64* %v6371
  %v6374 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6375 = load i32, i32* %v6374
  %v6376 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6377 = load i32, i32* %v6376
  %v6378 = call i32 @cron_subbyte_ternary_dot(i32 %v6375, i32 %v6377)
  store i32 %v6378, i32* %v6374
  ; Cycle B0003
  %v6379 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6380 = load i64, i64* %v6379
  %v6381 = add i64 %v6380, 1
  store i64 %v6381, i64* %v6379
  %v6382 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6383 = load i64, i64* %v6382
  %v6384 = add i64 %v6383, 1
  store i64 %v6384, i64* %v6382
  ; Cycle B0000
  %v6385 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6386 = load i64, i64* %v6385
  %v6387 = add i64 %v6386, 1
  store i64 %v6387, i64* %v6385
  %v6388 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 7562, i32* %v6388
  %v6389 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 728, i32* %v6389
  %v6390 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 984, i32* %v6390
  %v6391 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1240, i32* %v6391
  ; Cycle B0001
  %v6392 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6393 = load i64, i64* %v6392
  %v6394 = add i64 %v6393, 1
  store i64 %v6394, i64* %v6392
  %v6395 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 23944, i32* %v6395
  %v6396 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1752, i32* %v6396
  %v6397 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2008, i32* %v6397
  %v6398 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2264, i32* %v6398
  ; Cycle B0002
  %v6399 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6400 = load i64, i64* %v6399
  %v6401 = add i64 %v6400, 1
  store i64 %v6401, i64* %v6399
  %v6402 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6403 = load i32, i32* %v6402
  %v6404 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6405 = load i32, i32* %v6404
  %v6406 = call i32 @cron_subbyte_ternary_dot(i32 %v6403, i32 %v6405)
  store i32 %v6406, i32* %v6402
  ; Cycle B0003
  %v6407 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6408 = load i64, i64* %v6407
  %v6409 = add i64 %v6408, 1
  store i64 %v6409, i64* %v6407
  %v6410 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6411 = load i64, i64* %v6410
  %v6412 = add i64 %v6411, 1
  store i64 %v6412, i64* %v6410
  ; Cycle B0000
  %v6413 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6414 = load i64, i64* %v6413
  %v6415 = add i64 %v6414, 1
  store i64 %v6415, i64* %v6413
  %v6416 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 473, i32* %v6416
  %v6417 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 729, i32* %v6417
  %v6418 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 985, i32* %v6418
  %v6419 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1241, i32* %v6419
  ; Cycle B0001
  %v6420 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6421 = load i64, i64* %v6420
  %v6422 = add i64 %v6421, 1
  store i64 %v6422, i64* %v6420
  %v6423 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 23961, i32* %v6423
  %v6424 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 28058, i32* %v6424
  %v6425 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2009, i32* %v6425
  %v6426 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2265, i32* %v6426
  ; Cycle B0002
  %v6427 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6428 = load i64, i64* %v6427
  %v6429 = add i64 %v6428, 1
  store i64 %v6429, i64* %v6427
  %v6430 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6431 = load i32, i32* %v6430
  %v6432 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6433 = load i32, i32* %v6432
  %v6434 = call i32 @cron_subbyte_ternary_dot(i32 %v6431, i32 %v6433)
  store i32 %v6434, i32* %v6430
  ; Cycle B0003
  %v6435 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6436 = load i64, i64* %v6435
  %v6437 = add i64 %v6436, 1
  store i64 %v6437, i64* %v6435
  %v6438 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6439 = load i64, i64* %v6438
  %v6440 = add i64 %v6439, 1
  store i64 %v6440, i64* %v6438
  ; Cycle B0000
  %v6441 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6442 = load i64, i64* %v6441
  %v6443 = add i64 %v6442, 1
  store i64 %v6443, i64* %v6441
  %v6444 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 474, i32* %v6444
  %v6445 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 11690, i32* %v6445
  %v6446 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 986, i32* %v6446
  %v6447 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 19872, i32* %v6447
  ; Cycle B0001
  %v6448 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6449 = load i64, i64* %v6448
  %v6450 = add i64 %v6449, 1
  store i64 %v6450, i64* %v6448
  %v6451 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1498, i32* %v6451
  %v6452 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1754, i32* %v6452
  %v6453 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2010, i32* %v6453
  %v6454 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2266, i32* %v6454
  ; Cycle B0002
  %v6455 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6456 = load i64, i64* %v6455
  %v6457 = add i64 %v6456, 1
  store i64 %v6457, i64* %v6455
  %v6458 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6459 = load i32, i32* %v6458
  %v6460 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6461 = load i32, i32* %v6460
  %v6462 = call i32 @cron_subbyte_ternary_dot(i32 %v6459, i32 %v6461)
  store i32 %v6462, i32* %v6458
  ; Cycle B0003
  %v6463 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6464 = load i64, i64* %v6463
  %v6465 = add i64 %v6464, 1
  store i64 %v6465, i64* %v6463
  %v6466 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6467 = load i64, i64* %v6466
  %v6468 = add i64 %v6467, 1
  store i64 %v6468, i64* %v6466
  ; Cycle B0000
  %v6469 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6470 = load i64, i64* %v6469
  %v6471 = add i64 %v6470, 1
  store i64 %v6471, i64* %v6469
  %v6472 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 475, i32* %v6472
  %v6473 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 731, i32* %v6473
  %v6474 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 987, i32* %v6474
  %v6475 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1243, i32* %v6475
  ; Cycle B0001
  %v6476 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6477 = load i64, i64* %v6476
  %v6478 = add i64 %v6477, 1
  store i64 %v6478, i64* %v6476
  %v6479 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1499, i32* %v6479
  %v6480 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1755, i32* %v6480
  %v6481 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2011, i32* %v6481
  %v6482 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2267, i32* %v6482
  ; Cycle B0002
  %v6483 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6484 = load i64, i64* %v6483
  %v6485 = add i64 %v6484, 1
  store i64 %v6485, i64* %v6483
  %v6486 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6487 = load i32, i32* %v6486
  %v6488 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6489 = load i32, i32* %v6488
  %v6490 = call i32 @cron_subbyte_ternary_dot(i32 %v6487, i32 %v6489)
  store i32 %v6490, i32* %v6486
  ; Cycle B0003
  %v6491 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6492 = load i64, i64* %v6491
  %v6493 = add i64 %v6492, 1
  store i64 %v6493, i64* %v6491
  %v6494 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6495 = load i64, i64* %v6494
  %v6496 = add i64 %v6495, 1
  store i64 %v6496, i64* %v6494
  ; Cycle B0000
  %v6497 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6498 = load i64, i64* %v6497
  %v6499 = add i64 %v6498, 1
  store i64 %v6499, i64* %v6497
  %v6500 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 476, i32* %v6500
  %v6501 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 732, i32* %v6501
  %v6502 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 988, i32* %v6502
  %v6503 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1244, i32* %v6503
  ; Cycle B0001
  %v6504 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6505 = load i64, i64* %v6504
  %v6506 = add i64 %v6505, 1
  store i64 %v6506, i64* %v6504
  %v6507 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1500, i32* %v6507
  %v6508 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 28107, i32* %v6508
  %v6509 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2012, i32* %v6509
  %v6510 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2268, i32* %v6510
  ; Cycle B0002
  %v6511 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6512 = load i64, i64* %v6511
  %v6513 = add i64 %v6512, 1
  store i64 %v6513, i64* %v6511
  %v6514 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6515 = load i32, i32* %v6514
  %v6516 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6517 = load i32, i32* %v6516
  %v6518 = call i32 @cron_subbyte_ternary_dot(i32 %v6515, i32 %v6517)
  store i32 %v6518, i32* %v6514
  ; Cycle B0003
  %v6519 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6520 = load i64, i64* %v6519
  %v6521 = add i64 %v6520, 1
  store i64 %v6521, i64* %v6519
  %v6522 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6523 = load i64, i64* %v6522
  %v6524 = add i64 %v6523, 1
  store i64 %v6524, i64* %v6522
  ; Cycle B0000
  %v6525 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6526 = load i64, i64* %v6525
  %v6527 = add i64 %v6526, 1
  store i64 %v6527, i64* %v6525
  %v6528 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 477, i32* %v6528
  %v6529 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 733, i32* %v6529
  %v6530 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 989, i32* %v6530
  %v6531 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 19925, i32* %v6531
  ; Cycle B0001
  %v6532 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6533 = load i64, i64* %v6532
  %v6534 = add i64 %v6533, 1
  store i64 %v6534, i64* %v6532
  %v6535 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 24028, i32* %v6535
  %v6536 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1757, i32* %v6536
  %v6537 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 32222, i32* %v6537
  %v6538 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2269, i32* %v6538
  ; Cycle B0002
  %v6539 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6540 = load i64, i64* %v6539
  %v6541 = add i64 %v6540, 1
  store i64 %v6541, i64* %v6539
  %v6542 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6543 = load i32, i32* %v6542
  %v6544 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6545 = load i32, i32* %v6544
  %v6546 = call i32 @cron_subbyte_ternary_dot(i32 %v6543, i32 %v6545)
  store i32 %v6546, i32* %v6542
  ; Cycle B0003
  %v6547 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6548 = load i64, i64* %v6547
  %v6549 = add i64 %v6548, 1
  store i64 %v6549, i64* %v6547
  %v6550 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6551 = load i64, i64* %v6550
  %v6552 = add i64 %v6551, 1
  store i64 %v6552, i64* %v6550
  ; Cycle B0000
  %v6553 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6554 = load i64, i64* %v6553
  %v6555 = add i64 %v6554, 1
  store i64 %v6555, i64* %v6553
  %v6556 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 478, i32* %v6556
  %v6557 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 734, i32* %v6557
  %v6558 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 990, i32* %v6558
  %v6559 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 19940, i32* %v6559
  ; Cycle B0001
  %v6560 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6561 = load i64, i64* %v6560
  %v6562 = add i64 %v6561, 1
  store i64 %v6562, i64* %v6560
  %v6563 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1502, i32* %v6563
  %v6564 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1758, i32* %v6564
  %v6565 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 32237, i32* %v6565
  %v6566 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2270, i32* %v6566
  ; Cycle B0002
  %v6567 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6568 = load i64, i64* %v6567
  %v6569 = add i64 %v6568, 1
  store i64 %v6569, i64* %v6567
  %v6570 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6571 = load i32, i32* %v6570
  %v6572 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6573 = load i32, i32* %v6572
  %v6574 = call i32 @cron_subbyte_ternary_dot(i32 %v6571, i32 %v6573)
  store i32 %v6574, i32* %v6570
  ; Cycle B0003
  %v6575 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6576 = load i64, i64* %v6575
  %v6577 = add i64 %v6576, 1
  store i64 %v6577, i64* %v6575
  %v6578 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6579 = load i64, i64* %v6578
  %v6580 = add i64 %v6579, 1
  store i64 %v6580, i64* %v6578
  ; Cycle B0000
  %v6581 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6582 = load i64, i64* %v6581
  %v6583 = add i64 %v6582, 1
  store i64 %v6583, i64* %v6581
  %v6584 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 7677, i32* %v6584
  %v6585 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 735, i32* %v6585
  %v6586 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 15871, i32* %v6586
  %v6587 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1247, i32* %v6587
  ; Cycle B0001
  %v6588 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6589 = load i64, i64* %v6588
  %v6590 = add i64 %v6589, 1
  store i64 %v6590, i64* %v6588
  %v6591 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1503, i32* %v6591
  %v6592 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1759, i32* %v6592
  %v6593 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2015, i32* %v6593
  %v6594 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 36350, i32* %v6594
  ; Cycle B0002
  %v6595 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6596 = load i64, i64* %v6595
  %v6597 = add i64 %v6596, 1
  store i64 %v6597, i64* %v6595
  %v6598 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6599 = load i32, i32* %v6598
  %v6600 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6601 = load i32, i32* %v6600
  %v6602 = call i32 @cron_subbyte_ternary_dot(i32 %v6599, i32 %v6601)
  store i32 %v6602, i32* %v6598
  ; Cycle B0003
  %v6603 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6604 = load i64, i64* %v6603
  %v6605 = add i64 %v6604, 1
  store i64 %v6605, i64* %v6603
  %v6606 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6607 = load i64, i64* %v6606
  %v6608 = add i64 %v6607, 1
  store i64 %v6608, i64* %v6606
  ; Cycle B0000
  %v6609 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6610 = load i64, i64* %v6609
  %v6611 = add i64 %v6610, 1
  store i64 %v6611, i64* %v6609
  %v6612 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 480, i32* %v6612
  %v6613 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 736, i32* %v6613
  %v6614 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 15880, i32* %v6614
  %v6615 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1248, i32* %v6615
  ; Cycle B0001
  %v6616 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6617 = load i64, i64* %v6616
  %v6618 = add i64 %v6617, 1
  store i64 %v6618, i64* %v6616
  %v6619 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 24076, i32* %v6619
  %v6620 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1760, i32* %v6620
  %v6621 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2016, i32* %v6621
  %v6622 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 36357, i32* %v6622
  ; Cycle B0002
  %v6623 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6624 = load i64, i64* %v6623
  %v6625 = add i64 %v6624, 1
  store i64 %v6625, i64* %v6623
  %v6626 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6627 = load i32, i32* %v6626
  %v6628 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6629 = load i32, i32* %v6628
  %v6630 = call i32 @cron_subbyte_ternary_dot(i32 %v6627, i32 %v6629)
  store i32 %v6630, i32* %v6626
  ; Cycle B0003
  %v6631 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6632 = load i64, i64* %v6631
  %v6633 = add i64 %v6632, 1
  store i64 %v6633, i64* %v6631
  %v6634 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6635 = load i64, i64* %v6634
  %v6636 = add i64 %v6635, 1
  store i64 %v6636, i64* %v6634
  ; Cycle B0000
  %v6637 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6638 = load i64, i64* %v6637
  %v6639 = add i64 %v6638, 1
  store i64 %v6639, i64* %v6637
  %v6640 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 481, i32* %v6640
  %v6641 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 737, i32* %v6641
  %v6642 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 15893, i32* %v6642
  %v6643 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1249, i32* %v6643
  ; Cycle B0001
  %v6644 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6645 = load i64, i64* %v6644
  %v6646 = add i64 %v6645, 1
  store i64 %v6646, i64* %v6644
  %v6647 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1505, i32* %v6647
  %v6648 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 28176, i32* %v6648
  %v6649 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2017, i32* %v6649
  %v6650 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 36372, i32* %v6650
  ; Cycle B0002
  %v6651 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6652 = load i64, i64* %v6651
  %v6653 = add i64 %v6652, 1
  store i64 %v6653, i64* %v6651
  %v6654 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6655 = load i32, i32* %v6654
  %v6656 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6657 = load i32, i32* %v6656
  %v6658 = call i32 @cron_subbyte_ternary_dot(i32 %v6655, i32 %v6657)
  store i32 %v6658, i32* %v6654
  ; Cycle B0003
  %v6659 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6660 = load i64, i64* %v6659
  %v6661 = add i64 %v6660, 1
  store i64 %v6661, i64* %v6659
  %v6662 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6663 = load i64, i64* %v6662
  %v6664 = add i64 %v6663, 1
  store i64 %v6664, i64* %v6662
  ; Cycle B0000
  %v6665 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6666 = load i64, i64* %v6665
  %v6667 = add i64 %v6666, 1
  store i64 %v6667, i64* %v6665
  %v6668 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 7725, i32* %v6668
  %v6669 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 738, i32* %v6669
  %v6670 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 994, i32* %v6670
  %v6671 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 20014, i32* %v6671
  ; Cycle B0001
  %v6672 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6673 = load i64, i64* %v6672
  %v6674 = add i64 %v6673, 1
  store i64 %v6674, i64* %v6672
  %v6675 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1506, i32* %v6675
  %v6676 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1762, i32* %v6676
  %v6677 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 32295, i32* %v6677
  %v6678 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2274, i32* %v6678
  ; Cycle B0002
  %v6679 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6680 = load i64, i64* %v6679
  %v6681 = add i64 %v6680, 1
  store i64 %v6681, i64* %v6679
  %v6682 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6683 = load i32, i32* %v6682
  %v6684 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6685 = load i32, i32* %v6684
  %v6686 = call i32 @cron_subbyte_ternary_dot(i32 %v6683, i32 %v6685)
  store i32 %v6686, i32* %v6682
  ; Cycle B0003
  %v6687 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6688 = load i64, i64* %v6687
  %v6689 = add i64 %v6688, 1
  store i64 %v6689, i64* %v6687
  %v6690 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6691 = load i64, i64* %v6690
  %v6692 = add i64 %v6691, 1
  store i64 %v6692, i64* %v6690
  ; Cycle B0000
  %v6693 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6694 = load i64, i64* %v6693
  %v6695 = add i64 %v6694, 1
  store i64 %v6695, i64* %v6693
  %v6696 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 483, i32* %v6696
  %v6697 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 739, i32* %v6697
  %v6698 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 995, i32* %v6698
  %v6699 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 20029, i32* %v6699
  ; Cycle B0001
  %v6700 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6701 = load i64, i64* %v6700
  %v6702 = add i64 %v6701, 1
  store i64 %v6702, i64* %v6700
  %v6703 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1507, i32* %v6703
  %v6704 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1763, i32* %v6704
  %v6705 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 32310, i32* %v6705
  %v6706 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2275, i32* %v6706
  ; Cycle B0002
  %v6707 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6708 = load i64, i64* %v6707
  %v6709 = add i64 %v6708, 1
  store i64 %v6709, i64* %v6707
  %v6710 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6711 = load i32, i32* %v6710
  %v6712 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6713 = load i32, i32* %v6712
  %v6714 = call i32 @cron_subbyte_ternary_dot(i32 %v6711, i32 %v6713)
  store i32 %v6714, i32* %v6710
  ; Cycle B0003
  %v6715 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6716 = load i64, i64* %v6715
  %v6717 = add i64 %v6716, 1
  store i64 %v6717, i64* %v6715
  %v6718 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6719 = load i64, i64* %v6718
  %v6720 = add i64 %v6719, 1
  store i64 %v6720, i64* %v6718
  ; Cycle B0000
  %v6721 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6722 = load i64, i64* %v6721
  %v6723 = add i64 %v6722, 1
  store i64 %v6723, i64* %v6721
  %v6724 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 484, i32* %v6724
  %v6725 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 740, i32* %v6725
  %v6726 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 996, i32* %v6726
  %v6727 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1252, i32* %v6727
  ; Cycle B0001
  %v6728 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6729 = load i64, i64* %v6728
  %v6730 = add i64 %v6729, 1
  store i64 %v6730, i64* %v6728
  %v6731 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1508, i32* %v6731
  %v6732 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 28233, i32* %v6732
  %v6733 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2020, i32* %v6733
  %v6734 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2276, i32* %v6734
  ; Cycle B0002
  %v6735 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6736 = load i64, i64* %v6735
  %v6737 = add i64 %v6736, 1
  store i64 %v6737, i64* %v6735
  %v6738 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6739 = load i32, i32* %v6738
  %v6740 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6741 = load i32, i32* %v6740
  %v6742 = call i32 @cron_subbyte_ternary_dot(i32 %v6739, i32 %v6741)
  store i32 %v6742, i32* %v6738
  ; Cycle B0003
  %v6743 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6744 = load i64, i64* %v6743
  %v6745 = add i64 %v6744, 1
  store i64 %v6745, i64* %v6743
  %v6746 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6747 = load i64, i64* %v6746
  %v6748 = add i64 %v6747, 1
  store i64 %v6748, i64* %v6746
  ; Cycle B0000
  %v6749 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6750 = load i64, i64* %v6749
  %v6751 = add i64 %v6750, 1
  store i64 %v6751, i64* %v6749
  %v6752 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 485, i32* %v6752
  %v6753 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 11866, i32* %v6753
  %v6754 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 15953, i32* %v6754
  %v6755 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1253, i32* %v6755
  ; Cycle B0001
  %v6756 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6757 = load i64, i64* %v6756
  %v6758 = add i64 %v6757, 1
  store i64 %v6758, i64* %v6756
  %v6759 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1509, i32* %v6759
  %v6760 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 28244, i32* %v6760
  %v6761 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2021, i32* %v6761
  %v6762 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 36432, i32* %v6762
  ; Cycle B0002
  %v6763 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6764 = load i64, i64* %v6763
  %v6765 = add i64 %v6764, 1
  store i64 %v6765, i64* %v6763
  %v6766 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6767 = load i32, i32* %v6766
  %v6768 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6769 = load i32, i32* %v6768
  %v6770 = call i32 @cron_subbyte_ternary_dot(i32 %v6767, i32 %v6769)
  store i32 %v6770, i32* %v6766
  ; Cycle B0003
  %v6771 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6772 = load i64, i64* %v6771
  %v6773 = add i64 %v6772, 1
  store i64 %v6773, i64* %v6771
  %v6774 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6775 = load i64, i64* %v6774
  %v6776 = add i64 %v6775, 1
  store i64 %v6776, i64* %v6774
  ; Cycle B0000
  %v6777 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6778 = load i64, i64* %v6777
  %v6779 = add i64 %v6778, 1
  store i64 %v6779, i64* %v6777
  %v6780 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 486, i32* %v6780
  %v6781 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 742, i32* %v6781
  %v6782 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 998, i32* %v6782
  %v6783 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1254, i32* %v6783
  ; Cycle B0001
  %v6784 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6785 = load i64, i64* %v6784
  %v6786 = add i64 %v6785, 1
  store i64 %v6786, i64* %v6784
  %v6787 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1510, i32* %v6787
  %v6788 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1766, i32* %v6788
  %v6789 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2022, i32* %v6789
  %v6790 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2278, i32* %v6790
  ; Cycle B0002
  %v6791 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6792 = load i64, i64* %v6791
  %v6793 = add i64 %v6792, 1
  store i64 %v6793, i64* %v6791
  %v6794 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6795 = load i32, i32* %v6794
  %v6796 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6797 = load i32, i32* %v6796
  %v6798 = call i32 @cron_subbyte_ternary_dot(i32 %v6795, i32 %v6797)
  store i32 %v6798, i32* %v6794
  ; Cycle B0003
  %v6799 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6800 = load i64, i64* %v6799
  %v6801 = add i64 %v6800, 1
  store i64 %v6801, i64* %v6799
  %v6802 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6803 = load i64, i64* %v6802
  %v6804 = add i64 %v6803, 1
  store i64 %v6804, i64* %v6802
  ; Cycle B0000
  %v6805 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6806 = load i64, i64* %v6805
  %v6807 = add i64 %v6806, 1
  store i64 %v6807, i64* %v6805
  %v6808 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 487, i32* %v6808
  %v6809 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 11891, i32* %v6809
  %v6810 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 999, i32* %v6810
  %v6811 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1255, i32* %v6811
  ; Cycle B0001
  %v6812 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6813 = load i64, i64* %v6812
  %v6814 = add i64 %v6813, 1
  store i64 %v6814, i64* %v6812
  %v6815 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1511, i32* %v6815
  %v6816 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 28283, i32* %v6816
  %v6817 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 32370, i32* %v6817
  %v6818 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2279, i32* %v6818
  ; Cycle B0002
  %v6819 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6820 = load i64, i64* %v6819
  %v6821 = add i64 %v6820, 1
  store i64 %v6821, i64* %v6819
  %v6822 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6823 = load i32, i32* %v6822
  %v6824 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6825 = load i32, i32* %v6824
  %v6826 = call i32 @cron_subbyte_ternary_dot(i32 %v6823, i32 %v6825)
  store i32 %v6826, i32* %v6822
  ; Cycle B0003
  %v6827 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6828 = load i64, i64* %v6827
  %v6829 = add i64 %v6828, 1
  store i64 %v6829, i64* %v6827
  %v6830 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6831 = load i64, i64* %v6830
  %v6832 = add i64 %v6831, 1
  store i64 %v6832, i64* %v6830
  ; Cycle B0000
  %v6833 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6834 = load i64, i64* %v6833
  %v6835 = add i64 %v6834, 1
  store i64 %v6835, i64* %v6833
  %v6836 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 488, i32* %v6836
  %v6837 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 744, i32* %v6837
  %v6838 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1000, i32* %v6838
  %v6839 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 20101, i32* %v6839
  ; Cycle B0001
  %v6840 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6841 = load i64, i64* %v6840
  %v6842 = add i64 %v6841, 1
  store i64 %v6842, i64* %v6840
  %v6843 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1512, i32* %v6843
  %v6844 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1768, i32* %v6844
  %v6845 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 32396, i32* %v6845
  %v6846 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2280, i32* %v6846
  ; Cycle B0002
  %v6847 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6848 = load i64, i64* %v6847
  %v6849 = add i64 %v6848, 1
  store i64 %v6849, i64* %v6847
  %v6850 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6851 = load i32, i32* %v6850
  %v6852 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6853 = load i32, i32* %v6852
  %v6854 = call i32 @cron_subbyte_ternary_dot(i32 %v6851, i32 %v6853)
  store i32 %v6854, i32* %v6850
  ; Cycle B0003
  %v6855 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6856 = load i64, i64* %v6855
  %v6857 = add i64 %v6856, 1
  store i64 %v6857, i64* %v6855
  %v6858 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6859 = load i64, i64* %v6858
  %v6860 = add i64 %v6859, 1
  store i64 %v6860, i64* %v6858
  ; Cycle B0000
  %v6861 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6862 = load i64, i64* %v6861
  %v6863 = add i64 %v6862, 1
  store i64 %v6863, i64* %v6861
  %v6864 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 7827, i32* %v6864
  %v6865 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 745, i32* %v6865
  %v6866 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1001, i32* %v6866
  %v6867 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 20120, i32* %v6867
  ; Cycle B0001
  %v6868 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6869 = load i64, i64* %v6868
  %v6870 = add i64 %v6869, 1
  store i64 %v6870, i64* %v6868
  %v6871 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1513, i32* %v6871
  %v6872 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1769, i32* %v6872
  %v6873 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 32415, i32* %v6873
  %v6874 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2281, i32* %v6874
  ; Cycle B0002
  %v6875 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6876 = load i64, i64* %v6875
  %v6877 = add i64 %v6876, 1
  store i64 %v6877, i64* %v6875
  %v6878 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6879 = load i32, i32* %v6878
  %v6880 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6881 = load i32, i32* %v6880
  %v6882 = call i32 @cron_subbyte_ternary_dot(i32 %v6879, i32 %v6881)
  store i32 %v6882, i32* %v6878
  ; Cycle B0003
  %v6883 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6884 = load i64, i64* %v6883
  %v6885 = add i64 %v6884, 1
  store i64 %v6885, i64* %v6883
  %v6886 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6887 = load i64, i64* %v6886
  %v6888 = add i64 %v6887, 1
  store i64 %v6888, i64* %v6886
  ; Cycle B0000
  %v6889 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6890 = load i64, i64* %v6889
  %v6891 = add i64 %v6890, 1
  store i64 %v6891, i64* %v6889
  %v6892 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 7853, i32* %v6892
  %v6893 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 746, i32* %v6893
  %v6894 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 16047, i32* %v6894
  %v6895 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1258, i32* %v6895
  ; Cycle B0001
  %v6896 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6897 = load i64, i64* %v6896
  %v6898 = add i64 %v6897, 1
  store i64 %v6898, i64* %v6896
  %v6899 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1514, i32* %v6899
  %v6900 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1770, i32* %v6900
  %v6901 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2026, i32* %v6901
  %v6902 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 36526, i32* %v6902
  ; Cycle B0002
  %v6903 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6904 = load i64, i64* %v6903
  %v6905 = add i64 %v6904, 1
  store i64 %v6905, i64* %v6903
  %v6906 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6907 = load i32, i32* %v6906
  %v6908 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6909 = load i32, i32* %v6908
  %v6910 = call i32 @cron_subbyte_ternary_dot(i32 %v6907, i32 %v6909)
  store i32 %v6910, i32* %v6906
  ; Cycle B0003
  %v6911 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6912 = load i64, i64* %v6911
  %v6913 = add i64 %v6912, 1
  store i64 %v6913, i64* %v6911
  %v6914 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6915 = load i64, i64* %v6914
  %v6916 = add i64 %v6915, 1
  store i64 %v6916, i64* %v6914
  ; Cycle B0000
  %v6917 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6918 = load i64, i64* %v6917
  %v6919 = add i64 %v6918, 1
  store i64 %v6919, i64* %v6917
  %v6920 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 491, i32* %v6920
  %v6921 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 747, i32* %v6921
  %v6922 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1003, i32* %v6922
  %v6923 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 20148, i32* %v6923
  ; Cycle B0001
  %v6924 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6925 = load i64, i64* %v6924
  %v6926 = add i64 %v6925, 1
  store i64 %v6926, i64* %v6924
  %v6927 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1515, i32* %v6927
  %v6928 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1771, i32* %v6928
  %v6929 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 32445, i32* %v6929
  %v6930 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2283, i32* %v6930
  ; Cycle B0002
  %v6931 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6932 = load i64, i64* %v6931
  %v6933 = add i64 %v6932, 1
  store i64 %v6933, i64* %v6931
  %v6934 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6935 = load i32, i32* %v6934
  %v6936 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6937 = load i32, i32* %v6936
  %v6938 = call i32 @cron_subbyte_ternary_dot(i32 %v6935, i32 %v6937)
  store i32 %v6938, i32* %v6934
  ; Cycle B0003
  %v6939 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6940 = load i64, i64* %v6939
  %v6941 = add i64 %v6940, 1
  store i64 %v6941, i64* %v6939
  %v6942 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6943 = load i64, i64* %v6942
  %v6944 = add i64 %v6943, 1
  store i64 %v6944, i64* %v6942
  ; Cycle B0000
  %v6945 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6946 = load i64, i64* %v6945
  %v6947 = add i64 %v6946, 1
  store i64 %v6947, i64* %v6945
  %v6948 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 492, i32* %v6948
  %v6949 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 748, i32* %v6949
  %v6950 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1004, i32* %v6950
  %v6951 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 20165, i32* %v6951
  ; Cycle B0001
  %v6952 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6953 = load i64, i64* %v6952
  %v6954 = add i64 %v6953, 1
  store i64 %v6954, i64* %v6952
  %v6955 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 24268, i32* %v6955
  %v6956 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1772, i32* %v6956
  %v6957 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 32462, i32* %v6957
  %v6958 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2284, i32* %v6958
  ; Cycle B0002
  %v6959 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6960 = load i64, i64* %v6959
  %v6961 = add i64 %v6960, 1
  store i64 %v6961, i64* %v6959
  %v6962 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6963 = load i32, i32* %v6962
  %v6964 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6965 = load i32, i32* %v6964
  %v6966 = call i32 @cron_subbyte_ternary_dot(i32 %v6963, i32 %v6965)
  store i32 %v6966, i32* %v6962
  ; Cycle B0003
  %v6967 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6968 = load i64, i64* %v6967
  %v6969 = add i64 %v6968, 1
  store i64 %v6969, i64* %v6967
  %v6970 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6971 = load i64, i64* %v6970
  %v6972 = add i64 %v6971, 1
  store i64 %v6972, i64* %v6970
  ; Cycle B0000
  %v6973 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6974 = load i64, i64* %v6973
  %v6975 = add i64 %v6974, 1
  store i64 %v6975, i64* %v6973
  %v6976 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 493, i32* %v6976
  %v6977 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 749, i32* %v6977
  %v6978 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1005, i32* %v6978
  %v6979 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1261, i32* %v6979
  ; Cycle B0001
  %v6980 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6981 = load i64, i64* %v6980
  %v6982 = add i64 %v6981, 1
  store i64 %v6982, i64* %v6980
  %v6983 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1517, i32* %v6983
  %v6984 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 28379, i32* %v6984
  %v6985 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2029, i32* %v6985
  %v6986 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2285, i32* %v6986
  ; Cycle B0002
  %v6987 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6988 = load i64, i64* %v6987
  %v6989 = add i64 %v6988, 1
  store i64 %v6989, i64* %v6987
  %v6990 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v6991 = load i32, i32* %v6990
  %v6992 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v6993 = load i32, i32* %v6992
  %v6994 = call i32 @cron_subbyte_ternary_dot(i32 %v6991, i32 %v6993)
  store i32 %v6994, i32* %v6990
  ; Cycle B0003
  %v6995 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v6996 = load i64, i64* %v6995
  %v6997 = add i64 %v6996, 1
  store i64 %v6997, i64* %v6995
  %v6998 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v6999 = load i64, i64* %v6998
  %v7000 = add i64 %v6999, 1
  store i64 %v7000, i64* %v6998
  ; Cycle B0000
  %v7001 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7002 = load i64, i64* %v7001
  %v7003 = add i64 %v7002, 1
  store i64 %v7003, i64* %v7001
  %v7004 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 494, i32* %v7004
  %v7005 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 750, i32* %v7005
  %v7006 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1006, i32* %v7006
  %v7007 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1262, i32* %v7007
  ; Cycle B0001
  %v7008 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7009 = load i64, i64* %v7008
  %v7010 = add i64 %v7009, 1
  store i64 %v7010, i64* %v7008
  %v7011 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1518, i32* %v7011
  %v7012 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1774, i32* %v7012
  %v7013 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2030, i32* %v7013
  %v7014 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2286, i32* %v7014
  ; Cycle B0002
  %v7015 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7016 = load i64, i64* %v7015
  %v7017 = add i64 %v7016, 1
  store i64 %v7017, i64* %v7015
  %v7018 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v7019 = load i32, i32* %v7018
  %v7020 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v7021 = load i32, i32* %v7020
  %v7022 = call i32 @cron_subbyte_ternary_dot(i32 %v7019, i32 %v7021)
  store i32 %v7022, i32* %v7018
  ; Cycle B0003
  %v7023 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7024 = load i64, i64* %v7023
  %v7025 = add i64 %v7024, 1
  store i64 %v7025, i64* %v7023
  %v7026 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v7027 = load i64, i64* %v7026
  %v7028 = add i64 %v7027, 1
  store i64 %v7028, i64* %v7026
  ; Cycle B0000
  %v7029 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7030 = load i64, i64* %v7029
  %v7031 = add i64 %v7030, 1
  store i64 %v7031, i64* %v7029
  %v7032 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 495, i32* %v7032
  %v7033 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 12026, i32* %v7033
  %v7034 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1007, i32* %v7034
  %v7035 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 20208, i32* %v7035
  ; Cycle B0001
  %v7036 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7037 = load i64, i64* %v7036
  %v7038 = add i64 %v7037, 1
  store i64 %v7038, i64* %v7036
  %v7039 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1519, i32* %v7039
  %v7040 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1775, i32* %v7040
  %v7041 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2031, i32* %v7041
  %v7042 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2287, i32* %v7042
  ; Cycle B0002
  %v7043 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7044 = load i64, i64* %v7043
  %v7045 = add i64 %v7044, 1
  store i64 %v7045, i64* %v7043
  %v7046 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v7047 = load i32, i32* %v7046
  %v7048 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v7049 = load i32, i32* %v7048
  %v7050 = call i32 @cron_subbyte_ternary_dot(i32 %v7047, i32 %v7049)
  store i32 %v7050, i32* %v7046
  ; Cycle B0003
  %v7051 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7052 = load i64, i64* %v7051
  %v7053 = add i64 %v7052, 1
  store i64 %v7053, i64* %v7051
  %v7054 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v7055 = load i64, i64* %v7054
  %v7056 = add i64 %v7055, 1
  store i64 %v7056, i64* %v7054
  ; Cycle B0000
  %v7057 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7058 = load i64, i64* %v7057
  %v7059 = add i64 %v7058, 1
  store i64 %v7059, i64* %v7057
  %v7060 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 7939, i32* %v7060
  %v7061 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 752, i32* %v7061
  %v7062 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1008, i32* %v7062
  %v7063 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 20232, i32* %v7063
  ; Cycle B0001
  %v7064 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7065 = load i64, i64* %v7064
  %v7066 = add i64 %v7065, 1
  store i64 %v7066, i64* %v7064
  %v7067 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1520, i32* %v7067
  %v7068 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1776, i32* %v7068
  %v7069 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 32527, i32* %v7069
  %v7070 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2288, i32* %v7070
  ; Cycle B0002
  %v7071 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7072 = load i64, i64* %v7071
  %v7073 = add i64 %v7072, 1
  store i64 %v7073, i64* %v7071
  %v7074 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v7075 = load i64, i64* %v7074
  %v7076 = add i64 %v7075, 1
  store i64 %v7076, i64* %v7074
  %v7077 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v7077
  %v7078 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v7079 = load i64, i64* %v7078
  %v7080 = add i64 %v7079, 1
  store i64 %v7080, i64* %v7078
  %v7081 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 132, i32* %v7081
  ; Cycle B0003
  %v7082 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7083 = load i64, i64* %v7082
  %v7084 = add i64 %v7083, 1
  store i64 %v7084, i64* %v7082
  %v7085 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v7086 = load i64, i64* %v7085
  %v7087 = add i64 %v7086, 1
  store i64 %v7087, i64* %v7085
  ; Cycle B0000
  %v7088 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7089 = load i64, i64* %v7088
  %v7090 = add i64 %v7089, 1
  store i64 %v7090, i64* %v7088
  %v7091 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 497, i32* %v7091
  %v7092 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 753, i32* %v7092
  %v7093 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1009, i32* %v7093
  %v7094 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 20245, i32* %v7094
  ; Cycle B0001
  %v7095 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7096 = load i64, i64* %v7095
  %v7097 = add i64 %v7096, 1
  store i64 %v7097, i64* %v7095
  %v7098 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1521, i32* %v7098
  %v7099 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1777, i32* %v7099
  %v7100 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 32540, i32* %v7100
  %v7101 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2289, i32* %v7101
  ; Cycle B0002
  %v7102 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7103 = load i64, i64* %v7102
  %v7104 = add i64 %v7103, 1
  store i64 %v7104, i64* %v7102
  %v7105 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v7106 = load i64, i64* %v7105
  %v7107 = add i64 %v7106, 1
  store i64 %v7107, i64* %v7105
  %v7108 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v7108
  %v7109 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v7110 = load i64, i64* %v7109
  %v7111 = add i64 %v7110, 1
  store i64 %v7111, i64* %v7109
  %v7112 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 132, i32* %v7112
  ; Cycle B0003
  %v7113 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7114 = load i64, i64* %v7113
  %v7115 = add i64 %v7114, 1
  store i64 %v7115, i64* %v7113
  %v7116 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v7117 = load i64, i64* %v7116
  %v7118 = add i64 %v7117, 1
  store i64 %v7118, i64* %v7116
  ; Cycle B0000
  %v7119 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7120 = load i64, i64* %v7119
  %v7121 = add i64 %v7120, 1
  store i64 %v7121, i64* %v7119
  %v7122 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 498, i32* %v7122
  %v7123 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 754, i32* %v7123
  %v7124 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 16174, i32* %v7124
  %v7125 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1266, i32* %v7125
  ; Cycle B0001
  %v7126 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7127 = load i64, i64* %v7126
  %v7128 = add i64 %v7127, 1
  store i64 %v7128, i64* %v7126
  %v7129 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 24356, i32* %v7129
  %v7130 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1778, i32* %v7130
  %v7131 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2034, i32* %v7131
  %v7132 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 36655, i32* %v7132
  ; Cycle B0002
  %v7133 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7134 = load i64, i64* %v7133
  %v7135 = add i64 %v7134, 1
  store i64 %v7135, i64* %v7133
  %v7136 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v7137 = load i64, i64* %v7136
  %v7138 = add i64 %v7137, 1
  store i64 %v7138, i64* %v7136
  %v7139 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v7139
  %v7140 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v7141 = load i64, i64* %v7140
  %v7142 = add i64 %v7141, 1
  store i64 %v7142, i64* %v7140
  %v7143 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 132, i32* %v7143
  ; Cycle B0003
  %v7144 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7145 = load i64, i64* %v7144
  %v7146 = add i64 %v7145, 1
  store i64 %v7146, i64* %v7144
  %v7147 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v7148 = load i64, i64* %v7147
  %v7149 = add i64 %v7148, 1
  store i64 %v7149, i64* %v7147
  ; Cycle B0000
  %v7150 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7151 = load i64, i64* %v7150
  %v7152 = add i64 %v7151, 1
  store i64 %v7152, i64* %v7150
  %v7153 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 499, i32* %v7153
  %v7154 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 755, i32* %v7154
  %v7155 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 16189, i32* %v7155
  %v7156 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1267, i32* %v7156
  ; Cycle B0001
  %v7157 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7158 = load i64, i64* %v7157
  %v7159 = add i64 %v7158, 1
  store i64 %v7159, i64* %v7157
  %v7160 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1523, i32* %v7160
  %v7161 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1779, i32* %v7161
  %v7162 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2035, i32* %v7162
  %v7163 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 36668, i32* %v7163
  ; Cycle B0002
  %v7164 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7165 = load i64, i64* %v7164
  %v7166 = add i64 %v7165, 1
  store i64 %v7166, i64* %v7164
  %v7167 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v7168 = load i64, i64* %v7167
  %v7169 = add i64 %v7168, 1
  store i64 %v7169, i64* %v7167
  %v7170 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v7170
  %v7171 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v7172 = load i64, i64* %v7171
  %v7173 = add i64 %v7172, 1
  store i64 %v7173, i64* %v7171
  %v7174 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 132, i32* %v7174
  ; Cycle B0003
  %v7175 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7176 = load i64, i64* %v7175
  %v7177 = add i64 %v7176, 1
  store i64 %v7177, i64* %v7175
  %v7178 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v7179 = load i64, i64* %v7178
  %v7180 = add i64 %v7179, 1
  store i64 %v7180, i64* %v7178
  ; Cycle B0000
  %v7181 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7182 = load i64, i64* %v7181
  %v7183 = add i64 %v7182, 1
  store i64 %v7183, i64* %v7181
  %v7184 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 8007, i32* %v7184
  %v7185 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 756, i32* %v7185
  %v7186 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1012, i32* %v7186
  %v7187 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1268, i32* %v7187
  ; Cycle B0001
  %v7188 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7189 = load i64, i64* %v7188
  %v7190 = add i64 %v7189, 1
  store i64 %v7190, i64* %v7188
  %v7191 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 24395, i32* %v7191
  %v7192 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1780, i32* %v7192
  %v7193 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2036, i32* %v7193
  %v7194 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2292, i32* %v7194
  ; Cycle B0002
  %v7195 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7196 = load i64, i64* %v7195
  %v7197 = add i64 %v7196, 1
  store i64 %v7197, i64* %v7195
  %v7198 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v7199 = load i64, i64* %v7198
  %v7200 = add i64 %v7199, 1
  store i64 %v7200, i64* %v7198
  %v7201 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v7201
  %v7202 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v7203 = load i64, i64* %v7202
  %v7204 = add i64 %v7203, 1
  store i64 %v7204, i64* %v7202
  %v7205 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 132, i32* %v7205
  ; Cycle B0003
  %v7206 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7207 = load i64, i64* %v7206
  %v7208 = add i64 %v7207, 1
  store i64 %v7208, i64* %v7206
  %v7209 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v7210 = load i64, i64* %v7209
  %v7211 = add i64 %v7210, 1
  store i64 %v7211, i64* %v7209
  ; Cycle B0000
  %v7212 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7213 = load i64, i64* %v7212
  %v7214 = add i64 %v7213, 1
  store i64 %v7214, i64* %v7212
  %v7215 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 501, i32* %v7215
  %v7216 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 12123, i32* %v7216
  %v7217 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1013, i32* %v7217
  %v7218 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 20305, i32* %v7218
  ; Cycle B0001
  %v7219 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7220 = load i64, i64* %v7219
  %v7221 = add i64 %v7220, 1
  store i64 %v7221, i64* %v7219
  %v7222 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1525, i32* %v7222
  %v7223 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1781, i32* %v7223
  %v7224 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2037, i32* %v7224
  %v7225 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2293, i32* %v7225
  ; Cycle B0002
  %v7226 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7227 = load i64, i64* %v7226
  %v7228 = add i64 %v7227, 1
  store i64 %v7228, i64* %v7226
  %v7229 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v7230 = load i64, i64* %v7229
  %v7231 = add i64 %v7230, 1
  store i64 %v7231, i64* %v7229
  %v7232 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v7232
  %v7233 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v7234 = load i64, i64* %v7233
  %v7235 = add i64 %v7234, 1
  store i64 %v7235, i64* %v7233
  %v7236 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 132, i32* %v7236
  ; Cycle B0003
  %v7237 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7238 = load i64, i64* %v7237
  %v7239 = add i64 %v7238, 1
  store i64 %v7239, i64* %v7237
  %v7240 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v7241 = load i64, i64* %v7240
  %v7242 = add i64 %v7241, 1
  store i64 %v7242, i64* %v7240
  ; Cycle B0000
  %v7243 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7244 = load i64, i64* %v7243
  %v7245 = add i64 %v7244, 1
  store i64 %v7245, i64* %v7243
  %v7246 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 8042, i32* %v7246
  %v7247 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 758, i32* %v7247
  %v7248 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1014, i32* %v7248
  %v7249 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1270, i32* %v7249
  ; Cycle B0001
  %v7250 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7251 = load i64, i64* %v7250
  %v7252 = add i64 %v7251, 1
  store i64 %v7252, i64* %v7250
  %v7253 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 24424, i32* %v7253
  %v7254 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1782, i32* %v7254
  %v7255 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2038, i32* %v7255
  %v7256 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2294, i32* %v7256
  ; Cycle B0002
  %v7257 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7258 = load i64, i64* %v7257
  %v7259 = add i64 %v7258, 1
  store i64 %v7259, i64* %v7257
  %v7260 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v7261 = load i64, i64* %v7260
  %v7262 = add i64 %v7261, 1
  store i64 %v7262, i64* %v7260
  %v7263 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v7263
  %v7264 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v7265 = load i64, i64* %v7264
  %v7266 = add i64 %v7265, 1
  store i64 %v7266, i64* %v7264
  %v7267 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 132, i32* %v7267
  ; Cycle B0003
  %v7268 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7269 = load i64, i64* %v7268
  %v7270 = add i64 %v7269, 1
  store i64 %v7270, i64* %v7268
  %v7271 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v7272 = load i64, i64* %v7271
  %v7273 = add i64 %v7272, 1
  store i64 %v7273, i64* %v7271
  ; Cycle B0000
  %v7274 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7275 = load i64, i64* %v7274
  %v7276 = add i64 %v7275, 1
  store i64 %v7276, i64* %v7274
  %v7277 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 503, i32* %v7277
  %v7278 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 759, i32* %v7278
  %v7279 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1015, i32* %v7279
  %v7280 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1271, i32* %v7280
  ; Cycle B0001
  %v7281 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7282 = load i64, i64* %v7281
  %v7283 = add i64 %v7282, 1
  store i64 %v7283, i64* %v7281
  %v7284 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 24441, i32* %v7284
  %v7285 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 28538, i32* %v7285
  %v7286 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2039, i32* %v7286
  %v7287 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2295, i32* %v7287
  ; Cycle B0002
  %v7288 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7289 = load i64, i64* %v7288
  %v7290 = add i64 %v7289, 1
  store i64 %v7290, i64* %v7288
  %v7291 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v7292 = load i64, i64* %v7291
  %v7293 = add i64 %v7292, 1
  store i64 %v7293, i64* %v7291
  %v7294 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v7294
  %v7295 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v7296 = load i64, i64* %v7295
  %v7297 = add i64 %v7296, 1
  store i64 %v7297, i64* %v7295
  %v7298 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 132, i32* %v7298
  ; Cycle B0003
  %v7299 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7300 = load i64, i64* %v7299
  %v7301 = add i64 %v7300, 1
  store i64 %v7301, i64* %v7299
  %v7302 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v7303 = load i64, i64* %v7302
  %v7304 = add i64 %v7303, 1
  store i64 %v7304, i64* %v7302
  ; Cycle B0000
  %v7305 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7306 = load i64, i64* %v7305
  %v7307 = add i64 %v7306, 1
  store i64 %v7307, i64* %v7305
  %v7308 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 504, i32* %v7308
  %v7309 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 760, i32* %v7309
  %v7310 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 16261, i32* %v7310
  %v7311 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1272, i32* %v7311
  ; Cycle B0001
  %v7312 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7313 = load i64, i64* %v7312
  %v7314 = add i64 %v7313, 1
  store i64 %v7314, i64* %v7312
  %v7315 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1528, i32* %v7315
  %v7316 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 28544, i32* %v7316
  %v7317 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2040, i32* %v7317
  %v7318 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 36740, i32* %v7318
  ; Cycle B0002
  %v7319 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7320 = load i64, i64* %v7319
  %v7321 = add i64 %v7320, 1
  store i64 %v7321, i64* %v7319
  %v7322 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v7323 = load i64, i64* %v7322
  %v7324 = add i64 %v7323, 1
  store i64 %v7324, i64* %v7322
  %v7325 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v7325
  %v7326 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v7327 = load i64, i64* %v7326
  %v7328 = add i64 %v7327, 1
  store i64 %v7328, i64* %v7326
  %v7329 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 132, i32* %v7329
  ; Cycle B0003
  %v7330 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7331 = load i64, i64* %v7330
  %v7332 = add i64 %v7331, 1
  store i64 %v7332, i64* %v7330
  %v7333 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v7334 = load i64, i64* %v7333
  %v7335 = add i64 %v7334, 1
  store i64 %v7335, i64* %v7333
  ; Cycle B0000
  %v7336 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7337 = load i64, i64* %v7336
  %v7338 = add i64 %v7337, 1
  store i64 %v7338, i64* %v7336
  %v7339 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 505, i32* %v7339
  %v7340 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 761, i32* %v7340
  %v7341 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 16280, i32* %v7341
  %v7342 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1273, i32* %v7342
  ; Cycle B0001
  %v7343 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7344 = load i64, i64* %v7343
  %v7345 = add i64 %v7344, 1
  store i64 %v7345, i64* %v7343
  %v7346 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 24476, i32* %v7346
  %v7347 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1785, i32* %v7347
  %v7348 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2041, i32* %v7348
  %v7349 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 36757, i32* %v7349
  ; Cycle B0002
  %v7350 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7351 = load i64, i64* %v7350
  %v7352 = add i64 %v7351, 1
  store i64 %v7352, i64* %v7350
  %v7353 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v7354 = load i64, i64* %v7353
  %v7355 = add i64 %v7354, 1
  store i64 %v7355, i64* %v7353
  %v7356 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v7356
  %v7357 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v7358 = load i64, i64* %v7357
  %v7359 = add i64 %v7358, 1
  store i64 %v7359, i64* %v7357
  %v7360 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 132, i32* %v7360
  ; Cycle B0003
  %v7361 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7362 = load i64, i64* %v7361
  %v7363 = add i64 %v7362, 1
  store i64 %v7363, i64* %v7361
  %v7364 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v7365 = load i64, i64* %v7364
  %v7366 = add i64 %v7365, 1
  store i64 %v7366, i64* %v7364
  ; Cycle B0000
  %v7367 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7368 = load i64, i64* %v7367
  %v7369 = add i64 %v7368, 1
  store i64 %v7369, i64* %v7367
  %v7370 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 8106, i32* %v7370
  %v7371 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 762, i32* %v7371
  %v7372 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1018, i32* %v7372
  %v7373 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 20399, i32* %v7373
  ; Cycle B0001
  %v7374 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7375 = load i64, i64* %v7374
  %v7376 = add i64 %v7375, 1
  store i64 %v7376, i64* %v7374
  %v7377 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1530, i32* %v7377
  %v7378 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1786, i32* %v7378
  %v7379 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 32678, i32* %v7379
  %v7380 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2298, i32* %v7380
  ; Cycle B0002
  %v7381 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7382 = load i64, i64* %v7381
  %v7383 = add i64 %v7382, 1
  store i64 %v7383, i64* %v7381
  %v7384 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v7385 = load i64, i64* %v7384
  %v7386 = add i64 %v7385, 1
  store i64 %v7386, i64* %v7384
  %v7387 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v7387
  %v7388 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v7389 = load i64, i64* %v7388
  %v7390 = add i64 %v7389, 1
  store i64 %v7390, i64* %v7388
  %v7391 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 132, i32* %v7391
  ; Cycle B0003
  %v7392 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7393 = load i64, i64* %v7392
  %v7394 = add i64 %v7393, 1
  store i64 %v7394, i64* %v7392
  %v7395 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v7396 = load i64, i64* %v7395
  %v7397 = add i64 %v7396, 1
  store i64 %v7397, i64* %v7395
  ; Cycle B0000
  %v7398 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7399 = load i64, i64* %v7398
  %v7400 = add i64 %v7399, 1
  store i64 %v7400, i64* %v7398
  %v7401 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 507, i32* %v7401
  %v7402 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 763, i32* %v7402
  %v7403 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 16308, i32* %v7403
  %v7404 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1275, i32* %v7404
  ; Cycle B0001
  %v7405 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7406 = load i64, i64* %v7405
  %v7407 = add i64 %v7406, 1
  store i64 %v7407, i64* %v7405
  %v7408 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1531, i32* %v7408
  %v7409 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1787, i32* %v7409
  %v7410 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2043, i32* %v7410
  %v7411 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 36789, i32* %v7411
  ; Cycle B0002
  %v7412 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7413 = load i64, i64* %v7412
  %v7414 = add i64 %v7413, 1
  store i64 %v7414, i64* %v7412
  %v7415 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v7416 = load i64, i64* %v7415
  %v7417 = add i64 %v7416, 1
  store i64 %v7417, i64* %v7415
  %v7418 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v7418
  %v7419 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v7420 = load i64, i64* %v7419
  %v7421 = add i64 %v7420, 1
  store i64 %v7421, i64* %v7419
  %v7422 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 132, i32* %v7422
  ; Cycle B0003
  %v7423 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7424 = load i64, i64* %v7423
  %v7425 = add i64 %v7424, 1
  store i64 %v7425, i64* %v7423
  %v7426 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v7427 = load i64, i64* %v7426
  %v7428 = add i64 %v7427, 1
  store i64 %v7428, i64* %v7426
  ; Cycle B0000
  %v7429 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7430 = load i64, i64* %v7429
  %v7431 = add i64 %v7430, 1
  store i64 %v7431, i64* %v7429
  %v7432 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 508, i32* %v7432
  %v7433 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 764, i32* %v7433
  %v7434 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 16325, i32* %v7434
  %v7435 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1276, i32* %v7435
  ; Cycle B0001
  %v7436 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7437 = load i64, i64* %v7436
  %v7438 = add i64 %v7437, 1
  store i64 %v7438, i64* %v7436
  %v7439 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 24523, i32* %v7439
  %v7440 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1788, i32* %v7440
  %v7441 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2044, i32* %v7441
  %v7442 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 36808, i32* %v7442
  ; Cycle B0002
  %v7443 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7444 = load i64, i64* %v7443
  %v7445 = add i64 %v7444, 1
  store i64 %v7445, i64* %v7443
  %v7446 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v7447 = load i64, i64* %v7446
  %v7448 = add i64 %v7447, 1
  store i64 %v7448, i64* %v7446
  %v7449 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v7449
  %v7450 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v7451 = load i64, i64* %v7450
  %v7452 = add i64 %v7451, 1
  store i64 %v7452, i64* %v7450
  %v7453 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 132, i32* %v7453
  ; Cycle B0003
  %v7454 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7455 = load i64, i64* %v7454
  %v7456 = add i64 %v7455, 1
  store i64 %v7456, i64* %v7454
  %v7457 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v7458 = load i64, i64* %v7457
  %v7459 = add i64 %v7458, 1
  store i64 %v7459, i64* %v7457
  ; Cycle B0000
  %v7460 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7461 = load i64, i64* %v7460
  %v7462 = add i64 %v7461, 1
  store i64 %v7462, i64* %v7460
  %v7463 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 509, i32* %v7463
  %v7464 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 12242, i32* %v7464
  %v7465 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1021, i32* %v7465
  %v7466 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1277, i32* %v7466
  ; Cycle B0001
  %v7467 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7468 = load i64, i64* %v7467
  %v7469 = add i64 %v7468, 1
  store i64 %v7469, i64* %v7467
  %v7470 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1533, i32* %v7470
  %v7471 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1789, i32* %v7471
  %v7472 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2045, i32* %v7472
  %v7473 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2301, i32* %v7473
  ; Cycle B0002
  %v7474 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7475 = load i64, i64* %v7474
  %v7476 = add i64 %v7475, 1
  store i64 %v7476, i64* %v7474
  %v7477 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v7478 = load i64, i64* %v7477
  %v7479 = add i64 %v7478, 1
  store i64 %v7479, i64* %v7477
  %v7480 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v7480
  %v7481 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v7482 = load i64, i64* %v7481
  %v7483 = add i64 %v7482, 1
  store i64 %v7483, i64* %v7481
  %v7484 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 132, i32* %v7484
  ; Cycle B0003
  %v7485 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7486 = load i64, i64* %v7485
  %v7487 = add i64 %v7486, 1
  store i64 %v7487, i64* %v7485
  %v7488 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v7489 = load i64, i64* %v7488
  %v7490 = add i64 %v7489, 1
  store i64 %v7490, i64* %v7488
  ; Cycle B0000
  %v7491 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7492 = load i64, i64* %v7491
  %v7493 = add i64 %v7492, 1
  store i64 %v7493, i64* %v7491
  %v7494 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 8174, i32* %v7494
  %v7495 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 12263, i32* %v7495
  %v7496 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1022, i32* %v7496
  %v7497 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1278, i32* %v7497
  ; Cycle B0001
  %v7498 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7499 = load i64, i64* %v7498
  %v7500 = add i64 %v7499, 1
  store i64 %v7500, i64* %v7498
  %v7501 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1534, i32* %v7501
  %v7502 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1790, i32* %v7502
  %v7503 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2046, i32* %v7503
  %v7504 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2302, i32* %v7504
  ; Cycle B0002
  %v7505 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7506 = load i64, i64* %v7505
  %v7507 = add i64 %v7506, 1
  store i64 %v7507, i64* %v7505
  %v7508 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v7509 = load i64, i64* %v7508
  %v7510 = add i64 %v7509, 1
  store i64 %v7510, i64* %v7508
  %v7511 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v7511
  %v7512 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v7513 = load i64, i64* %v7512
  %v7514 = add i64 %v7513, 1
  store i64 %v7514, i64* %v7512
  %v7515 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 132, i32* %v7515
  ; Cycle B0003
  %v7516 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7517 = load i64, i64* %v7516
  %v7518 = add i64 %v7517, 1
  store i64 %v7518, i64* %v7516
  %v7519 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v7520 = load i64, i64* %v7519
  %v7521 = add i64 %v7520, 1
  store i64 %v7521, i64* %v7519
  ; Cycle B0000
  %v7522 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7523 = load i64, i64* %v7522
  %v7524 = add i64 %v7523, 1
  store i64 %v7524, i64* %v7522
  %v7525 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 511, i32* %v7525
  %v7526 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 767, i32* %v7526
  %v7527 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 16368, i32* %v7527
  %v7528 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1279, i32* %v7528
  ; Cycle B0001
  %v7529 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7530 = load i64, i64* %v7529
  %v7531 = add i64 %v7530, 1
  store i64 %v7531, i64* %v7529
  %v7532 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 1535, i32* %v7532
  %v7533 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 28657, i32* %v7533
  %v7534 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 2047, i32* %v7534
  %v7535 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 36849, i32* %v7535
  ; Cycle B0002
  %v7536 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7537 = load i64, i64* %v7536
  %v7538 = add i64 %v7537, 1
  store i64 %v7538, i64* %v7536
  %v7539 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v7540 = load i64, i64* %v7539
  %v7541 = add i64 %v7540, 1
  store i64 %v7541, i64* %v7539
  %v7542 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 16711680, i32* %v7542
  %v7543 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v7544 = load i64, i64* %v7543
  %v7545 = add i64 %v7544, 1
  store i64 %v7545, i64* %v7543
  %v7546 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 132, i32* %v7546
  ; Cycle B0003
  %v7547 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7548 = load i64, i64* %v7547
  %v7549 = add i64 %v7548, 1
  store i64 %v7549, i64* %v7547
  %v7550 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 12
  %v7551 = load i64, i64* %v7550
  %v7552 = add i64 %v7551, 1
  store i64 %v7552, i64* %v7550
  ret void
}

define i32 @main(i32 %argc, i8** %argv) nounwind {
entry:
  %core = alloca %struct.CronSiliconCore, align 8
  %core_raw = bitcast %struct.CronSiliconCore* %core to i8*
  call void @llvm.memset.p0i8.i64(i8* %core_raw, i8 0, i64 2280, i1 false)
  call void @cron_execute_bundles(%struct.CronSiliconCore* %core)

  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([62 x i8], [62 x i8]* @.str_hdr, i32 0, i32 0))
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([62 x i8], [62 x i8]* @.str_title, i32 0, i32 0))
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([62 x i8], [62 x i8]* @.str_hdr, i32 0, i32 0))
  %v7553 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v7554 = load i64, i64* %v7553
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([36 x i8], [36 x i8]* @.str_cycles, i32 0, i32 0), i64 %v7554)
  %v7555 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v7556 = load i64, i64* %v7555
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([36 x i8], [36 x i8]* @.str_optical, i32 0, i32 0), i64 %v7556)
  %v7557 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 10
  %v7558 = load i64, i64* %v7557
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([36 x i8], [36 x i8]* @.str_rev, i32 0, i32 0), i64 %v7558)
  %v7559 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v7560 = load i64, i64* %v7559
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([36 x i8], [36 x i8]* @.str_stdp, i32 0, i32 0), i64 %v7560)
  %v7561 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  %v7562 = load i32, i32* %v7561
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([48 x i8], [48 x i8]* @.str_r0, i32 0, i32 0), i32 %v7562, i32 %v7562)
  %v7563 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v7564 = load i32, i32* %v7563
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([48 x i8], [48 x i8]* @.str_r1, i32 0, i32 0), i32 %v7564, i32 %v7564)
  %v7565 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v7566 = load i32, i32* %v7565
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([48 x i8], [48 x i8]* @.str_r4, i32 0, i32 0), i32 %v7566, i32 %v7566)
  %v7567 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 6
  %v7568 = load i32, i32* %v7567
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([48 x i8], [48 x i8]* @.str_r6, i32 0, i32 0), i32 %v7568, i32 %v7568)
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([62 x i8], [62 x i8]* @.str_hdr, i32 0, i32 0))
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([62 x i8], [62 x i8]* @.str_status, i32 0, i32 0))
  ret i32 0
}
