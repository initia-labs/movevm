package types

import (
	"github.com/aptos-labs/serde-reflection/serde-generate/runtime/golang/bcs"
	"github.com/aptos-labs/serde-reflection/serde-generate/runtime/golang/serde"
)

var NewSerializer = bcs.NewSerializer
var NewDeserializer = bcs.NewDeserializer

// SerializeBytes serialize bytes to BCS bytes
func SerializeBytes(bz []byte) ([]byte, error) {
	s := NewSerializer()
	err := s.SerializeBytes(bz)
	if err != nil {
		return nil, err
	}

	return s.GetBytes(), nil
}

// DeserializeBytes deserialize BCS bytes to string
func DeserializeBytes(bz []byte) ([]byte, error) {
	s := NewDeserializer(bz)
	return s.DeserializeBytes()
}

// SerializeString serialize string to BCS bytes
func SerializeString(str string) ([]byte, error) {
	s := NewSerializer()
	if err := s.SerializeStr(str); err != nil {
		return nil, err
	}

	return s.GetBytes(), nil
}

// DeserializeString deserialize BCS bytes to string
func DeserializeString(bz []byte) (string, error) {
	s := NewDeserializer(bz)
	return s.DeserializeStr()
}

// SerializeBool serialize bool to BCS bytes
func SerializeBool(str bool) ([]byte, error) {
	s := NewSerializer()
	if err := s.SerializeBool(str); err != nil {
		return nil, err
	}

	return s.GetBytes(), nil
}

// DeserializeBool deserialize BCS bytes to bool
func DeserializeBool(bz []byte) (bool, error) {
	s := NewDeserializer(bz)
	return s.DeserializeBool()
}

// SerializeUint64 serialize num to BCS bytes
func SerializeUint64(num uint64) ([]byte, error) {
	s := NewSerializer()
	err := s.SerializeU64(num)
	if err != nil {
		return nil, err
	}
	return s.GetBytes(), nil
}

// DeserializeUint64 deserialize BCS bytes
func DeserializeUint64(bz []byte) (uint64, error) {
	d := NewDeserializer(bz)
	return d.DeserializeU64()
}

// SerializeUint128 serialize num to BCS bytes
func SerializeUint128(high, low uint64) ([]byte, error) {
	s := NewSerializer()
	err := s.SerializeU128(serde.Uint128{
		High: high,
		Low:  low,
	})
	if err != nil {
		return nil, err
	}
	return s.GetBytes(), nil
}

// SerializeUint256 serialize num to BCS bytes
func SerializeUint256(hh, hl, h, l uint64) ([]byte, error) {
	s := NewSerializer()
	err := s.SerializeU128(serde.Uint128{
		Low:  l,
		High: h,
	})
	if err != nil {
		return nil, err
	}
	err = s.SerializeU128(serde.Uint128{
		Low:  hl,
		High: hh,
	})
	if err != nil {
		return nil, err
	}
	return s.GetBytes(), nil
}

// DeserializeUint128 deserialize BCS bytes
func DeserializeUint128(bz []byte) (uint64, uint64, error) {
	d := NewDeserializer(bz)
	num, err := d.DeserializeU128()
	if err != nil {
		return 0, 0, err
	}

	return num.High, num.Low, nil
}

// SerializeAddressVector serialize address vector to BCS bytes
func SerializeAddressVector(addrs []AccountAddress) ([]byte, error) {
	s := NewSerializer()
	if err := s.SerializeLen(uint64(len(addrs))); err != nil {
		return nil, err
	}

	bcsBz := s.GetBytes()
	for _, item := range addrs {
		bcsBz = append(bcsBz, item[:]...)
	}

	return bcsBz, nil
}

// SerializeBytesVector serialize bytes vector to BCS bytes
func SerializeBytesVector(bz [][]byte) ([]byte, error) {
	s := NewSerializer()
	if err := s.SerializeLen(uint64(len(bz))); err != nil {
		return nil, err
	}

	for _, item := range bz {
		if err := s.SerializeBytes(item); err != nil {
			return nil, err
		}
	}

	return s.GetBytes(), nil
}

// DeserializeBytesVector deserialize BCS bytes to bytes vector
func DeserializeBytesVector(bz []byte) ([][]byte, error) {
	d := NewDeserializer(bz)
	len, err := d.DeserializeLen()
	if err != nil {
		return nil, err
	}

	resBytesVector := make([][]byte, len)
	for i := uint64(0); i < len; i++ {
		bz, err := d.DeserializeBytes()
		if err != nil {
			return nil, err
		}

		resBytesVector[i] = bz
	}

	return resBytesVector, nil
}

// SerializeStringVector serialize bytes vector to BCS bytes
func SerializeStringVector(bz []string) ([]byte, error) {
	s := NewSerializer()
	if err := s.SerializeLen(uint64(len(bz))); err != nil {
		return nil, err
	}

	for _, item := range bz {
		if err := s.SerializeStr(item); err != nil {
			return nil, err
		}
	}

	return s.GetBytes(), nil
}

// DeserializeStringVector deserialize BCS bytes to bytes vector
func DeserializeStringVector(bz []byte) ([]string, error) {
	d := NewDeserializer(bz)
	len, err := d.DeserializeLen()
	if err != nil {
		return nil, err
	}

	resStringVector := make([]string, len)
	for i := uint64(0); i < len; i++ {
		str, err := d.DeserializeStr()
		if err != nil {
			return nil, err
		}

		resStringVector[i] = str
	}

	return resStringVector, nil
}

// SerializeUint64Vector serialize bytes vector to BCS bytes
func SerializeUint64Vector(bz []uint64) ([]byte, error) {
	s := NewSerializer()
	if err := s.SerializeLen(uint64(len(bz))); err != nil {
		return nil, err
	}

	for _, item := range bz {
		if err := s.SerializeU64(item); err != nil {
			return nil, err
		}
	}

	return s.GetBytes(), nil
}

// DeserializeUint64Vector deserialize BCS bytes to bytes vector
func DeserializeUint64Vector(bz []byte) ([]uint64, error) {
	d := NewDeserializer(bz)
	len, err := d.DeserializeLen()
	if err != nil {
		return nil, err
	}

	resUint64Vector := make([]uint64, len)
	for i := uint64(0); i < len; i++ {
		num, err := d.DeserializeU64()
		if err != nil {
			return nil, err
		}

		resUint64Vector[i] = num
	}

	return resUint64Vector, nil
}
