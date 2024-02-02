import sdk_python
import unittest

class TestPerson(unittest.TestCase):

    def test_create_new_person(self):
        p1 = sdk_python.Person(5, "John Doe", "test@example.com")
        self.assertEqual(p1.id, 5)
        self.assertEqual(p1.name, "John Doe")
        self.assertEqual(p1.email, "test@example.com")

    def test_validate_email(self):
        p1 = sdk_python.Person(5, "John Doe", "test@example.com")
        self.assertTrue(p1.validate_email())

        p2 = sdk_python.Person(5, "John Doe", "testexample.com")
        self.assertFalse(p2.validate_email())


if __name__ == '__main__':
    unittest.main()