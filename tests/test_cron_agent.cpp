#include <iostream>
#include <cassert>
#include <vector>
#include "../include/cron_agent.hpp"

using namespace cron::agent;

int main() {
    std::cout << "====================================================================" << std::endl;
    std::cout << "🧠 TESTING CRON AGI COGNITIVE REASONER & AUTONOMOUS SOLVER" << std::endl;
    std::cout << "====================================================================" << std::endl;

    CognitiveReasoner reasoner;

    // Define a challenging real-world multi-step reasoning problem
    ProblemSpec spec;
    spec.problem_id = "PROB-DOR-TOPOLOGY-01";
    spec.title = "4D-Torus Deadlock-Free Invariant Proof & Active Hypothesis Selection";
    spec.objective = "Identify the optimal cognitive routing strategy that minimizes transmission entropy.";
    spec.initial_axioms = {
        "Axiom 1: 4D-Torus T^4 comprises 256 physical cores indexed as (x,y,z,w) in [0..3]^4.",
        "Axiom 2: Periodic Manhattan distance is bounded by 8 hops maximum."
    };
    spec.search_hypotheses = {
        "H1: Random walk dimension traversal (non-monotonic)",
        "H2: Dimension-Order Routing X -> Y -> Z -> W (strictly monotonic order)",
        "H3: Pure diagonal teleportation with priority inversion",
        "H4: Unchecked greedy routing with cyclic buffer queues"
    };
    spec.max_free_energy_threshold = 0.35f;
    spec.max_mcts_simulations = 50;

    std::cout << "Solving Problem: " << spec.title << "..." << std::endl;
    auto cert = reasoner.solve(spec);

    std::cout << "\n--- SOLUTION CERTIFICATE ---" << std::endl;
    std::cout << "Certificate ID:    " << cert.certificate_id << std::endl;
    std::cout << "Problem ID:        " << cert.problem_id << std::endl;
    std::cout << "Status:            " << (cert.is_solved ? "SOLVED (VERIFIED)" : "UNRESOLVED") << std::endl;
    std::cout << "Final Free Energy: " << cert.final_free_energy << std::endl;
    std::cout << "Optimal Selection: " << cert.optimal_hypothesis << std::endl;
    std::cout << "Execution Time:    " << cert.execution_time_ms << " ms" << std::endl;

    std::cout << "\nCommunication Route across 4D-Torus Cores:" << std::endl;
    std::cout << "  Path: ";
    for (size_t i = 0; i < cert.torus_communication_path.size(); ++i) {
        std::cout << "Core #" << (int)cert.torus_communication_path[i];
        if (i + 1 < cert.torus_communication_path.size()) std::cout << " -> ";
    }
    std::cout << std::endl;

    std::cout << "\nProof Trace Steps (" << cert.proof_trace.size() << " steps):" << std::endl;
    for (const auto& step : cert.proof_trace) {
        std::cout << "  [" << step.step_index << "] Core #" << (int)step.executing_core_id
                  << " | Rule: " << step.rule_applied << std::endl;
        std::cout << "      Deduction: " << step.deduction << std::endl;
    }

    // Assertions for formal correctness
    assert(cert.is_solved);
    assert(cert.final_free_energy < spec.max_free_energy_threshold);
    assert(!cert.torus_communication_path.empty());
    assert(cert.proof_trace.size() >= 2);

    std::cout << "\n🎉 CRON AGI COGNITIVE REASONER TEST PASSED 100%!" << std::endl;
    return 0;
}
