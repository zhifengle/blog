import json
import os
from typing import Any, Dict, List, Optional
from . import KvEngine

class JsonEngine(KvEngine):
    def __init__(self, filename: str):
        self.filename = filename
        self.config = self.read_json_file(filename)

    def read_json_file(self, filename: str) -> Dict[str, Any]:
        try:
            with open(filename, 'r') as file:
                return json.load(file)
        except (FileNotFoundError, json.JSONDecodeError):
            return {}

    def write_json_file(self, filename: str, config: Dict[str, Any]) -> None:
        with open(filename, 'w') as file:
            json.dump(config, file, indent=4)

    def save(self) -> None:
        self.write_json_file(self.filename, self.config)

    def set(self, key: str, val: Any) -> None:
        self.config[key] = val

    def get(self, key: str) -> Optional[Any]:
        return self.config.get(key)

    def remove(self, key: str) -> None:
        self.config.pop(key, None)

    def keys(self) -> List[str]:
        return list(self.config.keys())