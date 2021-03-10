from setuptools import find_packages, setup

setup(
    name="sodoku_swap",
    version="1.0.0",
    packages=find_packages(),
    include_package_data=True,
    zip_safe=False,
    entry_points={
        'console_scripts': [
            'prtest=prtest.main:main'
        ]
    }
)
