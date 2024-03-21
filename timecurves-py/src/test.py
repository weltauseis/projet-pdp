import timecurves_py as tc


data = tc.inputdata()
data.from_filename("../tcurves/data/template.json")
curve = tc.timecurve()
curves = curve.from_input_data(data)
curves[0].print()
export = tc.exporter("tikz")
output = export.export(curves)
print(output)
export = tc.exporter("toto") #Should be csv by default
output = export.export(curves)
print(output)