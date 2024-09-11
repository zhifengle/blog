import unittest
import os
import json
from json_engine import JsonEngine

class TestJsonEngine(unittest.TestCase):
    def setUp(self):
        self.test_file = 'test_config.json'
        self.engine = JsonEngine(self.test_file)

    def tearDown(self):
        """Clean up the test file after each test."""
        if os.path.exists(self.test_file):
            os.remove(self.test_file)

    def test_set_and_get(self):
        self.engine.set('key1', 'value1')
        self.assertEqual(self.engine.get('key1'), 'value1')

    def test_remove(self):
        self.engine.set('key1', 'value1')
        self.engine.remove('key1')
        self.assertIsNone(self.engine.get('key1'))

    def test_keys(self):
        self.engine.set('key1', 'value1')
        self.engine.set('key2', 'value2')
        self.assertIn('key1', self.engine.keys())
        self.assertIn('key2', self.engine.keys())
        self.assertEqual(len(self.engine.keys()), 2)

    def test_save_and_load(self):
        self.engine.set('key1', 'value1')
        self.engine.save()

        # Create a new engine instance to load the saved data
        new_engine = JsonEngine(self.test_file)
        self.assertEqual(new_engine.get('key1'), 'value1')

    def test_load_non_existent_file(self):
        non_existent_engine = JsonEngine('non_existent_file.json')
        self.assertIsNone(non_existent_engine.get('key1'))

    def test_json_format(self):
        self.engine.set('key1', 'value1')
        self.engine.save()

        with open(self.test_file, 'r') as file:
            data = json.load(file)
            self.assertEqual(data, {'key1': 'value1'})

if __name__ == '__main__':
    unittest.main()