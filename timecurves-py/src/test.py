import timecurves_py as tc


data = tc.input_from_filename("../tcurves/data/template.json")
curves =tc.timecurves_from_data(data)
curves.print()
export = tc.exporter("svg")
output = export.export(curves)
print(output)
export = tc.exporter() #Should be tikz by default
output = export.export(curves)
print(output)