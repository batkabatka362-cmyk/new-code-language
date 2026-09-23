# CRON Low-Level Machine Language (.cl) — Official Architecture Specification

**Version:** 90.0 (CL 2.0 SSS+ Enterprise Architecture Standard)  
**Target Architecture:** 256-Core ($4 \times 4 \times 4 \times 4$) 4D-Torus Photonic/Neuromorphic Processor  
**Formal Foundations:** 798-Page System Architecture Manual (Pages 46–55, 88–95, 146–165)  
**Encoding Paradigm:** Machine-Native, AI-Native, Deterministic Variable-Width VLIW, 94-Character ASCII Saturated  

---

## 1. Философи ба Зорилго (Architectural Philosophy)

`.cl` (Cognitive Low-level) бол зүгээр нэг компиляторын завсрын хоёртын файл бус, харин **биет цахиур (RTL Silicon) болон хиймэл оюун ухаанд (AI Agent / Autonomous Cognitive Unit) шууд зориулагдсан бие даасан машин-түвшний програмчлалын хэл** юм.

* **Хүмүүст зориулсан `.cr`:** Өндөр түвшний сэтгэхүй, шугаман төрөл (`lin`), амьдралын хугацааны бүсчлэл (`region`), логик дүрмүүд.
* **AI болон машинд зориулсан `.cl` (CL 2.0):** Ямар ч хоёрдмол утгагүй (Zero Ambiguity), хувьсах өргөнтэй (1..4 слот), алдаа илрүүлэх CRC-8 ATM / Parity хамгаалалттай, хоосон NOP зай үрэхгүй VLIW хэл.
* **94 Тэмдэгтийн Бүрэн Цагаан Толгой (Full 94-Character Alphabet):** ASCII 33 (`!`) -аас 126 (`~`) хүртэлх хэвлэгдэх бүх 94 тэмдэгтийг функциональ үүргүүдэд хуваарилан, мэдээллийн нягтралыг (Shannon Entropy $H \approx 5.1036$ bits/char) дээд зэргээр хангасан.
* **Семантик AI Директивүүд (Zero Comment Waste):** Хэрэгцээгүй тайлбар комментуудыг бүрэн халж, компилятор өөрөө баталгаажуулдаг `.stage`, `.tensor`, `.fuse`, `.flow`, `.layout`, `.weights` бүтцийг ашигладаг.

---

## 2. Кодын Бүтэц ба Багцын Бүтэц (Bundle Structure & EBNF)

### 2.1 Семантик AI Директивүүд (Semantic Directives)
Файлын эхэнд компилятор ба техник хангамжид зориулсан глобал тохиргоог тунхаглана:

```cl
.stage "<model_id>", params="<N>", precision="<type>", d_model=<D>, heads=<H>, kv_heads=<KV>, intermediate=<FFN>, zero_overhead=true
.tensor %Q: [<dim1>, <dim2>], %K: [...], %V: [...], %O: [...]
.fuse [<Layer1> -> <Layer2> & <Layer3> -> <Layer4>]
.flow (Core[x1,y1,z1,w1] -> Core[x2,y2,z2,w2] -> ...) {dor=XYZW}
.layout {TP=<n>, EP=<n>, CP=<n>, PP=<n>, dim="4x4x4x4", chip="256_core_torus"}
.weights bank=<id>, offset=<off>: [<f1>, <f2>, ...]
```

### 2.2 Хувьсах Өргөнтэй VLIW Багц (Variable-Width Bundles)
CL 2.0-д багц бүр нь 1-ээс 4 хүртэлх слотыг динамикаар агуулж болно:

```cl
B<cycle_index>: <Slot_0> [<Slot_1>] [<Slot_2>] [<Slot_3>]
```

