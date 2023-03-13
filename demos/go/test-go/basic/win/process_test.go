package win

import "testing"

func TestRestartExe(t *testing.T) {
	RestartExe("C:\\Windows\\System32\\notepad.exe")
}

func TestFindProcessId(t *testing.T) {
	pid, err := findProcessId("notepad.exe")
	if err != nil {
		t.Error(err)
	}
	t.Log(pid)
}
