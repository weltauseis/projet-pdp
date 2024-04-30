# Getting started

## Installation

- `python3 -m venv .env`
- `source .env/bin/activate` (`source .env/bin/activate` for fish shell users)
- `pip install maturin`

## Build

- `source .env/bin/activate` to be in python virtual env
- `maturin develop` to build the project

## Use from cli

Run **examples/cli.py [-h] [--format FORMAT] [--algo ALGO] input output**

- FORMAT in { svg, csv, tikz, vegalite }
- ALGO in { mds }

For exemple :
`python3 src/cli.py --format vegalite --algo mds ../tcurves/data/template.json output.json`

## Use lib in python

Build the project using maturin, then run python and import the lib as following :
**import timecurves_py**

# Maintenance

There is a high dependency between timecurve-rs and timecurve-py so any modification from the crate timecurve-rs can break this one.
