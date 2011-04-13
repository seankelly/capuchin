#!/usr/bin/python

from optparse import OptionParser
from Marcel import Marcel


def get_options():
    def build_help_message(c, w):
        if c == 'use':
            prefix = 'Use '
            suffix = ' (default)'
        else:
            prefix = "Don't use "
            suffix = ''
        return prefix + what + suffix

    parser = OptionParser()
    parser.add_option('--bdb', default='adminDB', dest='bdb', help='BDB directory')
    parser.add_option('-y', '--year', dest='year', help='For which year to generate Marcels', type='int')
    parser.add_option('-a', '--aging', default=29, dest='age', help='Peak age', type='int')
    parser.add_option('-r', '--regress', default=1200, dest='regression', help='Number of league average PAs', metavar='PAs')
    parser.add_option('-w', '--weights', default='5,4,3', dest='weights', help='Set weights for the seasons')
    parser.add_option('--age-adjustment', default=0.003, dest='ageadj', help='Age adjustment', type='float')
    parser.add_option('--seasons', default=3, dest='seasons', help='How many seasons to use', type='int')
    # Add the --use-foo and --skip-foo options.
    for what in ['aging', 'regression', 'weighting']:
        for choice in ['use', 'skip']:
            action = 'store_true' if choice == 'use' else 'store_false'
            parser.add_option('--' + choice + '-' + what,
                              action=action,
                              dest=what,
                              help=build_help_message(choice, what))
    (options, args) = parser.parse_args()
    return options


if __name__ == '__main__':
    options = get_options()
    marcels = Marcel(options.bdb)
