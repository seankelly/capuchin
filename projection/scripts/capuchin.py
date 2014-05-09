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
    parser.add_argument('--batter-in', default=None, dest='batting_input',
                        metavar='FILE', help='CSV file of batters input data')
    parser.add_argument('--batter-out', default=None, dest='batting_output',
                        metavar='FILE', help='CSV file of batters output')
    parser.add_argument('--pitcher-in', default=None, dest='pitching_input',
                        metavar='FILE', help='CSV file of pitchers input data')
    parser.add_argument('--pitcher-out', default=None, dest='pitching_output',
                        metavar='FILE', help='CSV file of pitchers output')
    parser.add_argument('-y', '--year', dest='year', type=int,
                        help='For which year to generate projections')
    parser.add_argument('-a', '--aging', dest='age', type=int,
                        help='Peak age')
    parser.add_argument('--age-adjustment', dest='ageadj', type=float,
                        help='Age adjustment')
    parser.add_argument('--batter-weights', dest='batter_weights',
                        metavar='TUPLE',
                        help="The weights for batters' previous seasons")
    parser.add_argument('--batter-regress', dest='batter_regress',
                        metavar='PAs', type=int,
                        help='Number of league average PAs to regress')
    parser.add_argument('--pa-weights', dest='pa_weights', metavar='TUPLE',
                        help='Set weights projecting PAs')
    parser.add_argument('--pa-base', dest='pa_base', metavar='PA',
                        help='Set base PA projection')
    parser.add_argument('--pitcher-weights', dest='pitcher_weights',
                        metavar='TUPLE',
                        help="The weights for pitchers' previous seasons")
    parser.add_argument('--pitcher-regress', dest='pitcher_regress',
                        metavar='IPs', type=int,
                        help='Number of league average PAs to regress')
    parser.add_argument('--ip-weights', dest='ip_weights', metavar='TUPLE',
                        help='Set weights projecting IPs')
    parser.add_argument('--starter-base', dest='starter_base', metavar='IP',
                        help='Set base IP projection for starters')
    parser.add_argument('--reliever-base', dest='reliever_base', metavar='IP',
                        help='Set base IP projection for relievers')
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
