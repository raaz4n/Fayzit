package bot

import (
	"fmt"
	"log"
	"os"
	"os/signal"
	"strings"

	"github.com/bwmarrin/discordgo"
)

// API tokens
var (
	FaceitToken string
	BotToken    string
)

func Run() {
	// new discord session
	discord, err := discordgo.New("Bot " + BotToken)
	if err != nil {
		log.Fatal(err)
	}

	// event handler
	discord.AddHandler(newMessage)

	// Open sesh
	discord.Open()
	defer discord.Close()

	// run code until termination
	fmt.Println("Bot is running")
	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt)
	<-c
}

func newMessage(discord *discordgo.Session, message *discordgo.MessageCreate) {
	// ignore bot messages
	if message.Author.ID == discord.State.User.ID {
		return
	}

	switch {
	case strings.HasPrefix(message.Content, "!stats"):
		faceitUser := getCurrentStats(message.Content)
		discord.ChannelMessageSendComplex(message.ChannelID, faceitUser)
	}
}
