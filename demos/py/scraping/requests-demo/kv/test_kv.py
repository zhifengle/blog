import unittest
import os
import time
import json
from . import JsonEngine
from . import KvExpiration

class TestKvExpiration(unittest.TestCase):
    def setUp(self):
        self.test_file = 'test_config.json'
        self.engine = JsonEngine(self.test_file)
        self.expiration = KvExpiration(self.engine, 'test_')

    def tearDown(self):
        """Clean up the test file after each test."""
        if os.path.exists(self.test_file):
            os.remove(self.test_file)

    def test_set_and_get(self):
        self.expiration.set('key1', 'value1', 5)
        self.assertEqual(self.expiration.get('key1'), 'value1')

    def test_get_expired_key(self):
        self.expiration.set('key2', 'value2', 1)
        time.sleep(2)  # Wait for the key to expire
        self.assertIsNone(self.expiration.get('key2'))

    def test_remove_key(self):
        self.expiration.set('key3', 'value3', 5)
        self.expiration.remove('key3')
        self.assertIsNone(self.expiration.get('key3'))

    def test_flush_expired(self):
        self.expiration.set('key4', 'value4', 1)
        self.expiration.set('key5', 'value5', 5)
        time.sleep(2)  # Wait for key4 to expire
        self.expiration.flush_expired()
        self.assertIsNone(self.expiration.get('key4'))
        self.assertEqual(self.expiration.get('key5'), 'value5')

    def test_set_next_day(self):
        self.expiration.set_next_day('key6', 'value6')
        self.assertEqual(self.expiration.get('key6'), 'value6')

        # Check expiration after 1 day simulation (using a shorter wait for the test)
        time.sleep(2)  # Simulate waiting for expiration (this should reflect a real use case)


    def test_json_persistence(self):
        self.expiration.set('key8', 'value8', 5)
        self.expiration.save()  # Save to JSON
        new_expiration = KvExpiration(JsonEngine(self.test_file), 'test_')  # Load from JSON
        self.assertEqual(new_expiration.get('key8'), 'value8')

if __name__ == '__main__':
    unittest.main()