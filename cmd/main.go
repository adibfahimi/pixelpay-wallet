package main

import (
	"flag"
	"fmt"

	"github.com/adibfahimi/pixelpay-wallet/core"
	"github.com/adibfahimi/pixelpay-wallet/wallet"
)

func main() {
	isSend := flag.Bool("send", false, "Send command")
	isBalance := flag.Bool("balance", false, "balance command")
	receiver := flag.String("receiver", "", "Set receiver address")
	amount := flag.Uint("amount", 0, "Set amount to send")
	nodeAddress := flag.String("node", "http://localhost:3000", "Set node address")

	flag.Parse()

	wallet := wallet.LoadWallet(*nodeAddress)

	core.GetBalance(&wallet)

	if *isBalance {
		fmt.Printf("Your balance is: %d PXL\n", wallet.Balance)
	} else if *isSend {
		core.SendTransaction(&wallet, *amount, *receiver)
	} else {
		fmt.Println("Command not found :(")
	}
}
