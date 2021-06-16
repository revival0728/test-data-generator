import importlib
import os
import sys
sys.path.append(os.path.abspath(__package__)[:-4])
from .cpe import *

def test_1():
    md = importlib.import_module('src.modules.opt')
    tests = ['test_log.tdg 10', 'abc.txt 10', 'abc.txt', 'test_log.tdg aa']
    answer = [
        {'filename': 'test_log.tdg', 'nftd': 10, 'error': None},
        {'filename': '', 'nftd': 0, 'error': 'File not found (no file abc.txt)'},
        {'filename': '', 'nftd': 0, 'error': 'Number of arguments error (except 2 get 1)'},
        {'filename': 'test_log.tdg', 'nftd': 0, 'error': 'Argument 2 must be positive integer (get aa)'}
    ]
    ret = []
    for i in tests:
        ret.append(md.process_opt(i))
    cpe(ret, answer)