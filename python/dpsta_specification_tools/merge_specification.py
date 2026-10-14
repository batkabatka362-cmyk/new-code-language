# -*- coding: utf-8 -*-
"""
merge_specification.py: Merges Clean Part 1, Part 2, and Part 3 into DPSTA_TBT_MASTER_SPECIFICATION.md
"""
import os
import sys

sys.stdout.reconfigure(encoding='utf-8')

script_dir = os.path.dirname(os.path.abspath(__file__))
workspace_dir = os.path.dirname(script_dir)
target_file = os.path.join(workspace_dir, "DPSTA_TBT_MASTER_SPECIFICATION.md")

part1_file = os.path.join(script_dir, "part1_append_clean.txt")
part2_file = os.path.join(script_dir, "part2_append.txt")
part3_file = os.path.join(script_dir, "part3_append.txt")

with open(target_file, "r", encoding="utf-8") as f:
    lines = f.readlines()

print(f"Total lines in current target: {len(lines)}")

# We know the base content ends at line 2209 ('\n' right after endmodule\n```\n)
base_content = "".join(lines[:2209])

with open(part1_file, "r", encoding="utf-8") as f:
    part1_content = f.read()

with open(part2_file, "r", encoding="utf-8") as f:
    part2_content = f.read()

with open(part3_file, "r", encoding="utf-8") as f:
    part3_content = f.read()

merged_content = base_content + "\n" + part1_content + "\n" + part2_content + "\n" + part3_content

with open(target_file, "w", encoding="utf-8") as f:
    f.write(merged_content)

print(f"Merge completed successfully! New total size: {len(merged_content)} chars.")
