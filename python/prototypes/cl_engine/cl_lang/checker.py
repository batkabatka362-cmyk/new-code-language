"""
================================================================================
CLIFFORD LANGUAGE (CL) TOPOLOGICAL INVARIANT CHECKER v2.0
================================================================================
"""

from typing import List, Tuple
from cl_lang.ast import ProgramNode, BundleDecl, SnakeDecl, FluidicTransferStmt

class InvariantViolation(Exception):
    pass

class CLChecker:
    @staticmethod
    def check_program(prog: ProgramNode) -> List[str]:
        warnings = []
        for bundle in prog.bundles:
            warnings.extend(CLChecker.check_bundle(bundle))
        return warnings

    @staticmethod
    def check_bundle(bundle: BundleDecl) -> List[str]:
        warnings = []
        # 1. 4 могой бүгд тодорхойлогдсон эсэх
        snake_ids = set()
        for s in bundle.snakes:
            if s.snake_id in snake_ids:
                raise InvariantViolation(f"Bundle '{bundle.name}' defines duplicate snake S{s.snake_id} at line {s.line}")
            snake_ids.add(s.snake_id)

        if snake_ids != {0, 1, 2, 3}:
            missing = {0, 1, 2, 3} - snake_ids
            raise InvariantViolation(f"Bundle '{bundle.name}' is missing snakes: {sorted(list(missing))} at line {bundle.line}")

        # 2. Гамильтонианы эрчим хүчний хадгалалт (sum(V) == 100%)
        total_vol = sum(s.vol for s in bundle.snakes)
        if total_vol != 100:
            raise InvariantViolation(
                f"Hamiltonian Violation in bundle '{bundle.name}' (line {bundle.line}): "
                f"Total volume is {total_vol}%, MUST be exactly 100%!"
            )

        # 3. Координатын хязгаар шалгах (0..99)
        for s in bundle.snakes:
            if not (0 <= s.w <= 99):
                raise InvariantViolation(f"Radial Depth W={s.w} out of range [0, 99] for S{s.snake_id} in bundle '{bundle.name}'")
            if not (0 <= s.x <= 99):
                raise InvariantViolation(f"Torus X={s.x} out of range [0, 99] for S{s.snake_id} in bundle '{bundle.name}'")
            if not (0 <= s.z <= 99):
                raise InvariantViolation(f"Torus Z={s.z} out of range [0, 99] for S{s.snake_id} in bundle '{bundle.name}'")
            if s.spin not in (0, 1, 2, 3, 90, 180, 270):
                raise InvariantViolation(f"Spin Phase={s.spin} must be 0, 90, 180, or 270 for S{s.snake_id} in bundle '{bundle.name}'")

        # 4. Брайдын мөргөлдөөний урьдчилсан сэрэмжлүүлэг
        for i in range(4):
            for j in range(i + 1, 4):
                s1 = bundle.snakes[i]
                s2 = bundle.snakes[j]
                if s1.x == s2.x and s1.z == s2.z:
                    if s1.w == s2.w:
                        warnings.append(
                            f"[BRAID WARNING] S{s1.snake_id} and S{s2.snake_id} occupy identical 4D position (W={s1.w}, X={s1.x}, Z={s1.z}) "
                            f"in bundle '{bundle.name}'. Hardware Braid Deflector will activate!"
                        )

        return warnings