* **Хувьсах өргөн:** 1..4 слот (хоосон NOP зай үрэх шаардлагагүй). Оруулсан слотуудыг техник хангамж 0-switching power-gated байдлаар гүйцэтгэнэ.
* **Такт:** Цахиурын 1 тактанд бүх слотууд зэрэгцээ ажиллана.
* **Хазард Шалгалт (Hazard Free):** 1 багц дотор ижил регистрт зэрэг бичих (WAW) болон 0-такт сааталтайгаар бичигдэж буй регистрийг шууд унших (RAW) нь хориотой бөгөөд `cron cl-audit` компилятор/шалгагчаар баталгаажна.

---

## 3. Слотын 10 Тэмдэгтийн Бүтэц (Slot Token Anatomy)

Слот бүр нь яг **10 ASCII тэмдэгтэд** тогтмол өргөнтэйгээр багтана:

| Байрлал (Индекс) | Зориулалт | Зөвшөөрөгдөх Тэмдэгтүүд | Тайлбар |
|---|---|---|---|
| **0** | **Prefix** | `_`, `'`, `~`, `@` | `_`: Стандарт синхрон, `'`: Спекулятив/Шууд ачаалагч, `~`: Стохастик/Ойролцоо, `@`: Санах ойн заагч |
| **1 .. 2** | **Opcode** | `OP`, `MD`, `RF`, `ST`, ... (40+ опкод) | 2 тэмдэгттэй үндсэн үйлдэл |
| **3 .. 4** | **Dest Reg** | `00` .. `0F` ($R_0$ .. $R_{15}$, мөн `0a`..`0f`) | Үр дүнг бичих 16 регистрийн дугаар |
| **5** | **Mode Delim** | `$`, `#`, `@`, `+`, `-`, `*`, `/`, `&`, `\|`, `^`, `~`, `%`, `=`, `<`, `>`, `?`, `!` | Полиморфик горимын тэмдэгт (ALU bypass, шууд утга, хаяглалт) |
| **6** | **Src Reg / Port** | `0` .. `F`, `x`, `y`, `z`, `w`, `u`, `d`, `l`, `r` | Эх регистр эсвэл 4D Torus NoC чиглэлийн порт |
| **7 (8 дахь)** | **Parity / CRC-8** | `0`..`9`, `A`..`F`, `a`..`z`, тусгай тэмдэгт | **CRC-8 ATM $x^8 + x^2 + x + 1$ бүрэн бүтэн байдлын токен** |
| **8** | **Immediate / Flag** | `0`..`9`, `A`..`F`, `a`..`z` | Шууд утгын ниббл эсвэл нөхцөлт төлөвийн туг |
| **9 (10 дахь)** | **Terminator** | `>`, `!`, `?`, `;` | `>`: Хэвийн синхрон төгсгөл, `!`: Чекпойнт/Трап, `?`: Предикат төгсгөл, `;`: Шилэн утас сэлгэх (Yield) |

---

## 4. Бүх 94 Тэмдэгтийн Функциональ Лавлах (94-Character Complete Architecture)

`.cl` хэл нь хэвлэгдэх боломжтой 94 ASCII тэмдэгтийг ($33 \le \text{ASCII} \le 126$) дараах 4 бүлэгт ангилан 100% ашигладаг:

### 4.1. Тоон Тэмдэгтүүд (Digits: 10 Chars, ASCII 48..57)
* `0` .. `9`: Регистрийн дугаарууд ($R_0$..$R_9$), Багцын тактын дугаар (`B0001:`), Шууд тоон утгууд, болон CRC-8 ATM чексум нибблүүд.

