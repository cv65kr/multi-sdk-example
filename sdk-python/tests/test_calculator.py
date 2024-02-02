import sdk_python
import unittest

class TestCalculator(unittest.TestCase):

    def test_sum(self):
        self.assertEqual(sdk_python.sum_numbers(2, 2), 4)
        self.assertEqual(sdk_python.sum_numbers(2, -2), 0)
        self.assertEqual(sdk_python.sum_numbers(-2, -2), -4)

if __name__ == '__main__':
    unittest.main()