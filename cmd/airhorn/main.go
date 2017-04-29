package main

import (
	"io/ioutil"
	"os"
	"os/exec"
)

func main() {
	// temporarily write
	f, err := ioutil.TempFile(os.TempDir(), "airhorn")
	if err != nil {
		panic(err)
	}
	defer os.Remove(f.Name())
	if _, err := f.Write(airhorn); err != nil {

	}

	// play sound
	cmd := exec.Command("afplay", f.Name())
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	if err := cmd.Run(); err != nil {
		panic(err)
	}
}
