#!/usr/bin/env python

from collections import defaultdict
from datetime import date
import argparse
import csv
import os.path

def autovivify():
    return defaultdict(autovivify)

class PlayerList(object):
    def __init__(self, bdb_directory):
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
        mapping = {'year': int, 'w': int, 'l': int, 'g': int, 'gs': int,
                   'cg': int, 'sho': int, 'sv': int, 'outs': int, 'h': int,
                   'er': int, 'hr': int, 'bb': int, 'so': int, 'baopp': float,
                   'era': float, 'ibb': int, 'wp': int, 'hbp': int, 'bk': int,
                   'bfp': int, 'gf': int, 'r': int, 'ab': int, '2b': int,
                   '3b': int, 'rbi': int, 'sb': int, 'cs': int, 'sh': int,
                   'sf': int, 'gidp': int}
        if section == 'batting':
            column_order = ['playerid', 'year', None, 'team', 'league', 'g',
                    None, 'ab', 'r', 'h', '2b', '3b', 'hr', 'rbi', 'sb', 'cs',
                    'bb', 'so', 'ibb', 'hbp', 'sh', 'sf', 'gidp']
        elif section == 'pitching':
            column_order = ['playerid', 'year', None, 'team', 'league', 'w',
                    'l', 'g', 'gs', 'cg', 'sho', 'sv', 'outs', 'h', 'er', 'hr',
                    'bb', 'so', 'baopp', 'era', 'ibb', 'wp', 'hbp', 'bk',
                    'bfp', 'gf', 'r']
        for player_season in csv_file:
            playerid, year = player_season[0], int(player_season[1])
            season_stats = {}
            for i, column in enumerate(column_order):
                if column is None:
                    continue
                f = mapping.get(column)
                if f:
                    if player_season[i]:
                        season_stats[column] = f(player_season[i])
                    else:
                        season_stats[column] = 0
                else:
                    season_stats[column] = player_season[i]
            player_list[playerid][section][year] = season_stats
        self.players = player_list

    # Map years into an iterable for the set() function.
    def make_set(self, years):
        try:
            years = set(years)
        except TypeError:
            years = set([years])
        return years

    def get_players(self, years):
        player_list = self.players
        matched_players = autovivify()
        years = self.make_set(years)
        for playerid in player_list:
            for section in player_list[playerid]:
                section_years = set(player_list[playerid][section])
                player_years = years & section_years
                if player_years:
                    matched_players[playerid][section] = self.get_player_years(playerid, section, player_years)
        return matched_players

    def get_player_years(self, playerid, section, years):
        years = self.make_set(years)
        # Return copies so things can be manipulated later.
        return { k:v.copy() for k, v in self.players[playerid][section].iteritems() if k in years }


class Marcel(object):
    def __init__(self, bdb_directory, **kwargs):
        self.bdb_directory = bdb_directory
        self.age_adjustment = kwargs.get('age_adjustment', (0.003, 0.006))
        self.pa_base = kwargs.get('pa_base', 200)
        self.pa_weights = kwargs.get('pa_weights', (0.5, 0.1))
        self.peak_age = kwargs.get('peak_age', 29)
        self.regress = kwargs.get('regress', 1200)
        self.weights = kwargs.get('weights', (5, 4, 3))
        self.use = kwargs.get('use', {'regression': True, 'weighting': True,
                                      'age': True})
        self.player_list = None
        self._validate_options()
        self.seasons = len(self.weights)

    def create(self, years, batters=True, pitchers=True):
        marcel_years = self._validate_years(years)
        self.load_players()
        # Find all players that played in the last self.seasons seasons.
        fetch_years = set()
        for year in marcel_years:
            for delta in range(self.seasons):
                fetch_years.add(year - (delta + 1))
        players = self.player_list.get_players(fetch_years)
        # TODO: Rename batters and pitchers variables.
        batters, pitchers = self.classify_players(players)
        batter_projections = {}
        pitcher_projections = {}
        # Finally! Start generating projections.
        for year in marcel_years:
            pass

    def classify_players(self, players):
        batters = autovivify()
        pitchers = autovivify()
        # Check each season for a player and classify the season as pitching or
        # batting.
        for playerid in players:
            # The majority of players only ever bat or pitch, so quickly check
            # if the player has only a single type and move on.
            if not players[playerid].get('pitching'):
                batters[playerid] = players[playerid]['batting']
                continue
            elif not players[playerid].get('batting'):
                pitchers[playerid] = players[playerid]['pitching']
                continue
            years = (set(players[playerid]['pitching']) |
                     set(players[playerid]['batting']))
            for year in years:
                outs, pas = 0, 0
                if year in players[playerid]['pitching']:
                    outs = players[playerid]['pitching'][year]['outs']
                if year in players[playerid]['batting']:
                    pas = (players[playerid]['batting'][year]['ab'] +
                           players[playerid]['batting'][year]['bb'] +
                           players[playerid]['batting'][year]['ibb'])
                if outs > pas:
                    pitchers[playerid][year] = players[playerid]['pitching'][year]
                else:
                    batters[playerid][year] = players[playerid]['batting'][year]
        return batters, pitchers

    def load_players(self):
        if self.player_list is not None:
            return
        self.player_list = PlayerList(self.bdb_directory)

    def _validate_years(self, years):
        if type(years) == int:
            marcel_years = [years]
        else:
            marcel_years = years
        return marcel_years

    def _validate_options(self):
        """Normalize options to ensure they are as expected."""
        from decimal import Decimal

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
