package main

import (
	"fmt"
	"log"
	"os"
	"os/signal"
	"syscall"

	"github.com/bwmarrin/discordgo"
	"github.com/joho/godotenv"
)

func e() {
	enverr := godotenv.Load()
	if enverr != nil {
		panic(enverr)
	}

	discToken := os.Getenv("TOKEN")
	newString := fmt.Sprintf("Bot %s", discToken)

	sesh, err := discordgo.New(newString)
	if err != nil {
		log.Fatal(err)
	}

	sesh.AddHandler(func(s *discordgo.Session, m *discordgo.MessageCreate) {
		if m.Author.ID == s.State.User.ID {
			return
		}

		if m.Content == "hello" {
			s.ChannelMessageSend(m.ChannelID, "world")
		}
	})

	sesh.Identify.Intents = discordgo.IntentsAllWithoutPrivileged

	err = sesh.Open()
	if err != nil {
		log.Fatal(err)
	}
	defer sesh.Close()

	fmt.Println("Online!")

	sc := make(chan os.Signal, 1)
	signal.Notify(sc, syscall.SIGINT, syscall.SIGTERM, os.Interrupt)
	<-sc
}
