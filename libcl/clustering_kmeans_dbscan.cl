@⌖ [4×4×4×4] : ⮑ (x+1, y, z, w)

@io {
    points:     X[16×16:FP32]           @sram[0x0000]
    centroids:  C[16×16:FP32]           @sram[0x0200]
    clusters:   Assignments[16×16:FP32] @sram[0x0400]
}

§ClusterMetricBlock[16×16:FP32] {
    EpsilonRadius: T[16×16:FP32], MinPtsDensity: T[16×16:FP32]

    X ↦ Assignments :
        DistMatrix := X ⊗ C
        Norm_D := N(DistMatrix)
        CorePoints := silu(Norm_D)
        Assignments := CorePoints * X
}

§ControlFlow {
    ⟳ (256) {
        #R1 ← @sram[0x0000]
        #R2 ← @sram[0x0200]
        #R3 := #R1 * #R2
        #R4 := #R3 + #R1
        @sram[0x0400] ← #R4
    }
}

!Δ { |Assignments| < 1e5, cluster_inertia < 1e3 }
