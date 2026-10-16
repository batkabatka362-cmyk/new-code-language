from setuptools import setup, find_packages
import os

here = os.path.abspath(os.path.dirname(__file__))

setup(
    name="cron-lang",
    version="1.7.0",
    description="Python & PyTorch Living AGI Accelerator Bridge for CRON 256-Core 4D-Torus Silicon Architecture",
    long_description="Universal Python bridge for CRON (.cl) machine language, 0-DSP BitNet ternary inference, sub-byte CL-LoRA adaptation, and pure-Rust voice agent.",
    author="CRON Language Systems Team",
    packages=find_packages(),
    py_modules=["cron_cortex"],
    package_data={
        "cron": ["*.dll", "*.so", "*.dylib", "*.h"],
        "": ["*.dll", "*.so", "*.dylib", "*.h"],
    },
    include_package_data=True,
    python_requires=">=3.8",
    install_requires=[
        "numpy>=1.20.0",
    ],
    classifiers=[
        "Programming Language :: Python :: 3",
        "Programming Language :: Rust",
        "Topic :: Scientific/Engineering :: Artificial Intelligence",
        "Topic :: System :: Hardware",
    ],
)
