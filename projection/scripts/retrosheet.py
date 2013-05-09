import argparse

def get_options():
    parser = argparse.ArgumentParser(argument_default=argparse.SUPPRESS)
    parser.add_argument('-f', '--files', dest='files',
                        help='Retrosheet event files')
    parser.add_argument('-o', '--output', dest='output',
                        help='Output file')
    args = parser.parse_args()
    return args

def run():
    options = get_options()
