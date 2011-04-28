from batting import Batting
from pitching import Pitching
import csv
import numpy
import os.path


class Marcel(object):
    def __init__(self, bdb_directory, **kwargs):
        from datetime import date

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

        self._normalize_options()
        self.load_marcels()

    def create(self, years):
        if type(years) == int:
            marcel_years = tuple(years)
        elif type(years) == tuple or type(years) == list:
            marcel_years = tuple(filter(lambda x: return type(x) == int, years))

    def load(self):
        # Load the past self.seasons worth of data.
        earliest_year = self.year - self.seasons
        batting_file = os.path.join(self.bdb_directory, 'Batting.txt')
        self.batters = Batting(batting_file, earliest_year)
        pitching_file = os.path.join(self.bdb_directory, 'Pitching.txt')
        self.pitchers = Pitching(pitching_file, earliest_year)


    def _normalize_options(self):
        """Normalize options to ensure they are as expected."""
        from decimal import Decimal
        if type(self.weights) == str:
            weights = self.weights.split(',')
        else:
            weights = self.weights
        # Calculate the total weight.
        decimal_weights = map(lambda x: Decimal(x), weights)
        total_weight = sum(decimal_weights)
        # Then make self.weights be weights/total_weight.
        self.weights = map(lambda x: float(x/total_weight), decimal_weights)
