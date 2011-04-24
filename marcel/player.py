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
        pass
