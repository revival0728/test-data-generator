import importlib
import os
import sys
sys.path.append(os.path.abspath(__package__)[:-4])
from .cpe import *

def test_1():
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

def test_2():
    md = importlib.import_module('src.modules.compiler')
    obj = md.compiler()
    tests = [['list', 'A(int)(100){(10)(20)}', 'F(float)(10){(1.3)(34.3)}'], 
             ['list', 'B(100)(int){(20)(300)}'],
             ['list', 'C(int)(100){(20)(300)'],
             ['list', 'D(int, 100){200, 100}'],
             ['list', 'E(int)(100{(100)(200)})']]
    ret = []
    for test in tests:
        obj.process_list(test)
        ret.append(obj.ret['error'])
        obj.ret['error'] = None
    ret.append(obj.var)
    ans = [None, 'Type error (length of a list must be integer)', 'Sytanx error (quote } expect 1 get 0)', 'Sytanx error (did not give list attribute length or element_type)', 'Sytanx error (variable E did not has right range)', {'A': {'range': ((10, 20),), 'len': 100, 'type': 'list', 'ele_type': 'int'}, 'F': {'range': ((1.3, 34.3),), 'len': 10, 'type': 'list', 'ele_type': 'float'}}] 
    cpe(ret, ans)