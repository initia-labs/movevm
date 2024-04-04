package precompile

import (
	"embed"
	"path"
)

//go:embed binaries/*
var files embed.FS

const (
	StdlibPath = "binaries/stdlib"
	MinlibPath = "binaries/minlib"
)

// ReadStdlib return stdlib module bytes
func ReadStdlib() ([][]byte, error) {
	entries, err := files.ReadDir(StdlibPath)
	if err != nil {
		return nil, err
	}

	modules := make([][]byte, len(entries))
	for i, entry := range entries {
		bz, err := files.ReadFile(path.Join(StdlibPath, entry.Name()))
		if err != nil {
			return nil, err
		}

		modules[i] = bz
	}

	return modules, nil
}

// ReadMinlib return minlib module bytes
func ReadMinlib() ([][]byte, error) {
	entries, err := files.ReadDir(MinlibPath)
	if err != nil {
		return nil, err
	}

	modules := make([][]byte, len(entries))
	for i, entry := range entries {
		bz, err := files.ReadFile(path.Join(MinlibPath, entry.Name()))
		if err != nil {
			return nil, err
		}

		modules[i] = bz
	}

	return modules, nil
}
