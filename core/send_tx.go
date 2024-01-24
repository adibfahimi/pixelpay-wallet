package core

import (
	"bytes"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"time"

	pcore "github.com/adibfahimi/pixelpay-node/core"
	"github.com/adibfahimi/pixelpay-wallet/wallet"
)

func SendTransaction(w *wallet.Wallet, amount uint, receiver string) {
	if amount > w.Balance {
		fmt.Println("you don't have enough balance")
		return
	}

	tx := pcore.Tx{
		From:      w.Address,
		To:        receiver,
		Signature: "",
		Hash:      "",
		Amount:    amount,
		Timestamp: uint(time.Now().Unix()),
	}

	tx.Hash = tx.CalculateHash()

	if err := w.SignTx(&tx); err != nil {
		log.Fatalf("an error occured while signing tx: %v", err)
	}

	body, err := json.Marshal(tx)
	if err != nil {
		log.Fatalf("an error occured while marshalling tx: %v", err)
	}

	resp, err := http.Post(fmt.Sprintf("%s/txs", w.NodeUrl), "application/json", bytes.NewBuffer(body))
	if err != nil {
		log.Fatalf("an error occured while getting balance: %v", err)
	}

	defer resp.Body.Close()

	// decode response
	var response map[string]interface{}
	if err := json.NewDecoder(resp.Body).Decode(&response); err != nil {
		log.Fatalf("an error occured while decoding response: %v", err)
	}

	if response["message"] == "tx added" {
		fmt.Println("transaction added successfully")
	} else {
		fmt.Println("transaction failed")
	}
}
