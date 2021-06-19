import importlib
import os
import sys
sys.path.append(os.path.abspath(__package__)[:-4])
from .cpe import *

def test_2():
    md = importlib.import_module('src.modules.compiler')
    obj = md.compiler()
    tests = [['int', 'A{(1)(100)}', 'B{(100)(200)/(150)/(180)}'],
             ['char', 'C{(64)(100)}'],
             ['float', 'D{(13.4)(490.23)}'],
             ['int', 'E(1)(100)'],
             ['int', 'F{(100)}'],
             ['hi', 'G{(100)(200)}'],
             ['int', 'H[(1)(2)]'],
             ['float', 'I'],
             ['int', 'A{(2)(300)}']]
    ret = []
    for test in tests:
        obj.process_single_type(test)
        ret.append(obj.ret['error'])
        obj.ret['error'] = None
    ret.append(obj.var)
    ans = [None, None, None, 'Sytanx error (variable did not declare range)', 'Sytanx error (variable F did not has right range)', 'Type error (do not has type hi)', 'Sytanx error (did not expect quotes [])', 'Sytanx error (variable did not declare range)', 'Declaration of variable A is ambiguous', {'A': {'range': ((1, 100),), 'len': None, 'type': 'int', 'ele_type': None}, 'B': {'range': ((100, 200, 150, 180),), 'len': None, 'type': 'int', 'ele_type': None}, 'C': {'range': ((64, 100),), 'len': None, 'type': 'char', 'ele_type': None}, 'D': {'range': ((13.4, 490.23),), 'len': None, 'type': 'float', 'ele_type': None}}]
    cpe(ret, ans)
