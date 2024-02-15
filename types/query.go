package types

// QueryRequest is an rust enum and only (exactly) one of the fields should be set
type QueryRequest struct {
	Custom   *CustomQuery   `json:"custom,omitempty"`
	Stargate *StargateQuery `json:"stargate,omitempty"`
}

type CustomQuery struct {
	// function name,
	// eg. amount_to_share
	Name string `json:"name"`
	Data []byte `json:"data"`
}

// StargateQuery is encoded the same way as abci_query, with path and protobuf encoded request data.
// The format is defined in [ADR-21](https://github.com/cosmos/cosmos-sdk/blob/master/docs/architecture/adr-021-protobuf-query-encoding.md).
// The response is protobuf encoded data directly without a JSON response wrapper.
// The caller is responsible for compiling the proper protobuf definitions for both requests and responses.
type StargateQuery struct {
	// this is the fully qualified service path used for routing,
	// eg. custom/cosmos_sdk.x.bank.v1.Query/QueryBalance
	Path string `json:"path"`
	// this is the expected protobuf message type (not any), binary encoded
	Data []byte `json:"data"`
}
