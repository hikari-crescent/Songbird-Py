# This is only used for readthe docs. Packaging is done with setuptools.

from setuptools import setup
from setuptools_rust import Binding, RustExtension, Strip

setup(
    name="songbird-py",
    author="Lunarmagpie",
    description="A Discord voice library using Python Songbird bindings.",
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    data_files=[
        "songbird/songbird.pyi"
    ],
    version="0.1.4",
    classifiers=[
        "Programming Language :: Python :: 3",
        "Programming Language :: Rust",
        "License :: OSI Approved :: GNU General Public License v2 (GPLv2)",
        "Operating System :: OS Independent",
    ],
    rust_extensions=[RustExtension("songbird.songbird", binding=Binding.PyO3, strip=Strip.All)],
    packages=["songbird"],
    install_requires=["yt-dlp"],
    url="https://github.com/Lunarmagpie/Songbird-Py",
    project_urls={
        "GitHub": "https://github.com/Lunarmagpie/Songbird-Py",
        "Docs": "https://songbird-py.readthedocs.io/en/latest/",
        "Bug Tracker": "https://github.com/Lunarmagpie/Songbird-Py/issues",
    },
    include_package_data=True,
    zip_safe=False,
)