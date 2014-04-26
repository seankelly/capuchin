from collections import defaultdict

def autovivify():
    return defaultdict(autovivify)

class PlayerList():
    def __init__(self, *args):
        self.season_stats = defaultdict(list)
        self.player_seasons = defaultdict(list)
        self.players = set()
        self._header = []
        self._header_order = {}
        self.is_batters = False
        self.is_pitchers = False

    def append(self, l):
        id, year = l[0], int(l[1])
        stats = map(lambda x: float(x), l[2:])
        self.season_stats[year].append(stats)
        self.player_seasons.append(id)
        self.players.add(id)

    def done(self):
        """
        Run post-processing on the stored seasons.
        """
        pass

    def set_header(self, header):
        # Create an uppercase header to normalize stat checks.
        uppercase_header = map(lambda s: s.upper(), header)
        self._header = uppercase_header
        for i, field in enumerate(header):
            self._header_order[field] = i
        header_stats = set(uppercase_header)
        # Check for stats that only a pitcher or batter can accumulate.
        if ('PA' in header_stats and
                'RBI' in header_stats and
                'SF' in header_stats and
                'SH' in header_stats):
            self.is_batters = True
        elif ('W' in header_stats and
                'L' in header_stats and
                'S' in header_stats and
                'CG' in header_stats):
            self.is_pitchers = True

class Capuchin():
    def __init__(self, **kwargs):
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
        batter_projections = {}
        pitcher_projections = {}
        # Finally! Start generating projections.
        for year in marcel_years:
            pass

    def _load_players(self):
        import csv
        if self.player_list is not None:
            return
        batters, pitchers = None, None
        for what, file in self.input_files.iteritems():
            if file is None:
                continue
            f = csv.reader(open(file, 'r'))
            players = PlayerList()
            read_header = False
            for l in f:
                if read_header:
                    players.append(l)
                else:
                    players.set_header(l)
            players.done()
            if players.is_batters:
                batters = players
            elif players.is_pitchers:
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
