package wallet

import (
	"crypto/ecdsa"
	"encoding/json"
	"log"
	"os"

	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/crypto"
)

type Wallet struct {
	PrivateKey string `json:"privateKey"`
	NodeUrl    string `json:"nodeUrl"`
	Address    string `json:"address"`
	Balance    uint   `json:"balance"`
}

func generateWallet() Wallet {
	privateKey, err := crypto.GenerateKey()
	if err != nil {
		log.Fatal(err)
	}

	privateKeyBytes := crypto.FromECDSA(privateKey)

	publicKey := privateKey.Public()
	publicKeyECDSA, ok := publicKey.(*ecdsa.PublicKey)
	if !ok {
		log.Fatal("cannot assert type: publicKey is not of type *ecdsa.PublicKey")
	}

	address := crypto.PubkeyToAddress(*publicKeyECDSA).Hex()

	w := Wallet{
		PrivateKey: hexutil.Encode(privateKeyBytes),
		NodeUrl:    "http://127.0.0.1:3000",
		Address:    address,
		Balance:    0,
	}

	w.SaveWallet()

	return w
}

func LoadWallet() Wallet {
	_, err := os.Stat("./wallet.json")

	if os.IsNotExist(err) {
		return generateWallet()
	}

	file, err := os.Open("./wallet.json")
	if err != nil {
		log.Fatalf("failed opening config file: %s", err)
	}
	defer file.Close()

	decoder := json.NewDecoder(file)

	walllet := Wallet{}
	if err := decoder.Decode(&walllet); err != nil {
		log.Fatalf("failed to decode config file: %s", err)
	}

	privateKeyBytes, err := hexutil.Decode(walllet.PrivateKey)
	if err != nil {
		log.Fatal(err)
	}

	privateKey, err := crypto.ToECDSA(privateKeyBytes)
	if err != nil {
		log.Fatal(err)
	}

	publicKey := privateKey.Public()

	publicKeyECDSA, ok := publicKey.(*ecdsa.PublicKey)
	if !ok {
		log.Fatal("cannot assert type: publicKey is not of type *ecdsa.PublicKey")
	}

	address := crypto.PubkeyToAddress(*publicKeyECDSA).Hex()

	walllet.Address = address

	return walllet
}

func (w *Wallet) SaveWallet() {
	file, err := os.Create("./wallet.json")
	if err != nil {
		log.Fatalf("failed creating file: %s", err)
	}
	defer file.Close()

	encoder := json.NewEncoder(file)
	encoder.SetIndent("", "\t")
	if err := encoder.Encode(w); err != nil {
		log.Fatalf("failed to encode config file: %s", err)
	}
}
