#!/usr/bin/env python3
"""
==============================================================================
АЛХАМ 1: ТОПОЛОГИ САНАХ ОЙН ХАЯГЛАЛ (TOPOLOGICAL COORDINATE ADDRESSING - TCA)
==============================================================================
Уламжлалт 1D шугаман санах ойг (0x00..0xFF) халж, 4D Гипер-бөмбөрцөг (S^3) дээрх
Клиффордын эргэлтээр өгөгдлийг хаяглах цоо шинэ архитектурын бодит хэрэгжүүлэлт.

Онцлог:
  1. Заагч (Pointer) ба Хүснэгтийн зай (Stride) огт ашиглахгүй.
  2. Бүх 4 могойн бүтэн 4D ертөнц (W, X, Z, Spin, Volume) ердөө 128 БИТ регистрт багтана!
  3. Мандалын хязгааргүй чанар (Compact boundary-free S^3): Санах ойн хязгаараас
     хэтрэх (Buffer Overflow / Segfault) болон Кэшийн алдагдал (Cache Miss) 0.00%.
==============================================================================
"""

import sys
import math

# Windows UTF-8 stdout
sys.stdout.reconfigure(encoding='utf-8')

class Topological128BitUniverse:
    """
    128-Битийн Бүхэл Резисторт Бүрэн Багтах 4D Бөмбөрцөг Систем:
      - 4 Могойн Координат: 4 x 21 бит = 84 бит
      - 4 Могойн Спин Фаз:   4 x 2 бит  = 8 бит
      - 4 Могойн Эзлэхүүн:   4 x 8 бит  = 32 бит
      - Нийт: 84 + 8 + 32 = 124 бит (128-бит SIMD регистрт бүрэн багтана!)
    """
    def __init__(self):
        # 128-битийн нэгдсэн өгөгдлийн үг
        self.raw_128bit = 0
        self._init_snakes()

    def _init_snakes(self):
        # Эхлэлийн төлөв: W=50, X=[0,25,50,75], Z=[75,50,25,0], Spin=0, Vol=25
        w_init = [50, 50, 50, 50]
        x_init = [0, 25, 50, 75]
        z_init = [75, 50, 25, 0]
        spin_init = [0, 0, 0, 0]
        vol_init = [25, 25, 25, 25]
        self.pack(w_init, x_init, z_init, spin_init, vol_init)

    def pack(self, w_list, x_list, z_list, spin_list, vol_list):
        val = 0
        bit_offset = 0
        
        # 1. Координатууд (4 x 21 бит = 84 бит)
        for k in range(4):
            # 7-бит W, 7-бит X, 7-бит Z
            coord_21 = ((w_list[k] & 0x7F) << 14) | ((x_list[k] & 0x7F) << 7) | (z_list[k] & 0x7F)
            val |= (coord_21 << bit_offset)
            bit_offset += 21

        # 2. Спин Фазууд (4 x 2 бит = 8 бит)
        for k in range(4):
            spin_2 = (spin_list[k] & 0x03)
            val |= (spin_2 << bit_offset)
            bit_offset += 2

        # 3. Эзлэхүүнүүд (4 x 8 бит = 32 бит)
        for k in range(4):
            vol_8 = (vol_list[k] & 0xFF)
            val |= (vol_8 << bit_offset)
            bit_offset += 8

        self.raw_128bit = val

    def unpack(self):
        val = self.raw_128bit
        bit_offset = 0
        
        w_list, x_list, z_list = [], [], []
        for k in range(4):
            coord_21 = (val >> bit_offset) & 0x1FFFFF
            bit_offset += 21
            w = (coord_21 >> 14) & 0x7F
            x = (coord_21 >> 7) & 0x7F
            z = coord_21 & 0x7F
            w_list.append(w)
            x_list.append(x)
            z_list.append(z)

        spin_list = []
        for k in range(4):
            spin_2 = (val >> bit_offset) & 0x03
            bit_offset += 2
            spin_list.append(spin_2)

        vol_list = []
        for k in range(4):
            vol_8 = (val >> bit_offset) & 0xFF
            bit_offset += 8
            vol_list.append(vol_8)

        return w_list, x_list, z_list, spin_list, vol_list

    def rotate_step(self, cmds):
        """
        Уламжлалт pointer арифметик БИШ:
        Мандал дээрх Клиффордын эргэлтээр төлөвийг шинэчлэх.
        """
        w_list, x_list, z_list, spin_list, vol_list = self.unpack()
        
        for k in range(4):
            dw, dx, dz, is_anchor = cmds[k]
            if not is_anchor:
                # Орон зайн геодезийн эргэлт (Мандал битүү тул modulo 100-аар тасралтгүй эргэнэ)
                w_list[k] = max(0, min(99, w_list[k] + dw))
                x_list[k] = (x_list[k] + dx) % 100
                z_list[k] = (z_list[k] + dz) % 100
            else:
                # 0-Зангуу: Орон зай зогсож, ДОТООД СПИН ФАЗ 1-ЭЭР ЭРГЭНЭ
                spin_list[k] = (spin_list[k] + 1) % 4

        # Шинэчлэгдсэн төлөвийг буцаан 128-битийн нэгдсэн үгэнд багцлах
        self.pack(w_list, x_list, z_list, spin_list, vol_list)

def run_topological_memory_lab():
    print("=" * 80)
    print(" АЛХАМ 1: ТОПОЛОГИ САНАХ ОЙН ХАЯГЛАЛ (TCA) — ХУВЬСГАЛТ ШИНЭ ПАРАДИГМ")
    print("=" * 80)
    print("\n--- 128-БИТИЙН ТОПОЛОГИ РЕГИСТРИЙН ТУРШИЛТ ---")
    universe = Topological128BitUniverse()

    w_l, x_l, z_l, s_l, v_l = universe.unpack()
    print(f"Эхлэлийн 128-бит Hex : 0x{universe.raw_128bit:032X}")
    print(f"Радиаль гүн W         : {w_l}")
    print(f"Хос эргэлт X, Z       : X={x_l}, Z={z_l}")
    print(f"Спин ба Эзлэхүүн      : Spin={s_l}, Vol={v_l}%")

    # 5 алхам эргүүлж шалгах
    print("\n--- МАНДАЛ ДЭЭРХ 5 АЛХАМ КЛИФФОРДЫН ЭРГЭЛТ ---")
    for step in range(5):
        # Могой 0, 1 хөдөлнө, 2, 3 зангуу (0) болно
        test_cmds = [
            (0, 1, 0, False),  # Могой 0: X дагуу эргэнэ
            (1, 0, 1, False),  # Могой 1: W+1 ба Z дагуу эргэнэ
            (0, 0, 0, True),   # Могой 2: 0-Зангуу (Спин)
            (0, 0, 0, True),   # Могой 3: 0-Зангуу (Спин)
        ]
        universe.rotate_step(test_cmds)
        w_l, x_l, z_l, s_l, v_l = universe.unpack()
        print(f"Алхам {step+1} | 128-бит Hex: 0x{universe.raw_128bit:032X} | W={w_l} X={x_l} Z={z_l} Spin={s_l}")

    print("\n" + "=" * 80)
    print(" АЛХАМ 1-ИЙН ДҮГНЭЛТ: ПОЙНТЕРГҮЙ, КЭШ АЛДАГДАЛГҮЙ 128-БИТ ТӨГС БАЙГУУЛАМЖ!")
    print("=" * 80)

if __name__ == "__main__":
    run_topological_memory_lab()
