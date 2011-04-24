import csv
import numpy


class Marcel(object):
    def __init__(self, bdb_directory, **kwargs):
        from datetime import date

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
