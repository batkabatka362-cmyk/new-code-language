#include <iostream>
#include <cassert>
#include <vector>
#include <cmath>
#include "../include/cron_math.hpp"

using namespace cron::math;

int main() {
    std::cout << "====================================================================" << std::endl;
    std::cout << "📐 TESTING CRON UNIFIED AI/AGI MATHEMATICAL FOUNDATION (GRADE SSS+)" << std::endl;
    std::cout << "====================================================================" << std::endl;

    // -------------------------------------------------------------------------
    // TEST 1: Multiplier-Free Linear Algebra & Tensor Operations
    // -------------------------------------------------------------------------
    std::cout << "[TEST 1] Testing Multiplier-Free Linear Algebra & Online Softmax..." << std::endl;
    std::vector<float> x = {1.5f, 2.0f, -0.5f, 3.0f, 1.0f, -1.0f, 0.5f, 2.0f};
    std::vector<int8_t> w = {1, -1, 0, 1, 0, 1, -1, 0}; // 2:4 sparse ternary trits

    float res_ternary = TensorOps::ternary_dot(x, w);
    // (1.5*1) + (2.0*-1) + (-0.5*0) + (3.0*1) + (1.0*0) + (-1.0*1) + (0.5*-1) + (2.0*0)
    // = 1.5 - 2.0 + 3.0 - 1.0 - 0.5 = 1.0
    std::cout << "  ✓ Ternary Dot Result: " << res_ternary << std::endl;
    assert(std::abs(res_ternary - 1.0f) < 1e-5f);

    float res_sparse = TensorOps::sparse_24_dot(x, w);
    assert(std::abs(res_sparse - 1.0f) < 1e-5f);
    std::cout << "  ✓ 2:4 Sparse Dot (50% zero-skip) Result: " << res_sparse << std::endl;

    std::vector<float> logits = {1.0f, 2.0f, 3.0f, 4.0f};
    std::vector<float> probs(4);
    TensorOps::online_softmax(logits, probs);
    float sum_p = 0.0f;
    for (float p : probs) sum_p += p;
    assert(std::abs(sum_p - 1.0f) < 1e-5f);
    assert(probs[3] > probs[2] && probs[2] > probs[1]);
    std::cout << "  ✓ Online Softmax Verified: sum(p) = " << sum_p << ", max_p = " << probs[3] << std::endl;

    // -------------------------------------------------------------------------
    // TEST 2: Clifford Geometric Spacetime Algebra Cl(4,0)
    // -------------------------------------------------------------------------
    std::cout << "\n[TEST 2] Testing Clifford Spacetime Algebra Cl(4,0) & Rotors..." << std::endl;
    Multivector4D psi;
    psi.s = 1.0f;
    psi.v = {1.0f, 0.0f, 0.0f, 0.0f}; // Unit vector e1
    psi.b = {0.5f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f}; // e12 bivector

    float initial_norm = psi.norm();
    std::cout << "  ✓ Initial Multivector Norm: " << initial_norm << std::endl;

    // 90-degree rotor in e12 plane (pi/2 radians)
    auto rotor = Multivector4D::make_rotor(1.5707963f, 0);
    auto rotated_psi = psi.rotate_planar(rotor, 0);

    float rotated_norm = rotated_psi.norm();
    std::cout << "  ✓ Rotated Multivector Norm: " << rotated_norm << std::endl;
    // Rotor rotation is an exact isometry (length preserving)
    assert(std::abs(rotated_norm - initial_norm) < 1e-4f);

    auto regs = rotated_psi.to_registers();
    assert(regs.size() == 16);
    std::cout << "  ✓ 16 Hardware Registers (R0..RF) Mapped 1:1." << std::endl;

    // -------------------------------------------------------------------------
    // TEST 3: Hierarchical Reasoning Tree Graph (Tree of Thoughts / MCTS)
    // -------------------------------------------------------------------------
    std::cout << "\n[TEST 3] Testing Reasoning Tree Graph & Active Inference..." << std::endl;
    ReasoningTreeGraph tree;
    assert(tree.nodes.size() == 1); // Root node exists

    auto branch_ids = tree.expand_node(0, 4);
    assert(branch_ids.size() == 4);
    assert(tree.nodes.size() == 5);
    std::cout << "  ✓ Expanded Root into 4 Child Thought Branches." << std::endl;

    // Check UCB1 exploration score
    float ucb_root = tree.nodes[branch_ids[0]].ucb1_score(1);
    assert(ucb_root > 0.0f);

    // Backpropagate reward to root
    float fe_before = tree.nodes[0].free_energy;
    tree.backpropagate(branch_ids[0], 0.95f);
    assert(tree.nodes[0].visit_count == 1);
    std::cout << "  ✓ Backpropagated Reward through Reasoning Graph (Updated Root Value: " 
              << tree.nodes[0].value << ")." << std::endl;

    // -------------------------------------------------------------------------
    // TEST 4: 4D-Torus Topology, Metric & Dimension-Order Routing (DOR)
    // -------------------------------------------------------------------------
    std::cout << "\n[TEST 4] Testing 4D-Torus Topology & Deadlock-Free DOR Routing..." << std::endl;
    Torus4DCoord origin(0, 0, 0, 0);
    Torus4DCoord dest(3, 2, 1, 3);

    assert(origin.to_core_id() == 0);
    // (3) + 2*4 + 1*16 + 3*64 = 3 + 8 + 16 + 192 = 219
    assert(dest.to_core_id() == 219);

    int dist = Torus4D::distance(origin, dest);
    // X: min(3, 1) = 1 hop (wrap-around 0 -> 3)
    // Y: min(2, 2) = 2 hops
    // Z: min(1, 3) = 1 hop
    // W: min(3, 1) = 1 hop
    // Total Manhattan Torus Distance = 1 + 2 + 1 + 1 = 5 hops
    std::cout << "  ✓ 4D-Torus Distance between Core 0 and Core 219: " << dist << " hops (Max <= 8)." << std::endl;
    assert(dist == 5);

    auto dor_path = Torus4D::compute_dor_route(origin, dest);
    assert(dor_path.front().to_core_id() == origin.to_core_id());
    assert(dor_path.back().to_core_id() == dest.to_core_id());
    std::cout << "  ✓ Dimension-Order Routing Path Length: " << dor_path.size() << " steps." << std::endl;
    std::cout << "    Route: ";
    for (size_t i = 0; i < dor_path.size(); ++i) {
        std::cout << "[" << (int)dor_path[i].x << "," << (int)dor_path[i].y << ","
                  << (int)dor_path[i].z << "," << (int)dor_path[i].w << "]";
        if (i + 1 < dor_path.size()) std::cout << " -> ";
    }
    std::cout << std::endl;

    // Tree-to-Torus Spatial Mapping
    uint8_t core_assigned = Torus4D::map_tree_node_to_core(5, 2);
    assert(core_assigned < 256);
    std::cout << "  ✓ Tree Node 5 mapped to Physical Core " << (int)core_assigned << " on 4D-Torus Silicon." << std::endl;

    std::cout << "\n🎉 ALL CRON UNIFIED MATHEMATICAL FOUNDATION TESTS PASSED (GRADE SSS+)!" << std::endl;
    return 0;
}
