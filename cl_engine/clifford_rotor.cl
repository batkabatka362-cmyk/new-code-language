# ==============================================================================
# CLIFFORD ROTOR MACHINE CODE (10-10-10-10 Base94 VLIW BUNDLE FORMAT)
# Language: CL (Clifford Language) v1.0
# Spec: 4 tacts / 4 ALUs per bundle, 10 characters per slot, exactly 40 chars
# ==============================================================================

# Bundle B0000: 4-Way Symmetric Balance (T0) - 25% each, All snakes move
B0000:==00#0100}==01#0200|==02#0300/==03#0400\

# Bundle B0001: Snake 3 0-Anchor Tri-Spiral (T1) - 33.3% each, Snake 3 in Spin Mode
B0001:==00#0101}==01#0201|_S0000000P==03#0401\

# Bundle B0002: Dual-Anchor Double Helix Push-Pull (T2) - Snakes 0 and 2 active (50% each)
B0002:==00#0102}_S0000000S==02#0302}_S0000000S

# Bundle B0003: Single Vortex (T3) - Snake 0 active 100%, Snakes 1,2,3 0-Anchored
B0003:==00#0103}_S0000000S_S0000000S_S0000000S

# Bundle B0004: Static Crystal Freeze & Radial Depth Hop (T4) - All 4 snakes 0-Anchored
B0004:_S00+1000P_S00+1000P_S00+1000P_S00+1000P

# Bundle B0005: Kinetic Recovery Vortex (T5) - Snake 1 active 100%
B0005:_S0000000S==01#0205|_S0000000S_S0000000S

# Bundle B0006: Dual Recovery Push-Pull (T6) - Snakes 1 and 3 active (50% each)
B0006:_S0000000S==01#0206|_S0000000S==03#0406\

# Bundle B0007: Tri-Recovery (T7) - Snakes 0, 1, 3 active (33.3% each)
B0007:==00#0107}==01#0207|_S0000000P==03#0407\
