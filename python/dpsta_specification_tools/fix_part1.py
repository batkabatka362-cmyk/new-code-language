# -*- coding: utf-8 -*-
import sys
sys.stdout.reconfigure(encoding='utf-8')

cp1252_to_byte = {
    '\u20AC': 0x80, '\x81': 0x81, '\u201A': 0x82, '\u0192': 0x83,
    '\u201E': 0x84, '\u2026': 0x85, '\u2020': 0x86, '\u2021': 0x87,
    '\u02C6': 0x88, '\u2030': 0x89, '\u0160': 0x8A, '\u2039': 0x8B,
    '\u0152': 0x8C, '\x8D': 0x8D, '\u017D': 0x8E, '\x8F': 0x8F,
    '\x90': 0x90, '\u2018': 0x91, '\u2019': 0x92, '\u201C': 0x93,
    '\u201D': 0x94, '\u2022': 0x95, '\u2013': 0x96, '\u2014': 0x97,
    '\u02DC': 0x98, '\u2122': 0x99, '\u0161': 0x9A, '\u203A': 0x9B,
    '\u0153': 0x9C, '\x9D': 0x9D, '\u017E': 0x9E, '\u0178': 0x9F
}

def unmojibake(s):
    byte_arr = bytearray()
    for ch in s:
        if ch in cp1252_to_byte:
            byte_arr.append(cp1252_to_byte[ch])
        elif ord(ch) < 256:
            byte_arr.append(ord(ch))
        else:
            # If not in table, keep as utf-8 bytes
            byte_arr.extend(ch.encode('utf-8'))
    return byte_arr.decode('utf-8', errors='replace')

test_line = '### 9.4. HDC Pipeline Testbench Ð±Ð° Ð¢ÑƒÑ€ÑˆÐ¸Ð»Ñ‚Ñ‹Ð½ Ò®Ñ€ Ð”Ò¯Ð½ (`tb_hdc_pipeline.v`, Ð¥ÑƒÑƒÐ´Ð°Ñ\x81 161â€“164)\n'
print("Original:", repr(test_line))
print("Fixed:   ", repr(unmojibake(test_line)))
print("Display: ", unmojibake(test_line).strip())
