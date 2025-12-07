package bot

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"regexp"
	"strconv"
	"time"

	"github.com/bwmarrin/discordgo"
)

const URL string = "https://open.faceit.com/data/v4/players?"

type FaceitData struct {
	Games struct {
		Cs2 struct {
			Faceit_Elo  int64  `json:"faceit_elo"`
			Region      string `json:"region"`
			Skill_level int64  `json:"skill_level"`
		} `json:"cs2"`
	} `json:"games"`
	Nickname  string `json:"nickname"`
	Player_ID string `json:"player_id"`
	Country   string `json:"country"`
}

func getCurrentUser(message string) *discordgo.MessageSend {
	r, _ := regexp.Compile(`\w{3,}`)
	user := r.FindString(message)

	if user == "" {
		return &discordgo.MessageSend{
			Content: "Sorry, that username doesn't look right",
		}
	}

	faceitURL := fmt.Sprintf("%snickname=%s", URL, user)

	// new HTTP client & timeout
	client := http.Client{Timeout: 5 * time.Second}

	response, err := client.Get(faceitURL)
	if err != nil {
		return &discordgo.MessageSend{
			Content: "Sorry, there was an error trying to get stats (Possibly API key issue?).",
		}
	}

	// HTTP response body
	body, _ := io.ReadAll(response.Body)
	defer response.Body.Close()

	var data FaceitData
	json.Unmarshal([]byte(body), &data)

	// pull out info
	username := data.Nickname
	elo := strconv.FormatInt(data.Games.Cs2.Faceit_Elo, 10)
	faceitlvl := strconv.FormatInt(data.Games.Cs2.Skill_level, 10)
	region := data.Games.Cs2.Region
	usercountry := data.Country

	// embed response
	embed := &discordgo.MessageSend{
		Embeds: []*discordgo.MessageEmbed{{
			Type:        discordgo.EmbedTypeRich,
			Title:       "User Stats",
			Description: "Stats for " + username,
			Fields: []*discordgo.MessageEmbedField{
				{
					Name:   "Elo",
					Value:  elo + "elo",
					Inline: true,
				},
				{
					Name:   "Level",
					Value:  "Level" + faceitlvl,
					Inline: true,
				},
				{
					Name:   "Region",
					Value:  region,
					Inline: true,
				},
				{
					Name:   "Country",
					Value:  usercountry,
					Inline: true,
				},
			},
		},
		},
	}

	return embed
}
