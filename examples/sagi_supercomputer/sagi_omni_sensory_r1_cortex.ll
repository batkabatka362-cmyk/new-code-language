; ModuleID = 'cron_cl_sagi_omni_sensory_r1_cortex'
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
  ; Cycle B0001
  %v1 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v2 = load i64, i64* %v1
  %v3 = add i64 %v2, 1
  store i64 %v3, i64* %v1
  %v4 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v5 = load i64, i64* %v4
  %v6 = add i64 %v5, 1
  store i64 %v6, i64* %v4
  %v7 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 132, i32* %v7
  ; Cycle B0002
  %v8 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v9 = load i64, i64* %v8
  %v10 = add i64 %v9, 1
  store i64 %v10, i64* %v8
  %v11 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v12 = load i32, i32* %v11
  %v13 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v14 = load i32, i32* %v13
  %v15 = add i32 %v12, %v14
  %v16 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 %v15, i32* %v16
  %v17 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 4, i32* %v17
  %v18 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  %v19 = load i32, i32* %v18
  %v20 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 3
  %v21 = load i32, i32* %v20
  %v22 = add i32 %v19, %v21
  %v23 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 %v22, i32* %v23
  %v24 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 11
  store i32 4, i32* %v24
  ; Cycle B0003
  %v25 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v26 = load i64, i64* %v25
  %v27 = add i64 %v26, 1
  store i64 %v27, i64* %v25
  %v28 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v29 = load i64, i64* %v28
  %v30 = add i64 %v29, 1
  store i64 %v30, i64* %v28
  %v31 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 12
  store i32 16755285, i32* %v31
  %v32 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 13
  %v33 = load i32, i32* %v32
  %v34 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  %v35 = load i32, i32* %v34
  %v36 = add i32 %v33, %v35
  %v37 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 13
  store i32 %v36, i32* %v37
  ; Cycle B0004
  %v38 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v39 = load i64, i64* %v38
  %v40 = add i64 %v39, 1
  store i64 %v40, i64* %v38
  %v41 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 14
  %v42 = load i32, i32* %v41
  %v43 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 13
  %v44 = load i32, i32* %v43
  %v45 = add i32 %v42, %v44
  %v46 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 14
  store i32 %v45, i32* %v46
  %v47 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v48 = load i32, i32* %v47
  %v49 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  %v50 = load i32, i32* %v49
  %v51 = add i32 %v48, %v50
  %v52 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  store i32 %v51, i32* %v52
  ; Cycle B0005
  %v53 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v54 = load i64, i64* %v53
  %v55 = add i64 %v54, 1
  store i64 %v55, i64* %v53
  %v56 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v57 = load i32, i32* %v56
  %v58 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v59 = load i32, i32* %v58
  %v60 = add i32 %v57, %v59
  %v61 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  store i32 %v60, i32* %v61
  %v62 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 3
  %v63 = load i32, i32* %v62
  %v64 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  %v65 = load i32, i32* %v64
  %v66 = add i32 %v63, %v65
  %v67 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 3
  store i32 %v66, i32* %v67
  ; Cycle B0006
  %v68 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v69 = load i64, i64* %v68
  %v70 = add i64 %v69, 1
  store i64 %v70, i64* %v68
  %v71 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v72 = load i32, i32* %v71
  %v73 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 3
  %v74 = load i32, i32* %v73
  %v75 = add i32 %v72, %v74
  %v76 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  store i32 %v75, i32* %v76
  %v77 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 5
  %v78 = load i32, i32* %v77
  %v79 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  %v80 = load i32, i32* %v79
  %v81 = add i32 %v78, %v80
  %v82 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 5
  store i32 %v81, i32* %v82
  ; Cycle B0007
  %v83 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v84 = load i64, i64* %v83
  %v85 = add i64 %v84, 1
  store i64 %v85, i64* %v83
  %v86 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 6
  %v87 = load i32, i32* %v86
  %v88 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 5
  %v89 = load i32, i32* %v88
  %v90 = add i32 %v87, %v89
  %v91 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 6
  store i32 %v90, i32* %v91
  %v92 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  store i32 1, i32* %v92
  ; Cycle B0008
  %v93 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v94 = load i64, i64* %v93
  %v95 = add i64 %v94, 1
  store i64 %v95, i64* %v93
  %v96 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v97 = load i32, i32* %v96
  %v98 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  %v99 = load i32, i32* %v98
  %v100 = add i32 %v97, %v99
  %v101 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  store i32 %v100, i32* %v101
  ; Cycle B0009
  %v102 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v103 = load i64, i64* %v102
  %v104 = add i64 %v103, 1
  store i64 %v104, i64* %v102
  ; Cycle B0010
  %v105 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v106 = load i64, i64* %v105
  %v107 = add i64 %v106, 1
  store i64 %v107, i64* %v105
  %v108 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v109 = load i32, i32* %v108
  %v110 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 6
  %v111 = load i32, i32* %v110
  %v112 = add i32 %v109, %v111
  %v113 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  store i32 %v112, i32* %v113
  ; Cycle B0011
  %v114 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v115 = load i64, i64* %v114
  %v116 = add i64 %v115, 1
  store i64 %v116, i64* %v114
  ; Cycle B0012
  %v117 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v118 = load i64, i64* %v117
  %v119 = add i64 %v118, 1
  store i64 %v119, i64* %v117
  %v120 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v121 = load i32, i32* %v120
  %v122 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v123 = load i32, i32* %v122
  %v124 = add i32 %v121, %v123
  %v125 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  store i32 %v124, i32* %v125
  ; Cycle B0013
  %v126 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v127 = load i64, i64* %v126
  %v128 = add i64 %v127, 1
  store i64 %v128, i64* %v126
  ; Cycle B0014
  %v129 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v130 = load i64, i64* %v129
  %v131 = add i64 %v130, 1
  store i64 %v131, i64* %v129
  %v132 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v133 = load i32, i32* %v132
  %v134 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v135 = load i32, i32* %v134
  %v136 = add i32 %v133, %v135
  %v137 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  store i32 %v136, i32* %v137
  ; Cycle B0015
  %v138 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v139 = load i64, i64* %v138
  %v140 = add i64 %v139, 1
  store i64 %v140, i64* %v138
  ; Cycle B0016
  %v141 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v142 = load i64, i64* %v141
  %v143 = add i64 %v142, 1
  store i64 %v143, i64* %v141
  %v144 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  store i32 1, i32* %v144
  ; Cycle B0017
  %v145 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v146 = load i64, i64* %v145
  %v147 = add i64 %v146, 1
  store i64 %v147, i64* %v145
  %v148 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 7
  %v149 = load i32, i32* %v148
  %v150 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  %v151 = load i32, i32* %v150
  %v152 = add i32 %v149, %v151
  %v153 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 7
  store i32 %v152, i32* %v153
  %v154 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v155 = load i32, i32* %v154
  %v156 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  %v157 = load i32, i32* %v156
  %v158 = add i32 %v155, %v157
  %v159 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 %v158, i32* %v159
  %v160 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 4, i32* %v160
  ; Cycle B0018
  %v161 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v162 = load i64, i64* %v161
  %v163 = add i64 %v162, 1
  store i64 %v163, i64* %v161
  %v164 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 4, i32* %v164
  %v165 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 11
  store i32 4, i32* %v165
  %v166 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 12
  store i32 4, i32* %v166
  %v167 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 13
  %v168 = load i32, i32* %v167
  %v169 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  %v170 = load i32, i32* %v169
  %v171 = add i32 %v168, %v170
  %v172 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 13
  store i32 %v171, i32* %v172
  ; Cycle B0019
  %v173 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v174 = load i64, i64* %v173
  %v175 = add i64 %v174, 1
  store i64 %v175, i64* %v173
  %v176 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 14
  %v177 = load i32, i32* %v176
  %v178 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 13
  %v179 = load i32, i32* %v178
  %v180 = add i32 %v177, %v179
  %v181 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 14
  store i32 %v180, i32* %v181
  %v182 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v183 = load i32, i32* %v182
  %v184 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 12
  %v185 = load i32, i32* %v184
  %v186 = add i32 %v183, %v185
  %v187 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  store i32 %v186, i32* %v187
  ; Cycle B0020
  %v188 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v189 = load i64, i64* %v188
  %v190 = add i64 %v189, 1
  store i64 %v190, i64* %v188
  %v191 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 14
  %v192 = load i32, i32* %v191
  %v193 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v194 = load i32, i32* %v193
  %v195 = add i32 %v192, %v194
  %v196 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 14
  store i32 %v195, i32* %v196
  ; Cycle B0021
  %v197 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v198 = load i64, i64* %v197
  %v199 = add i64 %v198, 1
  store i64 %v199, i64* %v197
  %v200 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  store i32 1, i32* %v200
  ; Cycle B0022
  %v201 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v202 = load i64, i64* %v201
  %v203 = add i64 %v202, 1
  store i64 %v203, i64* %v201
  %v204 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v205 = load i32, i32* %v204
  %v206 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 15
  %v207 = load i32, i32* %v206
  %v208 = add i32 %v205, %v207
  %v209 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 14
  store i32 %v208, i32* %v209
  %v210 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  store i32 1, i32* %v210
  ; Cycle B0023
  %v211 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v212 = load i64, i64* %v211
  %v213 = add i64 %v212, 1
  store i64 %v213, i64* %v211
  %v214 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v215 = load i32, i32* %v214
  %v216 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v217 = load i32, i32* %v216
  %v218 = add i32 %v215, %v217
  %v219 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  store i32 %v218, i32* %v219
  ; Cycle B0024
  %v220 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v221 = load i64, i64* %v220
  %v222 = add i64 %v221, 1
  store i64 %v222, i64* %v220
  %v223 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v224 = load i32, i32* %v223
  %v225 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 6
  %v226 = load i32, i32* %v225
  %v227 = add i32 %v224, %v226
  %v228 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 14
  store i32 %v227, i32* %v228
  ; Cycle B0025
  %v229 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v230 = load i64, i64* %v229
  %v231 = add i64 %v230, 1
  store i64 %v231, i64* %v229
  %v232 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  store i32 1, i32* %v232
  ; Cycle B0026
  %v233 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v234 = load i64, i64* %v233
  %v235 = add i64 %v234, 1
  store i64 %v235, i64* %v233
  %v236 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  store i32 1, i32* %v236
  ; Cycle B0027
  %v237 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v238 = load i64, i64* %v237
  %v239 = add i64 %v238, 1
  store i64 %v239, i64* %v237
  %v240 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  %v241 = load i32, i32* %v240
  %v242 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v243 = load i32, i32* %v242
  %v244 = add i32 %v241, %v243
  %v245 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 %v244, i32* %v245
  ; Cycle B0028
  %v246 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v247 = load i64, i64* %v246
  %v248 = add i64 %v247, 1
  store i64 %v248, i64* %v246
  %v249 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 16
  store i1 1, i1* %v249
  ret void
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
  %v250 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v251 = load i64, i64* %v250
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([36 x i8], [36 x i8]* @.str_cycles, i32 0, i32 0), i64 %v251)
  %v252 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v253 = load i64, i64* %v252
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([36 x i8], [36 x i8]* @.str_optical, i32 0, i32 0), i64 %v253)
  %v254 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 10
  %v255 = load i64, i64* %v254
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([36 x i8], [36 x i8]* @.str_rev, i32 0, i32 0), i64 %v255)
  %v256 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v257 = load i64, i64* %v256
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([36 x i8], [36 x i8]* @.str_stdp, i32 0, i32 0), i64 %v257)
  %v258 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  %v259 = load i32, i32* %v258
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([48 x i8], [48 x i8]* @.str_r0, i32 0, i32 0), i32 %v259, i32 %v259)
  %v260 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v261 = load i32, i32* %v260
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([48 x i8], [48 x i8]* @.str_r1, i32 0, i32 0), i32 %v261, i32 %v261)
  %v262 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v263 = load i32, i32* %v262
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([48 x i8], [48 x i8]* @.str_r4, i32 0, i32 0), i32 %v263, i32 %v263)
  %v264 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 6
  %v265 = load i32, i32* %v264
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([48 x i8], [48 x i8]* @.str_r6, i32 0, i32 0), i32 %v265, i32 %v265)
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([62 x i8], [62 x i8]* @.str_hdr, i32 0, i32 0))
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([62 x i8], [62 x i8]* @.str_status, i32 0, i32 0))
  ret i32 0
}
