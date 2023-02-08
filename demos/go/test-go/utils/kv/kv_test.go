package kv

import (
	"testing"
	"time"
)

func TestTime(t *testing.T) {
	t.Logf("%d", time.Now().Unix())
	t.Logf("%d", time.Now().UnixMilli())
}

func TestKvExpiration(t *testing.T) {
	jsonEngine := NewJsonEngine("test.json")
	defer jsonEngine.Save()
	kv := NewExpiration(jsonEngine, "MY_PREFIX")
	kv.Set("key", "val", 2)
	val := kv.Get("key")
	if val != "val" {
		t.Errorf("kv.Get(\"key\") = %v, want %v", val, "val")
	}
	time.Sleep(3 * time.Second)
	val = kv.Get("key")
	if val != nil {
		t.Errorf("kv.Get(\"key\") = %v, want %v", val, nil)
	}
	// kv.SetNextDay("key", "val")
	// kv.SetExpirationDays("key", "val", 1)
	// kv.Get("key")
	// kv.Remove("key")
}

func TestSleep(t *testing.T) {
	t.Log("start")
	time.Sleep(3 * time.Second)
	t.Log("end")
}

func TestMap(t *testing.T) {
	m := make(map[string]interface{})
	m["key"] = "val"
	t.Log(m)
}
