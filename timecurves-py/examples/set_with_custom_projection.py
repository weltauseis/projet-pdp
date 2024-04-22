import timecurves_py as tc
import random

# Load the template data
input_data = tc.input_from_filename("../tcurves/data/template.json")

# create custom projection class
class RandomProjection:
    def project(self, distance_matrix):
        points = []
        for row in distance_matrix:
            points.append(tc.Position(random.random(), random.random()))
        return points

# wrap it
proj_algo = tc.ProjectionAlgorithm(RandomProjection())

# create a timecurve set
set = tc.TimecurveSet(input_data, proj_algo)

# print the points
for curve in set.get_curves():
    print("Curve: ", curve.get_name())
    for point in curve.get_points():
        print("x: ", round(point.get_pos_x(), 2), "y: ",round(point.get_pos_y(), 2))