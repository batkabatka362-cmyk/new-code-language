# ============================================================================
# CRON Python / NumPy Microcode Transpiled from .cl Microcode
# ============================================================================
import numpy as np

def cron_ternary_dot(a: int, b: int) -> int:
    lo_a = a & 0x55555555
    hi_a = (a >> 1) & 0x55555555
    pos_a = lo_a & ~hi_a
    neg_a = ~lo_a & hi_a
    lo_b = b & 0x55555555
    hi_b = (b >> 1) & 0x55555555
    pos_b = lo_b & ~hi_b
    neg_b = ~lo_b & hi_b
    plus = (pos_a & pos_b) | (neg_a & neg_b)
    minus = (pos_a & neg_b) | (neg_a & pos_b)
    return bin(plus).count('1') - bin(minus).count('1')

def cron_sphere_step(s_id: int, w: int, lat_x: int, lon_z: int, quad: int, val: int, r_arr: list, reg_idx: int, alu_op: int, r_shift: int) -> int:
    cur_wt = ((lat_x + lon_z) >> 5) + 1
    q_sign = 1 if (quad == 1 or quad == 4) else -1
    geo_term = q_sign * int((val * cur_wt) >> r_shift)
    scaled = (val * cur_wt) & 0xFFFFFFFF
    if alu_op == 1:
        r_arr[reg_idx] = (r_arr[reg_idx] + scaled) & 0xFFFFFFFF
    elif alu_op == 2:
        r_arr[reg_idx] = (r_arr[reg_idx] - scaled) & 0xFFFFFFFF
    elif alu_op == 3:
        r_arr[reg_idx] = (r_arr[reg_idx] ^ (scaled << r_shift)) & 0xFFFFFFFF
    elif alu_op == 4:
        r_arr[reg_idx] = (r_arr[reg_idx] ^ scaled) & 0xFFFFFFFF
    elif alu_op == 5:
        r_arr[reg_idx] = scaled & 0xFFFFFFFF
    return geo_term

def execute_cl_kernel(registers=None, pgas_memory=None, sphere_acc=None, core_id: int = 0):
    r = [0] * 16
    s_acc = [0] * 4

    # Bundle 0000
    r[1] = 0
    r[2] = 4
    # Bundle 0001
    r[1] = cron_ternary_dot(r[1], r[1]) & 0xFFFFFFFF
    # Bundle 0002
    # Bundle 0003
    if pgas_memory is not None: pgas_memory[core_id * 16 + 0] = r[1]
    # Bundle 0004
    r[1] = 0
    r[2] = 4
    # Bundle 0005
    r[1] = cron_ternary_dot(r[1], r[1]) & 0xFFFFFFFF
    # Bundle 0006
    # Bundle 0007
    if pgas_memory is not None: pgas_memory[core_id * 16 + 0] = r[1]
    # Bundle 0008
    r[1] = 0
    r[2] = 4
    # Bundle 0009
    r[1] = cron_ternary_dot(r[1], r[1]) & 0xFFFFFFFF
    # Bundle 000A
    # Bundle 000B
    if pgas_memory is not None: pgas_memory[core_id * 16 + 0] = r[1]
    # Bundle 000C
    r[1] = 0
    r[2] = 4
    # Bundle 000D
    r[1] = cron_ternary_dot(r[1], r[1]) & 0xFFFFFFFF
    # Bundle 000E
    # Bundle 000F
    if pgas_memory is not None: pgas_memory[core_id * 16 + 0] = r[1]
    return r, s_acc

    if registers is not None:
        for i in range(16): registers[core_id * 16 + i] = r[i]
    if sphere_acc is not None:
        for s in range(4): sphere_acc[core_id * 4 + s] = s_acc[s]
    return r, s_acc
