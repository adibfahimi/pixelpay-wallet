package wallet

import (
	pcore "github.com/adibfahimi/pixelpay-node/core"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/crypto"
)

func (w *Wallet) SignTx(tx *pcore.Tx) error {
	privateKeyBytes, err := hexutil.Decode(w.PrivateKey)
	if err != nil {
		return err
	}

	privateKey, err := crypto.ToECDSA(privateKeyBytes)
	if err != nil {
		return err
	}

	hashBytes, err := hexutil.Decode(tx.Hash)
	if err != nil {
		return err
	}

	signature, err := crypto.Sign(hashBytes, privateKey)
	if err != nil {
		return err
	}

	tx.Signature = hexutil.Encode(signature)

	return nil
}
