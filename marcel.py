#!/usr/bin/env python

from collections import defaultdict
from datetime import date
import argparse
import csv
import os.path


def get_options():
    def build_help_message(c, w):
        if c == 'use':
            prefix = 'Use '
            suffix = ' (default)'
        else:
            prefix = "Don't use "
            suffix = ''
        return prefix + what + suffix
    parser = argparse.ArgumentParser(argument_default=argparse.SUPPRESS)
    parser.add_argument('--bdb', default='adminDB', dest='bdb',
                        help='BDB directory')
    parser.add_argument('-y', '--year', dest='year', type=int,
                        help='For which year to generate Marcels')
    parser.add_argument('-a', '--aging', dest='age', type=int,
                        help='Peak age')
    parser.add_argument('-r', '--regress', dest='regression', type=int,
                        help='Number of league average PAs', metavar='PAs')
    parser.add_argument('-w', '--weights', dest='weights',
                        help='Set weights for the seasons')
    parser.add_argument('--age-adjustment', dest='ageadj', type=float,
                        help='Age adjustment')
    # Add the --use-foo and --skip-foo options.
    for what in ('aging', 'regression', 'weighting'):
        for choice in ('use', 'skip'):
            action = 'store_true' if choice == 'use' else 'store_false'
            parser.add_argument('--' + choice + '-' + what,
                                action=action, dest=what,
                                help=build_help_message(choice, what))
    args = parser.parse_args()
    return args


if __name__ == '__main__':
    options = get_options()
    marcel_options = vars(options)
    marcels = Marcel(options.bdb, **marcel_options)
    marcels.create(date.today().year)
