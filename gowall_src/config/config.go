package config

import (
	"errors"
	"log"
	"os"
	"path/filepath"

	"gopkg.in/yaml.v3"
)

var SupportedImageExtensions = map[string]bool{
	".png":  true,
	".jpeg": true,
	".jpg":  true,
	".webp": true,
	".avif": true,
}

var SupportedTextExtensions = map[string]bool{
	".pdf": true,
}

type GlobalSubCommandFlags struct {
	OutputDestination string
	InputDir          string
	InputFiles        []string
	Format            string
	PreviewFlag       string
	Yes               bool
}

type themeWrapper struct {
	Name   string   `yaml:"name"`
	Colors []string `yaml:"colors"`
}

type Options struct {
	EnableImagePreviewing  bool           `yaml:"EnableImagePreviewing"`
	InlineImagePreview     bool           `yaml:"InlineImagePreview"`
	ImagePreviewBackend    string         `yaml:"ImagePreviewBackend"`
	ColorCorrectionBackend string         `yaml:"ColorCorrectionBackend"`
	OutputFolder           string         `yaml:"OutputFolder"`
	Themes                 []themeWrapper `yaml:"themes"`
	EnvConfig              *EnvConfig
	EnvFilePath            string `yaml:"EnvFilePath"`
	OnnxRuntimeFolderPath  string `yaml:"OnnxRuntimeFolderPath"`
	OnnxModelFolderPath    string `yaml:"OnnxModelFolderPath"`
}

func (o *Options) Resolve() error {
	var err error

	o.EnvFilePath, err = ResolveHomePath(o.EnvFilePath)
	if err != nil {
		return err
	}

	o.OutputFolder, err = ResolveHomePath(o.OutputFolder)
	if err != nil {
		return err
	}

	o.OnnxRuntimeFolderPath, err = ResolveHomePath(o.OnnxRuntimeFolderPath)
	if err != nil {
		return err
	}

	o.OnnxModelFolderPath, err = ResolveHomePath(o.OnnxModelFolderPath)
	if err != nil {
		return err
	}

	return nil
}

var GowallConfig = defaultConfig()
var GlobalFlags GlobalSubCommandFlags

func LoadConfig() {
	configDir, err := os.UserHomeDir()
	if err != nil {
		log.Fatalf("Error: Could not get home directory")
	}

	configPath := filepath.Join(configDir, ".config", "gowall", configFile)
	configFolder := filepath.Dir(configPath)

	err = os.MkdirAll(configFolder, 0755)
	if err != nil {
		log.Fatalf("could not create config directory %v", err)
	}

	if _, err := os.Stat(filepath.Join(configDir, ".config", "gowall", configFile)); errors.Is(err, os.ErrNotExist) {
		err = os.WriteFile(filepath.Join(configDir, ".config", "gowall", configFile), []byte(""), 0644)
		if err != nil {
			log.Fatalf("could not create config file %v", err)
		}
	}

	data, err := os.ReadFile(configPath)
	if err != nil {
		log.Printf("Error reading config file: %v", err)
		return
	}

	err = yaml.Unmarshal(data, &GowallConfig)
	if err != nil {
		log.Printf("Error unmarshalling config file: %v", err)
		return
	}

	if err := GowallConfig.Resolve(); err != nil {
		log.Printf("Error resolving config: %v", err)
		return
	}

	defaultDir, err := CreateDirectory()
	if err != nil {
		log.Fatalf("Error: Could not create output directories: %v", err)
	}
	GowallConfig.OutputFolder = defaultDir
	GowallConfig.EnvConfig = GetEnvConfig(GowallConfig.EnvFilePath)
}
