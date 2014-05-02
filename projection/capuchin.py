from collections import defaultdict
import numpy as np

def autovivify():
    return defaultdict(autovivify)

class PlayerList():
    def __init__(self, *args):
        self.season_stats = {}
        self._season_stats = defaultdict(list)
        # The player_seasons dict maps years => players => row id.
        self.player_seasons = defaultdict(lambda: defaultdict(int))
        self.players = set()
        self._header = []
        self._header_order = {}
        self.is_batters = False
        self.is_pitchers = False

    def append(self, l):
        id, year = l[0], int(l[1])
        stats = [float(x) for x in l[2:]]
        self._season_stats[year].append(stats)
        self.player_seasons[year][id] = len(self.player_seasons[year])
        self.players.add(id)

    def done(self):
        """
        Re-order the lists of season stats so every year's list is the same
        size. Players appearing in other seasons will be replaced by seasons
        with zero accumulated stats. The final lists will be arranged in
        alphabetic order by player id.
        """
        all_players = sorted(self.players)
        missing_season = [0.0] * len(self._header)
        for year in sorted(self.player_seasons):
            stats = self._season_stats[year]
            season_players = self.player_seasons[year]
            final_list = []
            for idx, player in enumerate(all_players):
                if player in season_players:
                    final_list.append(stats[season_players[player]])
                    season_players[player] = idx
                else:
                    final_list.append(missing_season)
            self.season_stats[year] = np.array(final_list)

    def set_header(self, header):
        # Create an uppercase header to normalize stat checks.
        uppercase_header = [s.upper() for s in header]
        self._entire_header = uppercase_header
        self._header = uppercase_header[2:]
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

    def header_index(self, stat):
        return self._header.index(stat)

class Capuchin():
    def __init__(self, **kwargs):
        # Age adjustment is the adjustment to make for players before or after
        # their peak age.
        self.age_adjustment = kwargs.get('age_adjustment', (0.003, 0.006))
        # Base PA for projected PA.
        self.pa_base = kwargs.get('pa_base', 200)
        # Weights for past N seasons' PAs to get projected PA.
        self.pa_weights = kwargs.get('pa_weights', (0.5, 0.1))
        # Player's peak age.
        self.peak_age = kwargs.get('peak_age', 29)
        # Amount of league average PAs to add.
        self.regress = kwargs.get('regress', 1200)
        # Weights for the past N seasons.
        self.weights = kwargs.get('weights', (5, 4, 3))
        # Allow disabling any of the three parts of the projection.
        self.use = kwargs.get('use', {'regression': True, 'weighting': True,
                                      'age': True})
        self.player_list = None
        self.input_files = {'batting': kwargs.get('batting'),
                            'pitching': kwargs.get('pitching')}
        self.seasons = len(self.weights)

    def create(self, years):
        project_years = self._validate_years(years)
        batters, pitchers = self._load_players()
        batter_projections = {}
        pitcher_projections = {}
        # Finally! Start generating projections.
        for year in project_years:
            if batters is not None:
                batter_projections[year] = self._create_projection_batters(batters, year)
            if pitchers is not None:
                batter_projections[year] = self._create_projection_pitchers(pitchers, year)

    def _create_projection_batters(self, batters, year):
        past_years = range(year - 1, year - (len(self.weights) + 1), -1)
        # Find the first year in the history and create an empty projection for
        # it to act as an accumulator.
        for y in past_years:
            if y in batters.season_stats:
                projection = np.zeros(batters.season_stats[y].shape)
                league_average = np.zeros(batters.season_stats[y].shape)
                # Get the PA index to calculate the league average.
                pa_idx = batters.header_index('PA')
                break
        else:
            # No years in the past, so no projection to create.
            return

        for idx, y in enumerate(past_years):
            weight = self.weights[idx]
            season = batters.season_stats[y]
            weighted_season = weight * season
            projection += weighted_season
            # Weight each player by the number of plate appearances.
            for row in range(len(weighted_season)):
                pa = season[row, pa_idx]
                weighted_season[row, :] *= pa
            league_average += weighted_season

    def _create_projection_pitchers(self, pitchers, year):
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
                    read_header = True
            players.done()
            if players.is_batters:
                batters = players
            elif players.is_pitchers:
                pitchers = players
        return batters, pitchers

    def _validate_years(self, years):
        if type(years) == int:
            project_years = [years]
        else:
            project_years = years
        return project_years
