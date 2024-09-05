package api

import (
	"errors"

	"cosmossdk.io/math"
	dbm "github.com/cosmos/cosmos-db"
	"github.com/initia-labs/movevm/types"
)

/*** Mock KVStore ****/

type Lookup struct {
	db *dbm.MemDB
}

func NewLookup() *Lookup {
	return &Lookup{
		db: dbm.NewMemDB(),
	}
}

// Get wraps the underlying DB's Get method panicing on error.
func (l Lookup) Get(key []byte) []byte {
	v, err := l.db.Get(key)
	if err != nil {
		panic(err)
	}

	return v
}

// Set wraps the underlying DB's Set method panicing on error.
func (l Lookup) Set(key, value []byte) {
	if err := l.db.Set(key, value); err != nil {
		panic(err)
	}
}

// Delete wraps the underlying DB's Delete method panicing on error.
func (l Lookup) Delete(key []byte) {
	if err := l.db.Delete(key); err != nil {
		panic(err)
	}
}

// Iterator wraps the underlying DB's Iterator method panicing on error.
func (l Lookup) Iterator(start, end []byte) dbm.Iterator {
	iter, err := l.db.Iterator(start, end)
	if err != nil {
		panic(err)
	}

	return iter
}

// ReverseIterator wraps the underlying DB's ReverseIterator method panicing on error.
func (l Lookup) ReverseIterator(start, end []byte) dbm.Iterator {
	iter, err := l.db.ReverseIterator(start, end)
	if err != nil {
		panic(err)
	}

	return iter
}

var _ KVStore = (*Lookup)(nil)

/***** Mock GoAPI ****/

const CanonicalLength = 32

const (
	CostTransfer uint64 = 100
)

var _ GoAPI = MockAPI{}

type MockAPI struct {
	AccountAPI *MockAccountAPI
	StakingAPI *MockStakingAPI
	QueryAPI   *MockQueryAPI
	OracleAPI  *MockOracleAPI
	BlockTime  uint64
}

func NewMockAPI(
	blockTime uint64,
	accountAPI *MockAccountAPI,
	stakingAPI *MockStakingAPI,
	queryAPI *MockQueryAPI,
	oracleAPI *MockOracleAPI,
) *MockAPI {

	return &MockAPI{
		AccountAPI: accountAPI,
		StakingAPI: stakingAPI,
		QueryAPI:   queryAPI,
		OracleAPI:  oracleAPI,
		BlockTime:  blockTime,
	}
}

func NewEmptyMockAPI(blockTime uint64) *MockAPI {
	accountAPI := NewMockAccountAPI()
	stakingAPI := NewMockStakingAPI()
	oracleAPI := NewMockOracleAPI()
	queryAPI := NewMockQueryAPI()
	return &MockAPI{
		AccountAPI: &accountAPI,
		StakingAPI: &stakingAPI,
		OracleAPI:  &oracleAPI,
		QueryAPI:   &queryAPI,
		BlockTime:  blockTime,
	}
}

func (m MockAPI) Query(request types.QueryRequest, gasBalance uint64) ([]byte, uint64, error) {
	return m.QueryAPI.Query(request, gasBalance)
}

func (m MockAPI) GetAccountInfo(addr types.AccountAddress) (bool, uint64, uint64, uint8, bool) {
	return m.AccountAPI.GetAccountInfo(addr)
}

func (m MockAPI) AmountToShare(validator []byte, metadata types.AccountAddress, amount uint64) (string, error) {
	return m.StakingAPI.AmountToShare(validator, metadata, amount)
}

func (m MockAPI) ShareToAmount(validator []byte, metadata types.AccountAddress, share string) (uint64, error) {
	return m.StakingAPI.ShareToAmount(validator, metadata, share)
}

func (m MockAPI) UnbondTimestamp() uint64 {
	return m.BlockTime + 60*60*24*7
}

func (m MockAPI) GetPrice(pairId string) ([]byte, uint64, uint64, error) {
	return m.OracleAPI.GetPrice(pairId)
}

type MockAccountAPI struct {
	accounts map[string][]uint64
}

// NewMockAccountAPI return MockAccountAPI instance
func NewMockAccountAPI() MockAccountAPI {
	return MockAccountAPI{
		accounts: make(map[string][]uint64),
	}
}

func (m *MockAccountAPI) SetAccountInfo(addr types.AccountAddress, accountNumber, sequence uint64, accountType uint8) {
	m.accounts[addr.String()] = []uint64{accountNumber, sequence, uint64(accountType)}
}

