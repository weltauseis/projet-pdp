import timecurves_py as ts


data = ts.inputdata()
data.from_filename("../tcurves/data/template.json")
curve = ts.timecurve("lala")
curves = curve.from_input_data(data)
curves[0].print()
export = ts.exporter("tikz")
output = export.export(curves)
print(output)
export = ts.exporter("blelz")
output = export.export(curves)
print(output)