### 4.2. Том Латин Үсгүүд (Uppercase: 26 Chars, ASCII 65..90)
* `A` .. `F`: Өргөтгөсөн физик регистрүүд ($R_{10}$..$R_{15}$, буюу $R_A$..$R_F$).
* `B`: VLIW 128-битийн багцын эхлэлийг заах такт танигч (`B0001:`).
* `G`: ALU их буюу тэнцүү (`val_d >= val_s ? 1 : 0`) салбарлалтгүй харьцуулах горим.
* `C`: CSR (Control & Status Register) техник хангамжийн гүйцэтгэлийн тоолуурыг унших горим.
* **40+ Үндсэн Опкодууд (6 Тархины Систем):**
  * **Brain 1 (Causal/Graph):** `SY` (Symbolify), `KG` (Knowledge Graph), `HE` (Hyper-Edge Associator).
  * **Brain 2 (Photonic Wave):** `OP` (MZI GEMM), `WD` (WDM Optical), `FA` (Autodiff Tap).
  * **Brain 3 (Reversible):** `BK` (Backward Invert), `RF` (Fredkin Gate Swap), `TO` (Toffoli 3-Wire Gate).
  * **Brain 4 (Neuromorphic):** `ST` (STDP Plasticity), `LF` (LIF Spike Gen), `LI` (LIF Step).
  * **Brain 5 (Chaos/Wave):** `OD` (Lorenz Attractor), `CA` (Cross-Attention Gating), `SW` (Superposition).
  * **Brain 6 (Self-Healing):** `SH` (Sentry Config), `AW` (Arbiter Weight), `RC` (Reroute / Checkpoint).
  * **ALU / SIMD Хурдасгуур:** `PO` (Predicated SIMD), `MD` (Ternary MAC/Mul), `PK` (Sub-byte Pack), `PS` (Prefix Sum), `CD` (CORDIC Trig), `TT` (Tensor Transpose).
  * **4D Torus NoC ба Удирдлага:** `TL` (Tile Coords), `SP` (Fiber Spawn), `FJ` (Fiber Join), `YD` (Fiber Yield), `TX` (NoC Packet Inject), `RX` (Mailbox Pop), `IR` (In-Network Reduction), `DF` (Deflection), `WH` (Wormhole Tunnel), `SB` (Spatial Broadcast), `AC` (Capability Token), `SN` (Sanitize), `SC` (Secure Patch), `RN` (LFSR PRNG), `PL` (NoC Poll), `RT` (Trap Return), `HL` (Halt), `NO` (NOP).

### 4.3. Жижиг Латин Үсгүүд (Lowercase: 26 Chars, ASCII 97..122)
* `a` .. `f`: Бага регистрийн хекс хоч нэрс ($r_a$..$r_f$).
* `x`, `y`, `z`, `w`: 4D-Torus сүлжээний 8 чиглэлт NoC замын портууд ($\pm X, \pm Y, \pm Z, \pm W$).
* `u`, `d`, `l`, `r`: Хавтгайн 2D чиглэлүүд (Up, Down, Left, Right).
* `i`, `o`: Цөмийн симплекс оролт/гаралт сувгууд (Inward/Outward FIFO).
* `s`, `m`, `h`, `t`: Спайк, мембран потенциал, гало бүс, торус алхам.
* `c`, `v`, `k`, `p`: Санамж, халилт, мэдлэгийн таг, паритет битийн төлөв.
* `q`, `g`, `j`, `n`: Дараалал дүүрсэн, их нөхцөл, үсрэлтийн туг, сөрөг утгын туг.
* `e`, `b`: Эпохын тоолуур, багц хоорондын саад (barrier).

### 4.4. Тусгай Тэмдэгтүүд (Special Symbols: 32 Chars)

