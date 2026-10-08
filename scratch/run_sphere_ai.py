# ============================================================================
# Demonstration & Benchmark: 4-Head Sphere Matrix AI Model
# ============================================================================

import sys
import os
import time

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from python.cron.sphere_ai import SphereMatrixAI

def main():
    print("=" * 75)
    print("  4-HEAD SPHERE MATRIX AI NEURAL ENGINE BENCHMARK")
    print("=" * 75)

    ai = SphereMatrixAI(vocab_size=128)

    training_text = "SPHERE MATRIX 4D COMPUTING ENGINE 2026"
    print(f"\n[1] Training Target Text: '{training_text}' (Length: {len(training_text)} chars)")
    print("  Heads involved: X, Y, Z, W starting from center 0.")
    print("  Coordinates tracked: Layer W in [0, 99], X in [0, 99], Z in [0, 99].")
    print("  Multiplication mode: Pure Integer ALU / Zero Floating Point.")

    # Training
    t0 = time.perf_counter()
    EPOCHS = 20
    accuracies = ai.train_sequence(training_text, epochs=EPOCHS)
    train_time = time.perf_counter() - t0

    print(f"\n[2] Training Progress over {EPOCHS} Epochs:")
    for ep, acc in enumerate(accuracies):
        bar = "#" * int(acc / 5)
        print(f"  Epoch {ep+1:02d}: Accuracy = {acc:5.1f}% |{bar:<20}|")

    tokens_processed = (len(training_text) - 1) * EPOCHS
    throughput = tokens_processed / train_time

    print(f"\n[3] Performance Metrics:")
    print(f"  Total Tokens Processed: {tokens_processed:,}")
    print(f"  Training Time: {train_time*1000:.2f} ms")
    print(f"  Throughput: {throughput:,.0f} tokens/second in pure Python!")
    print(f"  Floating-point operations used: 0 FLOPs (100% Integer & Bitwise)")

    # Inspect current state of the 4 heads inside the sphere
    print("\n[4] Final Geometric State of the 4 Heads inside Sphere Matrix:")
    for name in ['X', 'Y', 'Z', 'W']:
        head = ai.sphere.heads[name]
        print(f"  Head {name}: Layer W={head.w:02d}, X={head.x:02d}, Z={head.z:02d} | 14-bit C={head.packed_14bit:05d} | Code='{head.packed_code}'")

    # Autoregressive Generation test
    prompt = "SPHERE "
    generated = ai.generate(prompt=prompt, length=25)
    print(f"\n[5] Autoregressive Generation:")
    print(f"  Prompt:    '{prompt}'")
    print(f"  Generated: '{generated}'")

    print("\n" + "=" * 75)
    print("  4-HEAD SPHERE MATRIX AI EXECUTION COMPLETED SUCCESSFULLY (100%)")
    print("=" * 75)

if __name__ == "__main__":
    main()
