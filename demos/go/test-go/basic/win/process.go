package win

import (
	"fmt"
	"os/exec"
	"path/filepath"
	"strings"
)

func RestartExe(exePath string) {
	err := killProcess(filepath.Base(exePath))
	if err != nil {
		fmt.Println("Error:", err)
		return
	}

	// 启动新的进程
	cmd := exec.Command(exePath)
	err = cmd.Start()
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
}

func killProcess(processName string) error {
	// 查找进程 ID
	pid, err := findProcessId(processName)
	if err != nil {
		return err
	}

	// 结束进程
	cmd := exec.Command("taskkill", "/f", "/pid", fmt.Sprintf("%d", pid))
	err = cmd.Run()
	if err != nil {
		return err
	}

	return nil
}

func findProcessId(processName string) (int, error) {
	// 运行 tasklist 命令并查找进程 ID
	output, err := exec.Command("tasklist", "/nh", "/fi", fmt.Sprintf("imagename eq %s", processName)).Output()
	if err != nil {
		return 0, err
	}

	// iterates over the lines
	for _, line := range strings.Split(string(output), "\n") {
		if strings.Contains(line, processName) {
			var pid int
			// splits the line into columns
			columns := strings.Fields(line)
			if len(columns) > 1 {
				fmt.Sscanf(columns[1], "%d", &pid)
				return pid, nil
			}
		}
	}

	return 0, fmt.Errorf("process not found")
}
