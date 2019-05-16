package util

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"os"

	"github.com/logrusorgru/aurora"
	"github.com/sirupsen/logrus"
)

type ByteManager struct {
	Input    []byte
	Pos      int
	Filename string
	Little   bool
}

func NewBytes(filepath string, mode string) *ByteManager {
	var f *os.File
	var err error
	switch mode {
	case "r":
		f, err = os.OpenFile(filepath, os.O_RDONLY, 0644)
		if err != nil {
			logrus.Errorf("%+v", err)
		}
	case "w":
		f, err = os.OpenFile(filepath, os.O_RDWR|os.O_CREATE, 0755)
		if err != nil {
			logrus.Errorf("%+v", err)
		}
	case "a":
		f, err = os.OpenFile(filepath, os.O_APPEND, 0755)
		if err != nil {
			logrus.Errorf("%+v", err)
		}
	default:
		logrus.Errorf("no such an filemode")
	}
	b := &ByteManager{}
	b.Filename = filepath
	b.Input, err = ioutil.ReadAll(f)
	if err != nil {
		logrus.Errorf("%+v", err)
	}
	b.Pos = 0
	return b
}

func (b *ByteManager) Find(idx int) byte {
	return b.Input[idx]
}

func (b *ByteManager) Range(src, dst int) []byte {
	return b.Input[src:dst]
}
func (b *ByteManager) Check(orig, ano []byte) bool {
	return bytes.Equal(orig, ano)
}

func (b *ByteManager) WriteRange(src, dst int, content []byte) {
	b.Input = bytes.ReplaceAll(b.Input, b.Input[src:dst], content)
}

func (b *ByteManager) Flush() error {
	f, err := os.OpenFile(b.Filename, os.O_WRONLY, 0755)
	if err != nil {
		return err
	}
	var buf bytes.Buffer
	_, err = buf.Write(b.Input)
	if err != nil {
		return err
	}
	_, err = buf.WriteTo(f)
	if err != nil {
		return err
	}
	return nil
}

func ColorString(content string, color string) string {
	switch color {
	case "blue":
		return fmt.Sprintf("%s", aurora.Bold(aurora.BrightBlue(content)))
	case "red":
		return fmt.Sprintf("%s", aurora.Bold(aurora.BrightRed(content)))
	case "green":
		return fmt.Sprintf("%s", aurora.Bold(aurora.BrightGreen(content)))
	}
	return ""
}
