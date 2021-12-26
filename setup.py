from setuptools import setup
from setuptools_rust import Binding, RustExtension, Strip

setup(
    name="songbird-py",
    author="Lunarmagpie",
    description="A Discord voice library using Python Songbird bindings.",
    long_description=open("README.md").read(),
    data_files=[
        "songbird/__init__.pyi"
    ],
    version="0.0.2",
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
        "Bug Tracker": "https://github.com/Lunarmagpie/Songbird-Py/issues",
    },
    include_package_data=True,
    zip_safe=False,
)
