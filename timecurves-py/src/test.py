import timecurves_py as tc


data = tc.input_from_filename("../tcurves/data/template.json")
curves =tc.timecurve.from_data(data)
curves.print()
export = tc.exporter("tikz")
output = export.export(curves)
print(output)
export = tc.exporter("toto") #Should be csv by default
output = export.export(curves)
print(output)