func (m MockAccountAPI) GetAccountInfo(addr types.AccountAddress) (bool, uint64, uint64, uint8, bool) {
	info, found := m.accounts[addr.String()]
	if found {
		return found, info[0], info[1], uint8(info[2]), false
	}

	return false, 0, 0, 0, false
}

type ShareAmountRatio struct {
	share  string
	amount uint64
}

type MockStakingAPI struct {
	validators map[string]map[types.AccountAddress]ShareAmountRatio
}

// NewMockStakingAPI return MockStakingAPI instance
func NewMockStakingAPI() MockStakingAPI {
	return MockStakingAPI{
		validators: make(map[string]map[types.AccountAddress]ShareAmountRatio),
	}
}

func (m *MockStakingAPI) SetShareRatio(validator []byte, metadata types.AccountAddress, share string, amount uint64) {
	if ratios, ok := m.validators[string(validator)]; ok {
		ratios[metadata] = ShareAmountRatio{share, amount}
	} else {
		m.validators[string(validator)] = make(map[types.AccountAddress]ShareAmountRatio)
		m.validators[string(validator)][metadata] = ShareAmountRatio{share, amount}
	}
}

func (m MockStakingAPI) AmountToShare(validator []byte, metadata types.AccountAddress, amount uint64) (string, error) {
	ratios, ok := m.validators[string(validator)]
	if !ok {
		return "0", errors.New("validator not found")
	}

	ratio, ok := ratios[metadata]
	if !ok {
		return "0", errors.New("metadata not found")
	}

	return math.LegacyMustNewDecFromStr(ratio.share).MulInt64(int64(amount)).QuoInt64(int64(ratio.amount)).String(), nil
}

func (m MockStakingAPI) ShareToAmount(validator []byte, metadata types.AccountAddress, share string) (uint64, error) {
	ratios, ok := m.validators[string(validator)]
	if !ok {
		return 0, errors.New("validator not found")
	}

	ratio, ok := ratios[metadata]
	if !ok {
		return 0, errors.New("metadata not found")
	}

	return math.LegacyMustNewDecFromStr(share).MulInt64(int64(ratio.amount)).Quo(math.LegacyMustNewDecFromStr(ratio.share)).TruncateInt().Uint64(), nil
}

type MockQueryAPI struct {
	StargateQuerySet map[string][]byte
	CustomQuerySet   map[string][]byte
}

func NewMockQueryAPI() MockQueryAPI {
	q := MockQueryAPI{
		StargateQuerySet: make(map[string][]byte),
		CustomQuerySet:   make(map[string][]byte),
	}
	q.StargateQuerySet["/initia.gov.v1.Query/Proposal"] = []byte("{\"proposal\":{\"id\":0,\"title\":\"test_proposal\",\"summary\":\"test_proposal_summary\"}}")
	q.CustomQuerySet["amount_to_share"] = []byte("{\"share\": 0}")
	return q
}

func (m MockQueryAPI) Query(request types.QueryRequest, gasBalance uint64) ([]byte, uint64, error) {
	if request.Custom != nil {
		data, ok := m.CustomQuerySet[request.Custom.Name]
		if !ok {
			return nil, 0, errors.New("not registered custom function")
		}
		return data, 0, nil
	} else if request.Stargate != nil {
		data, ok := m.StargateQuerySet[request.Stargate.Path]
		if !ok {
			return nil, 0, errors.New("not registered stargate function")
		}
		return data, 0, nil
	}
	return nil, 0, nil
}

type MockOracleAPI struct {
	prices map[string][]uint64
}

// NewMockOracleAPI return MockOracleAPI instance
func NewMockOracleAPI() MockOracleAPI {
	return MockOracleAPI{
		prices: make(map[string][]uint64),
	}
}

func (m *MockOracleAPI) SetPrice(pairId string, price, updatedAt, decimals uint64) {
	m.prices[pairId] = []uint64{price, updatedAt, decimals}
}

func (m MockOracleAPI) GetPrice(pairId string) ([]byte, uint64, uint64, error) {
	info, found := m.prices[pairId]
	if !found {
		return nil, 0, 0, errors.New("pair not found")
	}

	priceBz, err := types.SerializeUint256(0, 0, 0, info[0])
	if err != nil {
		return nil, 0, 0, err
	}

	return priceBz, info[1], info[2], nil
}
