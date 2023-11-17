package kv

import (
	"strconv"
	"time"
)

type KvEngine interface {
	set(key string, val interface{})
	get(key string) interface{}
	remove(key string)
	keys() []string
}

type KvExpiration struct {
	engine         KvEngine
	prefix, suffix string
}

// new expiration
func NewExpiration(engine KvEngine, prefix string) *KvExpiration {
	return &KvExpiration{
		engine: engine,
		prefix: prefix,
		suffix: "-expiration",
	}
}

func NewExpirationWithSuffix(engine KvEngine, prefix, suffix string) *KvExpiration {
	return &KvExpiration{
		engine: engine,
		prefix: prefix,
		suffix: suffix,
	}
}

// set expiration
func (e *KvExpiration) Set(key string, val interface{}, duration int64) {
	e.engine.set(e.genKey(key), val)
	if duration > 0 {
		e.engine.set(e.genExpirationKey(key), time.Now().Unix()+duration)
	}
}

// set expiration next day
func (e *KvExpiration) SetNextDay(key string, val interface{}) {
	now := time.Now()
	nextDay := time.Now().AddDate(0, 0, 1)
	e.Set(key, val, nextDay.Unix()-now.Unix())
}

// set expiration days
func (e *KvExpiration) SetExpirationDays(key string, val interface{}, days int) {
	now := time.Now()
	nextDay := time.Now().AddDate(0, 0, days)
	e.Set(key, val, nextDay.Unix()-now.Unix())
}

// get expiration
func (e *KvExpiration) Get(key string) interface{} {
	if e.isExpired(key) {
		return nil
	}
	return e.engine.get(e.genKey(key))
}

// remove expiration
func (e *KvExpiration) Remove(key string) {
	e.engine.remove(e.genExpirationKey(key))
	e.engine.remove(e.genKey(key))
}

// keys expiration
func (e *KvExpiration) Keys() []string {
	return e.engine.keys()
}

func (e *KvExpiration) genExpirationKey(key string) string {
	return e.prefix + key + e.suffix
}

func (e *KvExpiration) genKey(key string) string {
	return e.prefix + key
}

func (e *KvExpiration) FlushExpired() {
	for _, key := range e.Keys() {
		// check startswith and not endswith
		if key[:len(e.prefix)] == e.prefix && key[len(key)-len(e.suffix):] != e.suffix {
			targetKey := key[len(e.prefix):]
			e.flushExpiredItem(targetKey)
		}
	}
}

// flush expired item
func (e *KvExpiration) flushExpiredItem(key string) bool {
	if e.isExpired(key) {
		e.Remove(key)
		return true
	}
	return false
}

func (e *KvExpiration) isExpired(key string) bool {
	expiration := e.engine.get(e.genExpirationKey(key))
	if expiration == "" {
		return false
	}
	now := time.Now()
	var num int64
	if _, ok := expiration.(int64); ok {
		num = expiration.(int64)
	} else {
		num, _ = strconv.ParseInt(expiration.(string), 10, 64)
	}
	expirationTime := time.Unix(num, 0)

	return now.After(expirationTime)
}