| Тэмдэгт | ASCII | Үүрэг | Машин Түвшний Утга |
|---|---|---|---|
| `!` | 33 | Чекпойнт / Трап | Аюулгүй байдлын шалгалт, техник хангамжийн тасалдал |
| `"` | 34 | Литерал хязгаарлагч | Статик нейрон жингийн өгөгдөл эхлэх |
| `#` | 35 | Шууд утгын горим | Тогтмол тоон утга шууд дамжуулах горим |
| `$` | 36 | Регистрийн горим | Биет регистрийг заах стандарт хаяглалт |
| `%` | 37 | Үлдэгдэл олох ALU | Нэг тактад модуло хуваах тооцоолол |
| `&` | 38 | Логик БОЛОН ALU | Битийн коньюнкц логик шүүлтүүр |
| `'` | 39 | Спекулятив угтвар | Салбар таамаглал бүхий өндөр зэрэглэлийн слот |
| `(` | 40 | Векторын эхлэл | Багцалсан SIMD олон сувгийн өгөгдлийн эхлэл |
| `)` | 41 | Векторын төгсгөл | Багцалсан SIMD олон сувгийн өгөгдлийн төгсгөл |
| `*` | 42 | Үржүүлэх ALU | Нэг тактын DSP техник хангамжийн үржигч |
| `+` | 43 | Нэмэх ALU | Шууд нийлбэрийн тойрох зам (ALU Adder Bypass) |
| `,` | 44 | Талбарын тусгаарлагч | Олон параметрт үйлдлийн аргумент салгагч |
| `-` | 45 | Хасах ALU | Нэг тактын хасагчийн тойрох зам (Subtractor Bypass) |
| `.` | 46 | Бутархай заагч | Q16.16 тогтмол таслалт нарийвчлалын заагч |
| `/` | 47 | Хуваах ALU | Тэгээр хуваах үед 0x0001 трап өдөөх хуваагч |
| `:` | 58 | Багцын тусгаарлагч | Тактын гарчиг тусгаарлагч (`B0001:`) |
| `;` | 59 | Сул төгсгөл / Коммент | Бага ач холбогдолтой төгсгөл эсвэл тайлбар мөр |
| `<` | 60 | Бага / Дотогш NoC | Тэмдэгттэй бага эсвэл NoC урсгал хүлээн авах |
| `=` | 61 | Утга олгох / Тэнцүү | Шууд утга ачаалах (`=0`) эсвэл тэнцүү шалгах |
| `>` | 62 | Стандарт төгсгөл | Синхрон үйлдлийн албан ёсны төгсгөлийн токен |
| `?` | 63 | Предикат горим | Нөхцөлт предикат гейт (`val_s != 0 ? val_d : 0`) |
| `@` | 64 | Санах ойн заагч | Орон зайн HBM3/SRAM шууд хаяглалт |
| `[` | 91 | Бүсийн эхлэл | Бүсчилсэн санах ойн арена эхлэх хаалт |
| `\` | 92 | Урвуу дамжуулалт | Буцах чиглэлтэй өгөгдлийн урсгалын тойрох зам |
| `]` | 93 | Бүсийн төгсгөл | Бүсчилсэн санах ойн арена хаагдах хаалт |
| `^` | 94 | Битийн XOR ALU | Паритет болон битийн хасах-эсвэл логик гейт |
| `_` | 95 | Стандарт угтвар | Детерминистик синхрон үйлдлийн стандарт эхлэл |
| `` ` `` | 96 | Тактын тамга | Микрокодын бодит цагийн тактын тамга тэмдэгт |
| `{` | 123 | Тархины кластер нээх | 6 тархины домэйн кластер бүлэглэлтийн эхлэл |
| `\|` | 124 | Логик ЭСВЭЛ / Саад | Дисъюнкц битийн үйлдэл эсвэл торусын саад |
| `}` | 125 | Тархины кластер хаах| 6 тархины домэйн кластер бүлэглэлтийн төгсгөл |
| `~` | 126 | Ойролцоо / БҮШ ALU | Ойролцоо угтвар эсвэл битийн урвуу горим (NOT) |

---

## 5. Полиморфик Горимын Семантик (Pages 88-95)

5 дахь байрлалд (`chars[5]`) байрлах горимын тэмдэгт нь `PO` (Predicated SIMD ALU) болон `MD` (Ternary MAC) слотуудын гүйцэтгэх логикийг шууд тодорхойлно:

```rust
// CoreEngine: execute_slot
match mode {
    '+' => val_d.wrapping_add(val_s),
    '-' => val_d.wrapping_sub(val_s),
    '*' => val_d.wrapping_mul(val_s),
    '/' => if val_s != 0 { val_d / val_s } else { self.trigger_trap(0x0001); 0 },
    '%' => if val_s != 0 { val_d % val_s } else { self.trigger_trap(0x0001); 0 },
    '&' => val_d & val_s,
    '|' => val_d | val_s,
    '^' => val_d ^ val_s,
    '~' => !val_s,
    '=' => if val_d == val_s { 1 } else { 0 },
    '<' => if val_d < val_s { 1 } else { 0 },
    '>' => if val_d > val_s { 1 } else { 0 },
    '?' => if val_s != 0 { val_d } else { 0 },
    '!' => { if val_s != 0 { self.trigger_trap(0x0002); } val_d },
    'G' => if val_d >= val_s { 1 } else { 0 },
    _   => /* Immediate fallthrough */,
}
```

