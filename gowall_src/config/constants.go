package config

import (
	"os"
	"path/filepath"
)

const (
	Version            = "v0.2.4"
	OutputFolder       = "Pictures/gowall"
	configFile         = "config.yml"
	OCRSchemaFile      = "schema.yml"
	WallOfTheDayUrl    = "https://old.reddit.com/r/wallpaper/top/"
	RepoUrl            = "https://github.com/Apepen/gowall"
	UpscalerBinaryName = "realesrgan-ncnn-vulkan"
	PngquantBinaryName = "pngquant"
	EnvFilePath        = ".gowall/.env"
	OnnxRuntimeVersion = "1.24.4"
)

var (
	EnableImagePreviewingDefault = true
	InlineImagePreviewDefault    = false
	ImagePreviewBackend          = ""
	ThemesDefault                = []themeWrapper{}
	OnnxRuntimeFolderPath        = OnnxRuntimeFolder()
	OnnxModelFolderPath          = OnnxRuntimeFolder()
)

func defaultConfig() Options {
	return Options{
		EnableImagePreviewing: EnableImagePreviewingDefault,
		Themes:                ThemesDefault,
		InlineImagePreview:    InlineImagePreviewDefault,
		ImagePreviewBackend:   ImagePreviewBackend,
		EnvFilePath:           EnvFilePath,
		OnnxRuntimeFolderPath: OnnxRuntimeFolderPath,
		OnnxModelFolderPath:   OnnxModelFolderPath,
	}
}

func OnnxRuntimeFolder() string {
	homeDir, _ := os.UserHomeDir()
	return filepath.Join(homeDir, ".u2net")
}
