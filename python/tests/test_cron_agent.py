"""
Automated Test Suite for CRON Autonomous AGI Cognitive Reasoner (cron.agent)
Testing:
1. Problem specification & hypothesis expansion
2. 4D Clifford multivector thought state rotation
3. Active Inference free energy minimization loop
4. 4D-Torus deadlock-free dimension-order routing (DOR)
5. Verifiable Solution Certificate generation
"""

import sys
import os

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

import cron
from cron.agent import CronReasoner, ProblemSpec, solve_problem


def test_cron_reasoner_solve():
    print("================================================================================")
    print("[AGI] TESTING CRON AUTONOMOUS AGI COGNITIVE REASONER (PYTHON)")
    print("================================================================================")

    spec = ProblemSpec(
        problem_id="PROB-PY-TOPOLOGY-01",
        title="4D-Torus Deadlock-Free Invariant Proof & Active Hypothesis Selection",
        objective="Identify the optimal cognitive routing strategy that minimizes transmission entropy.",
        initial_axioms=[
            "Axiom 1: 4D-Torus T^4 comprises 256 physical cores indexed as (x,y,z,w) in [0..3]^4.",
            "Axiom 2: Periodic Manhattan distance is bounded by 8 hops maximum."
        ],
        search_hypotheses=[
            "H1: Random walk dimension traversal (non-monotonic)",
            "H2: Dimension-Order Routing X -> Y -> Z -> W (strictly monotonic order)",
            "H3: Pure diagonal teleportation with priority inversion",
            "H4: Unchecked greedy routing with cyclic buffer queues"
        ],
        max_free_energy_threshold=0.35,
        max_mcts_simulations=50,
    )

    reasoner = CronReasoner()
    cert = reasoner.solve(spec)

    print(cert.summary())
    print("\nVerified Proof Steps:")
    for step in cert.proof_trace:
        print(f"  Step #{step.step_index} [Core #{step.executing_core_id}]: {step.rule_applied}")
        print(f"    -> {step.deduction}")

    # Assertions
    assert cert.is_solved, "Problem must be solved!"
    assert cert.final_free_energy < spec.max_free_energy_threshold, "Free energy must be below threshold!"
    assert "Dimension-Order" in cert.optimal_hypothesis, "Optimal hypothesis must be DOR!"
    assert len(cert.torus_communication_path) > 1, "Torus communication path must not be empty!"
    assert len(cert.proof_trace) == 2, "Must contain 2 verified proof steps!"

    print("\n>>> CRON PYTHON AGENT TEST PASSED 100% [GRADE SSS+] <<<")


def test_functional_solve_problem():
    print("\n[TEST] Testing convenience solve_problem() functional API...")
    cert = solve_problem(
        problem_id="QUICK-TEST-02",
        title="Optimal Monotonic Dimension Routing",
        objective="Select the monotonic routing algorithm without cyclic hazards.",
        hypotheses=[
            "Cyclic buffer routing",
            "Monotonic Dimension-Order Routing",
            "Random hop flooding"
        ],
    )
    assert cert.is_solved
    assert "Dimension-Order" in cert.optimal_hypothesis
    print(f"  -> Functional API returned certificate #{cert.certificate_id}: PASSED")


if __name__ == "__main__":
    test_cron_reasoner_solve()
    test_functional_solve_problem()
