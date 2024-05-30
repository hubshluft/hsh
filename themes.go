package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

const (
	Bold   = "\033[1m"
	Red    = "\033[31m"
	Purple = "\033[35m"
	Blue   = "\033[34m"
	Reset  = "\033[0m"
)

func defaultTheme(path string, user string) string {
	file, err := os.Open(path + "/.git/")
	if err != nil {
		return Blue + strings.ReplaceAll(string(path), "/home/"+user, "~") + " > " + Reset
	}

	defer file.Close()

	read, err := ioutil.ReadFile(path + "/.git/HEAD")
	return Blue + strings.ReplaceAll(string(path), "/home/"+user, "~") + Bold + " [GIT: " + strings.TrimSpace(strings.ReplaceAll(string(read), "ref: refs/heads/", "")) + "]" + Reset + " > " + Reset

}

func minimalTheme(path string, user string) string {
	return strings.ReplaceAll(string(path), "/home/"+user, "~") + " $ "
}

func fishTheme(path string, user string) string {
	hostname, err := os.Hostname()
	if err != nil {
		fmt.Println(err)
	}
	return Purple + user + Reset + "@" + hostname + " " + Purple + strings.ReplaceAll(string(path), "/home/"+user, "~") + " > " + Reset
}

func appleTheme(path string, user string) string {
	file, err := os.Open(path + "/.git/")
	if err != nil {
		return " " + strings.ReplaceAll(string(path), "/home/"+user, "~") + " [$] "
	}

	defer file.Close()
	return " " + strings.ReplaceAll(string(path), "/home/"+user, "~") + Bold + " [GIT]" + Reset + " [$] "
}
