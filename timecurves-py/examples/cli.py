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
    
    return parser.parse_args()

def main():
    parsed_args = parse_arguments()
    data = tc.input_from_filename(parsed_args.input)
    curves = tc.TimecurveSet(data, tc.ProjectionAlgorithm.classical_mds())
    
    match parsed_args.format:
        case 'tikz':
            output = tc.export_to_tikz(curves)
        case 'svg':
            output = tc.export_to_svg(curves)
        case 'vegalite':
            output = tc.export_to_vegalite(curves)
        case 'csv':
            output = tc.export_to_csv(curves)
        case _:
            raise ValueError(f"Invalid format: {parsed_args.format}")

    with open(parsed_args.output, 'w') as f:
        f.write(output)
        print(f"Exported to {parsed_args.output}")

if __name__ == "__main__":
    main()