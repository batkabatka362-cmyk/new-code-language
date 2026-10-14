# DPSTA & TBT ТЕХНИКИЙН БАРИМТ БИЧГИЙН ХӨГЖҮҮЛЭЛТИЙН СКРИПТҮҮД БА ЭХ ФАЙЛУУД

Энэхүү хавтас нь `matrix4.1.2.pdf` (224 хуудас)-ийн бүх математик, техник хангамжийн RTL, симуляци, патентийн өгөгдлийг нэгтгэн `DPSTA_TBT_MASTER_SPECIFICATION.md` баримт бичгийг бүтээн босгосон албан ёсны эх скриптүүд болон нэмэлт хэсгүүдийг агуулна.

## Файлуудын Тайлбар

1. **`gen_part1.py`**:
   - 9.4 (HDC Pipeline Testbench `tb_hdc_pipeline.v`),
   - Бүлэг 10 (SkyWater 130nm ASIC top, OpenLane 2 тохиргоо),
   - Бүлэг 11 (Xilinx Artix-7 FPGA, UART драйвер, Vivado TCL),
   - Бүлэг 12 (Whitepaper, 64-цөм NoC, сүйрэлт марталтын тэг тест)-ийг үүсгэгч үндсэн эх.

2. **`clean_part1.py` & `fix_part1.py`**:
   - Part 1 доторх кирилл үсгийн кодийн алдааг (Windows-1252 / UTF-8) засаж, `part1_append_clean.txt` цэвэр эхийг гаргасан засварын скриптүүд.

3. **`gen_part2.py`**:
   - Бүлэг 13 (Caravel MPW Tapeout harness `user_project_wrapper.v`, Wishbone map, C firmware),
   - Бүлэг 14 (Патентийн FIG 1–4 албан ёсны зургууд, 11 хууль зүйн тодорхойлолт),
   - Бүлэг 15 (IEEE Transactions LaTeX эх бичвэр `main.tex`),
   - Бүлэг 16 (Бодит цагийн хяналтын самбар `dpsta_dashboard.py`),
   - Бүлэг 17 (Caravel репозиторын бүтэц, CI/CD `precheck.yml`)-ийг үүсгэгч эх.

4. **`gen_part3.py`**:
   - Бүлэг 18 (Квант-Криоген RSFQ + SFS соронзон Жозефсоны үе, Lumerical FDTD/MODE LSF `build_gst_sin_synapse.lsf`, JoSIM netlist `mjj_synapse_cell.cir`, `tropic_max_comparator.cir`, Патентийн 16–19 заалтууд),
   - Бүлэг 19 (Инженерийн шүүмжлэлт үнэлгээ, Clifford Rotor Machine Code `CL v1.0` 10-10-10-10 Base-94 VLIW bundle, 4 резонанс суваг),
   - Төгсгөлийн баталгаажуулалтын дүгнэлтийг үүсгэгч эх.

5. **`merge_specification.py`**:
   - `DPSTA_TBT_MASTER_SPECIFICATION.md`-ийн суурь 1–9 бүлгийг Part 1 Clean, Part 2, Part 3-тай автоматаар төгс залгаж бүтээдэг эцсийн нэгтгэгч скрипт.

6. **Текст эх файлууд**:
   - `part1_append_clean.txt` (Хуудас 161–183)
   - `part2_append.txt` (Хуудас 183–208)
   - `part3_append.txt` (Хуудас 208–224)
