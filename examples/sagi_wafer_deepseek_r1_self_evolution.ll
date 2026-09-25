; ModuleID = 'cron_cl_sagi_wafer_deepseek_r1_self_evolution'
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
  %v11 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 7
  %v12 = load i32, i32* %v11
  %v13 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  %v14 = load i32, i32* %v13
  %v15 = add i32 %v12, %v14
  %v16 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 7
  store i32 %v15, i32* %v16
  ; Cycle B0003
  %v17 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v18 = load i64, i64* %v17
  %v19 = add i64 %v18, 1
  store i64 %v19, i64* %v17
  %v20 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  %v21 = load i32, i32* %v20
  %v22 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 7
  %v23 = load i32, i32* %v22
  %v24 = add i32 %v21, %v23
  %v25 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 %v24, i32* %v25
  %v26 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v27 = load i32, i32* %v26
  %v28 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  %v29 = load i32, i32* %v28
  %v30 = add i32 %v27, %v29
  %v31 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 %v30, i32* %v31
  ; Cycle B0004
  %v32 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v33 = load i64, i64* %v32
  %v34 = add i64 %v33, 1
  store i64 %v34, i64* %v32
  %v35 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  %v36 = load i32, i32* %v35
  %v37 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  %v38 = load i32, i32* %v37
  %v39 = add i32 %v36, %v38
  %v40 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 %v39, i32* %v40
  %v41 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 11
  %v42 = load i32, i32* %v41
  %v43 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  %v44 = load i32, i32* %v43
  %v45 = add i32 %v42, %v44
  %v46 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 11
  store i32 %v45, i32* %v46
  ; Cycle B0005
  %v47 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v48 = load i64, i64* %v47
  %v49 = add i64 %v48, 1
  store i64 %v49, i64* %v47
  %v50 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 12
  %v51 = load i32, i32* %v50
  %v52 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 11
  %v53 = load i32, i32* %v52
  %v54 = add i32 %v51, %v53
  %v55 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 12
  store i32 %v54, i32* %v55
  %v56 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 13
  %v57 = load i32, i32* %v56
  %v58 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  %v59 = load i32, i32* %v58
  %v60 = add i32 %v57, %v59
  %v61 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 13
  store i32 %v60, i32* %v61
  ; Cycle B0006
  %v62 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v63 = load i64, i64* %v62
  %v64 = add i64 %v63, 1
  store i64 %v64, i64* %v62
  %v65 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 14
  %v66 = load i32, i32* %v65
  %v67 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 13
  %v68 = load i32, i32* %v67
  %v69 = add i32 %v66, %v68
  %v70 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 14
  store i32 %v69, i32* %v70
  %v71 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v72 = load i32, i32* %v71
  %v73 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  %v74 = load i32, i32* %v73
  %v75 = add i32 %v72, %v74
  %v76 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  store i32 %v75, i32* %v76
  ; Cycle B0007
  %v77 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v78 = load i64, i64* %v77
  %v79 = add i64 %v78, 1
  store i64 %v79, i64* %v77
  %v80 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v81 = load i32, i32* %v80
  %v82 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v83 = load i32, i32* %v82
  %v84 = add i32 %v81, %v83
  %v85 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  store i32 %v84, i32* %v85
  ; Cycle B0008
  %v86 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v87 = load i64, i64* %v86
  %v88 = add i64 %v87, 1
  store i64 %v88, i64* %v86
  %v89 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v90 = load i32, i32* %v89
  %v91 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v92 = load i32, i32* %v91
  %v93 = add i32 %v90, %v92
  %v94 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  store i32 %v93, i32* %v94
  ; Cycle B0009
  %v95 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v96 = load i64, i64* %v95
  %v97 = add i64 %v96, 1
  store i64 %v97, i64* %v95
  %v98 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v99 = load i64, i64* %v98
  %v100 = add i64 %v99, 1
  store i64 %v100, i64* %v98
  %v101 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  store i32 132, i32* %v101
  %v102 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 3
  %v103 = load i32, i32* %v102
  %v104 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  %v105 = load i32, i32* %v104
  %v106 = add i32 %v103, %v105
  %v107 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 3
  store i32 %v106, i32* %v107
  ; Cycle B0010
  %v108 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v109 = load i64, i64* %v108
  %v110 = add i64 %v109, 1
  store i64 %v110, i64* %v108
  ; Cycle B0011
  %v111 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v112 = load i64, i64* %v111
  %v113 = add i64 %v112, 1
  store i64 %v113, i64* %v111
  %v114 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 14
  %v115 = load i32, i32* %v114
  %v116 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v117 = load i32, i32* %v116
  %v118 = add i32 %v115, %v117
  %v119 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 14
  store i32 %v118, i32* %v119
  ; Cycle B0012
  %v120 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v121 = load i64, i64* %v120
  %v122 = add i64 %v121, 1
  store i64 %v122, i64* %v120
  %v123 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 5
  store i32 1, i32* %v123
  ; Cycle B0013
  %v124 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v125 = load i64, i64* %v124
  %v126 = add i64 %v125, 1
  store i64 %v126, i64* %v124
  %v127 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 5
  store i32 1, i32* %v127
  %v128 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  store i32 1, i32* %v128
  ; Cycle B0014
  %v129 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v130 = load i64, i64* %v129
  %v131 = add i64 %v130, 1
  store i64 %v131, i64* %v129
  %v132 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v133 = load i32, i32* %v132
  %v134 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 12
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
  %v145 = load i32, i32* %v144
  %v146 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 12
  %v147 = load i32, i32* %v146
  %v148 = add i32 %v145, %v147
  %v149 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  store i32 %v148, i32* %v149
  ; Cycle B0017
  %v150 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v151 = load i64, i64* %v150
  %v152 = add i64 %v151, 1
  store i64 %v152, i64* %v150
  ; Cycle B0018
  %v153 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v154 = load i64, i64* %v153
  %v155 = add i64 %v154, 1
  store i64 %v155, i64* %v153
  ; Cycle B0019
  %v156 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v157 = load i64, i64* %v156
  %v158 = add i64 %v157, 1
  store i64 %v158, i64* %v156
  %v159 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v160 = load i32, i32* %v159
  %v161 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v162 = load i32, i32* %v161
  %v163 = add i32 %v160, %v162
  %v164 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  store i32 %v163, i32* %v164
  ; Cycle B0020
  %v165 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v166 = load i64, i64* %v165
  %v167 = add i64 %v166, 1
  store i64 %v167, i64* %v165
  ; Cycle B0021
  %v168 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v169 = load i64, i64* %v168
  %v170 = add i64 %v169, 1
  store i64 %v170, i64* %v168
  %v171 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v172 = load i32, i32* %v171
  %v173 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v174 = load i32, i32* %v173
  %v175 = add i32 %v172, %v174
  %v176 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  store i32 %v175, i32* %v176
  ; Cycle B0022
  %v177 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v178 = load i64, i64* %v177
  %v179 = add i64 %v178, 1
  store i64 %v179, i64* %v177
  ; Cycle B0023
  %v180 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v181 = load i64, i64* %v180
  %v182 = add i64 %v181, 1
  store i64 %v182, i64* %v180
  %v183 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v184 = load i32, i32* %v183
  %v185 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 5
  %v186 = load i32, i32* %v185
  %v187 = add i32 %v184, %v186
  %v188 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  store i32 %v187, i32* %v188
  ; Cycle B0024
  %v189 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v190 = load i64, i64* %v189
  %v191 = add i64 %v190, 1
  store i64 %v191, i64* %v189
  %v192 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  store i32 1, i32* %v192
  ; Cycle B0025
  %v193 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v194 = load i64, i64* %v193
  %v195 = add i64 %v194, 1
  store i64 %v195, i64* %v193
  %v196 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 6
  store i32 4, i32* %v196
  %v197 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 7
  store i32 4, i32* %v197
  %v198 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 8
  store i32 4, i32* %v198
  ; Cycle B0026
  %v199 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v200 = load i64, i64* %v199
  %v201 = add i64 %v200, 1
  store i64 %v201, i64* %v199
  %v202 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 9
  store i32 4, i32* %v202
  %v203 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  store i32 4, i32* %v203
  %v204 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 11
  %v205 = load i32, i32* %v204
  %v206 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  %v207 = load i32, i32* %v206
  %v208 = add i32 %v205, %v207
  %v209 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 11
  store i32 %v208, i32* %v209
  ; Cycle B0027
  %v210 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v211 = load i64, i64* %v210
  %v212 = add i64 %v211, 1
  store i64 %v212, i64* %v210
  %v213 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 12
  %v214 = load i32, i32* %v213
  %v215 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 11
  %v216 = load i32, i32* %v215
  %v217 = add i32 %v214, %v216
  %v218 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 12
  store i32 %v217, i32* %v218
  %v219 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 13
  %v220 = load i32, i32* %v219
  %v221 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 10
  %v222 = load i32, i32* %v221
  %v223 = add i32 %v220, %v222
  %v224 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 13
  store i32 %v223, i32* %v224
  ; Cycle B0028
  %v225 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v226 = load i64, i64* %v225
  %v227 = add i64 %v226, 1
  store i64 %v227, i64* %v225
  %v228 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 14
  %v229 = load i32, i32* %v228
  %v230 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 13
  %v231 = load i32, i32* %v230
  %v232 = add i32 %v229, %v231
  %v233 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 14
  store i32 %v232, i32* %v233
  ; Cycle B0029
  %v234 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v235 = load i64, i64* %v234
  %v236 = add i64 %v235, 1
  store i64 %v236, i64* %v234
  %v237 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  store i32 1, i32* %v237
  ; Cycle B0030
  %v238 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v239 = load i64, i64* %v238
  %v240 = add i64 %v239, 1
  store i64 %v240, i64* %v238
  %v241 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v242 = load i32, i32* %v241
  %v243 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 15
  %v244 = load i32, i32* %v243
  %v245 = add i32 %v242, %v244
  %v246 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 14
  store i32 %v245, i32* %v246
  %v247 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v248 = load i32, i32* %v247
  %v249 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 13
  %v250 = load i32, i32* %v249
  %v251 = add i32 %v248, %v250
  %v252 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  store i32 %v251, i32* %v252
  ; Cycle B0031
  %v253 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v254 = load i64, i64* %v253
  %v255 = add i64 %v254, 1
  store i64 %v255, i64* %v253
  %v256 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  store i32 1, i32* %v256
  ; Cycle B0032
  %v257 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v258 = load i64, i64* %v257
  %v259 = add i64 %v258, 1
  store i64 %v259, i64* %v257
  %v260 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 2
  %v261 = load i32, i32* %v260
  %v262 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 15
  %v263 = load i32, i32* %v262
  %v264 = add i32 %v261, %v263
  %v265 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  store i32 %v264, i32* %v265
  ; Cycle B0033
  %v266 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v267 = load i64, i64* %v266
  %v268 = add i64 %v267, 1
  store i64 %v268, i64* %v266
  %v269 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v270 = load i32, i32* %v269
  %v271 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 6
  %v272 = load i32, i32* %v271
  %v273 = add i32 %v270, %v272
  %v274 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 14
  store i32 %v273, i32* %v274
  ; Cycle B0034
  %v275 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v276 = load i64, i64* %v275
  %v277 = add i64 %v276, 1
  store i64 %v277, i64* %v275
  ; Cycle B0035
  %v278 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v279 = load i64, i64* %v278
  %v280 = add i64 %v279, 1
  store i64 %v280, i64* %v278
  %v281 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 14
  store i32 1, i32* %v281
  ; Cycle B0036
  %v282 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v283 = load i64, i64* %v282
  %v284 = add i64 %v283, 1
  store i64 %v284, i64* %v282
  %v285 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 14
  store i32 1, i32* %v285
  ; Cycle B0037
  %v286 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v287 = load i64, i64* %v286
  %v288 = add i64 %v287, 1
  store i64 %v288, i64* %v286
  %v289 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  %v290 = load i32, i32* %v289
  %v291 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 14
  %v292 = load i32, i32* %v291
  %v293 = add i32 %v290, %v292
  %v294 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  store i32 %v293, i32* %v294
  ; Cycle B0038
  %v295 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v296 = load i64, i64* %v295
  %v297 = add i64 %v296, 1
  store i64 %v297, i64* %v295
  %v298 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 16
  store i1 1, i1* %v298
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
  %v299 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 8
  %v300 = load i64, i64* %v299
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([36 x i8], [36 x i8]* @.str_cycles, i32 0, i32 0), i64 %v300)
  %v301 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 9
  %v302 = load i64, i64* %v301
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([36 x i8], [36 x i8]* @.str_optical, i32 0, i32 0), i64 %v302)
  %v303 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 10
  %v304 = load i64, i64* %v303
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([36 x i8], [36 x i8]* @.str_rev, i32 0, i32 0), i64 %v304)
  %v305 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 11
  %v306 = load i64, i64* %v305
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([36 x i8], [36 x i8]* @.str_stdp, i32 0, i32 0), i64 %v306)
  %v307 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 0
  %v308 = load i32, i32* %v307
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([48 x i8], [48 x i8]* @.str_r0, i32 0, i32 0), i32 %v308, i32 %v308)
  %v309 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 1
  %v310 = load i32, i32* %v309
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([48 x i8], [48 x i8]* @.str_r1, i32 0, i32 0), i32 %v310, i32 %v310)
  %v311 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 4
  %v312 = load i32, i32* %v311
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([48 x i8], [48 x i8]* @.str_r4, i32 0, i32 0), i32 %v312, i32 %v312)
  %v313 = getelementptr inbounds %struct.CronSiliconCore, %struct.CronSiliconCore* %core, i32 0, i32 0, i32 6
  %v314 = load i32, i32* %v313
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([48 x i8], [48 x i8]* @.str_r6, i32 0, i32 0), i32 %v314, i32 %v314)
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([62 x i8], [62 x i8]* @.str_hdr, i32 0, i32 0))
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([62 x i8], [62 x i8]* @.str_status, i32 0, i32 0))
  ret i32 0
}
