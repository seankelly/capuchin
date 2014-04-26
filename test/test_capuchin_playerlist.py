import random
import unittest

from projection.capuchin import PlayerList

class TestPlayerList(unittest.TestCase):
    def test_set_header(self):
        playerlist = PlayerList()
        # First append is the header.
        header = ['id','2014','pa']
        # The header should be transformed to uppercase.
        upper_header = map(lambda s: s.upper(), header)
        playerlist.set_header(header)
        self.assertEqual(playerlist._header, upper_header)

    def test_append(self):
        playerlist = PlayerList()
        header = ['id','2014','pa']
        playerlist.set_header(header)
        playerlist.append(['playerid', '2014', '7'])
        # playerid should appear in players set.
        self.assertTrue('playerid' in playerlist.players)
        # 2014 should be a key in player_seasons...
        self.assertTrue(2014 in playerlist.player_seasons)
        # ...and season_stats.
        self.assertTrue(2014 in playerlist.season_stats)
        # Both should have one entry in them.
        self.assertTrue(len(playerlist.player_seasons[2014]) == 1)
        self.assertTrue(len(playerlist.season_stats[2014]) == 1)
