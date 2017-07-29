#!/usr/bin/env python3

import argparse
import csv
import math
import numpy
import sys
from collections import defaultdict


def load_projection(file_path, playerid_column=0):
    projection = {}
    headers = []
    skipped_header = False
    with open(file_path) as fd:
        reader = csv.reader(fd)
        for row in reader:
            if skipped_header:
                # Skip pre-playerid columns.
                for i in range(playerid_column):
                    row.pop(0)
                playerid = row.pop(0)
                pre_stats = [int(stat) for stat in row[:2]]
                stats = [float(stat) for stat in row[2:]]
                projection[playerid] = pre_stats + stats
            else:
                skipped_header = True
                for i in range(playerid_column):
                    row.pop(0)
                # Pop the player id too.
                row.pop(0)
                headers = row
    return headers, projection


def compare(headers, marcel, new_proj):
    marcel_players = set(marcel.keys())
    new_players = set(new_proj.keys())
    mutual_players = marcel_players & new_players

    # Check for players only in the Marcel projections.
    only_marcel_players = sorted(marcel_players - new_players)
    if only_marcel_players:
        print("Player only in Marcels: %s" % ', '.join(only_marcel_players))
    # Check for players only in the new projections.
    only_new_players = sorted(new_players - marcel_players)
    if only_new_players:
        print("Player only in New: %s" % ', '.join(only_new_players))

    # Check for NaNs in the new projections.
    new_mutual_players = set()
    nan_players = []
    for player in mutual_players:
        stats = new_proj[player]
        if any([math.isnan(s) for s in stats]):
            nan_players.append(player)
        else:
            new_mutual_players.add(player)
    if nan_players:
        print("Player with NaNs: %s" % ', '.join(nan_players))

    mutual_players = new_mutual_players
    stat_differences = defaultdict(lambda: [])
    for player in mutual_players:
        marcel_stats = marcel[player]
        new_stats = new_proj[player]
        for stat, m_stat, n_stat in zip(headers, marcel_stats, new_stats):
            diff = m_stat - n_stat
            if stat == "Age" and abs(diff) > 0.5:
                print(f"{player}'s age doesn't agree: {m_stat} != {n_stat}")
            stat_differences[stat].append(diff)

    print("stat\trmse\t\tstddev\t\tvariance")
    for stat, stat_diff in stat_differences.items():
        rmse = numpy.sqrt((numpy.array(stat_diff) ** 2).mean())
        stddev = numpy.std(stat_diff)
        variance = numpy.var(stat_diff)
        print(f"{stat}\t{rmse:8.5}\t{stddev:8.5}\t{variance:8.5}")


def main(args):
    if len(args) != 3:
        sys.exit(1)
    marcel_file = args[1]
    new_file = args[2]
    print("Comparing %s to %s" % (marcel_file, new_file))

    marcel_headers, marcel = load_projection(marcel_file, playerid_column=2)
    _, new_proj = load_projection(new_file)
    compare(marcel_headers, marcel, new_proj)


if __name__ == '__main__':
    main(sys.argv)
