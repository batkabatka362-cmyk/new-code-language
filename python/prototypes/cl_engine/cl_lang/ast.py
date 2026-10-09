"""
================================================================================
CLIFFORD LANGUAGE (CL) AST (ABSTRACT SYNTAX TREE) DEFINITIONS v2.0
================================================================================
CL хэлний топологийн бүтэц:
  - Уламжлалт von Neumann хувьсагч, стек, заагч БАЙХГҮЙ.
  - Бүх төлөв 4D Торус дээрх 4 могой (Snakes S0..S3), Спин фаз, Гидравлик даралт.
================================================================================
"""

from typing import List, Optional, Dict, Any

class ASTNode:
    pass

class SnakeDecl(ASTNode):
    """
    Могойн төлөв зарлах:
    S0: mode(W=10, X=20, Z=30, spin=0, vol=25)
    """
    def __init__(self, snake_id: int, mode: str, w: int, x: int, z: int, spin: int, vol: int, line: int = 0):
        self.snake_id = snake_id
        self.mode = mode.upper()  # WRAP, TUNNEL, FLUID, ANCHOR
        self.w = w
        self.x = x
        self.z = z
        self.spin = spin          # 0, 90, 180, 270
        self.vol = vol            # 0..100%
        self.line = line

    def __repr__(self):
        return f"SnakeDecl(S{self.snake_id}, {self.mode}, W={self.w}, X={self.x}, Z={self.z}, spin={self.spin}, vol={self.vol}%)"

class BundleDecl(ASTNode):
    """
    1 тактын 4-АЛУ зэрэгцээ VLIW багц:
    bundle B0 { S0: ... S1: ... S2: ... S3: ... }
    """
    def __init__(self, name: str, snakes: List[SnakeDecl], line: int = 0):
        self.name = name
        self.snakes = snakes
        self.line = line

    def __repr__(self):
        return f"BundleDecl({self.name}, snakes=4)"

class FluidicTransferStmt(ASTNode):
    """
    Топологийн Гидравлик шилжүүлэг:
    transfer S0 -> S1: 10%
    """
    def __init__(self, src: int, dst: int, amount: int, line: int = 0):
        self.src = src
        self.dst = dst
        self.amount = amount
        self.line = line

    def __repr__(self):
        return f"FluidicTransfer(S{self.src} -> S{self.dst}, delta={self.amount}%)"

class ResonateStmt(ASTNode):
    """
    Долгионы Интерференц Анхаарал тооцох үйлдэл:
    resonate(S0, S1) эсвэл attend(all)
    """
    def __init__(self, target: str, s1: Optional[int] = None, s2: Optional[int] = None, line: int = 0):
        self.target = target  # 'ALL' or 'PAIR'
        self.s1 = s1
        self.s2 = s2
        self.line = line

    def __repr__(self):
        return f"Resonate({self.target}, S{self.s1}, S{self.s2})"

class FlowStep(ASTNode):
    """
    Траекторын нэг алхам/такт:
    step { bundle B0; transfer ...; resonate ...; }
    """
    def __init__(self, step_id: int, bundle: Optional[BundleDecl] = None, actions: Optional[List[ASTNode]] = None, line: int = 0):
        self.step_id = step_id
        self.bundle = bundle
        self.actions = actions or []
        self.line = line

    def __repr__(self):
        return f"FlowStep(step={self.step_id}, actions={len(self.actions)})"

class ProgramNode(ASTNode):
    """
    CL Програмын үндсэн эх:
    program MyTopology { ... }
    """
    def __init__(self, name: str, bundles: List[BundleDecl], steps: List[FlowStep], metadata: Dict[str, Any]):
        self.name = name
        self.bundles = bundles
        self.steps = steps
        self.metadata = metadata

    def __repr__(self):
        return f"ProgramNode({self.name}, bundles={len(self.bundles)}, steps={len(self.steps)})"
