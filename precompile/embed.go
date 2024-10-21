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

// ReadStdlib return stdlib module bytes.
//
// If filters is empty, return all modules.
// Ex) filters: "code.move", "coin.move"
func ReadStdlib(filters ...string) ([][]byte, error) {
	if len(filters) == 0 {
		entries, err := files.ReadDir(StdlibPath)
		if err != nil {
			return nil, err
		}

		filters = make([]string, len(entries))
		for i, entry := range entries {
			filters[i] = entry.Name()
		}
	}

	modules := make([][]byte, len(filters))
	for i, entry := range filters {
		bz, err := files.ReadFile(path.Join(StdlibPath, entry))
		if err != nil {
			return nil, err
		}

		modules[i] = bz
	}

	return modules, nil
}

// ReadMinlib return minlib module bytes.
//
// If filters is empty, return all modules.
// Ex) filters: "code.move", "coin.move"
func ReadMinlib(filters ...string) ([][]byte, error) {
	if len(filters) == 0 {
		entries, err := files.ReadDir(MinlibPath)
		if err != nil {
			return nil, err
		}

		filters = make([]string, len(entries))
		for i, entry := range entries {
			filters[i] = entry.Name()
		}
	}

	modules := make([][]byte, len(filters))
	for i, entry := range filters {
		bz, err := files.ReadFile(path.Join(MinlibPath, entry))
		if err != nil {
			return nil, err
		}

		modules[i] = bz
	}

	return modules, nil
}
