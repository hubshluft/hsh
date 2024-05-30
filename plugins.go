package main

import (
	"fmt"
	"os"
	"os/exec"
)

func gitPlugin(args []string) bool {
	switch args[0] {
	case "gc":
		cmd := exec.Command("git", append([]string{"clone"}, args[1:]...)...)
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
		err := cmd.Run()
		if err != nil {
			fmt.Println(err)
		} else {
			return true
		}
	}
	return false
}

func goPlugin(args []string) bool {
	switch args[0] {
	case "gr":
		cmd := exec.Command("go", append([]string{"run"}, args[1:]...)...)
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
		err := cmd.Run()
		if err != nil {
			fmt.Println(err)
		} else {
			return true
		}

	case "gb":
		cmd := exec.Command("go", "build", ".")
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
		err := cmd.Run()
		if err != nil {
			fmt.Println("Error:", err)
		} else {
			return true
		}
	}
	return false
}
