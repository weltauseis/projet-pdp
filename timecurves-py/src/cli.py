import argparse
import timecurves_py as tc

def parse_arguments():
    parser = argparse.ArgumentParser(
                        prog='cli.py',
                        description='Generate timecurves from a dataset',)

    parser.add_argument('input', type=str,
                        help='Input path to the file, must be json format')

    parser.add_argument('output', type=str,
                        help='Output path/filename')
    
    parser.add_argument('--format', type=str, default='tikz',
                        help='Output format (default: tikz)')
    
    parser.add_argument('--algo', type=str, default='mds',
                        help='Projection algorithm (default: mds)')
    
    return parser.parse_args()

def main():
    parsed_args = parse_arguments()
    data = tc.input_from_filename(parsed_args.input)
    curves = tc.timecurves_from_data(data,parsed_args.algo)
    export = tc.exporter(parsed_args.format)
    output = export.export(curves)
    with open(parsed_args.output, 'w') as f:
        f.write(output)
        print(f"Exported to {parsed_args.output}")

if __name__ == "__main__":
    main()