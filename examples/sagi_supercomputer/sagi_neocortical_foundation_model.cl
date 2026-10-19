; ============================================================================
; SAGI NEOCORTICAL FOUNDATION MODEL (Full 256-Core 4D-Torus Distributed AGI)
; Architecture: 6-Brain Multimodal Neuromorphic Neocortex
; Target: 256-Core 4D-Torus Neuromorphic Photonic Silicon (<20W Power Envelope)
; ============================================================================

.stage id="sagi_neocortex_7b" params="7B" precision="ternary_1.58b" d_model=4096 heads=32 kv_heads=8 intermediate=11008 zero_overhead=true
.layout { TP=4, EP=4, CP=4, PP=4, dim="4x4x4x4", chip="256_core_torus" }
.flow route="Neocortical-4D-Torus" dor="XYZW"
.clifford rotor=Rotor4D, vector=Vector4D, algebra="Cl(4,0)"

; --- 256 Neocortical Micro-Circuit Directives (Cores 0..255) ---
.circuit core=0 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=1 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=2 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=3 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=4 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=5 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=6 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=7 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=8 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=9 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=10 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=11 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=12 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=13 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=14 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=15 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=16 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=17 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=18 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=19 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=20 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=21 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=22 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=23 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=24 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=25 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=26 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=27 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=28 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=29 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=30 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=31 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=32 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=33 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=34 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=35 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=36 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=37 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=38 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=39 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=40 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=41 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=42 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=43 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=44 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=45 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=46 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=47 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=48 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=49 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=50 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=51 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=52 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=53 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=54 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=55 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=56 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=57 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=58 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=59 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=60 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=61 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=62 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=63 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=64 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=65 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=66 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=67 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=68 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=69 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=70 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=71 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=72 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=73 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=74 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=75 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=76 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=77 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=78 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=79 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=80 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=81 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=82 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=83 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=84 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=85 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=86 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=87 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=88 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=89 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=90 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=91 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=92 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=93 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=94 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=95 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=96 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=97 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=98 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=99 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=100 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=101 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=102 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=103 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=104 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=105 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=106 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=107 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=108 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=109 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=110 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=111 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=112 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=113 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=114 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=115 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=116 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=117 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=118 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=119 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=120 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=121 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=122 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=123 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=124 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=125 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=126 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=127 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=128 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=129 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=130 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=131 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=132 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=133 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=134 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=135 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=136 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=137 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=138 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=139 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=140 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=141 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=142 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=143 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=144 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=145 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=146 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=147 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=148 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=149 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=150 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=151 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=152 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=153 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=154 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=155 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=156 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=157 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=158 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=159 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=160 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=161 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=162 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=163 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=164 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=165 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=166 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=167 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=168 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=169 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=170 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=171 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=172 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=173 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=174 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=175 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=176 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=177 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=178 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=179 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=180 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=181 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=182 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=183 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=184 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=185 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=186 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=187 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=188 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=189 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=190 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=191 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=192 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=193 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=194 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=195 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=196 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=197 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=198 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=199 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=200 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=201 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=202 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=203 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=204 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=205 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=206 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=207 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=208 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=209 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=210 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=211 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=212 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=213 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=214 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=215 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=216 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=217 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=218 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=219 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=220 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=221 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=222 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=223 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=224 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=225 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=226 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=227 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=228 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=229 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=230 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=231 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=232 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=233 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=234 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=235 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=236 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=237 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=238 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=239 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=240 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=241 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=242 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=243 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=244 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=245 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=246 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=247 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=248 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=249 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=250 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=251 layer=L6 exc=75 inh=25 tau=30000 da=0.50
.circuit core=252 layer=L4 exc=90 inh=10 tau=10000 da=0.60
.circuit core=253 layer=L23 exc=80 inh=20 tau=15000 da=0.75
.circuit core=254 layer=L5 exc=85 inh=15 tau=25000 da=0.80
.circuit core=255 layer=L6 exc=75 inh=25 tau=30000 da=0.50

