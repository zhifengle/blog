import time
from datetime import datetime, timedelta
from typing import Any, Dict, List, Optional

class KvEngine:
    def set(self, key: str, val: Any) -> None:
        pass

    def get(self, key: str) -> Any:
        pass

    def remove(self, key: str) -> None:
        pass

    def keys(self) -> List[str]:
        pass

class KvExpiration:
    def __init__(self, engine: KvEngine, prefix: str, suffix: str = "-expiration"):
        self.engine = engine
        self.prefix = prefix
        self.suffix = suffix

    def set(self, key: str, val: Any, duration: int = 0) -> None:
        self.engine.set(self.gen_key(key), val)
        if duration > 0:
            expiration_time = (datetime.now() + timedelta(seconds=duration)).isoformat()
            self.engine.set(self.gen_expiration_key(key), expiration_time)

    def set_with_expiration(self, key: str, val: Any, expiration_time: str) -> None:
        self.engine.set(self.gen_key(key), val)
        self.engine.set(self.gen_expiration_key(key), expiration_time)

    def set_next_day(self, key: str, val: Any) -> None:
        now = datetime.now()
        next_day = now + timedelta(days=1)
        next_day_start = next_day.replace(hour=0, minute=0, second=0, microsecond=0)
        self.set_with_expiration(key, val, next_day_start.isoformat())

    def set_expiration_days(self, key: str, val: Any, days: int) -> None:
        now = datetime.now()
        expiration_time = now + timedelta(days=days)
        self.set_with_expiration(key, val, expiration_time.isoformat())

    def get(self, key: str) -> Optional[Any]:
        if self.is_expired(key):
            return None
        return self.engine.get(self.gen_key(key))

    def remove(self, key: str) -> None:
        self.engine.remove(self.gen_expiration_key(key))
        self.engine.remove(self.gen_key(key))

    def keys(self) -> List[str]:
        return self.engine.keys()

    def gen_expiration_key(self, key: str) -> str:
        return f"{self.prefix}{key}{self.suffix}"

    def gen_key(self, key: str) -> str:
        return f"{self.prefix}{key}"

    def flush_expired(self) -> None:
        for key in self.keys():
            if key.startswith(self.prefix) and not key.endswith(self.suffix):
                target_key = key[len(self.prefix):]
                self.flush_expired_item(target_key)

    def flush_expired_item(self, key: str) -> bool:
        if self.is_expired(key):
            self.remove(key)
            return True
        return False

    def is_expired(self, key: str) -> bool:
        expiration_str = self.engine.get(self.gen_expiration_key(key))
        if expiration_str is None:
            return False
        expiration_time = datetime.fromisoformat(expiration_str)
        now = datetime.now()
        return now > expiration_time
    
    def save(self) -> None:
        if hasattr(self.engine, 'save'):
            self.engine.save()