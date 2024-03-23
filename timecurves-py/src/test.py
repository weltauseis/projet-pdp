import timecurves_py as tc


data = tc.inputdata.from_filename("../tcurves/data/template.json")
curves =tc.timecurve.from_data(data)
curves[0].print()
export = tc.exporter("tikz")
output = export.export(curves)
print(output)
export = tc.exporter("toto") #Should be csv by default
output = export.export(curves)
print(output)