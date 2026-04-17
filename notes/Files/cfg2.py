#!/usr/bin/python

from lark import Lark, tree
from lark.visitors import CollapseAmbiguities
import matplotlib.pyplot as plt

def plot_trees(grammar:str,  text:str, start='sentence'):
    parser = Lark(grammar=grammar, start=start,ambiguity='explicit')  
    parsed = parser.parse(text)
    trees = CollapseAmbiguities().transform(parsed)
    for t in trees:
        tree.pydot__tree_to_png(t, filename='tree.png', rankdir='TB')
        plt.figure(figsize=(8,8))
        plt.imshow(plt.imread("tree.png"))
        plt.show()
        
grammar = r"""
%import common.NUMBER
%import common.WS


!sentence: "X" 
	| "Y" 
	|  sentence "X"


%ignore WS
"""

plot_trees(grammar, 'Y X X X')

