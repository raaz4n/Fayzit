package bot

import (
	"fmt"
	"log"
	"os"
	"os/signal"

	"github.com/bwmarrin/discordgo"
)

// variable declarations
var (
	GuildID     string
	FaceitToken string
	BotToken    string
	commands    = []*discordgo.ApplicationCommand{
		{
			Name:        "stats",
			Description: "Get a users FACEIT stats",
			Options: []*discordgo.ApplicationCommandOption{
				{
					Type:        discordgo.ApplicationCommandOptionString,
					Name:        "faceit-username",
					Description: "Search for the user by FACEIT username",
					Required:    false,
				},
				{
					Type:        discordgo.ApplicationCommandOptionString,
					Name:        "steam-id",
					Description: "Search for the user by Steam ID",
					Required:    false,
				},
			},
		},
	}
	commandHandlers = map[string]func(discord *discordgo.Session, message *discordgo.InteractionCreate){
		"stats": func(discord *discordgo.Session, message *discordgo.InteractionCreate) {
			stats := message.ApplicationCommandData().Options

			statsMap := make(map[string]*discordgo.ApplicationCommandInteractionDataOption, len(stats))
			for _, opt := range stats {
				statsMap[opt.Name] = opt
			}

			var searchType string
			var username string
			if option, ok := statsMap["faceit-username"]; ok {
				username = option.StringValue()
				searchType = "faceit-username"
			}

			if opt, ok := statsMap["steam-id"]; ok {
				username = opt.StringValue()
				searchType = "steam-id"
			}

			statsMsg := getCurrentStats(username, searchType)
			discord.InteractionRespond(message.Interaction, &discordgo.InteractionResponse{
				Type: discordgo.InteractionResponseChannelMessageWithSource,
				Data: &discordgo.InteractionResponseData{
					Content: statsMsg.Content,
					Embeds:  statsMsg.Embeds,
				},
			})
		},
	}
)

func Run() {
	// new discord session
	discord, err := discordgo.New("Bot " + BotToken)
	if err != nil {
		log.Fatal(err)
	}

	discord.AddHandler(func(discord *discordgo.Session, message *discordgo.InteractionCreate) {
		if h, ok := commandHandlers[message.ApplicationCommandData().Name]; ok {
			h(discord, message)
		}
	})

	// Open sesh
	discord.Open()

	regCommands := make([]*discordgo.ApplicationCommand, len(commands))
	for i, v := range commands {
		cmd, err := discord.ApplicationCommandCreate(discord.State.User.ID, GuildID, v)
		if err != nil {
			log.Panic("Cannot execute command")
		}
		regCommands[i] = cmd
	}

	defer discord.Close()

	// run code until termination
	fmt.Println("Bot is running")
	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt)
	<-c
}
