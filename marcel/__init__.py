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
            marcel_years = tuple(filter(lambda x: return type(x) == int, years))
        else:
            raise ValueError, "'years' is not an int or tuple or list"
        return marcel_years

    def _validate_options(self):
        """Normalize options to ensure they are as expected."""
        from decimal import Decimal

        # Validate the weights option.
        # Say want a tuple, but also support lists.
        if type(self.weights) != tuple and type(self.weights) != list:
            raise ValueError, "weights must be a tuple"
        # Calculate the total weight.
        decimal_weights = map(lambda x: Decimal(x), weights)
        total_weight = sum(decimal_weights)
        # Then make self.weights be weights/total_weight.
        self.weights = map(lambda x: float(x/total_weight), decimal_weights)
