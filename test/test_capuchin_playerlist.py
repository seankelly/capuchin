import random
import unittest

from projection.capuchin import PlayerList

class TestPlayerList(unittest.TestCase):
    def test_set_header(self):
        playerlist = PlayerList()
        # First append is the header.
        header = ['id','2014','pa']
        # The header should be transformed to uppercase.
        upper_header = [s.upper() for s in header]
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
        self.assertTrue(2014 in playerlist._season_stats)
        # Both should have one entry in them.
        self.assertEqual(len(playerlist.player_seasons[2014]), 1)
        self.assertEqual(len(playerlist._season_stats[2014]), 1)

    def test_done(self):
        playerlist = PlayerList()
        header = ['id', '2014', 'pa']
        playerlist.set_header(header)
        playerlist.append(['player1id', '2013', '7'])
        playerlist.append(['player2id', '2014', '8'])
        playerlist.append(['player3id', '2014', '9'])
        # Before the post-processing, each season remains as-is.
        self.assertEqual(len(playerlist.player_seasons[2013]), 1)
        self.assertEqual(len(playerlist._season_stats[2013]), 1)
        self.assertEqual(len(playerlist.player_seasons[2014]), 2)
        self.assertEqual(len(playerlist._season_stats[2014]), 2)
        playerlist.done()
        # After post-processing, the season_stats list will have a row for
        # every player.
        self.assertEqual(len(playerlist.player_seasons[2013]), 1)
        self.assertEqual(len(playerlist.season_stats[2013]), 3)
        self.assertEqual(len(playerlist.player_seasons[2014]), 2)
        self.assertEqual(len(playerlist.season_stats[2014]), 3)
        # Check all column counts in each year to ensure they're equal.
        for y in playerlist.season_stats:
            columns = [len(l) for l in playerlist.season_stats[y]]
            self.assertEqual(min(columns), max(columns))
