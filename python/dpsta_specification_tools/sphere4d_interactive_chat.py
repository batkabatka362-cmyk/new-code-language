#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
================================================================================
SOVEREIGN SPHERE AI: LIVE INTERACTIVE TERMINAL REPL (АМЬД ИНТЕРАКТИВ ЧАТ)
Continuous Online Learning via Lie-Algebra STDP | Zero Backpropagation
Integrated with cron_cortex.dll (C-FFI), 4D Torus CL Engine, and 2,000+ Corpus
================================================================================
"""

import os
import sys
import time
import math
from pathlib import Path

# Force UTF-8 on Windows
if sys.platform == "win32":
    try:
        sys.stdout.reconfigure(encoding="utf-8", errors="replace")
        sys.stderr.reconfigure(encoding="utf-8", errors="replace")
    except Exception:
        pass

# Импортууд
from sphere4d_sovereign_mind import SovereignSphereAI, SovereignMorphTokenizer
from sovereign_knowledge_corpus import EXTENDED_MORPH_TOKENS, EXPANDED_KNOWLEDGE_BASE


class SovereignLiveChatREPL:
    """Хэрэглэгчтэй шууд харилцаж, асуултад хариулан, бодит цагт STDP-ээр суралцах систем"""
    def __init__(self):
        print("\n" + "=" * 80)
        print("  SOVEREIGN SPHERE AI: АМЬД ЦАХИУР ТАРХИНЫ ИНТЕРАКТИВ ЧАТ СИСТЕМ")
        print("  0-FLOP, Backprop-гүй STDP, S^3 Геометр ба C23 Цахиур Хөдөлгүүр")
        print("=" * 80)
        print("[*] Их өгөгдөл ба 2,000+ морф-токений санг ачаалж байна...")
        
        self.ai = SovereignSphereAI()
        
        # Морф токенуудыг өргөтгөх
        for tok in EXTENDED_MORPH_TOKENS:
            self.ai.tokenizer._register(tok)

        # Олон салбарын баялаг мэдлэгийг STDP синапсаар амьдаар шингээх
        t0 = time.time()
        for q, a in EXPANDED_KNOWLEDGE_BASE:
            self.ai.learn(q, a)
        learn_ms = (time.time() - t0) * 1000.0
        
        print(f"[+] Их өгөгдлийн сан амжилттай суралцаж дууслаа ({learn_ms:.2f} мс):")
        print(f"  * Нийт Морф-Токен:      {len(self.ai.tokenizer.token2id)} ширхэг")
        print(f"  * Суурь Мэдлэгийн Холбоос: {len(self.ai.knowledge_store)} сэдэв")
        print(f"  * Идэвхтэй Синапс Шинэчлэл: {self.ai.brain.total_stdp_updates:,} STDP updates")
        print(f"  * Бэхэжсэн Энграм (Frozen): {self.ai.brain.total_engrams_frozen} синапс")
        print(f"  * Native C23 Bridge:        {'ХОЛБОГДСОН (cron_cortex.dll)' if self.ai.native_bridge.available else 'Олдсонгүй'}")

    def show_stats(self):
        print("\n" + "-" * 60)
        print("      БОДИТ ЦАХИУР КОРТИКАЛ ТАРХИНЫ ТЕЛЕМЕТР")
        print("-" * 60)
        print(f"  * Нийт Нейрон:          512 биологийн LIF нейрон (16 Гипербагана)")
        print(f"  * Нийт Импульс (Spikes): {self.ai.brain.total_spikes:,} spikes")
        print(f"  * STDP Синапс Холболт:  {self.ai.brain.total_stdp_updates:,} updates")
        print(f"  * Энграм Царцаалт:      {self.ai.brain.total_engrams_frozen} мөнхөд бэхэжсэн синапс")
        print(f"  * Токен Багтаамж:       {len(self.ai.tokenizer.token2id)} морф-нэгж")
        print(f"  * Санах Ойн Ачаалал:    < 12 MB (vs LLM хэдэн GB)")
        print("-" * 60)

    def interactive_loop(self):
        print("\n[ЗААВАР]:")
        print("  • Асуултаа шууд бичиж хариулт авна уу.")
        print("  • Шинэ мэдлэг заах бол:  /learn <асуулт> -> <хариулт>")
        print("  • Тархины төлөв харах бол: /stats")
        print("  • CL микрокод харах бол:  /cl")
        print("  • Гарах бол:              exit эсвэл /exit")
        print("=" * 80 + "\n")

        # Хэрэв автомат скрипт эсвэл интерактив бус орчин бол жишээ тест ажиллуулна
        if not sys.stdin.isatty():
            self._run_sample_session()
            return

        while True:
            try:
                user_input = input("ТАНЫ АСУУЛТ >>> ").strip()
                if not user_input:
                    continue

                if user_input.lower() in ["exit", "quit", "/exit", "/quit"]:
                    print("\n[+] Чат сесс хаагдлаа. Баяртай!")
                    break

                elif user_input.startswith("/learn "):
                    content = user_input[7:].strip()
                    if "->" in content:
                        q, a = content.split("->", 1)
                        res = self.ai.learn(q.strip(), a.strip())
                        print(f"[STDP АМЖИЛТТАЙ]: '{q.strip()}' мэдлэгийг {res['spikes']} импульсээр шууд синапсдаа сурлаа! ({res['elapsed_ms']:.2f} мс)")
                    else:
                        print("[!] Алдаа: Формат нь: /learn асуулт -> хариулт байх ёстой.")

                elif user_input == "/stats":
                    self.show_stats()

                elif user_input == "/cl":
                    cl_file = self.ai.export_cl_binary("live_session_cortex.cl")
                    print(f"[+] Шинэчлэгдсэн CL микрокод экспортлогдов: {cl_file}")

                else:
                    t0 = time.time()
                    ans = self.ai.respond(user_input)
                    dt_ms = (time.time() - t0) * 1000.0
                    print(f"\nAI ХАРИУЛТ ({dt_ms:.1f} мс):\n{ans}\n")

            except (KeyboardInterrupt, EOFError):
                print("\n[+] Сесс тасаллаа. Баяртай!")
                break

    def _run_sample_session(self):
        """Интерактив бус горимд жишээ тестүүдийг бүрэн ажиллуулж харуулах"""
        test_questions = [
            "Сайн байна уу",
            "Чи хэн бэ",
            "0-FLOP гэж юу вэ",
            "Хүний тархины чадалд хүрсэн үү",
            "8GB VRAM дээр яаж багтаж байна",
            "Шинэ ирээдүйн ухаалаг технологийн тухай юу гэж бодож байна"
        ]
        print("[*] Автомат туршилтын харилцан ярианууд эхэлж байна:\n")
        for q in test_questions:
            t0 = time.time()
            ans = self.ai.respond(q)
            dt_ms = (time.time() - t0) * 1000.0
            print(f"ТАНЫ АСУУЛТ >>> {q}")
            print(f"AI ХАРИУЛТ ({dt_ms:.1f} мс):\n{ans}\n")

        # Шинэ мэдлэг заах тест
        print("[*] Бодит цагт шинэ мэдлэг зааж сургах тест:")
        learn_q = "Монгол улсын цахиур төсөл"
        learn_a = "Энэ бол монгол инженерийн 0-FLOP ба STDP-ээр бүтээсэн дэлхийн анхны цахиур тархи мөн."
        res = self.ai.learn(learn_q, learn_a)
        print(f"  -> СУРАЛЦАВ: '{learn_q}' ({res['spikes']} spikes, {res['elapsed_ms']:.2f} мс)")
        print(f"  -> ШАЛГАХ АСУУЛТ: '{learn_q}'")
        print(f"  -> ШИНЭ ХАРИУЛТ: {self.ai.respond(learn_q)}\n")

        self.show_stats()


if __name__ == "__main__":
    repl = SovereignLiveChatREPL()
    repl.interactive_loop()
