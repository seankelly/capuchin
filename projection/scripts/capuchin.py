#!/usr/bin/env python

from datetime import date
from projection.capuchin import Capuchin
import argparse


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
    parser.add_argument('--batters', default=None, dest='batting',
                        help='CSV file of batters')
    parser.add_argument('--pitchers', default=None, dest='pitching',
                        help='CSV file of pitchers')
    parser.add_argument('-y', '--year', dest='year', type=int,
                        help='For which year to generate projections')
    parser.add_argument('-a', '--aging', dest='age', type=int,
                        help='Peak age')
    parser.add_argument('-r', '--regress', dest='regression', type=int,
                        help='Number of league average PAs to regress',
                        metavar='PAs')
    parser.add_argument('-w', '--weights', dest='weights',
                        help='Set weights for the seasons')
    parser.add_argument('-p', '--pa-weights', dest='pa_weights',
                        help='Set weights projecting PAs')
    parser.add_argument('-P', '--pa-base', dest='pa_base',
                        help='Set base PA projection')
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

def run():
    options = get_options()
    options = vars(options)
    capuchin = Capuchin(**options)
    capuchin.create(options.get('year', date.today().year))
