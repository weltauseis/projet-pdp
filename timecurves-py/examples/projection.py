import timecurves_py as tc

# Load the template data
input_data = tc.input_from_filename("../tcurves/data/template.json")

# Create a classical MDS projection object
mds = tc.ClassicalMDS()

# Project the points
points = mds.project(input_data.get_distance_matrix())

# Print the points 
for point in points:
    print("x: ", round(point.get_x(), 2), "y: ",round(point.get_y(), 2))