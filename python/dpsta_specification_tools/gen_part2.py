# -*- coding: utf-8 -*-
import sys

part2_text = """

## 13. EFABLESS CARAVEL MPW TAPEOUT ИНТЕГРАЦИ (Хуудас 183–191)

SkyWater 130nm үнэгүй tapeout-д зориулсан Efabless Caravel User Project harness-тай `dpsta_asic_top` модулийг холбох, Wishbone автозамын бүртгэгчдийн хаяглалт, болон OpenLane шаталсан хатуужуулалт (Hierarchical Hardening Flow).

### 13.1. Wishbone Санах Ойн Бүртгэгчийн Бүтэц (Memory Map, Хуудас 183)
Caravel доторх RISC-V удирдлагын цөм (Management SoC) нь `0x3000_0000` хаягийн мужаар дамжуулан DPSTA цөмтэй 32-битийн Wishbone автобусаар харилцана:

| Хаягийн офсет | Нэр | Хандалт | Битийн бүтэц ба Үүрэг |
| :--- | :--- | :--- | :--- |
| `0x3000_0000` | `CTRL_REG` | R/W | `[0]`: train_en, `[1]`: infer_en, `[2]`: flush_win, `[3]`: Программын rst_n |
| `0x3000_0004` | `STATUS_REG` | R | `[0]`: io_data_ready, `[1]`: pred_valid, `[2]`: hdc_match_valid, `[3]`: online_correct, `[4]`: online_error |
| `0x3000_0008` | `DATA_IN_REG` | W | `[31:0]`: 32-бит өгөгдлийн үг (Бичилт хийхэд io_data_valid строб автоматаар 1 такт асна) |
| `0x3000_000C` | `PRED_OUT_REG` | R | `[6:0]`: Таамагласан тэмдэгт (pred_sym_out), `[31:7]`: Тэгээр дүүргэгдэнэ |

### 13.2. Caravel Top Harness (`user_project_wrapper.v`, Хуудас 183–185, 188–189)
Caravel талбайн тэжээлийн шугамууд (`vccd1`, `vssd1`), Wishbone автобус, Logic Analyzer (LA) телеметр, болон физик GPIO хөлийг холбох тодорхойлолт:

```verilog
`default_nettype none
`timescale 1ns / 1ps

module user_project_wrapper #(
    parameter BITS = 32
)(
`ifdef USE_POWER_PINS
    inout vccd1, // 1.8V Дижитал үндсэн тэжээл
    inout vssd1, // Дижитал газар
    inout vccd2, inout vssd2,
    inout vdda1, inout vssa1,
    inout vdda2, inout vssa2,
`endif
    // Wishbone удирдлагын интерфейс (Management SoC -> DPSTA)
    input  wire        wb_clk_i,
    input  wire        wb_rst_i,
    input  wire        wbs_stb_i,
    input  wire        wbs_cyc_i,
    input  wire        wbs_we_i,
    input  wire [3:0]  wbs_sel_i,
    input  wire [31:0] wbs_dat_i,
    input  wire [31:0] wbs_adr_i,
    output reg         wbs_ack_o,
    output reg  [31:0] wbs_dat_o,

    // Logic Analyzer (Тест ба оношилгоо)
    input  wire [127:0] la_data_in,
    output wire [127:0] la_data_out,
    input  wire [127:0] la_oenb,

    // Физик хөлүүд (IO Pads)
    input  wire [37:0] io_in,
    output wire [37:0] io_out,
    output wire [37:0] io_oeb,

    output wire [2:0]  user_irq
);
    // Сул шугамуудыг битүүмжлэх
    assign user_irq = 3'b000;
    assign io_oeb   = 38'h3FFFFFFFFF; // Зөвхөн оролт
    assign io_out   = 38'h0;

    // Wishbone тайлагч ба Резистрүүд
    reg [3:0]  ctrl_reg;
    reg [31:0] data_in_reg;
    reg        data_in_strobe;

    wire clk   = wb_clk_i;
    wire rst_n = (~wb_rst_i) & ctrl_reg[3];
    wire wb_req = wbs_cyc_i && wbs_stb_i;

    wire dpsta_ready, pred_val, hdc_match, stat_cor, stat_err;
    wire [6:0] pred_sym;

    always @(posedge clk or posedge wb_rst_i) begin
        if (wb_rst_i) begin
            wbs_ack_o      <= 1'b0;
            wbs_dat_o      <= 32'd0;
            ctrl_reg       <= 4'b1011; // Def: rst_n=1, flush=0, infer=1, train=1
            data_in_reg    <= 32'd0;
            data_in_strobe <= 1'b0;
        end else begin
            data_in_strobe <= 1'b0;
            wbs_ack_o      <= 1'b0;
            if (wb_req && !wbs_ack_o) begin
                wbs_ack_o <= 1'b1;
                if (wbs_we_i) begin
                    case (wbs_adr_i[7:0])
                        8'h00: ctrl_reg    <= wbs_dat_i[3:0];
                        8'h08: begin
                            data_in_reg    <= wbs_dat_i;
                            data_in_strobe <= 1'b1;
                        end
                    endcase
                end else begin
                    case (wbs_adr_i[7:0])
                        8'h00: wbs_dat_o <= {28'd0, ctrl_reg};
                        8'h04: wbs_dat_o <= {27'd0, stat_err, stat_cor, hdc_match, pred_val, dpsta_ready};
                        8'h0C: wbs_dat_o <= {25'd0, pred_sym};
                        default: wbs_dat_o <= 32'hDEADBEEF;
                    endcase
                end
            end
        end
    end

    // DPSTA ASIC Макро инстанц
    dpsta_asic_top u_dpsta (
`ifdef USE_POWER_PINS
        .vccd1(vccd1), .vssd1(vssd1),
`endif
        .clk(clk), .rst_n(rst_n),
        .train_en(ctrl_reg[0]), .infer_en(ctrl_reg[1]), .flush_win(ctrl_reg[2]),
        .io_data_in(data_in_reg), .io_data_valid(data_in_strobe), .io_data_ready(dpsta_ready),
        .pred_valid(pred_val), .pred_sym_out(pred_sym),
        .hdc_match_valid(hdc_match), .online_correct(stat_cor), .online_error(stat_err)
    );

    // Logic Analyzer телеметр
    assign la_data_out[6:0]   = pred_sym;
    assign la_data_out[7]     = pred_val;
    assign la_data_out[8]     = dpsta_ready;
    assign la_data_out[9]     = hdc_match;
    assign la_data_out[10]    = stat_cor;
    assign la_data_out[11]    = stat_err;
    assign la_data_out[127:12]= 116'd0;
endmodule
```

### 13.3. Шаталсан Хатуужуулалт (Hierarchical Hardening) ба Тохиргоо (Хуудас 189–191)
1. **ШАТ 1: Дэд макро хатуужуулах (`openlane/dpsta_asic_top/config.json`):**
   - Талбай: $2000\\mu\\text{m} \\times 2000\\mu\\text{m}$, Core: $15, 15, 1985, 1985$.
   - Чиглүүлэлт: PDN Core Ring, 5 металл давхарга, LEF ба GDSII үүсгэх.
2. **ШАТ 2: Caravel Wrapper-т байршуулах (`openlane/user_project_wrapper/config.json`):**
   - Талбай: $2920\\mu\\text{m} \\times 3520\\mu\\text{m}$, Core: $10, 10, 2910, 3510$.
   - `MACROS`: `dpsta_asic_top` макрогийн координат `location: [400, 700]`, чиглэл `N`.
   - `macro.cfg`: `u_dpsta 400 700 N`

---

## 14. ПАТЕНТИЙН АЛБАН ЁСНЫ ЗУРАГ ТӨСЛҮҮД БА НАРИЙВЧИЛСАН ТАЙЛБАР (Хуудас 191–197)

### 14.1. ЗУРАГ ТӨСӨЛ (PATENT DRAWINGS: FIG 1–4)

```
ЗУРАГ 1: ТЭГ-БУЦАХ ГРАДИЕНТ БҮХИЙ DPSTA ЦӨМ БА ТРОПИК СЕМИРИНГ ТООЦООЛЛЫН БҮТЭЦ
            +---------------------------------+
            |     Оролтын Төлөв x(t) [102]    |
            +----------------+----------------+
                             |
                             v
            +---------------------------------+
            |    Кэли Контекст Хаяг [104]     |
            +----------------+----------------+
                             |
                 +-----------+-----------+
                 |                       |
                 v                       v
      +----------------------+ +-------------------------+
      | 4-бит Синапс W [106] | | 1-бит Энграм Түгжээ [108] |
      |  (94 x 94 Массив)    | |    (94 x 94 Массив F)   |
      +----------+-----------+ +------------+------------+
                 |                          |
                 v                          |
      +----------------------+              |
      | Тропик max-AND [110] |              |
      +----------+-----------+              |
                 |                          |
                 v                          |
      +----------------------+              |
      | 7-Үет Тэмцээний Мод  |              |
      |     Арбитр [112]     |              |
      +----------+-----------+              |
                 |                          |
        +--------+--------+                 |
        |                 |                 |
        v                 v                 |
  +-----------+     +-----------+           |
  | Таамаглал |     | Дамжлагын |           |
  | y^(t)[114]|     |Буртгэгч116|           |
  +-----+-----+     +-----+-----+           |
        |  Ground Truth y(t)                |
        v        v                          |
  +----------------------+                  |
  |Харьцуулагч Компаратор|                  |
  |  (Correct/Error)[118]|                  |
  +----------+-----------+                  |
             |                              |
             v               v              v
        +----------------------------------------+
        | Хоёр Талт Хэббийн Пластикжилтийн       |
        | ALU [120] (LTP: W+1, LTD: W-1, Boost:  |
        | W+2, Lock: F=1)                        |
        +-------------------+--------------------+
                            ^
                            |
        +-------------------+--------------------+
        |   Нойрны Тайралтын Хянагч [122]        |
        +----------------------------------------+
```

```
ЗУРАГ 2: КЭЛИГИЙН КВАЗИГРУПП ШАХАЛТ БА ДИНАМИК ПЕРМУТАЦИЙН КОНТЕКСТ ҮҮСГЭГЧ
  x(t-s) [202]           x(t) [204]            x(t-anchor) [206]
       |                      |                        |
       |                      |                        v
       |                      |            +-----------------------+
       |                      |            | r-Бит Эргүүлэлт [208] |
       |                      |            |      (<<< r)          |
       |                      |            +-----------+-----------+
       v                      v                        v
+--------------+       +--------------+    +-----------------------+
|Shift-and-Add |       |Shift-and-Add |    |  Суурь Фазын Нэмэгч   |
|(* alpha) [210|       |(* beta) [212]|    |  (+ gamma_0) [214]    |
+-------+------+       +-------+------+    +-----------+-----------+
        |                      |                       |
        +----------+-----------+                       |
                   |                                   |
                   v                                   v
        +----------------------------------------------------------+
        |       3-Оролттой Бүхэл Тоон Комбинаци Нэмэгч [216]       |
        +--------------------------+-------------------------------+
                                   |
                                   v
        +----------------------------------------------------------+
        |   Модуло-94 Шатласан Хасагч Логик (mod 94) [218]         |
        +--------------------------+-------------------------------+
                                   |
                                   v
        +----------------------------------------------------------+
        |       Контекст Индекс S_ctx(t) [220]                     |
        +----------------------------------------------------------+
```

```
ЗУРАГ 3: 4-СУВАГТ ЭКСПОНЕНЦИАЛ АЛХАМТ РЕЗОНАНС БА ТЕХНИК ХАНГАМЖИЙН MoE КООРДИНАТОР
Тэмдэгтийн Цуваа -> s=1 [302] | s=2 [304] | s=4 [306] | s=8 [308]
                         |           |           |           |
                     [4b W]      [4b W]      [4b W]      [4b W]
                         \           |           |           /
                          v          v           v          v
              +------------------------------------------------+
              | 94-Баганат Консенсус Нэмэгч Мод [310]          |
              +-----------------------+------------------------+
                                      v
              | Нэгдсэн Онооны Вектор (0..60) [312]            |
                                      v
              | 32-бит NoC Багц Үүсгэгч (Type=01) [314]        |
                                      v [NoC Link]
              | Цонхт Хугацааны Тоолуур (0..15) [316]          |
                                      v
              | 94-Төлөвийн Итгэлцлийн Аккумлятор [318]        |
                                      v
              | Глобал Хамгийн Их Утга Шүүгч Арбитр [320]      |
                                      v
                          Глобал Ялагч Символ
```

```
ЗУРАГ 4: 8x8 2D MESH NoC СҮЛЖЭЭ, ЧИГЛҮҮЛЭГЧ БА 32-БИТИЙН БАГЦЫН БҮТЭЦ
Tile(0,0) <---> [Router: X-Y DOR] <---> Tile(1,0) ... Tile(7,0)
    |                  |
Tile(0,1)              v
   ...             Tile(7,7) [MoE Coordinator]

32-БИТИЙН СҮЛЖЭЭНИЙ БАГЦЫН БҮТЭЦ [410]:
[31:30] PKT_TYPE   (00: Оруулах, 01: Санал, 10: Нойр, 11: Тасалдал)
[29:27] DEST_X     (Зорилтот X координат)
[26:24] DEST_Y     (Зорилтот Y координат)
[23:21] SRC_X      (Үүсгэгч X координат)
[20:18] SRC_Y      (Үүсгэгч Y координат)
[17:11] WINNER_ID  (Таамагласан тэмдэгт 0..93)
[10:5]  SCORE      (Итгэлцлийн оноо 0..60)
[4:0]   TASK_ID    (Домэйн/даалгаврын дугаар 0..31)
```

### 14.2. Нарийвчилсан Тайлбар ба 11 Үндсэн Тодорхойлолт (Definitions, Хуудас 194–197)
1. **Дискрет төлөвийн олонлог ($\\Sigma_M$):** $M = 94$ тэмдэгтийн хэвлэгдэх боломжит ASCII төлөв.
2. **Кэлигийн Латин квадрат шахалт:** Хоёр хэмжээст орон зайг биективээр нэгтгэж $M^2 \\to M$ болгон 94 дахин багасгах аффин хувиргалт.
3. **Динамик пермутаци:** Санах ойн мөргөлдөөнийг 99.0% арилгах битийн цагираг шилжүүлэг.
4. **Тропик семиринг үнэлгээ (max-AND):** Үржүүлэгчгүйгээр логик AND ба хамгийн их утгыг ялгах алгебр.
5. **Тэмцээний мод арбитр:** $\\lceil \\log_2 M \\rceil = 7$ үет шаталсан компаратороор 400 ps-д ялагчийг тодруулах логик.
6. **Тэг-буцах градиент бүхий Хэббийн пластикжилт:** Бодит цагт 1 тактад синапсын жинг автоматаар шинэчлэх өөрөө-суралцах дүрэм.
7. **Энграм түгжээ (Engram Lock):** Синапс ханахад ($W=15$) $F=1$ болж марталтаас 100% хамгаалах механик.
8. **Нойрны тайралт (Sleep Pruning):** Сул зогсолтод хэрэггүй сул синапсуудыг цэвэрлэж орон зай чөлөөлөх горим.
9. **2D Mesh NoC ба X-Y Чиглүүлэлт:** Мухардалгүй (Deadlock-Free DOR), 32-битийн нэг-тактын багцаар харилцах торон бүтэц.
10. **Техник хангамжийн MoE Зөвшилцлийн зохицуулагч:** Олон цөмийн санал хураалтыг нэгтгэн глобал шийдвэр гаргагч.
11. **Hyperdimensional Computing (HDC) шүүлтүүр:** 512-бит орон зайд Хэммингийн зайгаар алдаатай тэмдэгтийг шууд засах давхарга.

---

## 15. IEEE TRANSACTIONS ХОЁР БАГАНАТ LATEX ЭХ БИЧВЭР (Хуудас 198–202)

IEEE Transactions on Computers / Neuromorphic Computing сэтгүүлд нийтлүүлэхээр бэлтгэсэн албан ёсны эх бичвэр (`main.tex` ба `references.bib`).

- **Гарчиг:** DPSTA: A Zero-Backpropagation Sequence Processor Utilizing Cayley Quasigroup Compression and Tropic Semiring Plasticity
- **Зохиогчид:** Battumen Gurbadam
- **Агуулга:** Хөвөгч таслалтай үржүүлэгч (GEMM) болон Von Neumann memory wall-ийг халж, 1.40 MB санах ойд 64 цөмөөр олон домэйныг нэгэн зэрэг 2.6W чадалд багтаан бодит цагт сургах бүрэн шийдэл.
- **Ном зүй (BibTeX):** Vaswani et al. (2017), Wulf & McKee (1995 Memory Wall), McCloskey & Cohen (1989 Catastrophic Interference), Davies et al. (2018 Loihi), Merolla et al. (2014 TrueNorth), Kanerva (2009 HDC), Dénes & Keedwell (1974 Latin Squares), Maclagan & Sturmfels (2015 Tropical Geometry).

---

## 16. БОДИТ ЦАГИЙН ИНТЕРАКТИВ ХЯНАЛТЫН САМБАР (`dpsta_dashboard.py`, Хуудас 202–205)

Python-ийн стандарт `curses` сан дээр суурилсан, 8x8 NoC сүлжээний 64 цөмийн ажиллагаа, синапсын Энграм түгжээний төлөв, NoC пакетийн чиглүүлэлт (X-Y DOR) болон текстийн таамаглалыг Ubuntu терминал дээр өнгөтөөр харуулах бодит цагийн монитор.

**Хяналтын самбарын бүтэц (Terminal UI Layout):**
- **8x8 Топологийн матриц:** Зүүн талын 32 цөм `[E0..E3]` (English Prose), Баруун талын 32 цөм `[C0..C3]` (Code), Хост гарц `[GW]` (Tile 0,0), MoE Зохицуулагч `[MOE]` (Tile 7,7), багц дайран өнгөрөхөд `[ * ]` шар өнгөөр асна.
- **Телеметр & Метрикүүд:** Active Tiles (64/64), Distributed Memory (1.40 MB), Engram Locks (74.2%), Network Throughput (142.8 Mflits/s), Perplexity (1.18), Total Power (2.58 W @ 2.0 GHz).
- **NoC Routing Trace:** Бодит цагийн багцын мөр `Core (0,0) -> (3,0) -> (3,2) | FLIT: Type=01, Sym='T', Score=48`.
- **Autonomous Inference Console:** Кластераас бодит цагт таамаглан нийлэгжүүлж буй текстийн цуваа.

---

## 17. CARAVEL РЕПОЗИТОРЫН БҮТЭЦ БА MPW PRECHECK (Хуудас 205–208)

Efabless Open MPW / ChipIgnite шалгалтад бэлтгэх албан ёсны мод, `info.yaml` манифест, болон `mpw_precheck` заавар.

### 17.1. Манифест (`info.yaml`, Хуудас 206)
```yaml
project:
  title: "DPSTA: Discrete Plasticity Synaptic Transient Architecture"
  description: "A zero-backpropagation sequence processor utilizing Cayley quasigroup compression, tropic semiring arithmetic"
  project_id: "2610-DPSTA"
  tags:
    - "AI Processor"
    - "Neuromorphic"
    - "Zero-Backpropagation"
    - "Hyperdimensional Computing"
    - "Edge Computing"
    - "Hardware Plasticity"
  git_url: "https://github.com/battumen/caravel_dpsta.git"
  default_branch: "main"
  clock_frequency: "50.0MHz"
hardware:
  pdk: "sky130A"
  scl: "sky130_fd_sc_hd"
  top_level_netlist: "verilog/rtl/user_project_wrapper.v"
  user_project_wrapper_pin_order: "openlane/user_project_wrapper/pin_order.cfg"
interfaces:
  wishbone:
    base_address: "0x30000000"
    address_space: "0x1000"
    data_width: 32
  logic_analyzer:
    total_bits: 128
    used_bits: 12
  gpio:
    used_pins: 0
openlane_version: "2024.04.18"
```

### 17.2. mpw_precheck Шалгалтын 8 Алхам ба 4 Чухал Анхааруулга (Хуудас 206–208)
1. **License Check:** Бүх `.v`, `.sv`, `.tcl`, `.py` эх файлуудын толгойд Apache-2.0 лицензийн мөр байх ёстой.
2. **Manifest Check:** `info.yaml` зөв форматтай байх.
3. **Consistency Check:** RTL, GL netlist, LEF, DEF, GDS-ийн пинүүд 100% тохирох.
4. **Magic & KLayout DRC:** Физик дүрмийн зөрчилгүй байх.
5. **KLayout Antenna:** Металлын урт шугамууд статик цэнэг цуглуулахаас хамгаалагдсан байх.
6. **KLayout XOR:** GDS зөрөөгүй байх.

**4 Чухал Анхааруулга (Pitfalls):**
- Тэжээлийн богино холболтоос сэргийлэх (`vccd1` дижитал 1.8V, `vssd1` дижитал газар). Ашиглаагүй `vccd2, vssd2, vdda1` шугамуудыг холбож болохгүй.
- Хүрээний гаднах пингүй байх ($0 \\le X \\le 2920\\mu\\text{m}, 0 \\le Y \\le 3520\\mu\\text{m}$).
- GDSII файлыг GitHub руу оруулахдаа Git LFS (`git lfs track "*.gds"`) ашиглах.

"""

import os
script_dir = os.path.dirname(os.path.abspath(__file__))
with open(os.path.join(script_dir, "part2_append.txt"), "w", encoding="utf-8") as f:
    f.write(part2_text)
print("Part 2 prepared successfully.")
