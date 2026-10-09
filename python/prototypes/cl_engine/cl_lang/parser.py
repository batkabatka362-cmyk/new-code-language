"""
================================================================================
CLIFFORD LANGUAGE (CL) PARSER v2.0
================================================================================
"""

import sys
from typing import List, Optional
from cl_lang.lexer import Token, CLLexer, LexerError
from cl_lang.ast import (
    ASTNode, SnakeDecl, BundleDecl, FluidicTransferStmt,
    ResonateStmt, FlowStep, ProgramNode
)

class ParserError(Exception):
    pass

class CLParser:
    def __init__(self, tokens: List[Token]):
        self.tokens = tokens
        self.pos = 0

    def peek(self) -> Token:
        return self.tokens[self.pos]

    def consume(self, expected_type: Optional[str] = None) -> Token:
        tok = self.peek()
        if expected_type and tok.type != expected_type:
            raise ParserError(f"Expected token '{expected_type}', got '{tok.type}' ('{tok.value}') at line {tok.line}:{tok.column}")
        self.pos += 1
        return tok

    def match(self, *types: str) -> bool:
        if self.peek().type in types:
            self.consume()
            return True
        return False

    def parse(self) -> ProgramNode:
        prog_name = "DefaultCliffordProgram"
        bundles = []
        steps = []
        metadata = {}

        if self.peek().type == 'PROGRAM':
            self.consume('PROGRAM')
            prog_name = self.consume('IDENT').value
            self.consume('LBRACE')
            while self.peek().type != 'RBRACE' and self.peek().type != 'EOF':
                self._parse_top_level_item(bundles, steps, metadata)
            self.consume('RBRACE')
        else:
            # Standalone bundles and statements
            while self.peek().type != 'EOF':
                self._parse_top_level_item(bundles, steps, metadata)

        return ProgramNode(name=prog_name, bundles=bundles, steps=steps, metadata=metadata)

    def _parse_top_level_item(self, bundles: List[BundleDecl], steps: List[FlowStep], metadata: dict):
        tok = self.peek()
        if tok.type == 'BUNDLE':
            bundles.append(self.parse_bundle())
        elif tok.type == 'FLOW':
            steps.extend(self.parse_flow())
        elif tok.type == 'STEP':
            steps.append(self.parse_step())
        elif tok.type == 'TRANSFER':
            stmt = self.parse_transfer()
            steps.append(FlowStep(step_id=len(steps), actions=[stmt], line=stmt.line))
        elif tok.type in ('RESONATE', 'ATTEND'):
            stmt = self.parse_resonate()
            steps.append(FlowStep(step_id=len(steps), actions=[stmt], line=stmt.line))
        else:
            raise ParserError(f"Unexpected top-level token '{tok.type}' ('{tok.value}') at line {tok.line}:{tok.column}")

    def parse_bundle(self) -> BundleDecl:
        b_tok = self.consume('BUNDLE')
        name_tok = self.consume('IDENT')
        self.consume('LBRACE')
        
        snakes = []
        while self.peek().type != 'RBRACE' and self.peek().type != 'EOF':
            snakes.append(self.parse_snake_decl())
            self.match('SEMICOLON')

        self.consume('RBRACE')
        return BundleDecl(name=name_tok.value, snakes=snakes, line=b_tok.line)

    def parse_snake_decl(self) -> SnakeDecl:
        # Syntax: S0: WRAP W=10 X=20 Z=30 spin=0 vol=25 [;]
        # or S0: wrap(W=10, X=20, Z=30, spin=0, vol=25)
        s_tok = self.consume('IDENT')
        s_val = s_tok.value.upper()
        if not (s_val.startswith('S') and len(s_val) == 2 and s_val[1] in '0123'):
            raise ParserError(f"Expected snake identifier 'S0'..'S3', got '{s_tok.value}' at line {s_tok.line}")
        snake_id = int(s_val[1])

        self.consume('COLON')

        # Mode
        mode_tok = self.peek()
        if mode_tok.type in ('WRAP', 'TUNNEL', 'FLUID', 'ANCHOR'):
            mode = self.consume().type
        else:
            mode = self.consume('IDENT').value.upper()

        # Check for parentheses format or key=value format
        has_paren = self.match('LPAREN')

        params = {'w': 0, 'x': 0, 'z': 0, 'spin': 0, 'vol': 25}

        while True:
            if has_paren and self.peek().type == 'RPAREN':
                break
            if not has_paren and self.peek().type in ('SEMICOLON', 'RBRACE', 'IDENT', 'EOF'):
                break

            # key = value
            k_tok = self.peek()
            key_name = k_tok.type.lower()
            if key_name not in ('w', 'x', 'z', 'spin', 'vol'):
                key_name = self.consume('IDENT').value.lower()
            else:
                self.consume()

            self.consume('EQUAL')
            val = int(self.consume('NUMBER').value)
            if self.match('PERCENT'):
                pass  # allow 25%

            params[key_name] = val
            self.match('COMMA')

        if has_paren:
            self.consume('RPAREN')

        return SnakeDecl(
            snake_id=snake_id,
            mode=mode,
            w=params['w'],
            x=params['x'],
            z=params['z'],
            spin=params['spin'],
            vol=params['vol'],
            line=s_tok.line
        )

    def parse_flow(self) -> List[FlowStep]:
        self.consume('FLOW')
        self.consume('IDENT')
        self.consume('LBRACE')
        steps = []
        while self.peek().type != 'RBRACE' and self.peek().type != 'EOF':
            steps.append(self.parse_step())
        self.consume('RBRACE')
        return steps

    def parse_step(self) -> FlowStep:
        s_tok = self.consume('STEP')
        self.consume('LBRACE')
        actions = []
        bundle_ref = None

        while self.peek().type != 'RBRACE' and self.peek().type != 'EOF':
            tok = self.peek()
            if tok.type == 'BUNDLE':
                bundle_ref = self.parse_bundle()
            elif tok.type == 'TRANSFER':
                actions.append(self.parse_transfer())
            elif tok.type in ('RESONATE', 'ATTEND'):
                actions.append(self.parse_resonate())
            else:
                raise ParserError(f"Unexpected token '{tok.type}' in step at line {tok.line}")
            self.match('SEMICOLON')

        self.consume('RBRACE')
        return FlowStep(step_id=0, bundle=bundle_ref, actions=actions, line=s_tok.line)

    def parse_transfer(self) -> FluidicTransferStmt:
        t_tok = self.consume('TRANSFER')
        s1_tok = self.consume('IDENT')
        src = int(s1_tok.value.upper().replace('S', ''))
        self.consume('ARROW')
        s2_tok = self.consume('IDENT')
        dst = int(s2_tok.value.upper().replace('S', ''))
        self.consume('COLON')
        amount = int(self.consume('NUMBER').value)
        self.match('PERCENT')
        return FluidicTransferStmt(src=src, dst=dst, amount=amount, line=t_tok.line)

    def parse_resonate(self) -> ResonateStmt:
        tok = self.consume()
        if tok.type == 'ATTEND':
            # attend all or attend(all)
            has_p = self.match('LPAREN')
            self.match('ALL')
            if has_p:
                self.consume('RPAREN')
            return ResonateStmt(target='ALL', line=tok.line)
        else:
            # resonate(S0, S1)
            self.consume('LPAREN')
            s1_tok = self.consume('IDENT')
            src = int(s1_tok.value.upper().replace('S', ''))
            self.consume('COMMA')
            s2_tok = self.consume('IDENT')
            dst = int(s2_tok.value.upper().replace('S', ''))
            self.consume('RPAREN')
            return ResonateStmt(target='PAIR', s1=src, s2=dst, line=tok.line)
