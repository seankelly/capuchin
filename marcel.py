#!/usr/bin/env python

from datetime import date
import argparse
import csv
import os.path

class PlayerList(object):
    def __init__(self, bdb_directory):
        self.players = {}
        self.bdb_directory = bdb_directory
        self.load_players()

    def load_players(self):
        for which_file in ('Batting.txt', 'Pitching.txt'):
            file = os.patch.join(self.bdb_directory, which_file)
            csv_file = csv.reader(open(file, 'rb'), delimiter=',')
            self.read_csv_file(which_file, csv_file)

    def read_csv_file(self, which_file, csv_file):
        section = os.path.splitext(which_file.lower())
        player_list = self.player_list
        for player_season in csv_file:
            playerid = player_season[0]
            if playerid in player_list:
                player_list[playerid][section].append(player_season)
            else:
                player_list[playerid] = { section: [ player_season ] }
        self.player_list = player_list


class Marcel(object):
    def __init__(self, bdb_directory, **kwargs):
        self.bdb_directory = bdb_directory
        self.age_adjustment = kwargs.get('age_adjustment', (0.003, 0.006))
        self.pa_base = kwargs.get('pa_base', 200)
        self.pa_weights = kwargs.get('pa_weights', (0.5, 0.1))
        self.peak_age = kwargs.get('peak_age', 29)
        self.regress = kwargs.get('regress', 1200)
        self.seasons = kwargs.get('seasons', 3)
        self.weights = kwargs.get('weights', (5, 4, 3))
        self.use = kwargs.get('use', { 'regression': True, 'weighting': True, 'age': True })
        self.year = kwargs.get('year', date.today().year)
        self._validate_options()

    def create(self, years):
        marcel_years = self._validate_years(years)

    def _load(self, years):
        # Load the past self.seasons worth of data.
        batting_file = os.path.join(self.bdb_directory, 'Batting.txt')
        self.batters = Batting(batting_file, earliest_year)
        pitching_file = os.path.join(self.bdb_directory, 'Pitching.txt')
        self.pitchers = Pitching(pitching_file, earliest_year)

    def _validate_years(self, years):
        if type(years) == int:
            marcel_years = tuple(years)
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
    parser.add_argument('--seasons', dest='seasons', type=int,
                        help='How many seasons to use')
    # Add the --use-foo and --skip-foo options.
    for what in ['aging', 'regression', 'weighting']:
        for choice in ['use', 'skip']:
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