; --- Core [0,0,0,0] (ID 0): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=0] ---
.core [0,0,0,0]:
@core_0_entry:
B0000: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,0,0,0] (ID 1): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=0] ---
.core [1,0,0,0]:
@core_1_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,0,0,0] (ID 2): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=0] ---
.core [2,0,0,0]:
@core_2_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,0,0,0] (ID 3): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=0] ---
.core [3,0,0,0]:
@core_3_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,1,0,0] (ID 4): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=0] ---
.core [0,1,0,0]:
@core_4_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,1,0,0] (ID 5): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=0] ---
.core [1,1,0,0]:
@core_5_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,1,0,0] (ID 6): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=0] ---
.core [2,1,0,0]:
@core_6_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,1,0,0] (ID 7): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=0] ---
.core [3,1,0,0]:
@core_7_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,2,0,0] (ID 8): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=0] ---
.core [0,2,0,0]:
@core_8_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,2,0,0] (ID 9): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=0] ---
.core [1,2,0,0]:
@core_9_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,2,0,0] (ID 10): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=0] ---
.core [2,2,0,0]:
@core_10_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,2,0,0] (ID 11): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=0] ---
.core [3,2,0,0]:
@core_11_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,3,0,0] (ID 12): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=0] ---
.core [0,3,0,0]:
@core_12_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,3,0,0] (ID 13): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=0] ---
.core [1,3,0,0]:
@core_13_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,3,0,0] (ID 14): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=0] ---
.core [2,3,0,0]:
@core_14_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,3,0,0] (ID 15): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=0] ---
.core [3,3,0,0]:
@core_15_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,0,1,0] (ID 16): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=0] ---
.core [0,0,1,0]:
@core_16_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,0,1,0] (ID 17): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=0] ---
.core [1,0,1,0]:
@core_17_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,0,1,0] (ID 18): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=0] ---
.core [2,0,1,0]:
@core_18_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,0,1,0] (ID 19): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=0] ---
.core [3,0,1,0]:
@core_19_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,1,1,0] (ID 20): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=0] ---
.core [0,1,1,0]:
@core_20_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,1,1,0] (ID 21): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=0] ---
.core [1,1,1,0]:
@core_21_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,1,1,0] (ID 22): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=0] ---
.core [2,1,1,0]:
@core_22_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,1,1,0] (ID 23): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=0] ---
.core [3,1,1,0]:
@core_23_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,2,1,0] (ID 24): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=0] ---
.core [0,2,1,0]:
@core_24_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,2,1,0] (ID 25): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=0] ---
.core [1,2,1,0]:
@core_25_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,2,1,0] (ID 26): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=0] ---
.core [2,2,1,0]:
@core_26_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,2,1,0] (ID 27): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=0] ---
.core [3,2,1,0]:
@core_27_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,3,1,0] (ID 28): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=0] ---
.core [0,3,1,0]:
@core_28_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,3,1,0] (ID 29): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=0] ---
.core [1,3,1,0]:
@core_29_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,3,1,0] (ID 30): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=0] ---
.core [2,3,1,0]:
@core_30_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,3,1,0] (ID 31): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=0] ---
.core [3,3,1,0]:
@core_31_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,0,2,0] (ID 32): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=0] ---
.core [0,0,2,0]:
@core_32_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,0,2,0] (ID 33): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=0] ---
.core [1,0,2,0]:
@core_33_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,0,2,0] (ID 34): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=0] ---
.core [2,0,2,0]:
@core_34_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,0,2,0] (ID 35): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=0] ---
.core [3,0,2,0]:
@core_35_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,1,2,0] (ID 36): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=0] ---
.core [0,1,2,0]:
@core_36_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,1,2,0] (ID 37): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=0] ---
.core [1,1,2,0]:
@core_37_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,1,2,0] (ID 38): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=0] ---
.core [2,1,2,0]:
@core_38_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,1,2,0] (ID 39): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=0] ---
.core [3,1,2,0]:
@core_39_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,2,2,0] (ID 40): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=0] ---
.core [0,2,2,0]:
@core_40_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,2,2,0] (ID 41): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=0] ---
.core [1,2,2,0]:
@core_41_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,2,2,0] (ID 42): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=0] ---
.core [2,2,2,0]:
@core_42_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,2,2,0] (ID 43): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=0] ---
.core [3,2,2,0]:
@core_43_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,3,2,0] (ID 44): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=0] ---
.core [0,3,2,0]:
@core_44_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,3,2,0] (ID 45): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=0] ---
.core [1,3,2,0]:
@core_45_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,3,2,0] (ID 46): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=0] ---
.core [2,3,2,0]:
@core_46_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,3,2,0] (ID 47): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=0] ---
.core [3,3,2,0]:
@core_47_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,0,3,0] (ID 48): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=0] ---
.core [0,0,3,0]:
@core_48_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,0,3,0] (ID 49): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=0] ---
.core [1,0,3,0]:
@core_49_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,0,3,0] (ID 50): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=0] ---
.core [2,0,3,0]:
@core_50_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,0,3,0] (ID 51): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=0] ---
.core [3,0,3,0]:
@core_51_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,1,3,0] (ID 52): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=0] ---
.core [0,1,3,0]:
@core_52_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,1,3,0] (ID 53): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=0] ---
.core [1,1,3,0]:
@core_53_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,1,3,0] (ID 54): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=0] ---
.core [2,1,3,0]:
@core_54_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,1,3,0] (ID 55): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=0] ---
.core [3,1,3,0]:
@core_55_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,2,3,0] (ID 56): Brain 5 CORDIC RoPE & MoE Dynamic Router [Z=3,W=0] ---
.core [0,2,3,0]:
@core_56_entry:
B0000: _RX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0001: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0002: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0003: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0004: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0005: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0006: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0007: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0008: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0009: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0010: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0011: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0012: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0013: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0014: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0015: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0016: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0017: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0018: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0019: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0020: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0021: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0022: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0023: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0024: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0025: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0026: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0027: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0028: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0029: _TX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,2,3,0] (ID 57): Brain 5 CORDIC RoPE & MoE Dynamic Router [Z=3,W=0] ---
.core [1,2,3,0]:
@core_57_entry:
B0000: _RX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0001: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0002: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0003: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0004: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0005: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0006: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0007: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0008: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0009: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0010: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0011: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0012: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0013: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0014: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0015: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0016: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0017: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0018: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0019: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0020: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0021: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0022: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0023: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0024: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0025: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0026: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0027: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0028: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0029: _TX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,2,3,0] (ID 58): Brain 5 CORDIC RoPE & MoE Dynamic Router [Z=3,W=0] ---
.core [2,2,3,0]:
@core_58_entry:
B0000: _RX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0001: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0002: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0003: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0004: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0005: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0006: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0007: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0008: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0009: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0010: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0011: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0012: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0013: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0014: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0015: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0016: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0017: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0018: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0019: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0020: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0021: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0022: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0023: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0024: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0025: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0026: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0027: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0028: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0029: _TX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,2,3,0] (ID 59): Brain 5 CORDIC RoPE & MoE Dynamic Router [Z=3,W=0] ---
.core [3,2,3,0]:
@core_59_entry:
B0000: _RX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0001: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0002: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0003: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0004: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0005: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0006: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0007: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0008: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0009: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0010: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0011: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0012: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0013: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0014: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0015: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0016: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0017: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0018: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0019: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0020: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0021: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0022: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0023: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0024: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0025: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0026: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0027: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0028: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0029: _TX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,3,3,0] (ID 60): Brain 6 Metacognitive Sentry & Self-Rewrite [Z=3,W=0] ---
.core [0,3,3,0]:
@core_60_entry:
B0000: _RX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0001: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0002: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0003: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0004: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0005: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0006: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0007: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0008: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0009: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0010: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0011: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0012: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0013: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0014: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0015: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0016: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0017: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0018: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0019: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0020: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0021: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0022: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0023: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0024: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0025: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0026: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0027: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0028: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0029: _TX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,3,3,0] (ID 61): Brain 6 Metacognitive Sentry & Self-Rewrite [Z=3,W=0] ---
.core [1,3,3,0]:
@core_61_entry:
B0000: _RX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0001: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0002: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0003: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0004: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0005: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0006: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0007: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0008: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0009: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0010: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0011: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0012: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0013: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0014: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0015: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0016: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0017: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0018: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0019: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0020: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0021: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0022: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0023: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0024: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0025: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0026: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0027: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0028: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0029: _TX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,3,3,0] (ID 62): Brain 6 Metacognitive Sentry & Self-Rewrite [Z=3,W=0] ---
.core [2,3,3,0]:
@core_62_entry:
B0000: _RX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0001: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0002: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0003: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0004: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0005: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0006: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0007: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0008: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0009: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0010: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0011: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0012: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0013: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0014: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0015: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0016: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0017: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0018: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0019: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0020: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0021: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0022: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0023: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0024: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0025: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0026: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0027: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0028: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0029: _TX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,3,3,0] (ID 63): Brain 6 Metacognitive Sentry & Self-Rewrite [Z=3,W=0] ---
.core [3,3,3,0]:
@core_63_entry:
B0000: _RX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0001: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0002: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0003: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0004: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0005: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0006: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0007: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0008: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0009: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0010: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0011: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0012: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0013: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0014: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0015: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0016: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0017: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0018: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0019: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0020: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0021: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0022: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0023: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0024: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0025: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0026: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0027: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0028: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0029: _TX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,0,0,1] (ID 64): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=1] ---
.core [0,0,0,1]:
@core_64_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,0,0,1] (ID 65): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=1] ---
.core [1,0,0,1]:
@core_65_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,0,0,1] (ID 66): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=1] ---
.core [2,0,0,1]:
@core_66_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,0,0,1] (ID 67): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=1] ---
.core [3,0,0,1]:
@core_67_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,1,0,1] (ID 68): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=1] ---
.core [0,1,0,1]:
@core_68_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,1,0,1] (ID 69): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=1] ---
.core [1,1,0,1]:
@core_69_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,1,0,1] (ID 70): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=1] ---
.core [2,1,0,1]:
@core_70_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,1,0,1] (ID 71): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=1] ---
.core [3,1,0,1]:
@core_71_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,2,0,1] (ID 72): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=1] ---
.core [0,2,0,1]:
@core_72_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,2,0,1] (ID 73): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=1] ---
.core [1,2,0,1]:
@core_73_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,2,0,1] (ID 74): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=1] ---
.core [2,2,0,1]:
@core_74_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,2,0,1] (ID 75): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=1] ---
.core [3,2,0,1]:
@core_75_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,3,0,1] (ID 76): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=1] ---
.core [0,3,0,1]:
@core_76_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,3,0,1] (ID 77): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=1] ---
.core [1,3,0,1]:
@core_77_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,3,0,1] (ID 78): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=1] ---
.core [2,3,0,1]:
@core_78_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,3,0,1] (ID 79): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=1] ---
.core [3,3,0,1]:
@core_79_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,0,1,1] (ID 80): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=1] ---
.core [0,0,1,1]:
@core_80_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,0,1,1] (ID 81): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=1] ---
.core [1,0,1,1]:
@core_81_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,0,1,1] (ID 82): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=1] ---
.core [2,0,1,1]:
@core_82_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,0,1,1] (ID 83): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=1] ---
.core [3,0,1,1]:
@core_83_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,1,1,1] (ID 84): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=1] ---
.core [0,1,1,1]:
@core_84_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,1,1,1] (ID 85): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=1] ---
.core [1,1,1,1]:
@core_85_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,1,1,1] (ID 86): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=1] ---
.core [2,1,1,1]:
@core_86_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,1,1,1] (ID 87): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=1] ---
.core [3,1,1,1]:
@core_87_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,2,1,1] (ID 88): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=1] ---
.core [0,2,1,1]:
@core_88_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,2,1,1] (ID 89): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=1] ---
.core [1,2,1,1]:
@core_89_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,2,1,1] (ID 90): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=1] ---
.core [2,2,1,1]:
@core_90_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,2,1,1] (ID 91): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=1] ---
.core [3,2,1,1]:
@core_91_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,3,1,1] (ID 92): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=1] ---
.core [0,3,1,1]:
@core_92_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,3,1,1] (ID 93): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=1] ---
.core [1,3,1,1]:
@core_93_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,3,1,1] (ID 94): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=1] ---
.core [2,3,1,1]:
@core_94_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,3,1,1] (ID 95): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=1] ---
.core [3,3,1,1]:
@core_95_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,0,2,1] (ID 96): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=1] ---
.core [0,0,2,1]:
@core_96_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,0,2,1] (ID 97): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=1] ---
.core [1,0,2,1]:
@core_97_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,0,2,1] (ID 98): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=1] ---
.core [2,0,2,1]:
@core_98_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,0,2,1] (ID 99): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=1] ---
.core [3,0,2,1]:
@core_99_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,1,2,1] (ID 100): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=1] ---
.core [0,1,2,1]:
@core_100_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,1,2,1] (ID 101): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=1] ---
.core [1,1,2,1]:
@core_101_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,1,2,1] (ID 102): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=1] ---
.core [2,1,2,1]:
@core_102_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,1,2,1] (ID 103): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=1] ---
.core [3,1,2,1]:
@core_103_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,2,2,1] (ID 104): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=1] ---
.core [0,2,2,1]:
@core_104_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,2,2,1] (ID 105): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=1] ---
.core [1,2,2,1]:
@core_105_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,2,2,1] (ID 106): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=1] ---
.core [2,2,2,1]:
@core_106_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,2,2,1] (ID 107): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=1] ---
.core [3,2,2,1]:
@core_107_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,3,2,1] (ID 108): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=1] ---
.core [0,3,2,1]:
@core_108_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,3,2,1] (ID 109): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=1] ---
.core [1,3,2,1]:
@core_109_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,3,2,1] (ID 110): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=1] ---
.core [2,3,2,1]:
@core_110_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,3,2,1] (ID 111): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=1] ---
.core [3,3,2,1]:
@core_111_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,0,3,1] (ID 112): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=1] ---
.core [0,0,3,1]:
@core_112_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,0,3,1] (ID 113): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=1] ---
.core [1,0,3,1]:
@core_113_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,0,3,1] (ID 114): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=1] ---
.core [2,0,3,1]:
@core_114_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,0,3,1] (ID 115): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=1] ---
.core [3,0,3,1]:
@core_115_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,1,3,1] (ID 116): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=1] ---
.core [0,1,3,1]:
@core_116_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,1,3,1] (ID 117): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=1] ---
.core [1,1,3,1]:
@core_117_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,1,3,1] (ID 118): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=1] ---
.core [2,1,3,1]:
@core_118_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,1,3,1] (ID 119): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=1] ---
.core [3,1,3,1]:
@core_119_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,2,3,1] (ID 120): Brain 5 CORDIC RoPE & MoE Dynamic Router [Z=3,W=1] ---
.core [0,2,3,1]:
@core_120_entry:
B0000: _RX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0001: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0002: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0003: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0004: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0005: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0006: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0007: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0008: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0009: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0010: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0011: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0012: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0013: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0014: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0015: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0016: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0017: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0018: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0019: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0020: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0021: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0022: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0023: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0024: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0025: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0026: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0027: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0028: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0029: _TX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,2,3,1] (ID 121): Brain 5 CORDIC RoPE & MoE Dynamic Router [Z=3,W=1] ---
.core [1,2,3,1]:
@core_121_entry:
B0000: _RX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0001: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0002: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0003: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0004: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0005: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0006: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0007: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0008: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0009: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0010: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0011: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0012: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0013: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0014: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0015: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0016: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0017: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0018: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0019: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0020: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0021: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0022: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0023: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0024: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0025: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0026: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0027: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0028: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0029: _TX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,2,3,1] (ID 122): Brain 5 CORDIC RoPE & MoE Dynamic Router [Z=3,W=1] ---
.core [2,2,3,1]:
@core_122_entry:
B0000: _RX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0001: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0002: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0003: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0004: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0005: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0006: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0007: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0008: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0009: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0010: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0011: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0012: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0013: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0014: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0015: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0016: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0017: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0018: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0019: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0020: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0021: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0022: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0023: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0024: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0025: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0026: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0027: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0028: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0029: _TX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,2,3,1] (ID 123): Brain 5 CORDIC RoPE & MoE Dynamic Router [Z=3,W=1] ---
.core [3,2,3,1]:
@core_123_entry:
B0000: _RX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0001: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0002: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0003: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0004: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0005: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0006: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0007: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0008: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0009: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0010: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0011: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0012: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0013: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0014: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0015: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0016: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0017: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0018: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0019: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0020: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0021: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0022: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0023: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0024: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0025: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0026: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0027: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0028: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0029: _TX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,3,3,1] (ID 124): Brain 6 Metacognitive Sentry & Self-Rewrite [Z=3,W=1] ---
.core [0,3,3,1]:
@core_124_entry:
B0000: _RX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0001: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0002: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0003: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0004: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0005: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0006: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0007: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0008: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0009: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0010: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0011: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0012: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0013: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0014: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0015: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0016: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0017: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0018: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0019: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0020: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0021: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0022: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0023: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0024: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0025: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0026: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0027: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0028: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0029: _TX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,3,3,1] (ID 125): Brain 6 Metacognitive Sentry & Self-Rewrite [Z=3,W=1] ---
.core [1,3,3,1]:
@core_125_entry:
B0000: _RX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0001: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0002: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0003: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0004: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0005: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0006: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0007: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0008: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0009: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0010: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0011: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0012: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0013: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0014: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0015: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0016: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0017: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0018: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0019: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0020: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0021: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0022: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0023: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0024: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0025: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0026: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0027: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0028: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0029: _TX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,3,3,1] (ID 126): Brain 6 Metacognitive Sentry & Self-Rewrite [Z=3,W=1] ---
.core [2,3,3,1]:
@core_126_entry:
B0000: _RX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0001: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0002: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0003: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0004: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0005: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0006: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0007: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0008: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0009: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0010: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0011: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0012: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0013: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0014: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0015: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0016: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0017: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0018: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0019: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0020: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0021: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0022: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0023: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0024: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0025: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0026: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0027: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0028: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0029: _TX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,3,3,1] (ID 127): Brain 6 Metacognitive Sentry & Self-Rewrite [Z=3,W=1] ---
.core [3,3,3,1]:
@core_127_entry:
B0000: _RX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0001: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0002: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0003: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0004: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0005: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0006: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0007: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0008: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0009: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0010: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0011: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0012: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0013: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0014: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0015: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0016: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0017: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0018: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0019: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0020: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0021: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0022: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0023: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0024: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0025: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0026: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0027: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0028: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0029: _TX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,0,0,2] (ID 128): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=2] ---
.core [0,0,0,2]:
@core_128_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,0,0,2] (ID 129): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=2] ---
.core [1,0,0,2]:
@core_129_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,0,0,2] (ID 130): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=2] ---
.core [2,0,0,2]:
@core_130_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,0,0,2] (ID 131): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=2] ---
.core [3,0,0,2]:
@core_131_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,1,0,2] (ID 132): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=2] ---
.core [0,1,0,2]:
@core_132_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,1,0,2] (ID 133): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=2] ---
.core [1,1,0,2]:
@core_133_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,1,0,2] (ID 134): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=2] ---
.core [2,1,0,2]:
@core_134_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,1,0,2] (ID 135): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=2] ---
.core [3,1,0,2]:
@core_135_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,2,0,2] (ID 136): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=2] ---
.core [0,2,0,2]:
@core_136_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,2,0,2] (ID 137): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=2] ---
.core [1,2,0,2]:
@core_137_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,2,0,2] (ID 138): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=2] ---
.core [2,2,0,2]:
@core_138_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,2,0,2] (ID 139): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=2] ---
.core [3,2,0,2]:
@core_139_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,3,0,2] (ID 140): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=2] ---
.core [0,3,0,2]:
@core_140_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,3,0,2] (ID 141): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=2] ---
.core [1,3,0,2]:
@core_141_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,3,0,2] (ID 142): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=2] ---
.core [2,3,0,2]:
@core_142_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,3,0,2] (ID 143): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=2] ---
.core [3,3,0,2]:
@core_143_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,0,1,2] (ID 144): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=2] ---
.core [0,0,1,2]:
@core_144_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,0,1,2] (ID 145): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=2] ---
.core [1,0,1,2]:
@core_145_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,0,1,2] (ID 146): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=2] ---
.core [2,0,1,2]:
@core_146_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,0,1,2] (ID 147): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=2] ---
.core [3,0,1,2]:
@core_147_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,1,1,2] (ID 148): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=2] ---
.core [0,1,1,2]:
@core_148_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,1,1,2] (ID 149): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=2] ---
.core [1,1,1,2]:
@core_149_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,1,1,2] (ID 150): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=2] ---
.core [2,1,1,2]:
@core_150_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,1,1,2] (ID 151): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=2] ---
.core [3,1,1,2]:
@core_151_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,2,1,2] (ID 152): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=2] ---
.core [0,2,1,2]:
@core_152_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,2,1,2] (ID 153): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=2] ---
.core [1,2,1,2]:
@core_153_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,2,1,2] (ID 154): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=2] ---
.core [2,2,1,2]:
@core_154_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,2,1,2] (ID 155): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=2] ---
.core [3,2,1,2]:
@core_155_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,3,1,2] (ID 156): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=2] ---
.core [0,3,1,2]:
@core_156_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,3,1,2] (ID 157): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=2] ---
.core [1,3,1,2]:
@core_157_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,3,1,2] (ID 158): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=2] ---
.core [2,3,1,2]:
@core_158_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,3,1,2] (ID 159): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=2] ---
.core [3,3,1,2]:
@core_159_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,0,2,2] (ID 160): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=2] ---
.core [0,0,2,2]:
@core_160_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,0,2,2] (ID 161): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=2] ---
.core [1,0,2,2]:
@core_161_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,0,2,2] (ID 162): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=2] ---
.core [2,0,2,2]:
@core_162_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,0,2,2] (ID 163): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=2] ---
.core [3,0,2,2]:
@core_163_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,1,2,2] (ID 164): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=2] ---
.core [0,1,2,2]:
@core_164_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,1,2,2] (ID 165): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=2] ---
.core [1,1,2,2]:
@core_165_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,1,2,2] (ID 166): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=2] ---
.core [2,1,2,2]:
@core_166_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,1,2,2] (ID 167): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=2] ---
.core [3,1,2,2]:
@core_167_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,2,2,2] (ID 168): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=2] ---
.core [0,2,2,2]:
@core_168_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,2,2,2] (ID 169): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=2] ---
.core [1,2,2,2]:
@core_169_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,2,2,2] (ID 170): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=2] ---
.core [2,2,2,2]:
@core_170_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,2,2,2] (ID 171): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=2] ---
.core [3,2,2,2]:
@core_171_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,3,2,2] (ID 172): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=2] ---
.core [0,3,2,2]:
@core_172_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,3,2,2] (ID 173): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=2] ---
.core [1,3,2,2]:
@core_173_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,3,2,2] (ID 174): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=2] ---
.core [2,3,2,2]:
@core_174_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,3,2,2] (ID 175): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=2] ---
.core [3,3,2,2]:
@core_175_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,0,3,2] (ID 176): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=2] ---
.core [0,0,3,2]:
@core_176_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,0,3,2] (ID 177): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=2] ---
.core [1,0,3,2]:
@core_177_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,0,3,2] (ID 178): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=2] ---
.core [2,0,3,2]:
@core_178_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,0,3,2] (ID 179): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=2] ---
.core [3,0,3,2]:
@core_179_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,1,3,2] (ID 180): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=2] ---
.core [0,1,3,2]:
@core_180_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,1,3,2] (ID 181): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=2] ---
.core [1,1,3,2]:
@core_181_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,1,3,2] (ID 182): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=2] ---
.core [2,1,3,2]:
@core_182_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,1,3,2] (ID 183): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=2] ---
.core [3,1,3,2]:
@core_183_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,2,3,2] (ID 184): Brain 5 CORDIC RoPE & MoE Dynamic Router [Z=3,W=2] ---
.core [0,2,3,2]:
@core_184_entry:
B0000: _RX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0001: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0002: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0003: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0004: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0005: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0006: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0007: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0008: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0009: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0010: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0011: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0012: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0013: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0014: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0015: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0016: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0017: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0018: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0019: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0020: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0021: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0022: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0023: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0024: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0025: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0026: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0027: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0028: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0029: _TX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,2,3,2] (ID 185): Brain 5 CORDIC RoPE & MoE Dynamic Router [Z=3,W=2] ---
.core [1,2,3,2]:
@core_185_entry:
B0000: _RX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0001: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0002: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0003: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0004: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0005: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0006: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0007: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0008: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0009: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0010: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0011: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0012: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0013: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0014: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0015: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0016: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0017: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0018: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0019: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0020: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0021: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0022: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0023: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0024: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0025: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0026: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0027: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0028: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0029: _TX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,2,3,2] (ID 186): Brain 5 CORDIC RoPE & MoE Dynamic Router [Z=3,W=2] ---
.core [2,2,3,2]:
@core_186_entry:
B0000: _RX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0001: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0002: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0003: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0004: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0005: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0006: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0007: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0008: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0009: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0010: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0011: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0012: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0013: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0014: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0015: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0016: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0017: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0018: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0019: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0020: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0021: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0022: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0023: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0024: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0025: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0026: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0027: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0028: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0029: _TX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,2,3,2] (ID 187): Brain 5 CORDIC RoPE & MoE Dynamic Router [Z=3,W=2] ---
.core [3,2,3,2]:
@core_187_entry:
B0000: _RX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0001: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0002: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0003: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0004: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0005: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0006: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0007: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0008: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0009: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0010: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0011: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0012: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0013: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0014: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0015: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0016: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0017: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0018: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0019: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0020: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0021: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0022: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0023: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0024: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0025: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0026: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0027: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0028: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0029: _TX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,3,3,2] (ID 188): Brain 6 Metacognitive Sentry & Self-Rewrite [Z=3,W=2] ---
.core [0,3,3,2]:
@core_188_entry:
B0000: _RX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0001: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0002: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0003: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0004: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0005: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0006: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0007: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0008: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0009: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0010: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0011: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0012: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0013: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0014: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0015: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0016: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0017: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0018: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0019: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0020: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0021: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0022: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0023: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0024: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0025: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0026: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0027: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0028: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0029: _TX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,3,3,2] (ID 189): Brain 6 Metacognitive Sentry & Self-Rewrite [Z=3,W=2] ---
.core [1,3,3,2]:
@core_189_entry:
B0000: _RX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0001: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0002: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0003: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0004: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0005: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0006: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0007: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0008: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0009: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0010: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0011: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0012: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0013: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0014: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0015: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0016: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0017: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0018: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0019: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0020: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0021: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0022: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0023: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0024: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0025: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0026: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0027: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0028: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0029: _TX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,3,3,2] (ID 190): Brain 6 Metacognitive Sentry & Self-Rewrite [Z=3,W=2] ---
.core [2,3,3,2]:
@core_190_entry:
B0000: _RX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0001: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0002: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0003: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0004: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0005: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0006: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0007: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0008: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0009: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0010: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0011: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0012: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0013: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0014: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0015: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0016: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0017: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0018: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0019: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0020: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0021: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0022: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0023: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0024: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0025: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0026: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0027: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0028: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0029: _TX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,3,3,2] (ID 191): Brain 6 Metacognitive Sentry & Self-Rewrite [Z=3,W=2] ---
.core [3,3,3,2]:
@core_191_entry:
B0000: _RX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0001: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0002: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0003: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0004: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0005: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0006: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0007: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0008: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0009: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0010: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0011: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0012: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0013: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0014: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0015: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0016: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0017: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0018: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0019: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0020: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0021: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0022: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0023: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0024: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0025: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0026: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0027: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0028: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0029: _TX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,0,0,3] (ID 192): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=3] ---
.core [0,0,0,3]:
@core_192_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,0,0,3] (ID 193): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=3] ---
.core [1,0,0,3]:
@core_193_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,0,0,3] (ID 194): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=3] ---
.core [2,0,0,3]:
@core_194_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,0,0,3] (ID 195): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=3] ---
.core [3,0,0,3]:
@core_195_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,1,0,3] (ID 196): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=3] ---
.core [0,1,0,3]:
@core_196_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,1,0,3] (ID 197): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=3] ---
.core [1,1,0,3]:
@core_197_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,1,0,3] (ID 198): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=3] ---
.core [2,1,0,3]:
@core_198_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,1,0,3] (ID 199): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=3] ---
.core [3,1,0,3]:
@core_199_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,2,0,3] (ID 200): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=3] ---
.core [0,2,0,3]:
@core_200_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,2,0,3] (ID 201): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=3] ---
.core [1,2,0,3]:
@core_201_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,2,0,3] (ID 202): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=3] ---
.core [2,2,0,3]:
@core_202_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,2,0,3] (ID 203): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=3] ---
.core [3,2,0,3]:
@core_203_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,3,0,3] (ID 204): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=3] ---
.core [0,3,0,3]:
@core_204_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,3,0,3] (ID 205): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=3] ---
.core [1,3,0,3]:
@core_205_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,3,0,3] (ID 206): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=3] ---
.core [2,3,0,3]:
@core_206_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,3,0,3] (ID 207): Brain 1 Sensory Ingestion & Causal KG [Z=0,W=3] ---
.core [3,3,0,3]:
@core_207_entry:
B0000: _RX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0001: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0002: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0003: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0004: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0005: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0006: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0007: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0008: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0009: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0010: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0011: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0012: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0013: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0014: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0015: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0016: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0017: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0018: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0019: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0020: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0021: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0022: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0023: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0024: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0025: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0026: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0027: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0028: _SY01#010> _KG02#020> _HE03#030> _PK04#040>
B0029: _TX0F#0F0> _KG02#020> _HE03#030> _PK04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,0,1,3] (ID 208): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=3] ---
.core [0,0,1,3]:
@core_208_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,0,1,3] (ID 209): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=3] ---
.core [1,0,1,3]:
@core_209_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,0,1,3] (ID 210): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=3] ---
.core [2,0,1,3]:
@core_210_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,0,1,3] (ID 211): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=3] ---
.core [3,0,1,3]:
@core_211_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,1,1,3] (ID 212): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=3] ---
.core [0,1,1,3]:
@core_212_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,1,1,3] (ID 213): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=3] ---
.core [1,1,1,3]:
@core_213_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,1,1,3] (ID 214): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=3] ---
.core [2,1,1,3]:
@core_214_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,1,1,3] (ID 215): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=3] ---
.core [3,1,1,3]:
@core_215_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,2,1,3] (ID 216): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=3] ---
.core [0,2,1,3]:
@core_216_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,2,1,3] (ID 217): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=3] ---
.core [1,2,1,3]:
@core_217_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,2,1,3] (ID 218): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=3] ---
.core [2,2,1,3]:
@core_218_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,2,1,3] (ID 219): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=3] ---
.core [3,2,1,3]:
@core_219_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,3,1,3] (ID 220): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=3] ---
.core [0,3,1,3]:
@core_220_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,3,1,3] (ID 221): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=3] ---
.core [1,3,1,3]:
@core_221_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,3,1,3] (ID 222): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=3] ---
.core [2,3,1,3]:
@core_222_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,3,1,3] (ID 223): Brain 2 Photonic MZI Optical FlashAttn3 [Z=1,W=3] ---
.core [3,3,1,3]:
@core_223_entry:
B0000: _RX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0001: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0002: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0003: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0004: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0005: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0006: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0007: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0008: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0009: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0010: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0011: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0012: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0013: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0014: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0015: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0016: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0017: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0018: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0019: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0020: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0021: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0022: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0023: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0024: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0025: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0026: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0027: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0028: _OP05#050> _WD06#060> _TT07#070> _FA08#080>
B0029: _TX0F#0F0> _WD06#060> _TT07#070> _FA08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,0,2,3] (ID 224): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=3] ---
.core [0,0,2,3]:
@core_224_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,0,2,3] (ID 225): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=3] ---
.core [1,0,2,3]:
@core_225_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,0,2,3] (ID 226): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=3] ---
.core [2,0,2,3]:
@core_226_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,0,2,3] (ID 227): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=3] ---
.core [3,0,2,3]:
@core_227_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,1,2,3] (ID 228): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=3] ---
.core [0,1,2,3]:
@core_228_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,1,2,3] (ID 229): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=3] ---
.core [1,1,2,3]:
@core_229_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,1,2,3] (ID 230): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=3] ---
.core [2,1,2,3]:
@core_230_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,1,2,3] (ID 231): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=3] ---
.core [3,1,2,3]:
@core_231_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,2,2,3] (ID 232): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=3] ---
.core [0,2,2,3]:
@core_232_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,2,2,3] (ID 233): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=3] ---
.core [1,2,2,3]:
@core_233_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,2,2,3] (ID 234): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=3] ---
.core [2,2,2,3]:
@core_234_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,2,2,3] (ID 235): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=3] ---
.core [3,2,2,3]:
@core_235_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,3,2,3] (ID 236): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=3] ---
.core [0,3,2,3]:
@core_236_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,3,2,3] (ID 237): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=3] ---
.core [1,3,2,3]:
@core_237_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,3,2,3] (ID 238): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=3] ---
.core [2,3,2,3]:
@core_238_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,3,2,3] (ID 239): Brain 3 Reversible SwiGLU & Fredkin ALU [Z=2,W=3] ---
.core [3,3,2,3]:
@core_239_entry:
B0000: _RX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0001: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0002: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0003: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0004: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0005: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0006: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0007: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0008: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0009: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0010: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0011: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0012: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0013: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0014: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0015: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0016: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0017: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0018: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0019: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0020: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0021: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0022: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0023: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0024: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0025: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0026: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0027: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0028: _RF09#090> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0029: _TX0F#0F0> _TO0A#0A0> _BK0B#0B0> _RS00#0C0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,0,3,3] (ID 240): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=3] ---
.core [0,0,3,3]:
@core_240_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,0,3,3] (ID 241): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=3] ---
.core [1,0,3,3]:
@core_241_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,0,3,3] (ID 242): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=3] ---
.core [2,0,3,3]:
@core_242_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,0,3,3] (ID 243): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=3] ---
.core [3,0,3,3]:
@core_243_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,1,3,3] (ID 244): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=3] ---
.core [0,1,3,3]:
@core_244_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,1,3,3] (ID 245): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=3] ---
.core [1,1,3,3]:
@core_245_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,1,3,3] (ID 246): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=3] ---
.core [2,1,3,3]:
@core_246_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,1,3,3] (ID 247): Brain 4 Neuromorphic STDP & SNN Crossbar [Z=3,W=3] ---
.core [3,1,3,3]:
@core_247_entry:
B0000: _RX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0001: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0002: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0003: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0004: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0005: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0006: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0007: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0008: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0009: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0010: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0011: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0012: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0013: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0014: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0015: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0016: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0017: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0018: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0019: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0020: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0021: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0022: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0023: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0024: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0025: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0026: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0027: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0028: _ST0C#0C0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0029: _TX0F#0F0> _LF0D#0D0> _LI0E#0E0> _SB00#0F0>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,2,3,3] (ID 248): Brain 5 CORDIC RoPE & MoE Dynamic Router [Z=3,W=3] ---
.core [0,2,3,3]:
@core_248_entry:
B0000: _RX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0001: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0002: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0003: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0004: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0005: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0006: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0007: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0008: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0009: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0010: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0011: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0012: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0013: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0014: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0015: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0016: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0017: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0018: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0019: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0020: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0021: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0022: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0023: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0024: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0025: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0026: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0027: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0028: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0029: _TX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,2,3,3] (ID 249): Brain 5 CORDIC RoPE & MoE Dynamic Router [Z=3,W=3] ---
.core [1,2,3,3]:
@core_249_entry:
B0000: _RX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0001: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0002: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0003: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0004: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0005: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0006: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0007: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0008: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0009: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0010: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0011: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0012: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0013: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0014: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0015: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0016: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0017: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0018: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0019: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0020: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0021: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0022: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0023: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0024: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0025: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0026: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0027: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0028: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0029: _TX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,2,3,3] (ID 250): Brain 5 CORDIC RoPE & MoE Dynamic Router [Z=3,W=3] ---
.core [2,2,3,3]:
@core_250_entry:
B0000: _RX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0001: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0002: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0003: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0004: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0005: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0006: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0007: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0008: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0009: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0010: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0011: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0012: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0013: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0014: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0015: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0016: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0017: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0018: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0019: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0020: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0021: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0022: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0023: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0024: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0025: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0026: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0027: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0028: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0029: _TX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,2,3,3] (ID 251): Brain 5 CORDIC RoPE & MoE Dynamic Router [Z=3,W=3] ---
.core [3,2,3,3]:
@core_251_entry:
B0000: _RX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0001: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0002: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0003: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0004: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0005: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0006: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0007: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0008: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0009: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0010: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0011: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0012: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0013: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0014: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0015: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0016: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0017: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0018: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0019: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0020: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0021: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0022: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0023: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0024: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0025: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0026: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0027: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0028: _CD01#010> _SW02#020> _OD03#030> _CA04#040>
B0029: _TX0F#0F0> _SW02#020> _OD03#030> _CA04#040>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [0,3,3,3] (ID 252): Brain 6 Metacognitive Sentry & Self-Rewrite [Z=3,W=3] ---
.core [0,3,3,3]:
@core_252_entry:
B0000: _RX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0001: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0002: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0003: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0004: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0005: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0006: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0007: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0008: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0009: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0010: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0011: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0012: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0013: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0014: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0015: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0016: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0017: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0018: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0019: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0020: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0021: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0022: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0023: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0024: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0025: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0026: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0027: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0028: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0029: _TX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [1,3,3,3] (ID 253): Brain 6 Metacognitive Sentry & Self-Rewrite [Z=3,W=3] ---
.core [1,3,3,3]:
@core_253_entry:
B0000: _RX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0001: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0002: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0003: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0004: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0005: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0006: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0007: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0008: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0009: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0010: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0011: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0012: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0013: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0014: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0015: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0016: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0017: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0018: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0019: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0020: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0021: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0022: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0023: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0024: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0025: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0026: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0027: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0028: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0029: _TX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [2,3,3,3] (ID 254): Brain 6 Metacognitive Sentry & Self-Rewrite [Z=3,W=3] ---
.core [2,3,3,3]:
@core_254_entry:
B0000: _RX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0001: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0002: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0003: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0004: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0005: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0006: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0007: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0008: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0009: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0010: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0011: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0012: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0013: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0014: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0015: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0016: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0017: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0018: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0019: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0020: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0021: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0022: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0023: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0024: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0025: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0026: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0027: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0028: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0029: _TX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0030: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!

; --- Core [3,3,3,3] (ID 255): Brain 6 Metacognitive Sentry & Self-Rewrite [Z=3,W=3] ---
.core [3,3,3,3]:
@core_255_entry:
B0000: _RX0F#0F0> _PO06#060> _AW07#070> _SC08#080>
B0001: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0002: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0003: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0004: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0005: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0006: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0007: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0008: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0009: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0010: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0011: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0012: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0013: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0014: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0015: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0016: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0017: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0018: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0019: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0020: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0021: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0022: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0023: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0024: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0025: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0026: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0027: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0028: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0029: _MD05#050> _PO06#060> _AW07#070> _SC08#080>
B0030: _PT00#000> _SH00#000> _NO00#070> _HL00$0E8!

