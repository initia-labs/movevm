package precompile

import (
	"embed"
	"path"
)

//go:embed binaries/*
var files embed.FS

const (
	StdlibPath       = "binaries/stdlib"
	MinlibPath       = "binaries/minlib"
	CoinTypeFileName = "binaries/types/coin_type.mv"
	NftTypeFileName  = "binaries/types/nft_type.mv"
	SftTypeFileName  = "binaries/types/sft_type.mv"
)

// ReadCoinType return coin_type module bytes
func ReadCoinType() ([]byte, error) {
	return files.ReadFile(CoinTypeFileName)
}

// ReadNftType return nft_type_module bytes
func ReadNftType() ([]byte, error) {
	return files.ReadFile(NftTypeFileName)
}

// ReadSftType return nft_type_module bytes
func ReadSftType() ([]byte, error) {
	return files.ReadFile(SftTypeFileName)
}

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
