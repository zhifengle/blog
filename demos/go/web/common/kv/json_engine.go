package kv

import (
	"encoding/json"
	"os"
)

type JsonEngine struct {
	filename string
	config   map[string]interface{}
}

func NewJsonEngine(filename string) *JsonEngine {
	config, _ := readJsonFile(filename)

	return &JsonEngine{
		filename: filename,
		config:   config,
	}
}

func WriteJsonFile(filename string, config map[string]interface{}) error {
	contents, _ := json.Marshal(config)
	return os.WriteFile(filename, []byte(contents), 0644)
}

func readJsonFile(filename string) (map[string]interface{}, error) {
	config := make(map[string]interface{})
	data, err := os.ReadFile(filename)
	if err == nil {
		json.Unmarshal(data, &config)
	}
	return config, nil
}

func (j *JsonEngine) Save() error {
	return WriteJsonFile(j.filename, j.config)
}

func (j *JsonEngine) set(key string, val interface{}) {
	j.config[key] = val
}

func (j *JsonEngine) get(key string) interface{} {
	return j.config[key]
}

func (j *JsonEngine) remove(key string) {
	delete(j.config, key)
}

func (j *JsonEngine) keys() []string {
	var keys []string
	for key := range j.config {
		keys = append(keys, key)
	}
	return keys
}
