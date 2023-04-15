import unittest
from . import camp_cleanup

class TestCampCleanup(unittest.TestCase):
    def test_from_str(self):
        string_representation = "6-8"
        expected = camp_cleanup.ElfRange(6, 8)
        self.assertEqual(camp_cleanup.ElfRange.from_str(string_representation), expected)


    def test_from_line(self):
        line = "2-4,6-8"
        expected = (camp_cleanup.ElfRange(2,4), camp_cleanup.ElfRange(6,8))
        self.assertEqual(camp_cleanup.ElfRange.parse_line(line), expected)


    def test_fully_contains(self):
        elf1 = camp_cleanup.ElfRange(2,8)
        elf2 = camp_cleanup.ElfRange(3,7)

        self.assertTrue(elf1.fully_contains(elf2))


    def test_fully_contains(self):
        elf1 = camp_cleanup.ElfRange(5,7)
        elf2 = camp_cleanup.ElfRange(7,9)
        self.assertFalse(elf1.fully_contains(elf2))

    def test_file_parsing(self):
        actual = camp_cleanup.ElfRange.process_file("day_four/test.txt")
        self.assertEqual(actual, 2)
