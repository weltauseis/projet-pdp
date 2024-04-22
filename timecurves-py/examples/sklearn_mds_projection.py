from sklearn.manifold import MDS
import timecurves_py as tc

# Load the template data
input_data = tc.input_from_filename("../tcurves/data/chocolate.json")

# create custom projection class
class MDSProjection:
    def project(self, distance_matrix):
        mds_2d = MDS(n_components=2, dissimilarity='precomputed')
        points = mds_2d.fit_transform(distance_matrix)
        return [tc.Position(point[0], point[1]) for point in points]

# wrap it
proj_algo = tc.ProjectionAlgorithm(MDSProjection())

# Project the points
set = tc.TimecurveSet(input_data, proj_algo)

# Export to vegalite
export = tc.export_to_vegalite(set)

# Print the export
print(export)