---

## 6. Мэдээллийн Нягтрал ба Шеннон Энтропи (Information Entropy)

`.cl` кодын мэдээллийн нягтралыг Шенноны энтропийн томьёогоор тооцоолно:

$$H(X) = -\sum_{i=1}^{94} p(x_i) \log_2 p(x_i)$$

* **Онолын дээд энтропи ($H_{\max}$):** $\log_2(94) \approx 6.5546$ бит/тэмдэгт.
* **Шалгуурын үнэлгээ (Audit Rating):**
  * **SSS+ (Supreme Cognitive Harmony):** 94 тэмдэгт бүрэн ашиглагдсан (100% saturation), $H > 4.5$ бит/тэмдэгт, 0 техник хангамжийн хазард (RAW/WAW/Structural).
  * **S+ (High Information Efficiency):** $\ge 70\%$ ашиглалт, $H > 4.0$ бит/тэмдэгт.
  * **A (Standard Machine Encoding):** Хэвийн 40-70% түгээмэл машин код.

---

## 7. Бүрэн 94 Тэмдэгтийг Шийдсэн Програмын Жишээ

`examples/pure_machine_agent.cl` файл нь 6 тархины 40+ опкодыг зэрэгцүүлэн, 94 тэмдэгтийг 100% ашигласан SSS+ загвар програм юм:

```cl
; examples/pure_machine_agent.cl
; CRON Cognitive Machine Language — 94-Character Complete Architecture Demonstration
; Target: 256-Core 4D-Torus Neuromorphic Hardware (SSS+ Saturated VLIW)

B0001: '=00#0A04> '=10$040B? _SH00(1V4; _FJ00)0Z8!
B0002: _PK02+200> _TL03-004> _ST04*110> _YD00/005>
B0003: _OP05%00E> _FA06&420> _PO07|432> _RC08^2Q5>
B0004: _MD09~206> _BK0a=501> _BL0b<310> _RF0c>604>
B0005: _GU04?900> _ST0d!554> _SY0e@728> _RS00:04B>
B0006: _KG01{000> _OD02}000> _CS03[000> _TT04]000>
B0007: _PS05,100> _CA06.400> _CD07"100> _AW08`000>
B0008: _WH09\000> _LF0a_000> _LI0b$205> _TX0c#xp1>
B0009: ~RX0d$yq2? @IR0e$zn3; 'DF01$wj4! ~AC02$u05>
B0010: @SN03$d06> 'SC04$l07> _RN05$r08> _PL06$i09>
B0011: _PT07$o0a> _SW08$s0b> _DW00$m0c> _SB00$h0d>
B0012: _HE09$t0e> _RT00$c0f> _NO00$v0g> _HL00$k0h!
```

---

## 8. CLI Тушаалууд (Toolchain Support)

Компилятор ба хэрэгслийн түвшинд `.cl` хэлийг бүрэн шалгах коммандууд:

```bash
# 1. Бүх 94 тэмдэгтийн албан ёсны семантик лавлахыг хэвлэх
cron cl-alphabet

# 2. .cl файлын 94 тэмдэгтийн ашиглалт, энтропи, болон цахиурын хазардыг аудит хийх
cron cl-audit examples/pure_machine_agent.cl

# 3. 256-цөмт 4D-Torus симулятор дээр шууд ажиллуулах
cron sim examples/pure_machine_agent.cl

# 4. .cl машинаас .cr дээд хэл рүү декомпил хийх
cron decompile examples/pure_machine_agent.cl -o blueprint.cr

# 5. .cl машины кодыг автоматаар форматлах ба CRC-8 алдааг нөхөх
cron cl-fmt examples/cl/clifford_rotate4d.cl -o formatted.cl

