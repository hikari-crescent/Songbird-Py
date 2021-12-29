from inspect import signature
import importlib
from copy import copy
from types import ModuleType
from typing import get_type_hints
from typing import *
import sys
import typing

songbird = ModuleType('songbird', 'pyi loaded for typing data')

with open ("../songbird/songbird.pyi") as f:
    exec("from typing import *", songbird.__dict__)
    exec(f.read(), songbird.__dict__)

items = dir(songbird)

def find_item(name: str):
    """Name is expected to be like this ``songbird.sonbird.Config``"""
    end_of_name = name.split(".")[-1]

    if end_of_name in items:
        return getattr(songbird, end_of_name)
    else:
        upper = '.'.join(name.split(".")[:-1])

        if not upper:
            return

        found = find_item(upper)
        if not found:
            return
        return getattr(found, end_of_name)

def docstring(app, what, name, obj, options, lines):
    try:
        thing = find_item(name)
        if thing is not None and hasattr(thing, "__annotations__"):
            setattr(obj, "__annotations__", thing.__annotations__)
        if thing is not None and hasattr(thing, "__args__"):
            setattr(obj, "__args__", thing.__args__)

    except AttributeError as e:
        pass

def setup(app):
    app.connect('autodoc-process-docstring', docstring)
