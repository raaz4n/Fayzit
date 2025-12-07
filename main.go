package main

import (
	"Fayzit-discord-bot/bot"
	"log"
	"os"
)

func main() {
	// env. variables
	botToken, exist := os.LookupEnv("TOKEN")
	if !exist {
		log.Fatal("Must set Discord bot token as environment variable: TOKEN")
	}
	faceitToken, exist := os.LookupEnv("FACEIT")
	if !exist {
		log.Fatal("Must set FACEIT API token as environment variable: FACEIT ")
	}

	bot.BotToken = botToken
	bot.FaceitToken = faceitToken
	bot.Run()
}
