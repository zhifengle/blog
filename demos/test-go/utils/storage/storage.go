package storage

import (
	"encoding/json"
	"io/ioutil"
	"log"
)

func check(e error) {
	if e != nil {
		log.Fatal(e)
	}
}

type Storage struct {
	Filename string
}

func New(filename string) *Storage {
	s := &Storage{
		Filename: filename,
	}
	return s
}

func (s Storage) writeFile(contents string) {
	err := ioutil.WriteFile(s.Filename, []byte(contents), 0644)
	check(err)
}

func (s Storage) GetConfig() interface{} {
	var v interface{}
	dat, err := ioutil.ReadFile(s.Filename)
	if err != nil {
		json.Unmarshal([]byte(`{}`), &v)
		return v
	}
	//check(err)
	json.Unmarshal(dat, &v)
	//data := v.(map[string]interface{})
	return v
}

func (s Storage) Get(key string) interface{} {
	config := s.GetConfig()
	data := config.(map[string]interface{})
	return data[key]
}

func (s Storage) Set(key, val string) {
	config := s.GetConfig()
	data := config.(map[string]interface{})
	data[key] = val
	contents, err := json.Marshal(data)
	check(err)
	s.writeFile(string(contents))
}

func (s Storage) Delete(key string) {
	config := s.GetConfig()
	data := config.(map[string]interface{})
	delete(data, key)
	contents, err := json.Marshal(data)
	check(err)
	s.writeFile(string(contents))
}

// for test

//func main() {
//	var s storage.Storage = storage.Storage{
//		Filename: "t.json",
//	}
//	s.Set("foo", "bar")
//	fmt.Println(s.Get("foo"))
//}
