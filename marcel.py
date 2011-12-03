#!/usr/bin/env python

from datetime import date
import argparse
import csv
import os.path

class PlayerList(object):
    def __init__(self, bdb_directory):
        from collections import defaultdict
        def autovivify(): return defaultdict(autovivify)
        self.players = autovivify()
        self.bdb_directory = bdb_directory
        self.load_players()

    def load_players(self):
        for which_file in ('Batting.txt', 'Pitching.txt'):
            file = os.path.join(self.bdb_directory, which_file)
            csv_file = csv.reader(open(file, 'rb'), delimiter=',')
            self.read_csv_file(which_file, csv_file)

    def read_csv_file(self, which_file, csv_file):
        section = os.path.splitext(which_file.lower())[0]
        player_list = self.players
        if section == 'batting':
            column_order = ['playerid', 'year', None, 'team', 'league', 'g', None, 'ab', 'r', 'h', '2b', '3b', 'hr', 'rbi', 'sb', 'cs', 'bb', 'so', 'ibb', 'hbp', 'sh', 'sf', 'gidp']
        elif section == 'pitching':
            column_order = ['playerid', 'year', None, 'team', 'league', 'w', 'l', 'g', 'gs', 'cg', 'sho', 'sv', 'outs', 'h', 'er', 'hr', 'bb', 'so', 'baopp', 'era', 'ibb', 'wp', 'hbp', 'bk', 'bfp', 'gf', 'r']
        for player_season in csv_file:
            playerid, year = player_season[0:2]
            season_stats = {}
            for i, column in enumerate(column_order):
                if column is None:
                    continue
                season_stats[column] = player_season[i]
            player_list[playerid][section][year] = season_stats
        self.players = player_list


class Marcel(object):
    def __init__(self, bdb_directory, **kwargs):
        self.bdb_directory = bdb_directory
        self.age_adjustment = kwargs.get('age_adjustment', (0.003, 0.006))
        self.pa_base = kwargs.get('pa_base', 200)
        self.pa_weights = kwargs.get('pa_weights', (0.5, 0.1))
        self.peak_age = kwargs.get('peak_age', 29)
        self.regress = kwargs.get('regress', 1200)
        self.weights = kwargs.get('weights', (5, 4, 3))
        self.use = kwargs.get('use', { 'regression': True, 'weighting': True, 'age': True })
        self.player_list = None
        self._validate_options()
        self.seasons = len(self.weights)

    def create(self, years):
        marcel_years = self._validate_years(years)
        self.load_players()

    def load_players(self):
        if self.player_list is not None:
            return
        self.player_list = PlayerList(self.bdb_directory)

    def _validate_years(self, years):
        if type(years) == int:
            marcel_years = [years]
        elif type(years) == tuple or type(years) == list:
            marcel_years = tuple(filter(lambda x: type(x) == int, years))
        else:
            raise ValueError, "'years' is not an int or tuple or list"
        return marcel_years

    def _validate_options(self):
        """Normalize options to ensure they are as expected."""
        from decimal import Decimal

        # Validate the weights option.
        # Say want a tuple, but also support lists.
        if not isinstance(self.weights, (tuple, list)):
            raise ValueError, "weights must be a tuple"
        # Calculate the total weight.
        decimal_weights = map(lambda x: Decimal(x), self.weights)
        total_weight = sum(decimal_weights)
        # Then make self.weights be weights/total_weight.
        self.weights = map(lambda x: float(x/total_weight), decimal_weights)


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
