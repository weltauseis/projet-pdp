import timecurves_py as tc
import umap

# Load the template data
input_data = tc.input_from_filename("../tcurves/data/chocolate.json")

# create custom projection class
class UmapProjection:
    def project(self, distance_matrix):
        umap_2d = umap.UMAP(n_components=2, metric='precomputed')
        points = umap_2d.fit_transform(distance_matrix)
        return [tc.Position(point[0], point[1]) for point in points]

# wrap it
proj_algo = tc.ProjectionAlgorithm(UmapProjection())

# Project the points
set = tc.TimecurveSet(input_data, proj_algo)

# Export to vegalite
export = tc.export_to_vegalite(set)

# Print the export
print(export)