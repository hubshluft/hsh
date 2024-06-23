package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"os/exec"
	"os/signal"
	"os/user"
	"strings"
	"syscall"
)

func main() {
	reader := bufio.NewReader(os.Stdin)

	for {
		path, err := os.Getwd()
		if err != nil {
			fmt.Println(err)
		}

		user, err := user.Current()
		if err != nil {
			fmt.Println(err)
		}

		// --- THEME ---
		fmt.Print(defaultTheme(path, user.Username))
		// --- THEME ---

		input, err := reader.ReadString('\n')
		if err != nil {
			if err == io.EOF {
				fmt.Println("EOF")
				os.Exit(1)
			}
			fmt.Fprintln(os.Stderr, err)
		} else {
			if err = execInput(input); err != nil {
				fmt.Fprintln(os.Stderr, err)
			}
		}
	}
}

func execInput(input string) error {
	input = strings.TrimSuffix(input, "\n")

	args := strings.Split(input, " ")

	switch args[0] {
	case "cd":
		if len(args) < 2 {
			user, err := user.Current()
			if err != nil {
				fmt.Println(err)
			}

			return os.Chdir(user.HomeDir)
		}
		return os.Chdir(args[1])
	case "..":
		return os.Chdir("..")
	case "exit":
		os.Exit(0)
	}

	// --- PLUGINS ---
	if gitPlugin(args) {
		return nil
	}

	if goPlugin(args) {
		return nil
	}
	// --- PLUGINS ---

	cmd := exec.Command(args[0], args[1:]...)

	cmd.Stderr = os.Stderr
	cmd.Stdout = os.Stdout

	return cmd.Run()
}

func init() {
	c := make(chan os.Signal)
	signal.Notify(c, os.Interrupt, syscall.SIGTERM)
	go func() {
		<-c
	}()
}
