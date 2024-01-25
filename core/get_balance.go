package core

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"

	"github.com/adibfahimi/pixelpay-wallet/wallet"
)

type balanceResponse struct {
	Data struct {
		Amount float64 `json:"amount"`
	} `json:"data"`
}

func GetBalance(w *wallet.Wallet) {
	url := fmt.Sprintf("%s/balance/%s", w.NodeUrl, w.Address)
	resp, err := http.Get(url)
	if err != nil {
		log.Fatalf("an error occured while getting balance: %v", err)
	}

	defer resp.Body.Close()

	var res balanceResponse

	if err := json.NewDecoder(resp.Body).Decode(&res); err != nil {
		log.Fatalf("an error occured while decoding response: %v", err)
	}

	w.Balance = uint(res.Data.Amount)
}
