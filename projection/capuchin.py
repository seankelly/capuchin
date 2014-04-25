from collections import defaultdict

def autovivify():
    return defaultdict(autovivify)


class Capuchin():
    def __init__(self, **kwargs):
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
        self.input_files = {'batting': kwargs.get('batting'),
                            'pitching': kwargs.get('pitching')}
        self._validate_options()
        self.seasons = len(self.weights)

    def create(self, years, batters=True, pitchers=True):
        marcel_years = self._validate_years(years)
        batters, pitchers = self._load_players()
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

    def _load_players(self):
        import csv
        if self.player_list is not None:
            return
        batters, pitchers = None, None
        for what, file in self.input_files.iteritems():
            if file is None:
                continue
            f = csv.reader(open(file, 'r'))
            players = []
            read_header = False
            for l in f:
                if read_header:
                    players.append(l)
                else:
                    players._header = l
            if 'batting' in f:
                batters = players
            elif 'pitching' in f:
                pitchers = players
        return batters, pitchers

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
