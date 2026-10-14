from setuptools import setup, find_packages

setup(
    name="cron-cortex",
    version="1.4.0",
    description="Python PyTorch & JAX Native C23 Hardware Accelerator Extension for CRON Silicon Core",
    author="CRON Team",
    packages=find_packages(),
    py_modules=["cron_cortex"],
    package_data={
        "": ["*.dll", "*.so", "*.dylib", "*.h"],
    },
    include_package_data=True,
    python_requires=">=3.8",
    install_requires=[
        "numpy>=1.20.0",
    ],
    classifiers=[
        "Programming Language :: Python :: 3",
        "Topic :: Scientific/Engineering :: Artificial Intelligence",
        "Topic :: System :: Hardware",
    ],
)

