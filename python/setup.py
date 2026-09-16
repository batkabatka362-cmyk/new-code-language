from setuptools import setup, find_packages

setup(
    name="cron-lang",
    version="1.0.0",
    description="Python SDK & Interoperability Engine for CRON Cognitive Language & 4D-Torus Processor",
    author="CRON Team",
    packages=find_packages(),
    python_requires=">=3.8",
    install_requires=[
        "numpy>=1.20.0",
    ],
    classifiers=[
        "Programming Language :: Python :: 3",
        "Topic :: Scientific/Engineering :: Artificial Intelligence",
    ],
)
