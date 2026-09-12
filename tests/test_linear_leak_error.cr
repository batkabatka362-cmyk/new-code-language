.MODULE BuggyModule
.ENTRY _main

_main:
    let lin forgotten_tensor: wave_t = pack_wave(amp=[1, 2, 3, 4], phase=[0, 0, 0, 0])
    let x = 42
.END
