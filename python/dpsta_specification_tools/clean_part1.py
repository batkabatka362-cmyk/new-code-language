# -*- coding: utf-8 -*-
"""
clean_part1.py: Fixes mojibake in part1_append.txt and saves as part1_append_clean.txt
"""
import os
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

def unmojibake_line(s):
    # Only unmojibake if the line contains mojibake characters
    if not any(c in 'ÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞß' or c in cp1252_to_byte for c in s):
        return s
    
    byte_arr = bytearray()
    for ch in s:
        if ch in cp1252_to_byte:
            byte_arr.append(cp1252_to_byte[ch])
        elif ord(ch) < 256:
            byte_arr.append(ord(ch))
        else:
            byte_arr.extend(ch.encode('utf-8'))
    try:
        return byte_arr.decode('utf-8')
    except UnicodeDecodeError:
        return byte_arr.decode('utf-8', errors='replace')

script_dir = os.path.dirname(os.path.abspath(__file__))
in_path = os.path.join(script_dir, "part1_append.txt")
out_path = os.path.join(script_dir, "part1_append_clean.txt")

with open(in_path, "r", encoding="utf-8") as f:
    lines = f.readlines()

clean_lines = [unmojibake_line(line) for line in lines]

with open(out_path, "w", encoding="utf-8") as f:
    f.writelines(clean_lines)

print(f"Cleaned {len(clean_lines)} lines written to {out_path}")
print("Sample cleaned line:", clean_lines[2].strip())
