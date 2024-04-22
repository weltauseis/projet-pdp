import timecurves_py as tc
import random

# Load the template data
input_data = tc.input_from_filename("../tcurves/data/template.json")

# create custom projection class
class RandomProjection:
    def eproject(self, distance_matrix):
        points = []
        for row in distance_matrix:
            points.append(tc.Position(random.random(), random.random()))
        return points

# wrap it
proj_algo = tc.ProjectionAlgorithm(RandomProjection())

# Project the points
points = proj_algo.project(input_data.get_distance_matrix())

# Print the points 
for point in points:
    print("x: ", round(point.get_x(), 2), "y: ",round(point.get_y(), 2))