from collections import defaultdict
import csv
import os.path

def autovivify():
    return defaultdict(autovivify)


class Player():
    def project(self):
        raise NotImplementedError('Need to know how to project the player!')


class PlayerList():
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
        columns = {
            'batting': ['playerid', 'year', None, 'team', 'league', 'g',
                        None, 'ab', 'r', 'h', '2b', '3b', 'hr', 'rbi', 'sb',
                        'cs', 'bb', 'so', 'ibb', 'hbp', 'sh', 'sf', 'gidp'],
            'pitching': ['playerid', 'year', None, 'team', 'league', 'w', 'l',
                         'g', 'gs', 'cg', 'sho', 'sv', 'outs', 'h', 'er', 'hr',
                         'bb', 'so', 'baopp', 'era', 'ibb', 'wp', 'hbp', 'bk',
                         'bfp', 'gf', 'r'],
        }
        column_order = columns[section]
        for player_season in csv_file:
            playerid, year = player_season[0], int(player_season[1])
            season_stats = {}
            for i, column in enumerate(column_order):
                if column in mapping:
                    if player_season[i]:
                        f = mapping[column]
                        season_stats[column] = f(player_season[i])
                    else:
                        season_stats[column] = 0
                elif column:
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

    def get_players(self, years, batters=True, pitchers=True):
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
