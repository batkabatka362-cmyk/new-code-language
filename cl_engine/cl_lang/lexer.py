"""
================================================================================
CLIFFORD LANGUAGE (CL) LEXER / TOKENIZER v2.0
================================================================================
"""

import re
from typing import List, NamedTuple

class Token(NamedTuple):
    type: str
    value: str
    line: int
    column: int

class LexerError(Exception):
    pass

class CLLexer:
    TOKEN_SPEC = [
        ('COMMENT_LINE',  r'(//|#)[^\n]*'),
        ('COMMENT_BLOCK', r'/\*[\s\S]*?\*/'),
        ('ARROW',         r'->'),
        ('LBRACE',        r'\{'),
        ('RBRACE',        r'\}'),
        ('LPAREN',        r'\('),
        ('RPAREN',        r'\)'),
        ('LBRACKET',      r'\['),
        ('RBRACKET',      r'\]'),
        ('COLON',         r':'),
        ('SEMICOLON',     r';'),
        ('COMMA',         r','),
        ('EQUAL',         r'='),
        ('PERCENT',       r'%'),
        ('NUMBER',        r'\d+'),
        ('IDENT',         r'[A-Za-z_][A-Za-z0-9_]*'),
        ('NEWLINE',       r'\n'),
        ('SKIP',          r'[ \t\r]+'),
        ('MISMATCH',      r'.'),
    ]

    KEYWORDS = {
        'program', 'manifold', 'bundle', 'snake', 'step', 'flow',
        'transfer', 'resonate', 'attend', 'invariant', 'wrap',
        'tunnel', 'fluid', 'anchor', 'w', 'x', 'z', 'spin', 'vol'
    }

    def __init__(self, code: str):
        self.code = code
        self.regex = re.compile('|'.join(f'(?P<{name}>{pattern})' for name, pattern in self.TOKEN_SPEC))

    def tokenize(self) -> List[Token]:
        tokens = []
        line_num = 1
        line_start = 0

        for mo in self.regex.finditer(self.code):
            kind = mo.lastgroup
            val = mo.group()
            col = mo.start() - line_start + 1

            if kind in ('COMMENT_LINE', 'COMMENT_BLOCK', 'SKIP'):
                continue
            elif kind == 'NEWLINE':
                line_num += 1
                line_start = mo.end()
                continue
            elif kind == 'MISMATCH':
                raise LexerError(f"Unexpected character '{val}' at line {line_num}:{col}")
            elif kind == 'IDENT':
                # Check if keyword (case-insensitive for convenience)
                lower_val = val.lower()
                if lower_val in self.KEYWORDS:
                    kind = lower_val.upper()
                tokens.append(Token(kind, val, line_num, col))
            else:
                tokens.append(Token(kind, val, line_num, col))

        tokens.append(Token('EOF', '', line_num, len(self.code) - line_start + 1))
        return tokens
