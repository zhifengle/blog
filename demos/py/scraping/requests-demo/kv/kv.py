import time
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

    def set(self, key: str, val: Any, duration: int) -> None:
        self.engine.set(self.gen_key(key), val)
        if duration > 0:
            self.engine.set(self.gen_expiration_key(key), int(time.time()) + duration)

    def set_next_day(self, key: str, val: Any) -> None:
        now = int(time.time())
        next_day = now + 86400  # 1 day in seconds
        self.set(key, val, next_day - now)

    def set_expiration_days(self, key: str, val: Any, days: int) -> None:
        now = int(time.time())
        next_day = now + days * 86400  # days to seconds
        self.set(key, val, next_day - now)

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
        expiration = self.engine.get(self.gen_expiration_key(key))
        if expiration is None:
            return False
        now = int(time.time())
        expiration_time = int(expiration) if isinstance(expiration, int) else int(expiration)
        return now > expiration_time
    
    def save(self) -> None:
        self.engine.save()