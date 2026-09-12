.MODULE DoubleConsumeBug
.ENTRY _main

_main:
    let lin active_wave: wave_t = pack_wave(amp=[1, 2, 3, 4], phase=[0, 0, 0, 0])
    let first = consume(active_wave)
    let second = consume(active_wave)
.END
