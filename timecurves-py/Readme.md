# Getting started

## Installation

- `python3 -m venv .env`
- `source .env/bin/activate`
- `pip install maturin`

## Build

`source .env/bin/activate` to be in python venv
Then `maturin develop` to build the project

## Use from cli

Run **src/cli.py [-h] [--format FORMAT] [--algo ALGO] input output**

- FORMAT in { svg, csv, tikz, vegalite }
- ALGO in { mds }

For exemple :
`python3 src/cli.py --format vegalite --algo mds ../tcurves/data/template.json output.json`

## Use lib in python

Build the project using maturin then run python and import the lib as following :
**import timecurves_py**

# Maintenance

There is a high dependency between timecurve-rs and timecurve-py so any modification from the crate timecurve-rs can break this crate.

### Modification that will break the lib

If any current pub function or any Class that possessed pub function are altered

#### Adding feature

New **Exporter** need to be add in **export** function of _PyExporter_ from _exporter.rs_.

New **ProjectionAlgorithm** need to be add in **new** functon of _PyTimecurve_ from _timecurve.rs_.

Any new **pub method** need to be add and called from the Py*Class*.

Any new **pub class** need to have a new Py*Class* equivalent that embed this new class.
