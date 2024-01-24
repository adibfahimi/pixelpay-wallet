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

	flag.Parse()

	if *isBalance {
		w := wallet.LoadWallet()
		core.GetBalance(&w)
	} else if *isSend {

		w := wallet.LoadWallet()
		core.GetBalance(&w)
		core.SendTransaction(&w, *amount, *receiver)

	} else {
		fmt.Println("Command not found :(")
	}
}