# 6. .cl кодын тактын хуваарь, регистрийн төлөвийг эх кодын түвшинд шалгах
cron cl-debug-meta examples/cl/clifford_rotate4d.cl

# 7. 4D Clifford Algebra Cl(4,0) эргэлтийн гол цөмийг синтез хийх
cron cl-kernel clifford-rotate4d -o clifford_kernel.cl
```

---

## 9. 4D Clifford Algebra Cl(4,0) Цахиурын Өргөтгөл (Geometric Silicon Engine)

CRON-ийн $Cl(4,0)$ Spacetime Algebra нь стандарт $W \cdot x$ ерөнхий матриц үржвэрийг (GEMM) 4D орон зайн эргэлтээр ($v' = R v \tilde{R}$) орлуулан, санах ойн зурвасын ачааллыг 4 дахин бууруулдаг:

* **Бүтэц (Rotor in Spin(4) = SU(2) × SU(2)):** 8 скаляр бүрэлдэхүүн ($s, e_{12}, e_{13}, e_{14}, e_{23}, e_{24}, e_{34}, p$).
* **Машин түвшний гүйцэтгэл:**
  - `_MD` (Ternary MAC): Квадратик хэлбэрийн элементүүдийг $O(1)$ хугацаанд тооцоолох.
  - `_TT` (Tensor Transpose): $4 \times 4$ эргэлтийн матрицыг регистр дотор байрлуулах.
  - `_PO` (SIMD ALU): 4D орон-цагийн вектортой матриц-вектор үржвэрийг салбарлалтгүй (branchless) 1-2 тактад гүйцэтгэх.
* **Албан ёсны Директив:** `.clifford rotor=Rotor4D, vector=Vector4D, algebra="Cl(4,0)"`.

---

## 10. Семантик Директивүүдийн Дүрэм (Semantic Directives Reference)

`.cl` машин хэл нь файлын эхэнд компилятор болон техник хангамжид зориулсан директивүүдийг дэмждэг:

| Директив | Формат | Зориулалт |
|---|---|---|
| `.clifford` | `.clifford rotor=..., vector=..., algebra="..."` | Geometric Algebra төрлийг зааж өгөх |
| `.stage` | `.stage "<id>", params="...", precision="...", d_model=N, ...` | Моделийн түвшний гиперпараметр тодорхойлох |
| `.tensor` | `.tensor %Q: [dim1, dim2], %K: [...]` | Олон хэмжээст тензорын хэмжээс тунхаглах |
| `.fuse` | `.fuse [Layer1 -> Layer2 & Layer3 -> Layer4]` | Дараалсан цөмүүдийг нэгтгэн HBM бичилтийг алгасах |
| `.flow` | `.flow (Core[0,0,0,0] -> Core[0,0,1,0]) {dor=XYZW}` | 4D-Torus сүлжээний урсгалын чиглэл заах |
| `.layout` | `.layout {TP=1, EP=1, CP=1, PP=1, dim="4x4x4x4"}` | Зэрэгцээ тооцооллын кластер зохион байгуулалт |
| `.core` | `.core [x, y, z, w]:` | 4D координатын дагуух цөмийн кодын хил заах |
| `.weights` | `.weights bank=N, offset=M: [f1, f2, ...]` | SRAM банкны анхны жингүүдийг ачаалах |

---

## 11. Машин Хэрэгслийн Экосистем (Toolchain Architecture)

`.cl` машин хэл нь бүтэн циклтэй автоматжуулалтын хэрэгслүүдээр тоноглогдсон:

```
[.cr Өндөр хэл] ──(cron build)──> [.cl Машин код] ──(cron cl-opt)──> [Super-Optimized .cl]
      ▲                                   │                                 │
      │                               (cron cl-fmt)                    (cron cl-cosim)
(cron decompile)                          │                                 │
      │                             [Canonical .cl]                   [Verilog RTL Parity]
      └───────────────────────────────────┴─────────────────────────────────┘
```

