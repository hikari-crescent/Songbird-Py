# Configuration file for the Sphinx documentation builder.
#
# This file only contains a selection of the most common options. For a full
# list see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Path setup --------------------------------------------------------------

# If extensions (or modules to document with autodoc) are in another directory,
# add these directories to sys.path here. If the directory is relative to the
# documentation root, use os.path.abspath to make it absolute, like shown here.
#
# import os
# import sys
# sys.path.insert(0, os.path.abspath('.'))


# -- Project information -----------------------------------------------------

from inspect import signature
import sys
import os
project = 'Songbird-py'
copyright = '2021, Lunarmagpie'
author = 'Lunarmagpie'

# -- General configuration ---------------------------------------------------

# Add any Sphinx extension module names here, as strings. They can be
# extensions coming with Sphinx (named 'sphinx.ext.*') or your custom
# ones.
sys.path.append(os.path.abspath('extensions'))
sys.path.append(os.path.abspath('..'))
sys.path.append(os.path.abspath("../.."))

extensions = [
    'sphinx.ext.napoleon',
    'sphinx_design',
    'docstrings',
    'autotypehint'
]

from sphinx.ext.autodoc import FunctionDocumenter, MethodDocumenter
from docstrings import find_item

def new_format_signature(self, **kwargs) -> str:
    path = '.'.join(self.objpath)
    obj = self.object
    typehinter = find_item(path)

    if typehinter:
        obj = typehinter

    return str(signature(obj)).replace("'", "")


FunctionDocumenter.format_signature = new_format_signature
MethodDocumenter.format_signature = new_format_signature

# The "prefix" used in the `upload-artifact` step of the ac
autodoc_default_options = {
    'members': True
}


# Add any paths that contain templates here, relative to this directory.
templates_path = ['_templates']

# List of patterns, relative to source directory, that match files and
# directories to ignore when looking for source files.
# This pattern also affects html_static_path and html_extra_path.
exclude_patterns = ['_build', 'Thumbs.db', '.DS_Store']


# TYPE CHECKING
set_type_checking_flag = True
typehints_fully_qualified = True
always_document_param_types = True

# -- Options for HTML output -------------------------------------------------

# The theme to use for HTML and HTML Help pages.  See the documentation for
# a list of builtin themes.
#
html_theme = 'furo'

# Add any paths that contain custom static files (such as style sheets) here,
# relative to this directory. They are copied after the builtin static files,
# so a file named "default.css" will overwrite the builtin "default.css".
html_static_path = ['_static']

source_suffix = '.rst'
