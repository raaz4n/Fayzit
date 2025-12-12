package main

import (
	"Fayzit-discord-bot/bot"
	"log"
	"os"

	"github.com/joho/godotenv"
)

func main() {
	// Load .env file
	err := godotenv.Load()
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	// env. variables
	guildID := os.Getenv("GUILDID")
	botToken := os.Getenv("TOKEN")
	faceitToken := os.Getenv("FACEIT")

	bot.GuildID = guildID
	bot.BotToken = botToken
	bot.FaceitToken = faceitToken
	bot.Run()
}
