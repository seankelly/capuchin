import csv

class PlayerList(object):
    def __init__(self, player_file, start_year):
        self.player_file = player_file
        self.start_year = start_year
        self.load_file()

    def load_file(self):
        csv_file = csv.reader(open(self.player_file, 'rb'), delimiter=',')
        self.read_file(csv_file)

    def read_file(self, csv_file):
        player_list = {}
        start_year = self.start_year
        for player_season in csv_file:
            playerid, year = player_season[0:2]
            if year >= start_year:
                if playerid in player_list:
                    player_list[playerid].append(player_season)
                else:
                    player_list[playerid] = list(player_season)
        self.player_list = player_list
