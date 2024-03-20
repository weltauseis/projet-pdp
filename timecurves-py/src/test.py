import timecurves_py as ts


data = ts.PyInputData()
data.from_filename("../tcurves/data/template.json")
curve = ts.PyTimecurve("lala")
curves = curve.from_input_data(data)
curves[0].print()
export = ts.PyExporter("tikz")
output = export.export(curves)
print(output)
export = ts.PyExporter("blelz")
output = export.export(curves)
print(output)