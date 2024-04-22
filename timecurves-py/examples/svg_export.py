import timecurves_py as tc
import random

# Load the template data
input_data = tc.input_from_filename("../tcurves/data/chocolate.json")

# Create a classical MDS projection object
proj = tc.ProjectionAlgorithm.classical_mds()

# create a timecurve set
set = tc.TimecurveSet(input_data, proj)

# export
svg = tc.export_to_svg(set)
print(svg)