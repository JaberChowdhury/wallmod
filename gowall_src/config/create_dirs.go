package config

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

func CreateDirectory() (dirPath string, err error) {
	folderName := GowallConfig.OutputFolder
	if folderName == "" {
		folderName = OutputFolder
	}
	dirPath, err = ResolveHomePath(folderName)
	if err != nil {
		return "", err
	}

	// Handle XDG_PICTURES_DIR
	env := os.Getenv("XDG_PICTURES_DIR")
	if env != "" && GowallConfig.OutputFolder == "" {
		dirPath = filepath.Join(env, "gowall")
	}

	// Ensure all required directories exist
	subDirs := []string{"cluts", "gifs", "ocr"}
	for _, sub := range subDirs {
		subDir := filepath.Join(dirPath, sub)
		err = os.MkdirAll(subDir, 0755)
		if err != nil {
			return "", fmt.Errorf("while creating %s: %w", subDir, err)
		}
	}

	return dirPath, nil
}

func ExpandTilde(paths []string) []string {
	var expandedPaths []string
	homeDir, _ := os.UserHomeDir()

	for _, path := range paths {
		if strings.HasPrefix(path, "~") {
			path = filepath.Join(homeDir, path[1:])
		}

		expandedPaths = append(expandedPaths, path)
	}

	return expandedPaths
}

func ResolveHomePath(path string) (string, error) {
	expandedPath := ExpandTilde([]string{path})[0]
	if expandedPath == "" || filepath.IsAbs(expandedPath) {
		return expandedPath, nil
	}

	homeDir, err := os.UserHomeDir()
	if err != nil {
		return "", err
	}

	return filepath.Join(homeDir, expandedPath), nil
}
